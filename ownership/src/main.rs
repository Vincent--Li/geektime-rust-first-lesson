<<<<<<< Updated upstream
fn main() {
    let mut data = 1;
    let v = &mut data;
    *v += 1;
    println!("data: {:?}", &data);
    println!("data: {:?}", v1);
}
=======
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        
    }
}

fn main() {


}

>>>>>>> Stashed changes
