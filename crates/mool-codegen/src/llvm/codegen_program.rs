use super::super::scope::Scope;
use super::codegen_expr::codegen_expr;
use llvm_sys as llvm;
use mool_ir::ast;

pub unsafe fn codegen_program(
    context: llvm::prelude::LLVMContextRef,
    module: llvm::prelude::LLVMModuleRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    program: ast::Program,
) {
    match program {
        ast::Program::Expr(expr) => {
            codegen_expr(context, module, builder, scope, expr);
        }
        ast::Program::Let(variable, expr) => {
            // 获取右值
            let value = codegen_expr(context, module, builder, scope, expr);
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
