use crate::helper::*;

//================================================================

use raylib::ffi::{KeyboardKey::*, MouseButton::*};
use raylib::prelude::*;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

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

    pub fn new_from_file(path: &str) -> Self {
        // assemble path to user.json.
        let path = format!("{path}/{}", Self::FILE_NAME);

        // check if file does exist, otherwise, return default.
        if std::path::Path::new(&path).is_file() {
            // read file.
            let user = std::fs::read_to_string(path)
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

impl From<Input> for String {
    fn from(value: Input) -> Self {
        if let Some(modify) = value.modify {
            format!(
                "{} + {}",
                std::convert::Into::<&str>::into(modify),
                std::convert::Into::<&str>::into(value.button)
            )
        } else {
            format!("{}", std::convert::Into::<&str>::into(value.button))
        }
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "KEY_NULL" => Key::Keyboard(KEY_NULL),
            "KEY_APOSTROPHE" => Key::Keyboard(KEY_APOSTROPHE),
            "KEY_COMMA" => Key::Keyboard(KEY_COMMA),
            "KEY_MINUS" => Key::Keyboard(KEY_MINUS),
            "KEY_PERIOD" => Key::Keyboard(KEY_PERIOD),
            "KEY_SLASH" => Key::Keyboard(KEY_SLASH),
            "KEY_ZERO" => Key::Keyboard(KEY_ZERO),
            "KEY_ONE" => Key::Keyboard(KEY_ONE),
            "KEY_TWO" => Key::Keyboard(KEY_TWO),
            "KEY_THREE" => Key::Keyboard(KEY_THREE),
            "KEY_FOUR" => Key::Keyboard(KEY_FOUR),
            "KEY_FIVE" => Key::Keyboard(KEY_FIVE),
            "KEY_SIX" => Key::Keyboard(KEY_SIX),
            "KEY_SEVEN" => Key::Keyboard(KEY_SEVEN),
            "KEY_EIGHT" => Key::Keyboard(KEY_EIGHT),
            "KEY_NINE" => Key::Keyboard(KEY_NINE),
            "KEY_SEMICOLON" => Key::Keyboard(KEY_SEMICOLON),
            "KEY_EQUAL" => Key::Keyboard(KEY_EQUAL),
            "KEY_A" => Key::Keyboard(KEY_A),
            "KEY_B" => Key::Keyboard(KEY_B),
            "KEY_C" => Key::Keyboard(KEY_C),
            "KEY_D" => Key::Keyboard(KEY_D),
            "KEY_E" => Key::Keyboard(KEY_E),
            "KEY_F" => Key::Keyboard(KEY_F),
            "KEY_G" => Key::Keyboard(KEY_G),
            "KEY_H" => Key::Keyboard(KEY_H),
            "KEY_I" => Key::Keyboard(KEY_I),
            "KEY_J" => Key::Keyboard(KEY_J),
            "KEY_K" => Key::Keyboard(KEY_K),
            "KEY_L" => Key::Keyboard(KEY_L),
            "KEY_M" => Key::Keyboard(KEY_M),
            "KEY_N" => Key::Keyboard(KEY_N),
            "KEY_O" => Key::Keyboard(KEY_O),
            "KEY_P" => Key::Keyboard(KEY_P),
            "KEY_Q" => Key::Keyboard(KEY_Q),
            "KEY_R" => Key::Keyboard(KEY_R),
            "KEY_S" => Key::Keyboard(KEY_S),
            "KEY_T" => Key::Keyboard(KEY_T),
            "KEY_U" => Key::Keyboard(KEY_U),
            "KEY_V" => Key::Keyboard(KEY_V),
            "KEY_W" => Key::Keyboard(KEY_W),
            "KEY_X" => Key::Keyboard(KEY_X),
            "KEY_Y" => Key::Keyboard(KEY_Y),
            "KEY_Z" => Key::Keyboard(KEY_Z),
            "KEY_LEFT_BRACKET" => Key::Keyboard(KEY_LEFT_BRACKET),
            "KEY_BACKSLASH" => Key::Keyboard(KEY_BACKSLASH),
            "KEY_RIGHT_BRACKET" => Key::Keyboard(KEY_RIGHT_BRACKET),
            "KEY_GRAVE" => Key::Keyboard(KEY_GRAVE),
            "KEY_SPACE" => Key::Keyboard(KEY_SPACE),
            "KEY_ESCAPE" => Key::Keyboard(KEY_ESCAPE),
            "KEY_ENTER" => Key::Keyboard(KEY_ENTER),
            "KEY_TAB" => Key::Keyboard(KEY_TAB),
            "KEY_BACKSPACE" => Key::Keyboard(KEY_BACKSPACE),
            "KEY_INSERT" => Key::Keyboard(KEY_INSERT),
            "KEY_DELETE" => Key::Keyboard(KEY_DELETE),
            "KEY_RIGHT" => Key::Keyboard(KEY_RIGHT),
            "KEY_LEFT" => Key::Keyboard(KEY_LEFT),
            "KEY_DOWN" => Key::Keyboard(KEY_DOWN),
            "KEY_UP" => Key::Keyboard(KEY_UP),
            "KEY_PAGE_UP" => Key::Keyboard(KEY_PAGE_UP),
            "KEY_PAGE_DOWN" => Key::Keyboard(KEY_PAGE_DOWN),
            "KEY_HOME" => Key::Keyboard(KEY_HOME),
            "KEY_END" => Key::Keyboard(KEY_END),
            "KEY_CAPS_LOCK" => Key::Keyboard(KEY_CAPS_LOCK),
            "KEY_SCROLL_LOCK" => Key::Keyboard(KEY_SCROLL_LOCK),
            "KEY_NUM_LOCK" => Key::Keyboard(KEY_NUM_LOCK),
            "KEY_PRINT_SCREEN" => Key::Keyboard(KEY_PRINT_SCREEN),
            "KEY_PAUSE" => Key::Keyboard(KEY_PAUSE),
            "KEY_F1" => Key::Keyboard(KEY_F1),
            "KEY_F2" => Key::Keyboard(KEY_F2),
            "KEY_F3" => Key::Keyboard(KEY_F3),
            "KEY_F4" => Key::Keyboard(KEY_F4),
            "KEY_F5" => Key::Keyboard(KEY_F5),
            "KEY_F6" => Key::Keyboard(KEY_F6),
            "KEY_F7" => Key::Keyboard(KEY_F7),
            "KEY_F8" => Key::Keyboard(KEY_F8),
            "KEY_F9" => Key::Keyboard(KEY_F9),
            "KEY_F10" => Key::Keyboard(KEY_F10),
            "KEY_F11" => Key::Keyboard(KEY_F11),
            "KEY_F12" => Key::Keyboard(KEY_F12),
            "KEY_LEFT_SHIFT" => Key::Keyboard(KEY_LEFT_SHIFT),
            "KEY_LEFT_CONTROL" => Key::Keyboard(KEY_LEFT_CONTROL),
            "KEY_LEFT_ALT" => Key::Keyboard(KEY_LEFT_ALT),
            "KEY_LEFT_SUPER" => Key::Keyboard(KEY_LEFT_SUPER),
            "KEY_RIGHT_SHIFT" => Key::Keyboard(KEY_RIGHT_SHIFT),
            "KEY_RIGHT_CONTROL" => Key::Keyboard(KEY_RIGHT_CONTROL),
            "KEY_RIGHT_ALT" => Key::Keyboard(KEY_RIGHT_ALT),
            "KEY_RIGHT_SUPER" => Key::Keyboard(KEY_RIGHT_SUPER),
            "KEY_KB_MENU" => Key::Keyboard(KEY_KB_MENU),
            "KEY_KP_0" => Key::Keyboard(KEY_KP_0),
            "KEY_KP_1" => Key::Keyboard(KEY_KP_1),
            "KEY_KP_2" => Key::Keyboard(KEY_KP_2),
            "KEY_KP_3" => Key::Keyboard(KEY_KP_3),
            "KEY_KP_4" => Key::Keyboard(KEY_KP_4),
            "KEY_KP_5" => Key::Keyboard(KEY_KP_5),
            "KEY_KP_6" => Key::Keyboard(KEY_KP_6),
            "KEY_KP_7" => Key::Keyboard(KEY_KP_7),
            "KEY_KP_8" => Key::Keyboard(KEY_KP_8),
            "KEY_KP_9" => Key::Keyboard(KEY_KP_9),
            "KEY_KP_DECIMAL" => Key::Keyboard(KEY_KP_DECIMAL),
            "KEY_KP_DIVIDE" => Key::Keyboard(KEY_KP_DIVIDE),
            "KEY_KP_MULTIPLY" => Key::Keyboard(KEY_KP_MULTIPLY),
            "KEY_KP_SUBTRACT" => Key::Keyboard(KEY_KP_SUBTRACT),
            "KEY_KP_ADD" => Key::Keyboard(KEY_KP_ADD),
            "KEY_KP_ENTER" => Key::Keyboard(KEY_KP_ENTER),
            "KEY_KP_EQUAL" => Key::Keyboard(KEY_KP_EQUAL),
            "KEY_BACK" => Key::Keyboard(KEY_BACK),
            "KEY_MENU" => Key::Keyboard(KEY_MENU),
            "KEY_VOLUME_UP" => Key::Keyboard(KEY_VOLUME_UP),
            "KEY_VOLUME_DOWN" => Key::Keyboard(KEY_VOLUME_DOWN),
            "MOUSE_BUTTON_LEFT" => Key::Mouse(MOUSE_BUTTON_LEFT),
            "MOUSE_BUTTON_RIGHT" => Key::Mouse(MOUSE_BUTTON_RIGHT),
            "MOUSE_BUTTON_MIDDLE" => Key::Mouse(MOUSE_BUTTON_MIDDLE),
            "MOUSE_BUTTON_SIDE" => Key::Mouse(MOUSE_BUTTON_SIDE),
            "MOUSE_BUTTON_EXTRA" => Key::Mouse(MOUSE_BUTTON_EXTRA),
            "MOUSE_BUTTON_FORWARD" => Key::Mouse(MOUSE_BUTTON_FORWARD),
            "MOUSE_BUTTON_BACK" => Key::Mouse(MOUSE_BUTTON_BACK),
            _ => Key::Keyboard(KEY_NULL),
        }
    }
}

impl From<Key> for &str {
    fn from(value: Key) -> Self {
        match value {
            Key::Keyboard(keyboard_key) => match keyboard_key {
                KEY_NULL => "KEY_NULL",
                KEY_APOSTROPHE => "KEY_APOSTROPHE",
                KEY_COMMA => "KEY_COMMA",
                KEY_MINUS => "KEY_MINUS",
                KEY_PERIOD => "KEY_PERIOD",
                KEY_SLASH => "KEY_SLASH",
                KEY_ZERO => "KEY_ZERO",
                KEY_ONE => "KEY_ONE",
                KEY_TWO => "KEY_TWO",
                KEY_THREE => "KEY_THREE",
                KEY_FOUR => "KEY_FOUR",
                KEY_FIVE => "KEY_FIVE",
                KEY_SIX => "KEY_SIX",
                KEY_SEVEN => "KEY_SEVEN",
                KEY_EIGHT => "KEY_EIGHT",
                KEY_NINE => "KEY_NINE",
                KEY_SEMICOLON => "KEY_SEMICOLON",
                KEY_EQUAL => "KEY_EQUAL",
                KEY_A => "KEY_A",
                KEY_B => "KEY_B",
                KEY_C => "KEY_C",
                KEY_D => "KEY_D",
                KEY_E => "KEY_E",
                KEY_F => "KEY_F",
                KEY_G => "KEY_G",
                KEY_H => "KEY_H",
                KEY_I => "KEY_I",
                KEY_J => "KEY_J",
                KEY_K => "KEY_K",
                KEY_L => "KEY_L",
                KEY_M => "KEY_M",
                KEY_N => "KEY_N",
                KEY_O => "KEY_O",
                KEY_P => "KEY_P",
                KEY_Q => "KEY_Q",
                KEY_R => "KEY_R",
                KEY_S => "KEY_S",
                KEY_T => "KEY_T",
                KEY_U => "KEY_U",
                KEY_V => "KEY_V",
                KEY_W => "KEY_W",
                KEY_X => "KEY_X",
                KEY_Y => "KEY_Y",
                KEY_Z => "KEY_Z",
                KEY_LEFT_BRACKET => "KEY_LEFT_BRACKET",
                KEY_BACKSLASH => "KEY_BACKSLASH",
                KEY_RIGHT_BRACKET => "KEY_RIGHT_BRACKET",
                KEY_GRAVE => "KEY_GRAVE",
                KEY_SPACE => "KEY_SPACE",
                KEY_ESCAPE => "KEY_ESCAPE",
                KEY_ENTER => "KEY_ENTER",
                KEY_TAB => "KEY_TAB",
                KEY_BACKSPACE => "KEY_BACKSPACE",
                KEY_INSERT => "KEY_INSERT",
                KEY_DELETE => "KEY_DELETE",
                KEY_RIGHT => "KEY_RIGHT",
                KEY_LEFT => "KEY_LEFT",
                KEY_DOWN => "KEY_DOWN",
                KEY_UP => "KEY_UP",
                KEY_PAGE_UP => "KEY_PAGE_UP",
                KEY_PAGE_DOWN => "KEY_PAGE_DOWN",
                KEY_HOME => "KEY_HOME",
                KEY_END => "KEY_END",
                KEY_CAPS_LOCK => "KEY_CAPS_LOCK",
                KEY_SCROLL_LOCK => "KEY_SCROLL_LOCK",
                KEY_NUM_LOCK => "KEY_NUM_LOCK",
                KEY_PRINT_SCREEN => "KEY_PRINT_SCREEN",
                KEY_PAUSE => "KEY_PAUSE",
                KEY_F1 => "KEY_F1",
                KEY_F2 => "KEY_F2",
                KEY_F3 => "KEY_F3",
                KEY_F4 => "KEY_F4",
                KEY_F5 => "KEY_F5",
                KEY_F6 => "KEY_F6",
                KEY_F7 => "KEY_F7",
                KEY_F8 => "KEY_F8",
                KEY_F9 => "KEY_F9",
                KEY_F10 => "KEY_F10",
                KEY_F11 => "KEY_F11",
                KEY_F12 => "KEY_F12",
                KEY_LEFT_SHIFT => "KEY_LEFT_SHIFT",
                KEY_LEFT_CONTROL => "KEY_LEFT_CONTROL",
                KEY_LEFT_ALT => "KEY_LEFT_ALT",
                KEY_LEFT_SUPER => "KEY_LEFT_SUPER",
                KEY_RIGHT_SHIFT => "KEY_RIGHT_SHIFT",
                KEY_RIGHT_CONTROL => "KEY_RIGHT_CONTROL",
                KEY_RIGHT_ALT => "KEY_RIGHT_ALT",
                KEY_RIGHT_SUPER => "KEY_RIGHT_SUPER",
                KEY_KB_MENU => "KEY_KB_MENU",
                KEY_KP_0 => "KEY_KP_0",
                KEY_KP_1 => "KEY_KP_1",
                KEY_KP_2 => "KEY_KP_2",
                KEY_KP_3 => "KEY_KP_3",
                KEY_KP_4 => "KEY_KP_4",
                KEY_KP_5 => "KEY_KP_5",
                KEY_KP_6 => "KEY_KP_6",
                KEY_KP_7 => "KEY_KP_7",
                KEY_KP_8 => "KEY_KP_8",
                KEY_KP_9 => "KEY_KP_9",
                KEY_KP_DECIMAL => "KEY_KP_DECIMAL",
                KEY_KP_DIVIDE => "KEY_KP_DIVIDE",
                KEY_KP_MULTIPLY => "KEY_KP_MULTIPLY",
                KEY_KP_SUBTRACT => "KEY_KP_SUBTRACT",
                KEY_KP_ADD => "KEY_KP_ADD",
                KEY_KP_ENTER => "KEY_KP_ENTER",
                KEY_KP_EQUAL => "KEY_KP_EQUAL",
                KEY_BACK => "KEY_BACK",
                KEY_MENU => "KEY_MENU",
                KEY_VOLUME_UP => "KEY_VOLUME_UP",
                KEY_VOLUME_DOWN => "KEY_VOLUME_DOWN",
            },
            Key::Mouse(mouse_button) => match mouse_button {
                MOUSE_BUTTON_LEFT => "MOUSE_BUTTON_LEFT",
                MOUSE_BUTTON_RIGHT => "MOUSE_BUTTON_RIGHT",
                MOUSE_BUTTON_MIDDLE => "MOUSE_BUTTON_MIDDLE",
                MOUSE_BUTTON_SIDE => "MOUSE_BUTTON_SIDE",
                MOUSE_BUTTON_EXTRA => "MOUSE_BUTTON_EXTRA",
                MOUSE_BUTTON_FORWARD => "MOUSE_BUTTON_FORWARD",
                MOUSE_BUTTON_BACK => "MOUSE_BUTTON_BACK",
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
enum Key {
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
