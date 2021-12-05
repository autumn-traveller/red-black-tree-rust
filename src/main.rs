mod rb_tree;
use crate::rb_tree::*;

pub fn rb_str_print(s: &String) -> String {
    String::from(s)
}

fn main() {
    let mut root = RBTree::new(5,RB::Black,String::from("foo"),rb_str_print);
    let n1 = Node::new(11,RB::Red,String::from("bar"),rb_str_print);
    let n2 = Node::new(2,RB::Red,String::from("node3/n2"),rb_str_print);
    let n3 = Node::new(33,RB::Red,String::from("whats the motto"),rb_str_print);
    let n4 = Node::new(4,RB::Red,String::from("yolo"),rb_str_print);
    let n5 = Node::new(8,RB::Red,String::from("yolo"),rb_str_print);

    // let mut n2 = Node{data: 2,left: None,right: None, rb: RB::Black};
    // let mut n3 = Node{data: 33,left: None,right: None, rb: RB::Red};
    // let mut n4 = Node{data: 4,left: None,right: None, rb: RB::Black};
    // let mut n5 = Node{data: 8,left: None,right: None, rb: RB::Black};

    root.insert(n1);
    root.insert(n2);
    root.insert(n3);
    root.insert(n4);
    root.insert(n5);

    println!("Root node display : {}",root);
    println!("Root node debug : {:?}",root);
    println!("FUCKER1: {}",root.right.as_ref().unwrap());
    println!("FUCKER1: {:?}",*root.right.as_ref().unwrap());
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut root = RBTree::new(5,RB::Black,String::from("foo"),rb_str_print);
        let n1 = Node::new(11,RB::Red,String::from("bar"),rb_str_print);
        let n2 = Node::new(2,RB::Red,String::from("node3/n2"),rb_str_print);
        let n3 = Node::new(33,RB::Red,String::from("whats the motto"),rb_str_print);
        let n4 = Node::new(4,RB::Red,String::from("yolo"),rb_str_print);
        let n5 = Node::new(8,RB::Red,String::from("yolo"),rb_str_print);
    
        root.insert(n1);
        root.insert(n2);
    
        println!("Root node display : {}",root);
        println!("Root node debug : {:?}",root);

        assert_eq!(root.stringify_tree()," (2: node3/n2)  (5: foo)  (11: bar) ");

        root.insert(n3);
        root.insert(n4);
        root.insert(n5);

        assert_eq!(root.stringify_tree()," (2: node3/n2)  (4: yolo)  (5: foo)  (8: yolo)  (11: bar)  (33: whats the motto) ");
    }
}