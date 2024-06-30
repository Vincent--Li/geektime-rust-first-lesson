use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap()); // 如果直接使用downstream的话, 会转移所有权, 这行后续就使用不了node1.downstream 了; 相当于是加了一层.un
    println!("\nnode1: {:?},\nnode2: {:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5))); // 主要就是 borrow_mut() 方法暴露的可变引用
                                                                        // node3.borrow_mut().update_downstream(Rc::new(RefCell::new(node5)));

    println!("\nnode1: {:?},\nnode2: {:?}", node1, node2);
}
