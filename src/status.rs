use crate::editor::*;
use crate::game::*;
use crate::helper::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;

//================================================================

#[derive(Default)]
pub enum InitialState {
    #[default]
    Main,
    New,
}

#[derive(Default)]
pub enum SuccessState {
    #[default]
    Main,
    User,
}

pub enum Status {
    Initial(InitialState, Window, Vec<Game>),
    Success(SuccessState, Window, Editor),
    Failure(Window, String),
    Closure,
}

impl Status {
    pub const ICON: &'static [u8] = include_bytes!("asset/icon.png");

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self::Initial(
            InitialState::default(),
            Window::new(handle, thread),
            Game::new_list(),
        )
    }

    pub fn window() -> (RaylibHandle, RaylibThread) {
        let (mut handle, thread) = raylib::init()
            .resizable()
            .msaa_4x()
            .vsync()
            .size(1024, 768)
            .title("Mallet")
            .build();

        let icon = Image::load_image_from_mem(".png", Self::ICON)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();

        handle.set_window_icon(icon);

        (handle, thread)
    }

    pub fn initial(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        status: &mut InitialState,
        window: &mut Window,
        game: &[Game],
    ) -> Option<Status> {
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        window.initial(&mut draw, thread, status, game)
    }

    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        status: &mut SuccessState,
        window: &mut Window,
        editor: &mut Editor,
    ) -> Option<Status> {
        while !handle.window_should_close() {
            let mut draw = handle.begin_drawing(&thread);
            draw.clear_background(Color::WHITE);

            editor.update(&mut draw, &thread);
            if let Some(status) = window.success(&mut draw, &thread, status, editor) {
                return Some(status);
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
