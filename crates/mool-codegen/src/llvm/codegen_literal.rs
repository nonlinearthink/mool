use llvm_sys as llvm;
use mool_ir::ast;

pub unsafe fn codegen_literal(
    context: llvm::prelude::LLVMContextRef,
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
