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
            match scope.get(&variable.name) {
                Some(alloca) => {
                    llvm::core::LLVMBuildStore(builder, value, alloca);
                    scope.register(variable.name, alloca);
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
    mut scope: &mut Scope,
    expr: ast::Expr,
) -> llvm::prelude::LLVMValueRef {
    match expr {
        ast::Expr::Literal(literal) => codegen_literal(context, builder, scope, literal),
        ast::Expr::Assign(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, builder, scope, *expr);
            // 检查作用域内变量，如果存在就更新值，如果不存在就报错
            match scope.get(&variable.name) {
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
        ast::Expr::Operator(operator) => match operator {
            ast::Operator::Tensor(tensors) => {
                let mut tensor: Vec<llvm::prelude::LLVMValueRef> = tensors
                    .into_iter()
                    .map(|x| codegen_literal(context, builder, scope, x))
                    .collect();
                let element_type = llvm::core::LLVMTypeOf(*tensor.get(0).unwrap());
                llvm::core::LLVMConstArray(element_type, tensor.as_mut_ptr(), tensor.len() as u32)
            }
            ast::Operator::Add(x, y) => {
                // 创建 Add 作用域
                scope.push();
                // 分配 Add 算子的参数
                let x_value = codegen_expr(context, builder, scope, *x);
                let x_type = llvm::core::LLVMTypeOf(x_value);
                let x_alloca =
                    llvm::core::LLVMBuildAlloca(builder, x_type, b"x\0".as_ptr() as *const _);
                scope.register("x".to_string(), x_alloca);
                llvm::core::LLVMBuildStore(builder, x_value, x_alloca);
                let y_value = codegen_expr(context, builder, scope, *y);
                let y_type = llvm::core::LLVMTypeOf(y_value);
                let y_alloca =
                    llvm::core::LLVMBuildAlloca(builder, y_type, b"y\0".as_ptr() as *const _);
                scope.register("y".to_string(), y_alloca);
                llvm::core::LLVMBuildStore(builder, y_value, y_alloca);
                // 获取最大长度
                let x_length = llvm::core::LLVMGetArrayLength(x_type);
                let y_length = llvm::core::LLVMGetArrayLength(y_type);
                let min_length;
                let max_length;
                let max_length_type = if x_length >= y_length {
                    min_length = y_length;
                    max_length = x_length;
                    llvm::core::LLVMTypeOf(x_value)
                } else {
                    min_length = x_length;
                    max_length = y_length;
                    llvm::core::LLVMTypeOf(y_value)
                };
                // 分配 Add 算子返回值, 最大长度即为返回的张量长度
                let return_type = max_length_type;
                let return_alloca =
                    llvm::core::LLVMBuildAlloca(builder, return_type, b"z\0".as_ptr() as *const _);
                scope.register("z".to_string(), return_alloca);
                // 准备数据
                let int_type = llvm::core::LLVMInt64TypeInContext(context);
                let mut indice;
                let mut indices;
                // 遍历张量并运算
                for i in 0..max_length {
                    indice = llvm::core::LLVMConstInt(int_type, i as u64, 0);
                    indices = vec![indice];
                    let x_temp = llvm::core::LLVMBuildInBoundsGEP(
                        builder,
                        x_alloca,
                        indices.as_mut_ptr(),
                        indices.len() as u32,
                        b"x_temp\0".as_ptr() as *const _,
                    );
                    // let x_temp_address = llvm::core::LLVMBuildCast(
                    //     builder,
                    //     llvm::LLVMOpcode::LLVMLoad,
                    //     x_temp,
                    //     int_type,
                    //     b"x_temp_address\0".as_ptr() as *const _,
                    // );
                    let x_temp = llvm::core::LLVMBuildLoad(
                        builder,
                        x_temp,
                        b"x_temp_value\0".as_ptr() as *const _,
                    );
                    let y_temp = llvm::core::LLVMBuildInBoundsGEP(
                        builder,
                        y_alloca,
                        indices.as_mut_ptr(),
                        indices.len() as u32,
                        b"y_temp\0".as_ptr() as *const _,
                    );
                    // let y_temp_address = llvm::core::LLVMBuildPtrToInt(
                    //     builder,
                    //     y_temp,
                    //     int_type,
                    //     b"x_temp_address\0".as_ptr() as *const _,
                    // );
                    let y_temp = llvm::core::LLVMBuildLoad(
                        builder,
                        y_temp,
                        b"y_temp_value\0".as_ptr() as *const _,
                    );
                    let add_temp = llvm::core::LLVMBuildAdd(
                        builder,
                        x_temp,
                        y_temp,
                        b"add_temp\0".as_ptr() as *const _,
                    );
                    let z_temp = llvm::core::LLVMBuildInBoundsGEP(
                        builder,
                        return_alloca,
                        indices.as_mut_ptr(),
                        indices.len() as u32,
                        b"z_temp\0".as_ptr() as *const _,
                    );
                    llvm::core::LLVMBuildStore(builder, add_temp, z_temp);
                }
                // 清理 Add 作用域
                scope.pop();
                // 返回值
                return_alloca
            }
            _ => {
                let int_type = llvm::core::LLVMInt64TypeInContext(context);
                let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
                zero
            }
        },
        _ => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
            zero
        }
    }
}

unsafe fn codegen_literal(
    context: llvm::prelude::LLVMContextRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    literal: ast::Literal,
) -> llvm::prelude::LLVMValueRef {
    match literal {
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
    }
}
