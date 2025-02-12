use std::{cell::RefCell, rc::{Rc, Weak}, sync::mpsc::TryRecvError};

use crate::inf_to_pos::Token;
pub struct Tree{
    nodes: Vec<TreeNode>,
    root: Option<TreeNode>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TreeNode{
    value: Token,
    left: Option<Rc<TreeNode>>,
    right: Option<Rc<TreeNode>>
}
impl TreeNode{
    pub fn printTree(self, level: usize, prefix: &str)->String{
        let space = " ".repeat(level*4);
        let mut ret = format!("{}{}{:?}\n",space,prefix,self.value);
        match self.left{
            Some(left)=>{
                let lret = (*left).clone().printTree(level+1, "L----");
                ret+=&lret;
            }
            _=>{
            }
        }
        match self.right {
            Some(right)=>{
                let rret = (*right).clone().printTree(level+1,"R----");
                ret+=&rret;
            }
            _=>{
            }
            
        }
        ret
    }
}

impl Tree{
    pub fn new()->Self{
        Self { nodes: Vec::new(), root: None }
    }

    pub fn generate(&mut self, tokens: Vec<Token>)->Rc<TreeNode>{
        let mut stack : Vec<TreeNode> = Vec::new();

        for tk in tokens{
            match tk{
                Token::Literal(c) | Token::Range(c,_)=>{
                    let newnode = TreeNode{
                        value: tk, 
                        left: None,
                        right: None
                    };
                    stack.push(newnode);
                },
                Token::Concat | Token::Union=>{
                    match (stack.pop(), stack.pop()){
                        (Some(first), Some(second))=>{
                            let operator = TreeNode{
                                value: tk,
                                left: Some(Rc::new(first)),
                                right: Some(Rc::new(second))
                            };
                            stack.push(operator);
                        }
                        _=>{}
                    }

                },
                Token::Kleene | Token::Union=>{
                    match stack.pop(){
                        Some(first)=>{
                            let operator = TreeNode{
                                value: tk,
                                left: Some(Rc::new(first)),
                                right:None
                            };
                            stack.push(operator);
                        }
                        _=>{}
                    }
                }
                _=>{}
            }
            
        }
        Rc::new(stack[0].clone())
    }
}