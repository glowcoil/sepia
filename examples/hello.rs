use baseview::{Event, Window, WindowHandler, WindowScalePolicy};
use raw_gl_context::{GlConfig, GlContext};
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

struct Example {
    context: GlContext,
    ui: Ui,
    root: Parent,
}

impl WindowHandler for Example {
    fn on_frame(&mut self) {
        self.context.make_current();

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.context.make_not_current();
        self.context.swap_buffers();
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) {
        match event {
            Event::Mouse(e) => {}
            Event::Keyboard(e) => {}
            Event::Window(e) => {}
        }
    }
}

fn main() {
    let window_open_options = baseview::WindowOpenOptions {
        title: "sepia".into(),
        size: baseview::Size::new(512.0, 512.0),
        scale: WindowScalePolicy::SystemScaleFactor,
    };

    Window::open_blocking(window_open_options, |window| {
        let context = GlContext::create(window, GlConfig::default()).unwrap();

        context.make_current();
        gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
        context.make_not_current();

        let mut root = Parent {
            base: ElemBase::new(),
            child_a: Child {
                base: ElemBase::new(),
            },
            child_b: Child {
                base: ElemBase::new(),
            },
        };

        let mut ui = Ui::new();
        ui.init(&mut root);

        Example { context, ui, root }
    });
}
