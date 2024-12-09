use crate::asset::*;
use crate::game::*;
use crate::helper::*;
use crate::script::*;
use crate::window::*;

//================================================================

use raylib::{ffi::KeyboardKey::*, ffi::MouseButton::*, prelude::*};
use serde::{de, de::Visitor, Deserialize, Serialize};
use std::fmt;

//================================================================

pub struct Editor {
    pub brush: Vec<Brush>,
    pub entity: Vec<Entity>,
    pub widget: Widget,
    pub asset: Asset,
    pub view: [View; 4],
    pub game: Game,
    pub user: User,
    pub script: Script,
}

impl Editor {
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, game: Game) -> Self {
        let mut asset = Asset::new(handle, thread);
        let mut script = Script::new(&game);

        asset.outer.set_texture_list(handle, thread, &script.meta.texture);

        Self {
            //brush: vec![Brush::default()],
            brush: Vec::default(),
            entity: Vec::default(),
            widget: Widget::default(),
            asset,
            view: [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
            ],
            user: User::new(),
            script,
            game,
        }
    }

    pub fn reload(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        self.entity.clear();
        self.asset.outer.texture.clear();
        self.script = Script::new(&self.game);
        self.asset
            .outer
            .set_texture_list(handle, thread, &self.script.meta.texture);
    }

    #[rustfmt::skip]
    pub fn resize(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        if handle.is_window_resized() {
            self.view = [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
            ];
        }
    }

    pub fn update(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread) {
        for (i, view) in self.view.iter_mut().enumerate() {
            {
                let mut draw_texture = draw.begin_texture_mode(thread, &mut view.render_texture);

                draw_texture.clear_background(Color::WHITE);

                let mut draw = draw_texture.begin_mode3D(view.camera);

                draw.draw_grid(32, 1.0);

                for brush in &self.brush {
                    brush.draw(&self.asset.inner.default);
                }

                for entity in &self.entity {
                    entity.draw(&mut draw);
                }
            }

            let shift = Vector2::new(
                (i as f32 % 2.0).floor() * view.render_texture.width() as f32,
                (i as f32 / 2.0).floor() * view.render_texture.height() as f32 + Window::TOOL_SHAPE,
            );

            draw.draw_texture_rec(
                &view.render_texture,
                Rectangle::new(
                    0.0,
                    0.0,
                    view.render_texture.width() as f32,
                    -view.render_texture.height() as f32,
                ),
                shift,
                Color::WHITE,
            );
        }
    }
}

#[derive(Default)]
pub enum Widget {
    #[default]
    Position = 0,
    Rotation = 1,
    Scale = 2,
    Vertex = 3,
    Edge = 4,
    Face = 5,
}

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

//================================================================

#[derive(Deserialize, Serialize)]
pub struct User {
    pub mouse_speed: [f32; 2],
    pub move_x_a: Input,
    pub move_x_b: Input,
    pub move_y_a: Input,
    pub move_y_b: Input,
    pub look: Input,
    pub position: Input,
    pub rotation: Input,
    pub scale: Input,
    pub vertex: Input,
    pub edge: Input,
    pub face: Input,
    pub configuration: Input,
    pub reload: Input,
    pub import: Input,
    pub export: Input,
    pub exit: Input,
}

impl User {
    pub const FILE_NAME: &'static str = "user.json";

    pub fn new() -> Self {
        // check if file does exist, otherwise, return default.
        if std::path::Path::new(Self::FILE_NAME).is_file() {
            // read file.
            let user = std::fs::read_to_string(Self::FILE_NAME)
                .map_err(|e| panic(&e.to_string()))
                .unwrap();

            // return user.
            serde_json::from_str(&user)
                .map_err(|e| panic(&e.to_string()))
                .unwrap()
        } else {
            // return default.
            Self::default()
        }
    }
}

