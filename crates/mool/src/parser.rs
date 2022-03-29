use super::ast;

peg::parser! {
    pub grammar python_parser() for str{
        use ast::Expr;
        use ast::IConstant;
        pub rule program() -> Vec<Expr> = e:(expression() ** "\n") "\n"* {e}
        pub rule expression() -> Expr = precedence!{
            l:identifier() whitespace() "=" whitespace() r:expression() whitespace() semicolon() {
                Expr::Assign(l, Box::new(r))
            }
            --
            l:(@) whitespace() "+" whitespace() r:@ { Expr::Add(Box::new(l), Box::new(r)) }
            l:(@) whitespace() "-" whitespace() r:@ { Expr::Sub(Box::new(l), Box::new(r)) }
            --
            l:(@) whitespace() "*" whitespace() r:@ { Expr::Mul(Box::new(l), Box::new(r)) }
            l:(@) whitespace() "/" whitespace() r:@ { Expr::Div(Box::new(l), Box::new(r)) }
            --
            // x:@ "^" y:(@) { x.pow(y as u32) }
            // --
            il:identifier_or_literal() { il }
            // "(" e:expression() ")" { e }
        }
        rule identifier_or_literal() -> Expr =
            i:identifier() { Expr::Ref(i) }
            / int_literal()
            / string_literal()
            / float_literal()
            / bool_literal()
        rule identifier() -> String =
            p:position!() not_keyword() id:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) { id.to_owned() }
        rule int_literal() -> Expr = p:position!() n:$(['0'..='9']+) !"." {
            match n.parse::<i64>(){
                Ok(t) => { Expr::Constant(IConstant::Int(t)) },
                Err(e) => { panic!("{}无法解析为整数类型", p) }
            }
        }
        rule string_literal() -> Expr = ['\'' | '"'] s:[^'\'' | '"']* ['\'' | '"'] {
            Expr::Constant(IConstant::Str(s.into_iter().collect()))
        }
        rule float_literal() -> Expr = p:position!() n:$(['0'..='9']+"."['0'..='9']*) {
            match n.parse::<f64>() {
                Ok(t) => { Expr::Constant(IConstant::Float(t)) },
                Err(e) => { panic!("{}无法解析为浮点类型", p) }
            }
        }
        rule bool_literal() -> Expr = b:$("True" / "False") {
            Expr::Constant(IConstant::Bool(if b == "True" {true} else {false}))
        }
        rule whitespace() = quiet!{ [' ' | '\t']* }
        rule semicolon() = quiet!{ [';']* }
        rule not_keyword() = !(
            "and"/"as"/"assert"/"async"/"await"
            /"break"/"class"/"continue"/"def"/"del"
            /"elif"/"else"/"except"/"False"/"finally"
            /"for"/"from"/"global"/"if"/"import"
            /"in"/"is"/"lambda"/"None"/"nonlocal"
            /"not"/"or"/"pass"/"raise"/"return"
            /"True"/"try"/"while"/"with"/"yield")
    }
}
