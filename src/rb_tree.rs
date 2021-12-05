#[derive(Debug)]
pub enum RB{
    Red,
    Black
}

pub type RBTree<T> = Node<T>;

pub struct Node<T> {

    left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    // right: Option<Box<Node<T>>>,
    data: i32,
    pub value: T,
    rb : RB,
    str_func: fn(&T) -> String

}

impl<T> Node<T> {
    
    pub fn find(&self,id: i32) -> Option<&Node<T>> {
        let mut i : &Node<T> = self;
        loop{
            if i.data == id {
                return Some(i);
            }
            
            if i.data > id {
                match i.left {
                    Some(ref child) => {
                        i = child; 
                    },
                    None => {
                        break;
                    }
                } 
            } else {
                match i.right {
                    Some(ref child) => {
                        i = child;
                    },
                    None => {
                        break;
                    }
                }
            }
        }
        None
    }

    pub fn new(id: i32, rb : RB, val : T, pfunc : fn(&T) -> String) -> Node<T> {
        Node {
            left: None, right: None, data:id, rb: rb, value: val, str_func: pfunc
        }

    }

    pub fn insert(&mut self,node : Node<T>) -> () {
        let mut i : &mut Node<T> = self;
        loop{
            if i.data > node.data {
                match i.left {
                    Some(ref mut child) => {
                        i = child; 
                    },
                    None => {
                        i.left = Some(Box::new(node));
                        break;
                    }
                } 
            } else {
                match i.right {
                    Some(ref mut child) => {
                        i = child;
                    },
                    None => {
                        i.right = Some(Box::new(node));
                        break;
                    }
                }
            }
        }
        // *self

    }


    pub fn stringify(&self) -> String {
        let mut ls = String::new();
        let mut rs = String::new();
        if let Some(l) = &self.left {
            ls = (*l).stringify_tree()
        }
        if let Some(r) = &self.right {
            rs = (*r).stringify_tree()
        }


        format!("{} *{}: {}* {}",ls,self.data,(self.str_func)(&self.value),rs)
    }

    pub fn stringify_tree(&self) -> String {
        let mut ls = String::new();
        let mut rs = String::new();
        if let Some(l) = &self.left {
            ls = (*l).stringify_tree()
        }
        if let Some(r) = &self.right {
            rs = (*r).stringify_tree()
        }


        format!("{} ({}: {}) {}",ls,self.data,(self.str_func)(&self.value),rs)
    }

}
    
use std::fmt;

impl<T> fmt::Display for Node<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.stringify_tree())
    }

} 
impl<T> fmt::Debug for Node<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.stringify())
    }

}
