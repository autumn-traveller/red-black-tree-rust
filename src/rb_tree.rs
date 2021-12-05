#[derive(Debug)]
pub enum RB{
    Red,
    Black
}

pub type RBTree<T> = Node<T>;

pub struct Node<T> {

    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    id: i32,
    pub value: T,
    rb : RB,
    str_func: fn(&T) -> String

}

use std::cmp::max;

impl<T> Node<T> {

    pub fn max_depth(&self) -> i32 {
        let mut i : &Node<T> = self;
        loop{
            if i.left.is_none() && i.right.is_none() {
                return 1;
            }
            let l = if i.left.is_some() {i.left.as_ref().unwrap().max_depth()} else {0};
            let r = if i.right.is_some() {i.right.as_ref().unwrap().max_depth()} else {0};
            return max(l,r) + 1;
        }
    }

    pub fn remove(&mut self, id: i32) -> Option<Node<T>> {
        unimplemented!();
    }

    pub fn find(&self,id: i32) -> Option<&Node<T>> {
        let mut i : &Node<T> = self;
        loop{
            if i.id == id {
                return Some(i);
            }
            
            if i.id > id {
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
            left: None, right: None, id:id, rb: rb, value: val, str_func: pfunc
        }

    }

    pub fn insert(&mut self,node : Node<T>) -> () {
        let mut i : &mut Node<T> = self;
        loop{
            if i.id > node.id {
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


        format!("{} *{}: {}* {}",ls,self.id,(self.str_func)(&self.value),rs)
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


        format!("{} ({}: {}) {}",ls,self.id,(self.str_func)(&self.value),rs)
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
