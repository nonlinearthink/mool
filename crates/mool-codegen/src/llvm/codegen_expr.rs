use super::super::scope::Scope;
use super::codegen_literal::codegen_literal;
use super::codegen_operator::codegen_operator;
use super::codegen_program::codegen_program;
use llvm_sys as llvm;
use mool_ir::ast;
use regex::Regex;
use std::vec::Vec;

pub unsafe fn codegen_expr(
    context: llvm::prelude::LLVMContextRef,
    module: llvm::prelude::LLVMModuleRef,
    builder: llvm::prelude::LLVMBuilderRef,
    block: llvm::prelude::LLVMBasicBlockRef,
    scope: &mut Scope,
    expr: ast::Expr,
) -> llvm::prelude::LLVMValueRef {
    match expr {
        ast::Expr::Literal(literal) => codegen_literal(context, literal),
        ast::Expr::Assign(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, module, builder, block, scope, *expr);
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
        ast::Expr::Operator(operator) => {
            codegen_operator(context, module, builder, block, scope, operator)
        }
        ast::Expr::Function(function) => {
            // 获取函数返回值
            let return_type = mool_type_ref(context, module, builder, function.rtn.clone());
            // 生成参数类型列表
            let mut arg_types: Vec<llvm::prelude::LLVMTypeRef> = Vec::new();
            for arg in function.args.iter() {
                arg_types.push(mool_type_ref(
                    context,
                    module,
                    builder,
                    arg.annotation.clone(),
                ));
            }
            // 创建函数
            let function_type = llvm::core::LLVMFunctionType(
                return_type,
                arg_types.as_mut_ptr(),
                arg_types.len() as u32,
                0,
            );
            let func = llvm::core::LLVMAddFunction(
                module,
                b"function\0".as_ptr() as *const _,
                function_type,
            );
            // 创建 Add 作用域
            scope.push();
            // 注册形参
            for i in 0..arg_types.len() {
                let value = llvm::core::LLVMGetParam(func, i as u32);
                scope.register(function.args[i].arg.name.clone(), value);
            }
            // 创建BasicBlock
            let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                func,
                b"function_entry\0".as_ptr() as *const _,
            );
            // 重置 builder 的位置
            llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
            // 设置默认返回值
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let mut return_value = llvm::core::LLVMConstInt(int_type, 0, 0);
            // 解析函数体，获取返回值
            for program in function.body {
                return_value =
                    codegen_program(context, module, builder, basic_block, scope, program);
            }
            // 构造返回值
            llvm::core::LLVMBuildRet(builder, return_value);
            // 弹出 Add 作用域
            scope.pop();
            llvm::core::LLVMPositionBuilderAtEnd(builder, block);
            // 返回函数
            func
        }
        ast::Expr::Variable(variable) => match scope.get(&variable.name) {
            Some(value) => value,
            None => panic!("没有找到变量"),
        },
        ast::Expr::Call(name, exprs) => {
            match scope.get(&name) {
                Some(func) => {
                    let mut real_args = Vec::new();
                    for expr in exprs {
                        real_args.push(codegen_expr(context, module, builder, block, scope, expr));
                    }
                    llvm::core::LLVMBuildCall(
                        builder,
                        func,
                        real_args.as_mut_ptr(),
                        2,
                        b"result\0".as_ptr() as *const _,
                    );
                }
                None => panic!("没有找到变量"),
            }
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
            zero
        }
    }
}

unsafe fn mool_type_ref(
    context: llvm::prelude::LLVMContextRef,
    module: llvm::prelude::LLVMModuleRef,
    builder: llvm::prelude::LLVMBuilderRef,
    ty: String,
) -> llvm::prelude::LLVMTypeRef {
    match ty.as_str() {
        // Int 返回值
        "int" => llvm::core::LLVMInt64TypeInContext(context),
        // Bool 返回值
        "bool" => llvm::core::LLVMInt1TypeInContext(context),
        // Double 返回值
        "float" => llvm::core::LLVMDoubleTypeInContext(context),
        // 默认返回值0
        _ => {
            let re = Regex::new(r"Tensor\[\((\d+)\),(int|float|bool)\]").unwrap();
            match re.captures(ty.as_str()) {
                None => llvm::core::LLVMInt64TypeInContext(context),
                Some(cap) => llvm::core::LLVMVectorType(
                    mool_type_ref(context, module, builder, cap[2].to_string()),
                    cap[1].parse().unwrap(),
                ),
            }
        }
    }
}
