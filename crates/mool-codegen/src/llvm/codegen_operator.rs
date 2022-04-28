use super::super::scope::Scope;
use super::codegen_expr::codegen_expr;
use super::codegen_literal::codegen_literal;
use llvm_sys as llvm;
use mool_ir::ast;
use regex::Regex;
use std::ffi::CStr;

pub unsafe fn codegen_operator(
    context: llvm::prelude::LLVMContextRef,
    module: llvm::prelude::LLVMModuleRef,
    builder: llvm::prelude::LLVMBuilderRef,
    scope: &mut Scope,
    operator: ast::Operator,
) -> llvm::prelude::LLVMValueRef {
    match operator {
        ast::Operator::Tensor(tensors) => {
            let mut tensor: Vec<llvm::prelude::LLVMValueRef> = tensors
                .into_iter()
                .map(|x| codegen_literal(context, x))
                .collect();
            llvm::core::LLVMConstVector(tensor.as_mut_ptr(), tensor.len() as u32)
        }
        ast::Operator::Add(x, y) => {
            // 构建 Add 的实参
            let x_value = codegen_expr(context, module, builder, scope, *x);
            let x_type = llvm::core::LLVMTypeOf(x_value);
            let y_value = codegen_expr(context, module, builder, scope, *y);
            let y_type = llvm::core::LLVMTypeOf(y_value);
            // 创建 Add 函数
            let mut arg_types = vec![x_type, y_type];
            let function_type = llvm::core::LLVMFunctionType(x_type, arg_types.as_mut_ptr(), 2, 0);
            let add =
                llvm::core::LLVMAddFunction(module, b"add\0".as_ptr() as *const _, function_type);
            // 创建 Add 作用域
            scope.push();
            // 调用 Add 函数
            let mut real_args = vec![x_value, y_value];
            llvm::core::LLVMBuildCall(
                builder,
                add,
                real_args.as_mut_ptr(),
                2,
                b"result\0".as_ptr() as *const _,
            );
            // 注册形参
            let x_value = llvm::core::LLVMGetParam(add, 0);
            scope.register("x".to_string(), x_value);
            let y_value = llvm::core::LLVMGetParam(add, 1);
            scope.register("x".to_string(), y_value);
            // 创建BasicBlock
            let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                add,
                b"add_entry\0".as_ptr() as *const _,
            );
            // 重置 builder 的位置
            llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
            // 判断张量类型是否相等
            if x_type != y_type {
                panic!("张量加法中张量类型必须相等")
            }
            // 分配 Add 算子返回值, 最大长度即为返回的张量长度
            let return_type = x_type;
            let return_alloca = llvm::core::LLVMBuildAlloca(
                builder,
                return_type,
                b"return_alloca\0".as_ptr() as *const _,
            );
            let add_temp = llvm::core::LLVMBuildAdd(
                builder,
                x_value,
                y_value,
                b"add_temp\0".as_ptr() as *const _,
            );
            llvm::core::LLVMBuildStore(builder, add_temp, return_alloca);
            llvm::core::LLVMBuildRet(
                builder,
                llvm::core::LLVMBuildLoad(
                    builder,
                    return_alloca,
                    b"return_value\0".as_ptr() as *const _,
                ),
            );
            // 弹出 Add 作用域
            scope.pop();
            // 返回值
            return_alloca
        }
        ast::Operator::Sub(x, y) => {
            // 构建 Sub 的实参
            let x_value = codegen_expr(context, module, builder, scope, *x);
            let x_type = llvm::core::LLVMTypeOf(x_value);
            let y_value = codegen_expr(context, module, builder, scope, *y);
            let y_type = llvm::core::LLVMTypeOf(y_value);
            // 创建 Sub 函数
            let mut arg_types = vec![x_type, y_type];
            let function_type = llvm::core::LLVMFunctionType(x_type, arg_types.as_mut_ptr(), 2, 0);
            let sub =
                llvm::core::LLVMAddFunction(module, b"sub\0".as_ptr() as *const _, function_type);
            // 创建 Sub 作用域
            scope.push();
            // 调用 Sub 函数
            let mut real_args = vec![x_value, y_value];
            llvm::core::LLVMBuildCall(
                builder,
                sub,
                real_args.as_mut_ptr(),
                2,
                b"result\0".as_ptr() as *const _,
            );
            // 注册形参
            let x_value = llvm::core::LLVMGetParam(sub, 0);
            scope.register("x".to_string(), x_value);
            let y_value = llvm::core::LLVMGetParam(sub, 1);
            scope.register("x".to_string(), y_value);
            // 创建BasicBlock
            let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                sub,
                b"sub_entry\0".as_ptr() as *const _,
            );
            // 重置 builder 的位置
            llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
            // 判断张量类型是否相等
            if x_type != y_type {
                panic!("张量减法中张量类型必须相等")
            }
            // 分配 Sub 算子返回值, 最大长度即为返回的张量长度
            let return_type = x_type;
            let return_alloca = llvm::core::LLVMBuildAlloca(
                builder,
                return_type,
                b"return_alloca\0".as_ptr() as *const _,
            );
            let sub_temp = llvm::core::LLVMBuildSub(
                builder,
                x_value,
                y_value,
                b"sub_temp\0".as_ptr() as *const _,
            );
            llvm::core::LLVMBuildStore(builder, sub_temp, return_alloca);
            llvm::core::LLVMBuildRet(
                builder,
                llvm::core::LLVMBuildLoad(
                    builder,
                    return_alloca,
                    b"return_value\0".as_ptr() as *const _,
                ),
            );
            // 弹出 Sub 作用域
            scope.pop();
            // 返回值
            return_alloca
        }
        ast::Operator::Mul(x, y) => {
            // 构建 Mul 的实参
            let x_value = codegen_expr(context, module, builder, scope, *x);
            let x_type = llvm::core::LLVMTypeOf(x_value);
            let y_value = codegen_expr(context, module, builder, scope, *y);
            let y_type = llvm::core::LLVMTypeOf(y_value);
            // 创建 Mul 函数
            let mut arg_types = vec![x_type, y_type];
            let function_type = llvm::core::LLVMFunctionType(x_type, arg_types.as_mut_ptr(), 2, 0);
            let mul =
                llvm::core::LLVMAddFunction(module, b"mul\0".as_ptr() as *const _, function_type);
            // 创建 Mul 作用域
            scope.push();
            // 调用 Mul 函数
            let mut real_args = vec![x_value, y_value];
            llvm::core::LLVMBuildCall(
                builder,
                mul,
                real_args.as_mut_ptr(),
                2,
                b"result\0".as_ptr() as *const _,
            );
            // 注册形参
            let x_value = llvm::core::LLVMGetParam(mul, 0);
            scope.register("x".to_string(), x_value);
            let y_value = llvm::core::LLVMGetParam(mul, 1);
            scope.register("x".to_string(), y_value);
            // 创建BasicBlock
            let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                mul,
                b"mul_entry\0".as_ptr() as *const _,
            );
            // 重置 builder 的位置
            llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
            // 判断张量类型是否相等
            if x_type != y_type {
                panic!("张量减法中张量类型必须相等")
            }
            // 分配 Mul 算子返回值, 最大长度即为返回的张量长度
            let return_type = x_type;
            let return_alloca = llvm::core::LLVMBuildAlloca(
                builder,
                return_type,
                b"return_alloca\0".as_ptr() as *const _,
            );
            let mul_temp = llvm::core::LLVMBuildMul(
                builder,
                x_value,
                y_value,
                b"mul_temp\0".as_ptr() as *const _,
            );
            llvm::core::LLVMBuildStore(builder, mul_temp, return_alloca);
            llvm::core::LLVMBuildRet(
                builder,
                llvm::core::LLVMBuildLoad(
                    builder,
                    return_alloca,
                    b"return_value\0".as_ptr() as *const _,
                ),
            );
            // 弹出 Mul 作用域
            scope.pop();
            // 返回值
            return_alloca
        }
        ast::Operator::Div(x, y) => {
            // 构建 Div 的实参
            let x_value = codegen_expr(context, module, builder, scope, *x);
            let x_type = llvm::core::LLVMTypeOf(x_value);
            let y_value = codegen_expr(context, module, builder, scope, *y);
            let y_type = llvm::core::LLVMTypeOf(y_value);
            // 创建 Div 函数
            let mut arg_types = vec![x_type, y_type];
            let function_type = llvm::core::LLVMFunctionType(x_type, arg_types.as_mut_ptr(), 2, 0);
            let div =
                llvm::core::LLVMAddFunction(module, b"div\0".as_ptr() as *const _, function_type);
            // 创建 Div 作用域
            scope.push();
            // 调用 Div 函数
            let mut real_args = vec![x_value, y_value];
            llvm::core::LLVMBuildCall(
                builder,
                div,
                real_args.as_mut_ptr(),
                2,
                b"result\0".as_ptr() as *const _,
            );
            // 注册形参
            let x_value = llvm::core::LLVMGetParam(div, 0);
            scope.register("x".to_string(), x_value);
            let y_value = llvm::core::LLVMGetParam(div, 1);
            scope.register("x".to_string(), y_value);
            // 创建BasicBlock
            let basic_block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                div,
                b"div_entry\0".as_ptr() as *const _,
            );
            // 重置 builder 的位置
            llvm::core::LLVMPositionBuilderAtEnd(builder, basic_block);
            // 判断张量类型是否相等
            if x_type != y_type {
                panic!("张量减法中张量类型必须相等")
            }
            // 分配 Div 算子返回值, 最大长度即为返回的张量长度
            let return_type = x_type;
            let return_alloca = llvm::core::LLVMBuildAlloca(
                builder,
                return_type,
                b"return_alloca\0".as_ptr() as *const _,
            );
            // 判断张量类型，如果是 int 和 bool 类型则使用 LLVMBuildUDiv，否则使用 LLVMBuildFDiv
            let type_string = CStr::from_ptr(llvm::core::LLVMPrintTypeToString(x_type))
                .to_str()
                .unwrap();
            let re = Regex::new(r"<(\d+) x (double|i64|i1)>").unwrap();
            let cap = re.captures(type_string).unwrap();
            let div_temp;
            match &cap[2] {
                "double" => {
                    div_temp = llvm::core::LLVMBuildFDiv(
                        builder,
                        x_value,
                        y_value,
                        b"div_temp\0".as_ptr() as *const _,
                    );
                }
                _ => {
                    div_temp = llvm::core::LLVMBuildUDiv(
                        builder,
                        x_value,
                        y_value,
                        b"div_temp\0".as_ptr() as *const _,
                    );
                }
            }
            llvm::core::LLVMBuildStore(builder, div_temp, return_alloca);
            llvm::core::LLVMBuildRet(
                builder,
                llvm::core::LLVMBuildLoad(
                    builder,
                    return_alloca,
                    b"return_value\0".as_ptr() as *const _,
                ),
            );
            // 弹出 Div 作用域
            scope.pop();
            // 返回值
            return_alloca
        }
    }
}
