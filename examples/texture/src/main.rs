use window::{Window, ShaderBuilder, Shader, Shape, ShapeBuilder, Transform, Group, Graphic};
use std::mem::MaybeUninit;

static mut CONTEXT: MaybeUninit<Context> = MaybeUninit::uninit();

pub struct Context {
    window: Window,
    shader: Shader,
    group: Group,
    graphic: Graphic,
}

fn main() {
    pub fn run(nanos: u64) {
        let context: &mut Context = unsafe { &mut *CONTEXT.as_mut_ptr() };

        context.window.draw_graphic(&context.shader, &mut context.group, &context.graphic);
    }

    println!("{:?}", std::thread::current().name());
    let mut window = window::Window::new("My Window", run);
    let mut shader: Shader = window.shader_new(include!(concat!(env!("OUT_DIR"), "/res/", "graphic", ".rs")));
    let shape: Shape = ShapeBuilder::new(&mut shader)
        .vert(&[
              0.125, 0.895, 0.0,  0.0, 1.0,
              0.895, 0.895, 0.0,  1.0, 1.0,
              0.895, 0.125, 0.0,  1.0, 0.0,
              0.125, 0.125, 0.0,  0.0, 0.0,
              0.125, 0.895, 0.0,  0.0, 1.0,
              0.895, 0.125, 0.0,  1.0, 0.0,
        ])
        .face(Transform::new())
        .finish();
    let mut group: Group = window.group_new();
    group.push(&shape, &Transform::new());

    let graphic = window.graphic(&[
        0, 0, 0, 255,
        255, 0, 0, 255,
        0, 255, 0, 255,
        0, 0, 255, 255], 2, 2);

    let mut context = Context {
        window, shader, group, graphic,
    };

    context.window.background(0.1, 0.0, 0.1);

    unsafe { CONTEXT = MaybeUninit::new(context) };

    while unsafe { (*CONTEXT.as_mut_ptr()).window.run() } {  }
}
