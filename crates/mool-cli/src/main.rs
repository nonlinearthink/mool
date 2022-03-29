use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mool", about = "An Machine Learning Compiler.")]
struct Opt {
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
    #[structopt(short, long, default_value = "llvm", help = "Compile Target")]
    target: String,

    /// Show Compilation Process
    #[structopt(short, long, help = "Show Compilation Process")]
    debug: bool,
}

fn main() {
    // 获取配置
    let opt = Opt::from_args();
    let mut current_filename: String;
    // let mut current_code: String;
    // 设置全局变量 FILENAME
    for file in opt.input.into_iter() {
        current_filename = file.into_os_string().into_string().unwrap();
        println!("{:?}", current_filename);
        // 读取代码
        let mut code = String::new();
        let mut f = File::open(current_filename).unwrap();
        f.read_to_string(&mut code).unwrap();
        // 设置全局变量 CODE
        // current_code = code.clone();
        // 生成抽象语法树
        let python_ast = mool::parser::python_parser::program(&code).unwrap();
        // 输出抽象语法树
        if opt.debug {
            println!("AST:");
            println!("AST:{}", serde_json::to_string(&python_ast).unwrap());
            println!();
        }
        // println!("{}", current_code);
    }
}
