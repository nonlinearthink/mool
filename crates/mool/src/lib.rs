pub mod ast;
pub mod parser;
pub use mool_torchscript as torchscript;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
