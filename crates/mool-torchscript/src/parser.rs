use super::ast;

peg::parser! {
    pub grammar torchscript_parser() for str {
        use ast::{Program, Function, FunctionArg, FunctionStatement, Expr, Literal, Operator};
        pub rule program() -> Vec<Program> = f:((function() / statement())*) { f }
        rule statement() -> Program = s:expression() ig_line() { Program::Statement(s) }
        rule function() -> Program =
            "def" " " name:identifier_str() ig_space() "(" ig_line() args:function_args() ig_line() ")" 
                                                        ig_space() "->" ig_space() rt:identifier_str() ig_space() ":" ig_space() "\n" e:function_body() ig_line() {
                Program::Function(Function{name:name.to_string(), args:args, rtn:rt.to_string(), body:e})
            }
        rule function_args() -> Vec<FunctionArg> = args:(function_arg() ** ",") ","? { args }
        rule function_arg() -> FunctionArg = ig_line() arg:identifier_str() ig_line() ":" ig_line() annotation:identifier_str() ig_line() {
            FunctionArg{name: arg.to_string(), annotation: annotation.to_string()}
        }
        rule function_body() -> Vec<FunctionStatement> = e:(function_statement() ** "\n") {e}
        rule function_statement() -> FunctionStatement = 
            indent() "return" ig_space() e:expression() { FunctionStatement::Return(e) } 
            / indent() e:expression() { FunctionStatement::Expr(e) }
        rule expression() -> Expr = literal() / operator() / call() / identifier()
        rule literal() -> Expr = int_literal() / float_literal() / bool_literal()
        rule int_literal() -> Expr = p:position!() ig_space() n:$(['0'..='9']+) !"." ig_space() {
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
        rule bool_literal() -> Expr = ig_space() b:$("True" / "False") ig_space(){
            Expr::Literal(Literal::Bool(if b == "True" { true } else { false }))
        }
        rule identifier() -> Expr = not_keyword() id:identifier_str() { Expr::Identifier(id) }
        rule identifier_str() -> String = id:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) { id.to_owned() }
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
            / ig_space() "torch.tensor" ig_line() "(" ig_line() t:tensor() ig_line() ")" ig_space() { Expr::Operator(Operator::Tensor(t)) }
        rule tensor() -> Vec<Literal> = "[" ig_line() t:(tensor_type()** ",") ig_line() "]" { t }
        rule tensor_type() -> Literal = 
            p:position!() ig_space() n:$(['0'..='9']+) !"." ig_space(){
                match n.parse::<i64>(){
                    Ok(t) => { Literal::Int(t) },
                    Err(e) => { panic!("{}:无法解析为整数类型", p) }
                }
            }
            / p:position!() ig_space() n:$(['0'..='9']+"."['0'..='9']*) ig_space(){
                match n.parse::<f64>() {
                    Ok(t) => { Literal::Float(t) },
                    Err(e) => { panic!("{}:无法解析为浮点类型", p) }
                }
            }
            / ig_space() b:$("True" / "False") ig_space(){
                Literal::Bool(if b == "True" { true } else { false })
            }
        rule call() -> Expr = id:identifier_str() ig_line() "(" ig_line() args:call_args() ig_line() ")" ig_space() {
            Expr::Call(id, args)
        }
        rule call_args() -> Vec<Expr> = args:(expression() ** ",") ","? { args }
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
        rule indent() = quiet!{ "  " }
    }
}
