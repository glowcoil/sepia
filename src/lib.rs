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

pub struct Constraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }
}

pub struct DisplayList {}

pub trait Elem {
    fn base(&self) -> &ElemBase;

    fn walk(&mut self, walker: &mut ElemWalker) {}

    fn layout(&mut self, constraints: Constraints) -> Size {
        Size::new(constraints.min_width, constraints.min_height)
    }

    fn render(&mut self, display_list: &mut DisplayList) {}
}
