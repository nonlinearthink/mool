// use scope::Scope;
use super::scope::Scope;
use llvm_sys as llvm;
use mool_ir::ast;
use std::ffi::CStr;
use std::ptr;

pub unsafe fn codegen(programs: Vec<ast::Program>) -> String {
    // 创建context、module、builder、names
    let context = llvm::core::LLVMContextCreate();
    let module = llvm::core::LLVMModuleCreateWithName(b"example_moddule\0".as_ptr() as *const _);
    let builder = llvm::core::LLVMCreateBuilderInContext(context);
    let mut scope = Scope::new();

    // 创建main函数
    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function =
        llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

    // 创建BasicBlock
    let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
        context,
        function,
        b"entry\0".as_ptr() as *const _,
    );
    llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);

    // 根据AST生成代码
    for program in programs {
        codegen_program(context, builder, &mut scope, program);
    }

    // 设置 main 函数默认返回值 0
    let default_return = llvm::core::LLVMConstInt(int_type, 0, 0);
    llvm::core::LLVMBuildRet(builder, default_return);

    // 保存 LLVM IR 代码
    let module_string: String = CStr::from_ptr(llvm::core::LLVMPrintModuleToString(module))
        .to_str()
        .unwrap()
        .to_owned();

    // 清理
    llvm::core::LLVMDisposeBuilder(builder);
    llvm::core::LLVMDisposeModule(module);
    llvm::core::LLVMContextDispose(context);

    // 返回 LLVM IR 代码
    module_string
}

unsafe fn codegen_program(
    context: llvm::prelude::LLVMContextRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    program: ast::Program,
) {
    match program {
        ast::Program::Expr(expr) => {
            codegen_expr(context, builder, scope, expr);
        }
        ast::Program::Let(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, builder, scope, expr);
            // 检查作用域内变量，如果存在就更新值，如果不存在就创建值
            match scope.get(variable.name.clone()) {
                Some(alloca) => {
                    llvm::core::LLVMBuildStore(builder, value, alloca);
                    scope.register(variable.name, value);
                }
                None => {
                    let alloca = llvm::core::LLVMBuildAlloca(
                        builder,
                        llvm::core::LLVMTypeOf(value),
                        variable.name.as_ptr() as *const _,
                    );
                    llvm::core::LLVMBuildStore(builder, value, alloca);
                    scope.register(variable.name, alloca);
                }
            }
        }
    }
}

unsafe fn codegen_expr(
    context: llvm::prelude::LLVMContextRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    expr: ast::Expr,
) -> llvm::prelude::LLVMValueRef {
    match expr {
        ast::Expr::Literal(literal) => match literal {
            ast::Literal::Int(int_literal) => {
                let int_type = llvm::core::LLVMInt64TypeInContext(context);
                llvm::core::LLVMConstInt(int_type, int_literal as u64, 0)
            }
            ast::Literal::Float(float_literal) => {
                let float_type = llvm::core::LLVMDoubleTypeInContext(context);
                llvm::core::LLVMConstReal(float_type, float_literal)
            }
            ast::Literal::Bool(bool_literal) => {
                let bool_type = llvm::core::LLVMInt1TypeInContext(context);
                llvm::core::LLVMConstInt(bool_type, bool_literal as u64, 0)
            }
        },
        ast::Expr::Assign(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, builder, scope, *expr);
            // 检查作用域内变量，如果存在就更新值，如果不存在就报错
            match scope.get(variable.name.clone()) {
                Some(alloca) => {
                    llvm::core::LLVMBuildStore(builder, value, alloca);
                    scope.register(variable.name, value);
                    value
                }
                None => {
                    panic!("变量不存在")
                }
            }
        }
        _ => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
            zero
        }
    }
}
