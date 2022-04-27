use llvm_sys as llvm;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    names: HashMap<String, llvm::prelude::LLVMValueRef>,
    next: Option<Box<Scope>>,
}

impl Scope {
    /// 创建作用域
    pub fn new() -> Self {
        Scope {
            names: HashMap::new(),
            next: None,
        }
    }

    /// 压入作用域
    pub fn push(self) -> Self {
        let mut new_scope = Self::new();
        new_scope.next = Some(Box::new(self));
        new_scope
    }

    /// 弹出作用域
    pub fn pop(self) -> Self {
        match self.next {
            Some(scope) => *scope,
            None => Self::new(),
        }
    }

    // 注册变量
    pub fn register(&mut self, name: String, value: llvm::prelude::LLVMValueRef) {
        self.names.insert(name, value);
    }

    // 注册全局变量
    // pub fn register_global(&mut self, name: String, value: llvm::prelude::LLVMValueRef) {
    //     match self.next {
    //         Some(scope) => scope.register_global(name, value),
    //     }
    //     match self.next {
    //         None => self.names.insert(name, value),
    //         Some(scope) => {
    //             let next = scope;
    //             while scope.next.is_some() {}
    //         }
    //     }
    //     self.names.insert(name, value);
    // }

    // 获取变量值
    pub fn get(&self, name: String) -> Option<llvm::prelude::LLVMValueRef> {
        match self.names.get(&name) {
            Some(&value) => Some(value),
            None => match &self.next {
                Some(scope) => scope.get(name),
                None => None,
            },
        }
    }
}
