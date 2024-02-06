use glfw::{fail_on_errors, Action, Context as _, Key};

mod color32;
mod renderer;

fn main() -> anyhow::Result<(), anyhow::Error> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let (mut window, events) = glfw
        .create_window(1024, 768, "Unnamed Game", glfw::WindowMode::Windowed)
        .ok_or_else(|| anyhow::anyhow!("failed to create glfw window"))?;

    window.make_current();
    window.set_key_polling(true);

    let mut renderer = renderer::Renderer::new(&|s| glfw.get_proc_address_raw(s))?;

    let mut dt = 0.0;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        dt += 0.01;
        renderer.update(dt);
        window.swap_buffers();
    }

    Ok(())
}
