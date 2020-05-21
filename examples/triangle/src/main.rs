use window::{Window, ShaderBuilder, Shader, Shape, ShapeBuilder, Transform, Group};
use std::mem::MaybeUninit;

static mut CONTEXT: MaybeUninit<Context> = MaybeUninit::uninit();

pub struct Context {
    window: Window,
    shader: Shader,
    group: Group,
}

fn main() {
    pub fn run(nanos: u64) {
        let context: &mut Context = unsafe { &mut *CONTEXT.as_mut_ptr() };

        context.window.draw(&context.shader, &mut context.group);
    }

    println!("{:?}", std::thread::current().name());
    let mut window = window::Window::new("My Window", run);
    let mut shader: Shader = window.shader_new(include!(concat!(env!("OUT_DIR"), "/res/", "color", ".rs")));
    let shape: Shape = ShapeBuilder::new(&mut shader)
        .vert(&[
              0.125, 0.895,   1.0, 0.0, 0.0,
              0.895, 0.895,   0.0, 1.0, 0.0,
              0.5,   0.125,   0.0, 0.0, 1.0,
        ])
        .face(Transform::new())
        .finish();
    let mut group: Group = window.group_new();
    group.push(&shape, &Transform::new());

    let mut context = Context {
        window, shader, group
    };

    context.window.background(0.1, 0.0, 0.1);

    unsafe { CONTEXT = MaybeUninit::new(context) };

    while unsafe { (*CONTEXT.as_mut_ptr()).window.run() } {  }
}
