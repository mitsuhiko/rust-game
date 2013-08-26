extern mod glfw;
extern mod gl;

mod game {
    mod engine;

    mod shader;
    mod program;
    mod vertexarray;
    mod main;
}

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    use game::main::main;
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
