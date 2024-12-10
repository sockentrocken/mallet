mod editor;
mod game;
mod helper;
mod status;
mod window;

//================================================================

use raylib::prelude::*;

use crate::status::*;

//================================================================

#[rustfmt::skip]
fn main() {
    let (mut handle, thread) = Status::window();
    let mut status = Status::new(&mut handle, &thread);


    loop {
        match status {
            Status::Initial(ref mut sub_state, ref mut window, ref game) => {
                if let Some(state) = Status::initial(&mut handle, &thread, sub_state, window, game) {
                    status = state;
                }
            }
            Status::Success(ref mut sub_state, ref mut window, ref mut editor) => {
                if let Some(state) = Status::success(&mut handle, &thread, sub_state, window, editor) {
                    status = state;
                }
            }
            Status::Failure(ref mut window, ref error) => {
                if let Some(state) = Status::failure(&mut handle, &thread, window, error) {
                    status = state;
                }
            }
            Status::Closure => break,
        }
    }
}
