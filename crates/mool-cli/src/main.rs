use once_cell::sync::OnceCell;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mool", about = "An Machine Learning Compiler.")]
struct Opt {
    // Input File
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    // Output File
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

// 全局变量，可在所有文件中共享
// static CODE: OnceCell<String> = OnceCell::new();
static FILENAME: OnceCell<String> = OnceCell::new();

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    FILENAME.set("example/test.py".to_string()).unwrap();
}
