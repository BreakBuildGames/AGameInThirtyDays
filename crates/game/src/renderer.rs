use gl_bindings::gl43 as gl;

use crate::color32;

pub struct Renderer {
    gl: gl::Api,
}

impl Renderer {
    pub fn new(
        proc_address: &impl Fn(&str) -> *const std::ffi::c_void,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let gl = unsafe { gl::Api::with_loader(proc_address) }?;

        unsafe {
            gl.enable(gl::Capability::DEBUG_OUTPUT);
            gl.debug_message_callback(Some(debug_message_callback), std::ptr::null_mut());
        }

        unsafe {
            let [r, g, b, a] = color32::Linear32::PERSIAN_INDIGO.as_rgba();
            gl.clear_color(r, g, b, a);
        }

        Ok(Self { gl })
    }

    pub fn update(&mut self) {
        let gl = &self.gl;

        unsafe {
            gl.clear(gl::ClearMask::COLOR);
        }
    }
}

extern "system" fn debug_message_callback(
    source: gl::DebugSource,
    kind: gl::DebugType,
    id: gl::GLuint,
    severity: gl::DebugSeverity,
    _length: gl::GLsizei,
    message: *const gl::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    let error_message = unsafe {
        std::ffi::CStr::from_ptr(message.cast())
            .to_str()
            .unwrap_or("[FAILED TO READ GL ERROR MESSAGE]")
    };

    match severity {
        gl::DebugSeverity::HIGH => log::error!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::MEDIUM => log::warn!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::LOW => log::info!("{id}: {kind} from {source}: {error_message}"),
        _ => log::trace!("{id}: {kind} from {source}: {error_message}"),
    }
}
