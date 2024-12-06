use crate::window::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub struct View {
    pub render_texture: RenderTexture2D,
    pub camera: Camera3D,
}

impl View {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, camera: Camera3D) -> Self {
        Self {
            render_texture: handle
                .load_render_texture(
                    &thread,
                    ((handle.get_screen_width() - Window::EDIT_SHAPE as i32) as f32 / 2.0) as u32,
                    ((handle.get_screen_height() - Window::TOOL_SHAPE as i32) as f32 / 2.0) as u32,
                )
                .unwrap(),
            camera,
        }
    }
}
