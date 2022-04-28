use llvm_sys as llvm;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    current: Option<Box<ScopeNode>>,
}

#[derive(Debug, Clone)]
struct ScopeNode {
    names: HashMap<String, llvm::prelude::LLVMValueRef>,
    next: Option<Box<ScopeNode>>,
}

impl Scope {
    /// 创建作用域
    pub fn new() -> Self {
        Self {
            current: Some(Box::new(ScopeNode {
                names: HashMap::new(),
                next: None,
            })),
        }
    }

    /// 压入作用域
    pub fn push(&mut self) {
        let mut new_scope = Box::new(ScopeNode {
            names: HashMap::new(),
            next: None,
        });
        let next = self.current.take();
        new_scope.next = next;
        self.current = Some(new_scope);
    }

    /// 弹出作用域
    pub fn pop(&mut self) {
        let scope_possible = self.current.take();
        match scope_possible {
            Some(mut scope) => {
                let next = scope.next.take();
                if next.is_none() {
                    panic!("不能弹出全局作用域");
                }
                self.current = next;
            }
            None => panic!("作用域不能为空"),
        }
    }

    // 注册变量
    pub fn register(&mut self, name: String, value: llvm::prelude::LLVMValueRef) {
        match self.current.as_mut() {
            None => panic!("作用域不能为空"),
            Some(scope) => {
                scope.names.insert(name, value);
            }
        }
    }

    // 获取变量值
    pub fn get(&mut self, name: &String) -> Option<llvm::prelude::LLVMValueRef> {
        match self.current.as_mut() {
            None => panic!("作用域不能为空"),
            Some(scope) => scope.clone().get(&name),
        }
    }
}

impl ScopeNode {
    fn get(self, name: &String) -> Option<llvm::prelude::LLVMValueRef> {
        match self.names.get(name) {
            Some(&value) => Some(value),
            None => match self.next {
                Some(scope) => scope.get(name),
                None => None,
            },
        }
    }
}
