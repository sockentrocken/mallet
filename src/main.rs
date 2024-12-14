mod editor;
mod game;
mod helper;
mod status;
mod window;

//================================================================

use crate::status::*;

//================================================================

// the main entry-point.
#[rustfmt::skip]
fn main() {
    // create RL context.
    let (mut handle, thread) = Status::window();
    
    // create state, using the inital state (new map, load map, etc.).
    let mut status = Status::new(&mut handle, &thread);

    loop {
        match status {
            Status::Initial(ref mut sub_state, ref mut asset, ref mut window, ref game) => {
                if let Some(state) = Status::initial(&mut handle, &thread, sub_state, asset, window, game) {
                    status = state;
                }
            }
            Status::Success(ref mut sub_state, ref mut asset, ref mut window, ref mut editor) => {
                if let Some(state) = Status::success(&mut handle, &thread, sub_state, asset, window, editor) {
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
