use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Read;
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

    /// Output File
    #[structopt(short, long, parse(from_os_str), help = "Output File")]
    output: Option<PathBuf>,

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
        let mut f = File::open(current_filename).unwrap();
        f.read_to_string(&mut code).unwrap();
        // 编译
        match &opt.source as &str {
            "torchscript" => {
                compile_torchscript(&code);
            }
            _ => {
                println!("暂不支持编译{}", opt.source)
            }
        }
    }
}

fn compile_torchscript(code: &String) {
    let python_ast = mool::torchscript::parse(&code).unwrap();
    // 输出抽象语法树
    match DEBUG.get() {
        Some(&debug) => {
            if debug {
                println!("AST:\n{}\n", serde_json::to_string(&python_ast).unwrap());
            }
        }
        None => panic!("未运行初始化"),
    }
}
