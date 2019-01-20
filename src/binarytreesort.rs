use self::Node::*;

pub fn binarytreesort(mut array: Vec<i32>) -> Vec<i32> {
    /*
    fn insert_node(node: Node, num: i32) -> Node {
        match node {
            Null => Leaf(num),
            Leaf(old) if num <= old => Branch(old, Box::new(Leaf(num)), Box::new(Null)),
            Leaf(old) if num >  old => Branch(old, Box::new(Null), Box::new(Leaf(num))),
            Branch(old, left, right) if num <= old => Branch(old, Box::new(insert_node(* left, num)), right),
        }
    }

    let mut btree = Null;
    for i in array.iter() {
        btree = insert_node(btree, *i);
    }
    */

    array
}

enum Node {
    Branch(i32, Box<Node>, Box<Node>),
    Leaf(i32),
    Null,
}

