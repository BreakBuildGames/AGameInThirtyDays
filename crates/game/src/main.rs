use glam::Vec3;
use glfw::{fail_on_errors, Action, Context as _, Key};

mod camera;
mod color32;
mod renderer;

pub struct GameState {
    camera: camera::Orbit,
}

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

    glfw.set_swap_interval(glfw::SwapInterval::None);

    let mut renderer = renderer::Renderer::new(&|s| glfw.get_proc_address_raw(s))?;

    let mut timer = std::time::Instant::now();

    let proj = camera::Projection::with_perspective(0.6, 1024.0 / 768.0, 0.01, 1000.0);
    let mut camera = camera::Orbit::with_camera(proj);
    camera.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, 10.0);
    let mut game_state = GameState { camera };

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::Key(Key::F, _, Action::Repeat, _) => {
                    game_state.camera.zoom(-1.0);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                    game_state.camera.zoom(1.0);
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Repeat, _) => {
                    game_state.camera.rotate(Vec3::new(0.0, -0.1, 0.0));
                }
                glfw::WindowEvent::Key(Key::T, _, Action::Repeat, _) => {
                    game_state.camera.rotate(Vec3::new(0.0, 0.1, 0.0));
                }
                _ => (),
            }
        }

        let current_time = std::time::Instant::now();
        let dt = current_time - timer;
        renderer.update(dt.as_secs_f32(), &mut game_state);
        window.swap_buffers();
        timer = current_time;
    }

    Ok(())
}
