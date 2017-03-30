#[derive(Clone)]
#[derive(Debug)]
pub struct Node {
    value: usize,
    branches: Vec<Node>
}

impl Node {
    pub fn new() -> Node {
        Node {
            branches: vec!(),
            value: 0
        }
    }

    pub fn expand(&mut self, arity: usize) {
        for _ in 0..arity {
            self.branches.push(Node::new());
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.branches.len() == 0
    }

    pub fn generate(&mut self, arity: usize, depth: usize, max_depth: usize) {

        self.value = depth;

        if depth == max_depth {
            return
        }

        if self.is_leaf() {
            self.expand(arity);
        }

        for b in &mut self.branches {
            b.generate( arity, depth + 1, max_depth );
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }
}