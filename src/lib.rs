use std::rc::Rc;

pub struct Ui {}

impl Ui {
    pub fn new() -> Ui {
        Ui {}
    }

    pub fn init(&mut self, root: &mut dyn Elem) {
        let mut walker = ElemWalker {};
        walker.walk(root);
    }
}

pub struct ElemWalker {}

impl ElemWalker {
    pub fn walk(&mut self, elem: &mut dyn Elem) {
        elem.walk(self);
    }
}

pub struct ElemBase(Rc<Node>);

impl ElemBase {
    pub fn new() -> ElemBase {
        ElemBase(Rc::new(Node::new()))
    }
}

struct Node;

impl Node {
    fn new() -> Node {
        Node
    }
}

pub trait Elem {
    fn base(&self) -> &ElemBase;

    fn walk(&mut self, walker: &mut ElemWalker) {}
}
