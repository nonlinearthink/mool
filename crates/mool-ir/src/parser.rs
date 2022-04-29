use super::ast;

peg::parser! {
    pub grammar mool_parser() for str {
        use ast::{Program, Variable, Expr, Literal, Function, FunctionArg, Operator};
        pub rule program() -> Vec<Program> =
            p:((expression_program() / let())*) { p }
        rule let() -> Program =
            "let" ig_space() name:variable() ig_space() "=" ig_line() e:expression() ig_line() {
                Program::Let(name, e)
            }
        rule expression_program() -> Program = ig_line() e:expression() ig_line() { Program::Expr(e) }
        rule expression() -> Expr =
            literal()
            / function()
            / operator()
            / call()
            / assign()
            / v:variable() { Expr::Variable(v) }
        rule assign() -> Expr = 
            ig_space() name:variable() ig_space() "=" ig_line() e:expression() ig_line() {
                Expr::Assign(name, Box::new(e))
            }
        rule literal() -> Expr = l:(int_literal() / float_literal() / bool_literal()) { Expr::Literal(l) }
        rule int_literal() -> Literal = p:position!() ig_space() n:$(['0'..='9']+) !"." ig_space() {
            match n.parse::<i64>(){
                Ok(t) => { Literal::Int(t) },
                Err(e) => { panic!("{}:无法解析为整数类型", p) }
            }
        }
        rule float_literal() -> Literal = p:position!() n:$(['0'..='9']+"."['0'..='9']*) {
            match n.parse::<f64>() {
                Ok(t) => { Literal::Float(t) },
                Err(e) => { panic!("{}:无法解析为浮点类型", p) }
            }
        }
        rule bool_literal() -> Literal = ig_space() b:$("true" / "false") ig_space(){
            Literal::Bool(if b == "true" { true } else { false })
        }
        rule function() -> Expr =
            "fn" ig_space() "(" ig_line() args:function_args() ig_line() ")" ig_space()
                "->" ig_space() rt:mool_type() ig_space() "{" ig_line() e:program() ig_line() "}" ig_line(){
                Expr::Function(Function{args:args, rtn:rt.to_string(), body:e})
            }
        rule function_args() -> Vec<FunctionArg> = args:(function_arg() ** ",") ","? { args }
        rule function_arg() -> FunctionArg =
            ig_line() arg:variable() ig_line() ":" ig_line() annotation:mool_type() ig_line() {
                FunctionArg{arg: arg, annotation: annotation.to_string()}
            }
        rule variable() -> Variable =
            scope:$("%"/"@") name:identifier() {
                Variable{name: name, global: if scope == "@" { true } else { false }}
            }
        rule identifier() -> String =
            id:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) {
                id.to_owned()
            }
        rule mool_type() -> String =
            id:$("int"/"bool"/"float"
                /(
                    "Tensor" "[" ig_line() "(" ig_line() ['0'..='9']* ig_line() ")" ig_space() "," ig_space() ("int"/"bool"/"float") ig_line() "]"
                )) {
                id.to_owned()
            }
        rule operator() -> Expr =
            p:position!() op:$("Add" / "Sub" / "Mul" / "Div") ig_space()
                "(" ig_line() x:expression() ig_line() "," ig_line() y:expression() ig_line() ")" ig_space() {
                match op {
                    "Add" => { Expr::Operator(Operator::Add(Box::new(x), Box::new(y))) },
                    "Sub" => { Expr::Operator(Operator::Sub(Box::new(x), Box::new(y))) },
                    "Mul" => { Expr::Operator(Operator::Mul(Box::new(x), Box::new(y))) },
                    "Div" => { Expr::Operator(Operator::Div(Box::new(x), Box::new(y))) },
                    _ => { panic!("{}:暂不支持{}算子", p ,op) }
                }
            }
            / ig_space() "Tensor" ig_line() "(" ig_line() t:tensor() ig_line() ")" ig_space() {
                Expr::Operator(Operator::Tensor(t))
            }
        rule tensor() -> Vec<Literal> = "[" ig_line() t:(tensor_type()** ",") ig_line() "]" { t }
        rule tensor_type() -> Literal = int_literal() / float_literal() / bool_literal()
        rule call() -> Expr = ("%"/"@") id:identifier() ig_line() "(" ig_line() args:call_args() ig_line() ")" ig_line() {
            Expr::Call(id, args)
        }
        rule call_args() -> Vec<Expr> = args:(expression() ** ",") ","? { args }
        rule not_keyword() = !("let" / "fn")
        rule ig_space() = quiet!{ [' ' | '\t']* }
        rule ig_line() = quiet!{ [' ' | '\t' | '\n']* }
    }
}
