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
