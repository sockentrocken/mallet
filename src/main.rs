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
    // create the RL context.
    let (mut handle, thread) = Status::window();
    // create the Mallet state.
    let mut status = Status::new(&mut handle, &thread);

    loop {
        match status {
            // initial status: initialization.
            Status::Initial(ref mut sub_state, ref mut asset, ref mut window, ref game) => {
                if let Some(state) = Status::initial(&mut handle, &thread, sub_state, asset, window, game) {
                    status = state;
                }
            }
            // success status: standard state.
            Status::Success(ref mut sub_state, ref mut asset, ref mut window, ref mut editor) => {
                if let Some(state) = Status::success(&mut handle, &thread, sub_state, asset, window, editor) {
                    status = state;
                }
            }
            // failure status: an error has been thrown from Lua, show crash-handler.
            Status::Failure(ref mut window, ref error) => {
                if let Some(state) = Status::failure(&mut handle, &thread, window, error) {
                    status = state;
                }
            }
            // closure status: break the infinite loop and close.
            Status::Closure => break,
        }
    }
}