impl Default for User {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            mouse_speed: [1.0, 1.0],
            move_x_a:      Input::new(None, Key::Keyboard(KEY_W)),
            move_x_b:      Input::new(None, Key::Keyboard(KEY_S)),
            move_y_a:      Input::new(None, Key::Keyboard(KEY_A)),
            move_y_b:      Input::new(None, Key::Keyboard(KEY_D)),
            look:          Input::new(None, Key::Mouse(MOUSE_BUTTON_RIGHT)),
            position:      Input::new(None, Key::Keyboard(KEY_Q)),
            rotation:      Input::new(None, Key::Keyboard(KEY_W)),
            scale:         Input::new(None, Key::Keyboard(KEY_E)),
            vertex:        Input::new(None, Key::Keyboard(KEY_Z)),
            edge:          Input::new(None, Key::Keyboard(KEY_X)),
            face:          Input::new(None, Key::Keyboard(KEY_C)),
            configuration: Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_Z)),
            reload:        Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_X)),
            import:        Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_C)),
            export:        Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_V)),
            exit:          Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_B)),
        }
    }
}

//================================================================

#[derive(Clone, Deserialize, Serialize)]
pub struct Input {
    pub modify: Option<Key>,
    pub button: Key,
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "Null" => Key::Keyboard(KEY_NULL),
            "Apostrophe" => Key::Keyboard(KEY_APOSTROPHE),
            "Comma" => Key::Keyboard(KEY_COMMA),
            "Minus" => Key::Keyboard(KEY_MINUS),
            "Period" => Key::Keyboard(KEY_PERIOD),
            "Slash" => Key::Keyboard(KEY_SLASH),
            "Zero" => Key::Keyboard(KEY_ZERO),
            "1" => Key::Keyboard(KEY_ONE),
            "2" => Key::Keyboard(KEY_TWO),
            "3" => Key::Keyboard(KEY_THREE),
            "4" => Key::Keyboard(KEY_FOUR),
            "5" => Key::Keyboard(KEY_FIVE),
            "6" => Key::Keyboard(KEY_SIX),
            "7" => Key::Keyboard(KEY_SEVEN),
            "8" => Key::Keyboard(KEY_EIGHT),
            "9" => Key::Keyboard(KEY_NINE),
            "Semicolon" => Key::Keyboard(KEY_SEMICOLON),
            "Equal" => Key::Keyboard(KEY_EQUAL),
            "A" => Key::Keyboard(KEY_A),
            "B" => Key::Keyboard(KEY_B),
            "C" => Key::Keyboard(KEY_C),
            "D" => Key::Keyboard(KEY_D),
            "E" => Key::Keyboard(KEY_E),
            "F" => Key::Keyboard(KEY_F),
            "G" => Key::Keyboard(KEY_G),
            "H" => Key::Keyboard(KEY_H),
            "I" => Key::Keyboard(KEY_I),
            "J" => Key::Keyboard(KEY_J),
            "K" => Key::Keyboard(KEY_K),
            "L" => Key::Keyboard(KEY_L),
            "M" => Key::Keyboard(KEY_M),
            "N" => Key::Keyboard(KEY_N),
            "O" => Key::Keyboard(KEY_O),
            "P" => Key::Keyboard(KEY_P),
            "Q" => Key::Keyboard(KEY_Q),
            "R" => Key::Keyboard(KEY_R),
            "S" => Key::Keyboard(KEY_S),
            "T" => Key::Keyboard(KEY_T),
            "U" => Key::Keyboard(KEY_U),
            "V" => Key::Keyboard(KEY_V),
            "W" => Key::Keyboard(KEY_W),
            "X" => Key::Keyboard(KEY_X),
            "Y" => Key::Keyboard(KEY_Y),
            "Z" => Key::Keyboard(KEY_Z),
            "Left Bracket" => Key::Keyboard(KEY_LEFT_BRACKET),
            "Backslash" => Key::Keyboard(KEY_BACKSLASH),
            "Right Bracket" => Key::Keyboard(KEY_RIGHT_BRACKET),
            "Grave" => Key::Keyboard(KEY_GRAVE),
            "Space" => Key::Keyboard(KEY_SPACE),
            "Escape" => Key::Keyboard(KEY_ESCAPE),
            "Enter" => Key::Keyboard(KEY_ENTER),
            "Tabulator" => Key::Keyboard(KEY_TAB),
            "Backspace" => Key::Keyboard(KEY_BACKSPACE),
            "Insert" => Key::Keyboard(KEY_INSERT),
            "Delete" => Key::Keyboard(KEY_DELETE),
            "Right" => Key::Keyboard(KEY_RIGHT),
            "Left" => Key::Keyboard(KEY_LEFT),
            "Down" => Key::Keyboard(KEY_DOWN),
            "Up" => Key::Keyboard(KEY_UP),
            "Page Up" => Key::Keyboard(KEY_PAGE_UP),
            "Page Down" => Key::Keyboard(KEY_PAGE_DOWN),
            "Home" => Key::Keyboard(KEY_HOME),
            "End" => Key::Keyboard(KEY_END),
            "Caps Lock" => Key::Keyboard(KEY_CAPS_LOCK),
            "Scroll Lock" => Key::Keyboard(KEY_SCROLL_LOCK),
            "Number Lock" => Key::Keyboard(KEY_NUM_LOCK),
            "Print Screen" => Key::Keyboard(KEY_PRINT_SCREEN),
            "Pause" => Key::Keyboard(KEY_PAUSE),
            "F1" => Key::Keyboard(KEY_F1),
            "F2" => Key::Keyboard(KEY_F2),
            "F3" => Key::Keyboard(KEY_F3),
            "F4" => Key::Keyboard(KEY_F4),
            "F5" => Key::Keyboard(KEY_F5),
            "F6" => Key::Keyboard(KEY_F6),
            "F7" => Key::Keyboard(KEY_F7),
            "F8" => Key::Keyboard(KEY_F8),
            "F9" => Key::Keyboard(KEY_F9),
            "F10" => Key::Keyboard(KEY_F10),
            "F11" => Key::Keyboard(KEY_F11),
            "F12" => Key::Keyboard(KEY_F12),
            "L. Shift" => Key::Keyboard(KEY_LEFT_SHIFT),
            "L. Control" => Key::Keyboard(KEY_LEFT_CONTROL),
            "L. Alt" => Key::Keyboard(KEY_LEFT_ALT),
            "L. Super" => Key::Keyboard(KEY_LEFT_SUPER),
            "R. Shift" => Key::Keyboard(KEY_RIGHT_SHIFT),
            "R. Control" => Key::Keyboard(KEY_RIGHT_CONTROL),
            "R. Alt" => Key::Keyboard(KEY_RIGHT_ALT),
            "R. Super" => Key::Keyboard(KEY_RIGHT_SUPER),
            "Keyboard Menu" => Key::Keyboard(KEY_KB_MENU),
            "Pad 0" => Key::Keyboard(KEY_KP_0),
            "Pad 1" => Key::Keyboard(KEY_KP_1),
            "Pad 2" => Key::Keyboard(KEY_KP_2),
            "Pad 3" => Key::Keyboard(KEY_KP_3),
            "Pad 4" => Key::Keyboard(KEY_KP_4),
            "Pad 5" => Key::Keyboard(KEY_KP_5),
            "Pad 6" => Key::Keyboard(KEY_KP_6),
            "Pad 7" => Key::Keyboard(KEY_KP_7),
            "Pad 8" => Key::Keyboard(KEY_KP_8),
            "Pad 9" => Key::Keyboard(KEY_KP_9),
            "Pad Decimal" => Key::Keyboard(KEY_KP_DECIMAL),
            "Pad Divide" => Key::Keyboard(KEY_KP_DIVIDE),
            "Pad Multiply" => Key::Keyboard(KEY_KP_MULTIPLY),
            "Pad Subtract" => Key::Keyboard(KEY_KP_SUBTRACT),
            "Pad Add" => Key::Keyboard(KEY_KP_ADD),
            "Pad Enter" => Key::Keyboard(KEY_KP_ENTER),
            "Pad Equal" => Key::Keyboard(KEY_KP_EQUAL),
            "Back" => Key::Keyboard(KEY_BACK),
            "Menu" => Key::Keyboard(KEY_MENU),
            "Volume Up" => Key::Keyboard(KEY_VOLUME_UP),
            "Volume Down" => Key::Keyboard(KEY_VOLUME_DOWN),
            "Mouse Left" => Key::Mouse(MOUSE_BUTTON_LEFT),
            "Mouse Right" => Key::Mouse(MOUSE_BUTTON_RIGHT),
            "Mouse Middle" => Key::Mouse(MOUSE_BUTTON_MIDDLE),
            "Mouse Side" => Key::Mouse(MOUSE_BUTTON_SIDE),
            "Mouse Extra" => Key::Mouse(MOUSE_BUTTON_EXTRA),
            "Mouse Forward" => Key::Mouse(MOUSE_BUTTON_FORWARD),
            "Mouse Back" => Key::Mouse(MOUSE_BUTTON_BACK),
            _ => Key::Keyboard(KEY_NULL),
        }
    }
}

