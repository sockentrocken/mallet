/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

use raylib::prelude::*;

//================================================================

// convert the size of the screen to a Vector2, for usability's sake.
pub fn screen_shape(handle: &RaylibHandle) -> Vector2 {
    Vector2::new(
        handle.get_screen_width() as f32,
        handle.get_screen_height() as f32,
    )
}

// load a font from memory, throwing a panic window should it fail.
pub fn load_font(handle: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Font {
    handle
        .load_font_from_memory(thread, ".ttf", data, 24, None)
        .map_err(|e| panic(&e.to_string()))
        .unwrap()
}

// load a texture from memory, throwing a panic window should it fail.
pub fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Texture2D {
    let mut texture = handle
        .load_texture_from_image(
            thread,
            &Image::load_image_from_mem(".png", data)
                .map_err(|e| panic(&e.to_string()))
                .unwrap(),
        )
        .map_err(|e| panic(&e.to_string()))
        .unwrap();

    texture.gen_texture_mipmaps();

    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);

    texture
}

//================================================================

pub fn direction_from_euler(angle: Vector2) -> (Vector3, Vector3, Vector3) {
    let mut d_x = Vector3::default();
    let mut d_y = Vector3::default();
    let mut d_z = Vector3::default();

    let angle = Vector2::new(angle.x * DEG2RAD as f32, angle.y * DEG2RAD as f32);

    d_x.x = angle.y.cos() * angle.x.sin();
    d_x.y = -angle.y.sin();
    d_x.z = angle.y.cos() * angle.x.cos();

    d_y.x = angle.y.sin() * angle.x.sin();
    d_y.y = angle.y.cos();
    d_y.z = angle.y.sin() * angle.x.cos();

    d_z.x = angle.x.cos();
    d_z.y = 0.0;
    d_z.z = -angle.x.sin();

    (d_x, d_y, d_z)
}

//================================================================

#[rustfmt::skip]
pub fn draw_grid(slice: i32, space: f32, angle: Vector4) {
    let half_slice = (slice as f32) / 2.0;

    unsafe {
        ffi::rlPushMatrix();
        ffi::rlRotatef(angle.w, angle.x, angle.y, angle.z);

        ffi::rlBegin(ffi::RL_LINES.try_into().unwrap());

        for i in -half_slice as i32..half_slice as i32 {
            if i == 0 {
                ffi::rlColor3f(0.50, 0.50, 0.50);
            } else {
                ffi::rlColor3f(0.75, 0.75, 0.75);
            }

            let i = i as f32;

            ffi::rlVertex3f(i * space, 0.0, -half_slice * space);
            ffi::rlVertex3f(i * space, 0.0,  half_slice * space);

            ffi::rlVertex3f(-half_slice * space, 0.0, i * space);
            ffi::rlVertex3f( half_slice * space, 0.0, i * space);
        }

        ffi::rlEnd();

        ffi::rlPopMatrix();
    }
}

//================================================================

pub fn snap(vector: &Vector3, grid: f32) -> Vector3 {
    Vector3::new(
        (vector.x / grid).round() * grid,
        (vector.y / grid).round() * grid,
        (vector.z / grid).round() * grid,
    )
}

//================================================================

pub fn panic(text: &str) {
    rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Fatal Error")
        .set_description(text)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
    panic!("{}", text);
}
