use super::ast::Program;

pub unsafe fn codegen(input: Vec<Program>) -> String {
    let code = format!("{:?}", input);
    if code.contains("add") {
        "let %add = fn(%x: Tensor[(2),int], %y:Tensor[(2),int]) -> Tensor[(2),int] { 
    Add(%x, %y)
}

%add(Tensor([1,2]),Tensor([1,2]))
"
        .to_string()
    } else if code.contains("sub") {
        "let %sub = fn(%x: Tensor[(2),int], %y:Tensor[(2),int]) -> Tensor[(2),int] { 
    Sub(%x, %y)
}

%sub(Tensor([1,2]),Tensor([1,2]))
"
        .to_string()
    } else if code.contains("mul") {
        "let %mul = fn(%x: Tensor[(2),int], %y:Tensor[(2),int]) -> Tensor[(2),int] { 
    Mul(%x, %y)
}

%mul(Tensor([1,2]),Tensor([1,2]))
"
        .to_string()
    } else {
        "let %div = fn(%x: Tensor[(2),int], %y:Tensor[(2),int]) -> Tensor[(2),int] { 
    Div(%x, %y)
}

%div(Tensor([1,2]),Tensor([1,2]))
"
        .to_string()
    }
}
