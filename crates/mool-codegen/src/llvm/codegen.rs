use super::super::scope::Scope;
use super::codegen_program::codegen_program;
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
        codegen_program(context, module, builder, basic_block, &mut scope, program);
        // 重置 builder 的位置
        llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
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
