/*
* MIT License
*
* Copyright (c) 2024 sockentrocken
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use crate::editor::*;
use crate::game::*;
use crate::helper::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub enum Status {
    Initial(InitialState, Asset, Window, Vec<Game>),
    Success(SuccessState, Asset, Window, Editor),
    Failure(Window, String),
    Closure,
}

impl Status {
    // get a new status instance.
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self::Initial(
            InitialState::default(),
            Asset::new(handle, thread),
            Window::new(handle, thread),
            Game::new_list(),
        )
    }

    // create a RL context.
    pub fn window() -> (RaylibHandle, RaylibThread) {
        // create RL window, thread.
        let (mut handle, thread) = raylib::init()
            .resizable()
            .msaa_4x()
            .vsync()
            .size(1024, 768)
            .title("Mallet")
            .build();

        // load default Mallet icon.
        let icon = Image::load_image_from_mem(".png", Inner::ICON)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();
        handle.set_window_icon(icon);

        (handle, thread)
    }

    // initial state.
    pub fn initial(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        status: &mut InitialState,
        asset: &mut Asset,
        window: &mut Window,
        game: &[Game],
    ) -> Option<Status> {
        // begin drawing.
        let mut draw = handle.begin_drawing(thread);
        draw.clear_background(Color::WHITE);

        // draw initial window.
        window.initial(&mut draw, thread, status, asset, game)
    }

    // success state.
    pub fn success(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        status: &mut SuccessState,
        asset: &mut Asset,
        window: &mut Window,
        editor: &mut Editor,
    ) -> Option<Status> {
        // run as long as the window should not close.
        while !handle.window_should_close() {
            // begin drawing.
            let mut draw = handle.begin_drawing(thread);
            draw.clear_background(Color::WHITE);

            // update editor.
            editor.update(&mut draw, thread, asset);

            // update window, change state if window has given back a new state.
            if let Some(status) = window.success(&mut draw, thread, status, asset, editor) {
                return Some(status);
            }
        }

        // window should close, close mallet.
        Some(Status::Closure)
    }

    // failure state.
    pub fn failure(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        window: &mut Window,
        text: &str,
    ) -> Option<Status> {
        None
    }
}

//================================================================

#[derive(Default)]
pub enum InitialState {
    #[default]
    Main,
    New,
    //Load,
}

//================================================================

#[derive(Default)]
pub enum SuccessState {
    #[default]
    Main,
    User,
}
