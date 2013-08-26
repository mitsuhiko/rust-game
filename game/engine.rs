use gl;
use glfw;

pub struct Engine;

impl Engine {
    pub fn new() -> Engine {
        Engine
    }

    pub fn run(&self, f: ~fn(win: &glfw::Window)) {
        do glfw::set_error_callback |_, description| {
            printfln!("GLFW Error: %s", description);
        };
        do glfw::start {
            glfw::window_hint::context_version(3, 2);
            glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
            glfw::window_hint::opengl_forward_compat(true);
            glfw::window_hint::samples(4);

            let window = glfw::Window::create(1280, 720, "Demo",
                glfw::Windowed).unwrap();
            window.make_context_current();

            gl::load_with(glfw::get_proc_address);

            f(&window);
        }
    }
}
