use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mool", about = "An Machine Learning Compiler.")]
pub struct Opt {
    /// Input File
    #[structopt(
        required = true,
        parse(from_os_str),
        multiple = true,
        help = "Input File"
    )]
    input: Vec<PathBuf>,

    /// Compile Target（llvm、wasm、cuda）
    #[structopt(short, long, default_value = "mool", help = "Compile Source")]
    source: String,

    /// Compile Target（torchscript、mool）
    #[structopt(short, long, default_value = "llvm", help = "Compile Target")]
    target: String,

    /// Show Compilation Process
    #[structopt(short, long, help = "Show Compilation Process")]
    debug: bool,
}

static DEBUG: OnceCell<bool> = OnceCell::new();

fn main() {
    // 获取配置
    let opt = Opt::from_args();
    let mut current_filename: String;
    // 全局保存配置信息
    DEBUG.set(opt.debug).unwrap();
    // 编译每一个文件
    for file in opt.input.into_iter() {
        // 从参数列表获取文件名
        current_filename = file.into_os_string().into_string().unwrap();
        println!(
            "\n\ncompiling {:?} from {:?} to {:?}\n",
            current_filename, opt.source, opt.target
        );
        // 读取代码
        let mut code = String::new();
        let mut f = File::open(current_filename.clone()).unwrap();
        f.read_to_string(&mut code).unwrap();
        // 编译
        match &opt.source as &str {
            "torchscript" => {
                let llvm_code = compile_torchscript(&code);
                let mut output_filename = current_filename.replace("torchscript", "llvm");
                output_filename.push_str(".ll");
                let mut f = File::create(output_filename).unwrap();
                f.write_all(llvm_code.as_bytes()).unwrap();
            }
            "mool" => {
                let llvm_code = compile_mool(&code);
                let mut f = File::create(
                    current_filename
                        .replace("mool", "llvm")
                        .replace(".llvm", ".ll"),
                )
                .unwrap();
                f.write_all(llvm_code.as_bytes()).unwrap();
            }
            _ => {
                println!("暂不支持编译{}", opt.source)
            }
        }
    }
}

fn compile_torchscript(code: &String) -> String {
    let torchscript_ast = mool::torchscript::parse(&code).unwrap();
    // 输出抽象语法树
    match DEBUG.get() {
        Some(&debug) => {
            if debug {
                println!(
                    "AST:\n{}\n",
                    serde_json::to_string(&torchscript_ast).unwrap()
                );
            }
        }
        None => panic!("未运行初始化"),
    }
    unsafe {
        let mool_code = mool::torchscript::codegen(torchscript_ast);
        match DEBUG.get() {
            Some(&debug) => {
                if debug {
                    println!("Mool:\n{}\n", mool_code);
                }
            }
            None => panic!("未运行初始化"),
        }
        compile_mool(&mool_code)
    }
}

fn compile_mool(code: &String) -> String {
    let mool_ast = mool::ir::parse(&code).unwrap();
    // 输出抽象语法树
    match DEBUG.get() {
        Some(&debug) => {
            if debug {
                println!("AST:\n{}\n", serde_json::to_string(&mool_ast).unwrap());
            }
        }
        None => panic!("未运行初始化"),
    }
    unsafe {
        let llvm_code = mool::codegen::llvm(mool_ast);
        match DEBUG.get() {
            Some(&debug) => {
                if debug {
                    println!("LLVM:\n{}\n", llvm_code);
                }
            }
            None => panic!("未运行初始化"),
        }
        llvm_code
    }
}
