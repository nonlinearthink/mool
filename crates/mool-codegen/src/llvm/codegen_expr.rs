use super::super::scope::Scope;
use super::codegen_literal::codegen_literal;
use super::codegen_operator::codegen_operator;
use llvm_sys as llvm;
use mool_ir::ast;

pub unsafe fn codegen_expr(
    context: llvm::prelude::LLVMContextRef,
    module: llvm::prelude::LLVMModuleRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    expr: ast::Expr,
) -> llvm::prelude::LLVMValueRef {
    match expr {
        ast::Expr::Literal(literal) => codegen_literal(context, literal),
        ast::Expr::Assign(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, module, builder, scope, *expr);
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
            codegen_operator(context, module, builder, scope, operator)
        }
        _ => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);
            zero
        }
    }
}
