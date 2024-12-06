use crate::editor::*;
use crate::game::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub enum Status {
    Initial(Window, Vec<Game>),
    Success(Window, Editor),
    Failure(Window, String),
    Closure,
}

impl Status {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self::Initial(Window::new(handle, thread), Game::new_list())
    }

    pub fn window() -> (RaylibHandle, RaylibThread) {
        raylib::init()
            .resizable()
            .msaa_4x()
            .vsync()
            .size(1024, 768)
            .title("Brushy")
            .build()
    }

    pub fn initial(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        game: &[Game],
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        window.begin();

        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );

        /*
        let logo_shape = Vector2::new(window.logo.width as f32, window.logo.height as f32);
        let logo_point = Vector2::new(
            draw_shape.x * 0.5 - logo_shape.x * 0.5,
            draw_shape.y * 0.5 - logo_shape.y * 0.5 - Self::LOGO_SHAPE * 0.5,
        );
        let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, draw_shape.y - Self::LOGO_SHAPE);

        window.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN, false);

        draw.draw_texture_v(&window.logo, logo_point, Color::WHITE);

        window.point(Vector2::new(20.0, draw_shape.y - Self::LOGO_SHAPE + 24.0));

        if window.button(&mut draw, "New Map") {
            let module = rfd::FileDialog::new().set_directory("/").pick_folder();
        }
        if window.button(&mut draw, "Load Map") {
            let module = rfd::FileDialog::new().set_directory("/").pick_folder();
        }
        if window.button(&mut draw, "Exit Brushy") {
            return Some(Status::Closure);
        }
        */

        let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, 48.0);

        window.card_sharp(&mut draw, card_shape, Window::COLOR_PRIMARY_MAIN, true);

        window.point(Vector2::new(20.0, 14.0));

        window.text(&mut draw, "Game Selection", Window::COLOR_TEXT);

        window.point(Vector2::new(20.0, 72.0));

        for g in game {
            if window.button(&mut draw, &g.info.name) {
                drop(draw);
                return Some(Status::Success(
                    Window::new(handle, thread),
                    Editor::new(handle, thread, g.clone()),
                ));
            }
        }

        if draw.window_should_close() {
            return Some(Status::Closure);
        }

        None
    }

    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        editor: &mut Editor,
    ) -> Option<Status> {
        while !handle.window_should_close() {
            editor.resize(handle, &thread);

            let mut draw = handle.begin_drawing(&thread);

            draw.clear_background(Color::WHITE);

            editor.update(&mut draw, &thread);
            if window.update(&mut draw, &thread, editor) {
                drop(draw);
                return Some(Status::new(handle, thread));
            }
        }

        Some(Status::Closure)
    }

    pub fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        text: &str,
    ) -> Option<Status> {
        None
    }
}