impl From<Key> for &str {
    fn from(value: Key) -> Self {
        match value {
            Key::Keyboard(keyboard_key) => match keyboard_key {
                KEY_NULL => "Null",
                KEY_APOSTROPHE => "Apostrophe",
                KEY_COMMA => "Comma",
                KEY_MINUS => "Minus",
                KEY_PERIOD => "Period",
                KEY_SLASH => "Slash",
                KEY_ZERO => "Zero",
                KEY_ONE => "1",
                KEY_TWO => "2",
                KEY_THREE => "3",
                KEY_FOUR => "4",
                KEY_FIVE => "5",
                KEY_SIX => "6",
                KEY_SEVEN => "7",
                KEY_EIGHT => "8",
                KEY_NINE => "9",
                KEY_SEMICOLON => "Semicolon",
                KEY_EQUAL => "Equal",
                KEY_A => "A",
                KEY_B => "B",
                KEY_C => "C",
                KEY_D => "D",
                KEY_E => "E",
                KEY_F => "F",
                KEY_G => "G",
                KEY_H => "H",
                KEY_I => "I",
                KEY_J => "J",
                KEY_K => "K",
                KEY_L => "L",
                KEY_M => "M",
                KEY_N => "N",
                KEY_O => "O",
                KEY_P => "P",
                KEY_Q => "Q",
                KEY_R => "R",
                KEY_S => "S",
                KEY_T => "T",
                KEY_U => "U",
                KEY_V => "V",
                KEY_W => "W",
                KEY_X => "X",
                KEY_Y => "Y",
                KEY_Z => "Z",
                KEY_LEFT_BRACKET => "Left Bracket",
                KEY_BACKSLASH => "Backslash",
                KEY_RIGHT_BRACKET => "Right Bracket",
                KEY_GRAVE => "Grave",
                KEY_SPACE => "Space",
                KEY_ESCAPE => "Escape",
                KEY_ENTER => "Enter",
                KEY_TAB => "Tabulator",
                KEY_BACKSPACE => "Backspace",
                KEY_INSERT => "Insert",
                KEY_DELETE => "Delete",
                KEY_RIGHT => "Right",
                KEY_LEFT => "Left",
                KEY_DOWN => "Down",
                KEY_UP => "Up",
                KEY_PAGE_UP => "Page Up",
                KEY_PAGE_DOWN => "Page Down",
                KEY_HOME => "Home",
                KEY_END => "End",
                KEY_CAPS_LOCK => "Caps Lock",
                KEY_SCROLL_LOCK => "Scroll Lock",
                KEY_NUM_LOCK => "Number Lock",
                KEY_PRINT_SCREEN => "Print Screen",
                KEY_PAUSE => "Pause",
                KEY_F1 => "F1",
                KEY_F2 => "F2",
                KEY_F3 => "F3",
                KEY_F4 => "F4",
                KEY_F5 => "F5",
                KEY_F6 => "F6",
                KEY_F7 => "F7",
                KEY_F8 => "F8",
                KEY_F9 => "F9",
                KEY_F10 => "F10",
                KEY_F11 => "F11",
                KEY_F12 => "F12",
                KEY_LEFT_SHIFT => "L. Shift",
                KEY_LEFT_CONTROL => "L. Control",
                KEY_LEFT_ALT => "L. Alt",
                KEY_LEFT_SUPER => "L. Super",
                KEY_RIGHT_SHIFT => "R. Shift",
                KEY_RIGHT_CONTROL => "R. Control",
                KEY_RIGHT_ALT => "R. Alt",
                KEY_RIGHT_SUPER => "R. Super",
                KEY_KB_MENU => "Keyboard Menu",
                KEY_KP_0 => "Pad 0",
                KEY_KP_1 => "Pad 1",
                KEY_KP_2 => "Pad 2",
                KEY_KP_3 => "Pad 3",
                KEY_KP_4 => "Pad 4",
                KEY_KP_5 => "Pad 5",
                KEY_KP_6 => "Pad 6",
                KEY_KP_7 => "Pad 7",
                KEY_KP_8 => "Pad 8",
                KEY_KP_9 => "Pad 9",
                KEY_KP_DECIMAL => "Pad Decimal",
                KEY_KP_DIVIDE => "Pad Divide",
                KEY_KP_MULTIPLY => "Pad Multiply",
                KEY_KP_SUBTRACT => "Pad Subtract",
                KEY_KP_ADD => "Pad Add",
                KEY_KP_ENTER => "Pad Enter",
                KEY_KP_EQUAL => "Pad Equal",
                KEY_BACK => "Back",
                KEY_MENU => "Menu",
                KEY_VOLUME_UP => "Volume Up",
                KEY_VOLUME_DOWN => "Volume Down",
            },
            Key::Mouse(mouse_button) => match mouse_button {
                MOUSE_BUTTON_LEFT => "Mouse Left",
                MOUSE_BUTTON_RIGHT => "Mouse Right",
                MOUSE_BUTTON_MIDDLE => "Mouse Middle",
                MOUSE_BUTTON_SIDE => "Mouse Side",
                MOUSE_BUTTON_EXTRA => "Mouse Extra",
                MOUSE_BUTTON_FORWARD => "Mouse Forward",
                MOUSE_BUTTON_BACK => "Mouse Back",
            },
        }
    }
}

