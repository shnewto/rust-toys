#[derive(Clone)]
#[derive(Debug)]
pub struct Node {
    pub val: usize,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}