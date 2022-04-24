use super::ast;

peg::parser! {
    pub grammar torchscript_parser() for str{
        use ast::Expr;
        use ast::Literal;
        pub rule program() -> Vec<Expr> = e:(expression() ** "\n") "\n" {e}
        rule expression() -> Expr = precedence!{
            il:identifier_or_literal() { il }
        }
        rule identifier_or_literal() -> Expr =
            int_literal()
            / float_literal()
            / bool_literal()
        rule int_literal() -> Expr = p:position!() n:$(['0'..='9']+) !"." {
            match n.parse::<i64>(){
                Ok(t) => { Expr::Literal(Literal::Int(t)) },
                Err(e) => { panic!("{}无法解析为整数类型", p) }
            }
        }
        rule float_literal() -> Expr = p:position!() n:$(['0'..='9']+"."['0'..='9']*) {
            match n.parse::<f64>() {
                Ok(t) => { Expr::Literal(Literal::Float(t)) },
                Err(e) => { panic!("{}无法解析为浮点类型", p) }
            }
        }
        rule bool_literal() -> Expr = b:$("True" / "False") {
            Expr::Literal(Literal::Bool(if b == "True" {true} else {false}))
        }
        rule ignore() = quiet!{ [' ' | '\t']* }
    }
}