impl Input {
    pub fn new(modify: Option<Key>, button: Key) -> Self {
        Self { modify, button }
    }

    pub fn get_up(&self, handle: &RaylibHandle) -> bool {
        if let Some(modify) = &self.modify {
            return modify.get_up(handle) && self.button.get_up(handle);
        }

        self.button.get_up(handle)
    }

    pub fn get_down(&self, handle: &RaylibHandle) -> bool {
        if let Some(modify) = &self.modify {
            return modify.get_down(handle) && self.button.get_down(handle);
        }

        self.button.get_down(handle)
    }

    pub fn get_press(&self, handle: &RaylibHandle) -> bool {
        if let Some(modify) = &self.modify {
            return modify.get_down(handle) && self.button.get_press(handle);
        }

        self.button.get_press(handle)
    }

    pub fn get_release(&self, handle: &RaylibHandle) -> bool {
        if let Some(modify) = &self.modify {
            return modify.get_down(handle) && self.button.get_release(handle);
        }

        self.button.get_release(handle)
    }

    pub fn draw(&self) {}
}

//================================================================

#[derive(Clone)]
pub enum Key {
    Keyboard(KeyboardKey),
    Mouse(MouseButton),
}

impl Key {
    fn get_up(&self, handle: &RaylibHandle) -> bool {
        match self {
            Key::Keyboard(keyboard_key) => handle.is_key_up(*keyboard_key),
            Key::Mouse(mouse_button) => handle.is_mouse_button_up(*mouse_button),
        }
    }

