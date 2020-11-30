use sepia::{elem_base, elem_children, Elem, ElemBase, ElemWalker, Ui};

struct Parent {
    base: ElemBase,
    child_a: Child,
    child_b: Child,
}

impl Elem for Parent {
    elem_base!(base);
    elem_children!(child_a, child_b);
}

struct Child {
    base: ElemBase,
}

impl Elem for Child {
    elem_base!(base);
}

fn main() {
    let mut elem = Parent {
        base: ElemBase::new(),
        child_a: Child {
            base: ElemBase::new(),
        },
        child_b: Child {
            base: ElemBase::new(),
        },
    };

    let mut ui = Ui::new();
    ui.init(&mut elem);
}
