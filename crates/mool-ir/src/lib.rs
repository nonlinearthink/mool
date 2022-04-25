mod ast;
mod parser;
pub use parser::mool_parser::program as parse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