    fn get_down(&self, handle: &RaylibHandle) -> bool {
        match self {
            Key::Keyboard(keyboard_key) => handle.is_key_down(*keyboard_key),
            Key::Mouse(mouse_button) => handle.is_mouse_button_down(*mouse_button),
        }
    }

    fn get_press(&self, handle: &RaylibHandle) -> bool {
        match self {
            Key::Keyboard(keyboard_key) => {
                handle.is_key_pressed(*keyboard_key) || handle.is_key_pressed_repeat(*keyboard_key)
            }
            Key::Mouse(mouse_button) => handle.is_mouse_button_pressed(*mouse_button),
        }
    }

    fn get_release(&self, handle: &RaylibHandle) -> bool {
        match self {
            Key::Keyboard(keyboard_key) => handle.is_key_released(*keyboard_key),
            Key::Mouse(mouse_button) => handle.is_mouse_button_released(*mouse_button),
        }
    }
}

struct KeyVisitor;

impl Visitor<'_> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Unknown key code.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.into())
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyVisitor)
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.clone().into())
    }
}

//================================================================

pub struct Brush {
    pub vertex: [[f32; 3]; 8],
    pub face: [Face; 6],
}

impl Brush {
    pub const DEFAULT_SHAPE: f32 = 1.0;

    pub fn draw(&self, default: &Texture2D) {
        unsafe {
            // begin quad draw.
            ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());

            // for each vertex index, draw the corresponding face.
            for f in &self.face {
                // if we have a texture for this face, use it. otherwise, use the default.
                if let Some(_texture) = &f.texture {
                } else {
                    ffi::rlSetTexture(default.id);
                }

                ffi::rlTexCoord2f(
                    f.scale[0] * (f.shift[0] + 0.0),
                    f.scale[1] * (f.shift[1] + 1.0),
                );
                ffi::rlVertex3f(
                    self.vertex[f.index[0]][0],
                    self.vertex[f.index[0]][1],
                    self.vertex[f.index[0]][2],
                );
                ffi::rlTexCoord2f(
                    f.scale[0] * (f.shift[0] + 1.0),
                    f.scale[1] * (f.shift[1] + 1.0),
                );
                ffi::rlVertex3f(
                    self.vertex[f.index[1]][0],
                    self.vertex[f.index[1]][1],
                    self.vertex[f.index[1]][2],
                );
                ffi::rlTexCoord2f(
                    f.scale[0] * (f.shift[0] + 1.0),
                    f.scale[1] * (f.shift[1] + 0.0),
                );
                ffi::rlVertex3f(
                    self.vertex[f.index[2]][0],
                    self.vertex[f.index[2]][1],
                    self.vertex[f.index[2]][2],
                );
                ffi::rlTexCoord2f(
                    f.scale[0] * (f.shift[0] + 0.0),
                    f.scale[1] * (f.shift[1] + 0.0),
                );
                ffi::rlVertex3f(
                    self.vertex[f.index[3]][0],
                    self.vertex[f.index[3]][1],
                    self.vertex[f.index[3]][2],
                );
            }

            // end quad draw.
            ffi::rlEnd();

            // clear texture.
            ffi::rlSetTexture(0);
        }
    }
}

