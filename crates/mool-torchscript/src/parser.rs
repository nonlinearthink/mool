use super::ast;

peg::parser! {
    pub grammar torchscript_parser() for str {
        use ast::{Program, FunctionReturn, FunctionArg, Statement, Expr, Literal, Operator};
        pub rule program() -> Program = "\n"* f:function() "\n"* { f }
        rule function() -> Program =
            "def" " " name:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) ig_space() "(" ig_line() args:function_args() ig_line() ")" 
                                                        ig_space() "->" ig_space() rt:$(['a'..='z' | 'A'..='Z' ]*) ig_space() ":" ig_space() "\n" e:function_body() {
                Program::Function(name.to_string(), args, FunctionReturn{rt: rt.to_string()}, e)
            }
        rule function_args() -> Vec<FunctionArg> = args:(function_arg() ** ",") ","? { args }
        rule function_arg() -> FunctionArg = ig_line() arg:$(['a'..='z' | 'A'..='Z' ]*) ig_line() ":" ig_line() annotation:$(['a'..='z' | 'A'..='Z' ]*) ig_line() {
            FunctionArg{arg: arg.to_string(), annotation: annotation.to_string()}
        }
        rule function_body() -> Vec<Statement> = e:(indent_statement() ** "\n") {e}
        rule indent_statement() -> Statement = "  " s:statement() { s }
        rule statement() -> Statement = 
            "return" " " e:expression() { Statement::Return(e) } 
            / e:expression() { Statement::Expr(e) }
        rule expression() -> Expr = literal() / operator() / identifier()
        rule literal() -> Expr = int_literal() / float_literal() / bool_literal()
        rule int_literal() -> Expr = p:position!() n:$(['0'..='9']+) !"." {
            match n.parse::<i64>(){
                Ok(t) => { Expr::Literal(Literal::Int(t)) },
                Err(e) => { panic!("{}:无法解析为整数类型", p) }
            }
        }
        rule float_literal() -> Expr = p:position!() n:$(['0'..='9']+"."['0'..='9']*) {
            match n.parse::<f64>() {
                Ok(t) => { Expr::Literal(Literal::Float(t)) },
                Err(e) => { panic!("{}:无法解析为浮点类型", p) }
            }
        }
        rule bool_literal() -> Expr = b:$("True" / "False") {
            Expr::Literal(Literal::Bool(if b == "True" { true } else { false }))
        }
        rule identifier() -> Expr = not_keyword() id:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) { Expr::Identifier(id.to_owned()) }
        rule operator() -> Expr =
            p:position!() "torch." op:$("add"/"sub"/"mul"/"div") ig_line() "(" ig_line() x:expression() ig_line() "," ig_line() y:expression() ig_line() ")" ig_space() {
                match op {
                    "add" => { Expr::Operator(Operator::Add(Box::new(x), Box::new(y))) },
                    "sub" => { Expr::Operator(Operator::Sub(Box::new(x), Box::new(y))) },
                    "mul" => { Expr::Operator(Operator::Mul(Box::new(x), Box::new(y))) },
                    "div" => { Expr::Operator(Operator::Div(Box::new(x), Box::new(y))) },
                    _ => { panic!("{}:暂不支持{}算子", p ,op) }
                }
            }
        rule not_keyword() = !(
            "and"/"as"/"assert"/"async"/"await"
            /"break"/"class"/"continue"/"def"/"del"
            /"elif"/"else"/"except"/"False"/"finally"
            /"for"/"from"/"global"/"if"/"import"
            /"in"/"is"/"lambda"/"None"/"nonlocal"
            /"not"/"or"/"pass"/"raise"/"return"
            /"True"/"try"/"while"/"with"/"yield")
        rule ig_space() = quiet!{ [' ' | '\t']* }
        rule ig_line() = quiet!{ [' ' | '\t' | '\n']* }
    }
}
