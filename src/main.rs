mod editor;
mod helper;
mod status;
mod window;

//================================================================

use raylib::prelude::*;

//================================================================

use crate::editor::*;
use crate::window::*;

//================================================================

pub fn direction_from_euler(pitch: f32, yaw: f32) -> (Vector3, Vector3, Vector3) {
    let pitch = pitch * DEG2RAD as f32;
    let yaw = yaw * DEG2RAD as f32;

    let mut x = Vector3::default();
    let mut y = Vector3::default();
    let mut z = Vector3::default();

    // forward
    x.x = pitch.cos() * yaw.sin();
    x.y = -pitch.sin();
    x.z = pitch.cos() * yaw.cos();

    // up
    y.x = pitch.sin() * yaw.sin();
    y.y = pitch.cos();
    y.z = pitch.sin() * yaw.cos();

    // right
    z.x = yaw.cos();
    z.y = 0.0;
    z.z = -yaw.sin();

    (x, y, z)
}

#[rustfmt::skip]
fn main() {
    let (mut handle, thread) = raylib::init()
        .resizable()
        .msaa_4x()
        .vsync()
        .size(1024, 768)
        .title("Brushy")
        .build();

    let mut window = Window::new(&mut handle, &thread);
    let mut editor = Editor::new(&mut handle, &thread);

    let mut camera = Camera3D::perspective(
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::zero(),
        Vector3::up(),
        90.0,
    );

    //handle.disable_cursor();

    while !handle.window_should_close() {
        editor.resize(&mut handle, &thread);

        let mut draw = handle.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);

        /*
        let mut draw = draw.begin_mode3D(camera);

        draw.draw_grid(64, 1.0);

        // x let direction = direction_from_euler(0.0, 90.0);
        // y let direction = direction_from_euler(90.0, 90.0);
        // z let direction = direction_from_euler(0.0, 0.0);

        if draw.is_key_down(KeyboardKey::KEY_Q) {
            draw.draw_line_3D(Vector3::zero(), Vector3::new(1.0, 0.0, 0.0), Color::new(255, 0, 0, 255));
            draw.draw_line_3D(Vector3::zero(), Vector3::new(0.0, 1.0, 0.0), Color::new(0, 255, 0, 255));
            draw.draw_line_3D(Vector3::zero(), Vector3::new(0.0, 0.0, 1.0), Color::new(0, 0, 255, 255));   
        } else {
            draw.draw_line_3D(Vector3::zero(), direction.0, Color::new(255, 0, 0, 255));
            draw.draw_line_3D(Vector3::zero(), direction.1, Color::new(0, 255, 0, 255));
            draw.draw_line_3D(Vector3::zero(), direction.2, Color::new(0, 0, 255, 255));
        }
        */

        editor.update(&mut draw, &thread);
        window.update(&mut draw, &thread, &mut editor);
    }
}