impl Default for Brush {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            vertex: [
                [-Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE],
                [ Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE],
                [ Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE],
                [-Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE],
                [-Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE],
                [ Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE],
                [ Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE],
                [-Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE],
            ],
            face: Face::new_list()
        }
    }
}

//================================================================

pub struct Face {
    pub index: [usize; 4],
    pub shift: [f32; 2],
    pub scale: [f32; 2],
    pub texture: Option<String>,
}

impl Face {
    pub fn new(index: [usize; 4]) -> Self {
        Self {
            index,
            shift: [0.0, 0.0],
            scale: [1.0, 1.0],
            texture: None,
        }
    }

    pub fn new_list() -> [Self; 6] {
        [
            Face::new([0, 1, 2, 3]),
            Face::new([5, 4, 7, 6]),
            Face::new([3, 2, 6, 7]),
            Face::new([1, 0, 4, 5]),
            Face::new([1, 5, 6, 2]),
            Face::new([4, 0, 3, 7]),
        ]
    }
}

//================================================================

pub struct Entity {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub lua: EntityLua,
}

impl Entity {
    pub fn new_from_lua(lua: EntityLua) -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            lua,
        }
    }

    pub fn draw(&self, draw: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        let shape = self.lua.meta.shape;
        let min = Vector3::new(shape[0][0], shape[0][1], shape[0][2]);
        let max = Vector3::new(shape[1][0], shape[1][1], shape[1][2]);

        draw.draw_bounding_box(BoundingBox::new(min, max), Color::GREEN);

        if let Some(call) = &self.lua.call {
            call.call::<()>(self.position).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct EntityLua {
    pub meta: EntityMeta,
    pub call: Option<mlua::Function>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EntityMeta {
    pub name: String,
    pub info: String,
    pub data: Vec<EntityData>,
    pub shape: [[f32; 3]; 2],
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EntityData {
    pub name: String,
    pub info: String,
    pub kind: serde_json::Value,
}
