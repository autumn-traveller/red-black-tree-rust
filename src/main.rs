mod rb_tree;
use crate::rb_tree::*;
mod find_files;
use crate::find_files::*;

pub fn rb_str_print(s: &String) -> String {
    String::from(s)
}

fn main() {
    let mut s = String::new();
    find_files(".",&mut s);
    println!("{}",s);
    return;

    let mut root = RBTree::new(5,RB::Black,String::from("foo"),rb_str_print);
    let n1 = Node::new(11,RB::Red,String::from("bar"),rb_str_print);
    let n2 = Node::new(2,RB::Red,String::from("node3/n2"),rb_str_print);
    let n3 = Node::new(33,RB::Red,String::from("whats the motto"),rb_str_print);
    let n4 = Node::new(4,RB::Red,String::from("yolo"),rb_str_print);
    let n5 = Node::new(8,RB::Red,String::from("yolo"),rb_str_print);

    root.insert(n1);
    root.insert(n2);
    root.insert(n3);
    root.insert(n4);
    root.insert(n5);

    println!("Root node display : {}",root);
    println!("Root node debug : {:?}",root);

    if let Some(f) = root.find(33) {
        println!("Node 33 ? : {}",f);
    } else {
        println!("Node 33 not found");
    }

    if let Some(f) = root.find(11) {
        println!("Node 11 ? : {}",f);
    } else {
        println!("Node 11 not found");
    }

    println!("maxdepth from 'root': {} ",root.max_depth());
    println!("maxdepth from 33: {} ",root.find(33).unwrap().max_depth());
    println!("maxdepth from 11: {} ",root.find(11).unwrap().max_depth());

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rb_tree_basics() {
        let mut root = RBTree::new(5,RB::Black,String::from("foo"),rb_str_print);
        let n1 = Node::new(11,RB::Red,String::from("bar"),rb_str_print);
        let n2 = Node::new(2,RB::Black,String::from("node3/n2"),rb_str_print);
        let n3 = Node::new(33,RB::Red,String::from("whats the motto"),rb_str_print);
        let n4 = Node::new(4,RB::Black,String::from("yolo"),rb_str_print);
        let n5 = Node::new(8,RB::Red,String::from("yolo"),rb_str_print);
    
        root.insert(n1);
        root.insert(n2);

        assert_eq!(root.stringify_tree()," (2: node3/n2)  (5: foo)  (11: bar) ");

        root.insert(n3);
        root.insert(n4);
        root.insert(n5);

        assert_eq!(root.stringify_tree()," (2: node3/n2)  (4: yolo)  (5: foo)  (8: yolo)  (11: bar)  (33: whats the motto) ");

        assert!(root.find(2).is_some(),"Coundnt find 2 in the tree");
        assert!(root.find(2).unwrap().value == "node3/n2","Value for 2 is wrong");
        assert!(root.find(4).is_some(),"Coundnt find 4 in the tree");
        assert!(root.find(4).unwrap().value == "yolo","Value for 4 is wrong");
        assert!(root.find(8).is_some(),"Coundnt find 8 in the tree");
        assert!(root.find(8).unwrap().value == "yolo","Value for 8 is wrong");
        assert!(root.find(5).is_some(),"Coundnt find 5 in the tree");
        assert!(root.find(5).unwrap().value == "foo","Value for 5 is wrong");

        assert!(root.find(0).is_none(),"Found 0 even though it is not in the tree");
        assert!(root.find(12).is_none(),"Found 12 even though it is not in the tree");
        assert!(root.find(-2147483648).is_none(),"Found -2147483648 even though it is not in the tree");

        assert_eq!(root.max_depth(),3,"Maxdepth from root is {} instead of 3",root.max_depth());
        assert_eq!(root.find(33).unwrap().max_depth(),1,"Maxdepth from 33 is {} instead of 1",root.find(33).unwrap().max_depth());
        assert_eq!(root.find(11).unwrap().max_depth(),2,"Maxdepth from 11 is {} instead of 2",root.find(11).unwrap().max_depth());
    }

    #[test]
    fn find_files_basics(){
        
        let mut s = String::new();
        find_files("/home/nic/Projects/make_os",&mut s);

        assert_eq!(s,"File /home/nic/Projects/make_os/chp2.bin is a file of size 512\nFile /home/nic/Projects/make_os/boot.bin is a file of size 512\nFile /home/nic/Projects/make_os/chp1.asm is a file of size 43\nFile /home/nic/Projects/make_os/playground is a directory \nFile /home/nic/Projects/make_os/chp2.asm is a file of size 516\nFile /home/nic/Projects/make_os/notes is a file of size 429\nFile /home/nic/Projects/make_os/EBIOS-UM.PDF is a file of size 301626\nFile /home/nic/Projects/make_os/boot.asm is a file of size 617\nFile /home/nic/Projects/make_os/playground/if_e_print_wow.bin is a file of size 512\nFile /home/nic/Projects/make_os/playground/if_e_print_wow.asm is a file of size 589\nFile /home/nic/Projects/make_os/playground/boot.bin is a file of size 512\nFile /home/nic/Projects/make_os/playground/boot.asm is a file of size 518\nFile /home/nic/Projects/make_os/playground/echo.asm is a file of size 503\n");        

    }
}
