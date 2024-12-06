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
    pub save: Input,
    pub load: Input,
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
    fn default() -> Self {
        Self {
            mouse_speed: [1.0, 1.0],
            move_x_a: Input::new(None, Key::Keyboard(KEY_W)),
            move_x_b: Input::new(None, Key::Keyboard(KEY_S)),
            move_y_a: Input::new(None, Key::Keyboard(KEY_A)),
            move_y_b: Input::new(None, Key::Keyboard(KEY_D)),
            look: Input::new(None, Key::Mouse(MOUSE_BUTTON_RIGHT)),
            position: Input::new(None, Key::Keyboard(KEY_Q)),
            rotation: Input::new(None, Key::Keyboard(KEY_W)),
            scale: Input::new(None, Key::Keyboard(KEY_E)),
            vertex: Input::new(None, Key::Keyboard(KEY_Z)),
            edge: Input::new(None, Key::Keyboard(KEY_X)),
            face: Input::new(None, Key::Keyboard(KEY_C)),
            save: Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_S)),
            load: Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_L)),
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Input {
    pub modify: Option<Key>,
    pub button: Key,
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
            return modify.get_press(handle) && self.button.get_press(handle);
        }

        self.button.get_press(handle)
    }

    pub fn get_release(&self, handle: &RaylibHandle) -> bool {
        if let Some(modify) = &self.modify {
            return modify.get_release(handle) && self.button.get_release(handle);
        }

        self.button.get_release(handle)
    }

    pub fn draw(&self) {}
}

//================================================================

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
        match v {
            "KEY_NULL" => Ok(Key::Keyboard(KEY_NULL)),
            "KEY_APOSTROPHE" => Ok(Key::Keyboard(KEY_APOSTROPHE)),
            "KEY_COMMA" => Ok(Key::Keyboard(KEY_COMMA)),
            "KEY_MINUS" => Ok(Key::Keyboard(KEY_MINUS)),
            "KEY_PERIOD" => Ok(Key::Keyboard(KEY_PERIOD)),
            "KEY_SLASH" => Ok(Key::Keyboard(KEY_SLASH)),
            "KEY_ZERO" => Ok(Key::Keyboard(KEY_ZERO)),
            "KEY_ONE" => Ok(Key::Keyboard(KEY_ONE)),
            "KEY_TWO" => Ok(Key::Keyboard(KEY_TWO)),
            "KEY_THREE" => Ok(Key::Keyboard(KEY_THREE)),
            "KEY_FOUR" => Ok(Key::Keyboard(KEY_FOUR)),
            "KEY_FIVE" => Ok(Key::Keyboard(KEY_FIVE)),
            "KEY_SIX" => Ok(Key::Keyboard(KEY_SIX)),
            "KEY_SEVEN" => Ok(Key::Keyboard(KEY_SEVEN)),
            "KEY_EIGHT" => Ok(Key::Keyboard(KEY_EIGHT)),
            "KEY_NINE" => Ok(Key::Keyboard(KEY_NINE)),
            "KEY_SEMICOLON" => Ok(Key::Keyboard(KEY_SEMICOLON)),
            "KEY_EQUAL" => Ok(Key::Keyboard(KEY_EQUAL)),
            "KEY_A" => Ok(Key::Keyboard(KEY_A)),
            "KEY_B" => Ok(Key::Keyboard(KEY_B)),
            "KEY_C" => Ok(Key::Keyboard(KEY_C)),
            "KEY_D" => Ok(Key::Keyboard(KEY_D)),
            "KEY_E" => Ok(Key::Keyboard(KEY_E)),
            "KEY_F" => Ok(Key::Keyboard(KEY_F)),
            "KEY_G" => Ok(Key::Keyboard(KEY_G)),
            "KEY_H" => Ok(Key::Keyboard(KEY_H)),
            "KEY_I" => Ok(Key::Keyboard(KEY_I)),
            "KEY_J" => Ok(Key::Keyboard(KEY_J)),
            "KEY_K" => Ok(Key::Keyboard(KEY_K)),
            "KEY_L" => Ok(Key::Keyboard(KEY_L)),
            "KEY_M" => Ok(Key::Keyboard(KEY_M)),
            "KEY_N" => Ok(Key::Keyboard(KEY_N)),
            "KEY_O" => Ok(Key::Keyboard(KEY_O)),
            "KEY_P" => Ok(Key::Keyboard(KEY_P)),
            "KEY_Q" => Ok(Key::Keyboard(KEY_Q)),
            "KEY_R" => Ok(Key::Keyboard(KEY_R)),
            "KEY_S" => Ok(Key::Keyboard(KEY_S)),
            "KEY_T" => Ok(Key::Keyboard(KEY_T)),
            "KEY_U" => Ok(Key::Keyboard(KEY_U)),
            "KEY_V" => Ok(Key::Keyboard(KEY_V)),
            "KEY_W" => Ok(Key::Keyboard(KEY_W)),
            "KEY_X" => Ok(Key::Keyboard(KEY_X)),
            "KEY_Y" => Ok(Key::Keyboard(KEY_Y)),
            "KEY_Z" => Ok(Key::Keyboard(KEY_Z)),
            "KEY_LEFT_BRACKET" => Ok(Key::Keyboard(KEY_LEFT_BRACKET)),
            "KEY_BACKSLASH" => Ok(Key::Keyboard(KEY_BACKSLASH)),
            "KEY_RIGHT_BRACKET" => Ok(Key::Keyboard(KEY_RIGHT_BRACKET)),
            "KEY_GRAVE" => Ok(Key::Keyboard(KEY_GRAVE)),
            "KEY_SPACE" => Ok(Key::Keyboard(KEY_SPACE)),
            "KEY_ESCAPE" => Ok(Key::Keyboard(KEY_ESCAPE)),
            "KEY_ENTER" => Ok(Key::Keyboard(KEY_ENTER)),
            "KEY_TAB" => Ok(Key::Keyboard(KEY_TAB)),
            "KEY_BACKSPACE" => Ok(Key::Keyboard(KEY_BACKSPACE)),
            "KEY_INSERT" => Ok(Key::Keyboard(KEY_INSERT)),
            "KEY_DELETE" => Ok(Key::Keyboard(KEY_DELETE)),
            "KEY_RIGHT" => Ok(Key::Keyboard(KEY_RIGHT)),
            "KEY_LEFT" => Ok(Key::Keyboard(KEY_LEFT)),
            "KEY_DOWN" => Ok(Key::Keyboard(KEY_DOWN)),
            "KEY_UP" => Ok(Key::Keyboard(KEY_UP)),
            "KEY_PAGE_UP" => Ok(Key::Keyboard(KEY_PAGE_UP)),
            "KEY_PAGE_DOWN" => Ok(Key::Keyboard(KEY_PAGE_DOWN)),
            "KEY_HOME" => Ok(Key::Keyboard(KEY_HOME)),
            "KEY_END" => Ok(Key::Keyboard(KEY_END)),
            "KEY_CAPS_LOCK" => Ok(Key::Keyboard(KEY_CAPS_LOCK)),
            "KEY_SCROLL_LOCK" => Ok(Key::Keyboard(KEY_SCROLL_LOCK)),
            "KEY_NUM_LOCK" => Ok(Key::Keyboard(KEY_NUM_LOCK)),
            "KEY_PRINT_SCREEN" => Ok(Key::Keyboard(KEY_PRINT_SCREEN)),
            "KEY_PAUSE" => Ok(Key::Keyboard(KEY_PAUSE)),
            "KEY_F1" => Ok(Key::Keyboard(KEY_F1)),
            "KEY_F2" => Ok(Key::Keyboard(KEY_F2)),
            "KEY_F3" => Ok(Key::Keyboard(KEY_F3)),
            "KEY_F4" => Ok(Key::Keyboard(KEY_F4)),
            "KEY_F5" => Ok(Key::Keyboard(KEY_F5)),
            "KEY_F6" => Ok(Key::Keyboard(KEY_F6)),
            "KEY_F7" => Ok(Key::Keyboard(KEY_F7)),
            "KEY_F8" => Ok(Key::Keyboard(KEY_F8)),
            "KEY_F9" => Ok(Key::Keyboard(KEY_F9)),
            "KEY_F10" => Ok(Key::Keyboard(KEY_F10)),
            "KEY_F11" => Ok(Key::Keyboard(KEY_F11)),
            "KEY_F12" => Ok(Key::Keyboard(KEY_F12)),
            "KEY_LEFT_SHIFT" => Ok(Key::Keyboard(KEY_LEFT_SHIFT)),
            "KEY_LEFT_CONTROL" => Ok(Key::Keyboard(KEY_LEFT_CONTROL)),
            "KEY_LEFT_ALT" => Ok(Key::Keyboard(KEY_LEFT_ALT)),
            "KEY_LEFT_SUPER" => Ok(Key::Keyboard(KEY_LEFT_SUPER)),
            "KEY_RIGHT_SHIFT" => Ok(Key::Keyboard(KEY_RIGHT_SHIFT)),
            "KEY_RIGHT_CONTROL" => Ok(Key::Keyboard(KEY_RIGHT_CONTROL)),
            "KEY_RIGHT_ALT" => Ok(Key::Keyboard(KEY_RIGHT_ALT)),
            "KEY_RIGHT_SUPER" => Ok(Key::Keyboard(KEY_RIGHT_SUPER)),
            "KEY_KB_MENU" => Ok(Key::Keyboard(KEY_KB_MENU)),
            "KEY_KP_0" => Ok(Key::Keyboard(KEY_KP_0)),
            "KEY_KP_1" => Ok(Key::Keyboard(KEY_KP_1)),
            "KEY_KP_2" => Ok(Key::Keyboard(KEY_KP_2)),
            "KEY_KP_3" => Ok(Key::Keyboard(KEY_KP_3)),
            "KEY_KP_4" => Ok(Key::Keyboard(KEY_KP_4)),
            "KEY_KP_5" => Ok(Key::Keyboard(KEY_KP_5)),
            "KEY_KP_6" => Ok(Key::Keyboard(KEY_KP_6)),
            "KEY_KP_7" => Ok(Key::Keyboard(KEY_KP_7)),
            "KEY_KP_8" => Ok(Key::Keyboard(KEY_KP_8)),
            "KEY_KP_9" => Ok(Key::Keyboard(KEY_KP_9)),
            "KEY_KP_DECIMAL" => Ok(Key::Keyboard(KEY_KP_DECIMAL)),
            "KEY_KP_DIVIDE" => Ok(Key::Keyboard(KEY_KP_DIVIDE)),
            "KEY_KP_MULTIPLY" => Ok(Key::Keyboard(KEY_KP_MULTIPLY)),
            "KEY_KP_SUBTRACT" => Ok(Key::Keyboard(KEY_KP_SUBTRACT)),
            "KEY_KP_ADD" => Ok(Key::Keyboard(KEY_KP_ADD)),
            "KEY_KP_ENTER" => Ok(Key::Keyboard(KEY_KP_ENTER)),
            "KEY_KP_EQUAL" => Ok(Key::Keyboard(KEY_KP_EQUAL)),
            "KEY_BACK" => Ok(Key::Keyboard(KEY_BACK)),
            "KEY_MENU" => Ok(Key::Keyboard(KEY_MENU)),
            "KEY_VOLUME_UP" => Ok(Key::Keyboard(KEY_VOLUME_UP)),
            "KEY_VOLUME_DOWN" => Ok(Key::Keyboard(KEY_VOLUME_DOWN)),
            "MOUSE_BUTTON_LEFT" => Ok(Key::Mouse(MOUSE_BUTTON_LEFT)),
            "MOUSE_BUTTON_RIGHT" => Ok(Key::Mouse(MOUSE_BUTTON_RIGHT)),
            "MOUSE_BUTTON_MIDDLE" => Ok(Key::Mouse(MOUSE_BUTTON_MIDDLE)),
            "MOUSE_BUTTON_SIDE" => Ok(Key::Mouse(MOUSE_BUTTON_SIDE)),
            "MOUSE_BUTTON_EXTRA" => Ok(Key::Mouse(MOUSE_BUTTON_EXTRA)),
            "MOUSE_BUTTON_FORWARD" => Ok(Key::Mouse(MOUSE_BUTTON_FORWARD)),
            "MOUSE_BUTTON_BACK" => Ok(Key::Mouse(MOUSE_BUTTON_BACK)),
            _ => Ok(Key::Keyboard(KEY_NULL)),
        }
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
        match self {
            Key::Keyboard(keyboard_key) => match keyboard_key {
                KEY_NULL => serializer.serialize_str("KEY_NULL"),
                KEY_APOSTROPHE => serializer.serialize_str("KEY_APOSTROPHE"),
                KEY_COMMA => serializer.serialize_str("KEY_COMMA"),
                KEY_MINUS => serializer.serialize_str("KEY_MINUS"),
                KEY_PERIOD => serializer.serialize_str("KEY_PERIOD"),
                KEY_SLASH => serializer.serialize_str("KEY_SLASH"),
                KEY_ZERO => serializer.serialize_str("KEY_ZERO"),
                KEY_ONE => serializer.serialize_str("KEY_ONE"),
                KEY_TWO => serializer.serialize_str("KEY_TWO"),
                KEY_THREE => serializer.serialize_str("KEY_THREE"),
                KEY_FOUR => serializer.serialize_str("KEY_FOUR"),
                KEY_FIVE => serializer.serialize_str("KEY_FIVE"),
                KEY_SIX => serializer.serialize_str("KEY_SIX"),
                KEY_SEVEN => serializer.serialize_str("KEY_SEVEN"),
                KEY_EIGHT => serializer.serialize_str("KEY_EIGHT"),
                KEY_NINE => serializer.serialize_str("KEY_NINE"),
                KEY_SEMICOLON => serializer.serialize_str("KEY_SEMICOLON"),
                KEY_EQUAL => serializer.serialize_str("KEY_EQUAL"),
                KEY_A => serializer.serialize_str("KEY_A"),
                KEY_B => serializer.serialize_str("KEY_B"),
                KEY_C => serializer.serialize_str("KEY_C"),
                KEY_D => serializer.serialize_str("KEY_D"),
                KEY_E => serializer.serialize_str("KEY_E"),
                KEY_F => serializer.serialize_str("KEY_F"),
                KEY_G => serializer.serialize_str("KEY_G"),
                KEY_H => serializer.serialize_str("KEY_H"),
                KEY_I => serializer.serialize_str("KEY_I"),
                KEY_J => serializer.serialize_str("KEY_J"),
                KEY_K => serializer.serialize_str("KEY_K"),
                KEY_L => serializer.serialize_str("KEY_L"),
                KEY_M => serializer.serialize_str("KEY_M"),
                KEY_N => serializer.serialize_str("KEY_N"),
                KEY_O => serializer.serialize_str("KEY_O"),
                KEY_P => serializer.serialize_str("KEY_P"),
                KEY_Q => serializer.serialize_str("KEY_Q"),
                KEY_R => serializer.serialize_str("KEY_R"),
                KEY_S => serializer.serialize_str("KEY_S"),
                KEY_T => serializer.serialize_str("KEY_T"),
                KEY_U => serializer.serialize_str("KEY_U"),
                KEY_V => serializer.serialize_str("KEY_V"),
                KEY_W => serializer.serialize_str("KEY_W"),
                KEY_X => serializer.serialize_str("KEY_X"),
                KEY_Y => serializer.serialize_str("KEY_Y"),
                KEY_Z => serializer.serialize_str("KEY_Z"),
                KEY_LEFT_BRACKET => serializer.serialize_str("KEY_LEFT_BRACKET"),
                KEY_BACKSLASH => serializer.serialize_str("KEY_BACKSLASH"),
                KEY_RIGHT_BRACKET => serializer.serialize_str("KEY_RIGHT_BRACKET"),
                KEY_GRAVE => serializer.serialize_str("KEY_GRAVE"),
                KEY_SPACE => serializer.serialize_str("KEY_SPACE"),
                KEY_ESCAPE => serializer.serialize_str("KEY_ESCAPE"),
                KEY_ENTER => serializer.serialize_str("KEY_ENTER"),
                KEY_TAB => serializer.serialize_str("KEY_TAB"),
                KEY_BACKSPACE => serializer.serialize_str("KEY_BACKSPACE"),
                KEY_INSERT => serializer.serialize_str("KEY_INSERT"),
                KEY_DELETE => serializer.serialize_str("KEY_DELETE"),
                KEY_RIGHT => serializer.serialize_str("KEY_RIGHT"),
                KEY_LEFT => serializer.serialize_str("KEY_LEFT"),
                KEY_DOWN => serializer.serialize_str("KEY_DOWN"),
                KEY_UP => serializer.serialize_str("KEY_UP"),
                KEY_PAGE_UP => serializer.serialize_str("KEY_PAGE_UP"),
                KEY_PAGE_DOWN => serializer.serialize_str("KEY_PAGE_DOWN"),
                KEY_HOME => serializer.serialize_str("KEY_HOME"),
                KEY_END => serializer.serialize_str("KEY_END"),
                KEY_CAPS_LOCK => serializer.serialize_str("KEY_CAPS_LOCK"),
                KEY_SCROLL_LOCK => serializer.serialize_str("KEY_SCROLL_LOCK"),
                KEY_NUM_LOCK => serializer.serialize_str("KEY_NUM_LOCK"),
                KEY_PRINT_SCREEN => serializer.serialize_str("KEY_PRINT_SCREEN"),
                KEY_PAUSE => serializer.serialize_str("KEY_PAUSE"),
                KEY_F1 => serializer.serialize_str("KEY_F1"),
                KEY_F2 => serializer.serialize_str("KEY_F2"),
                KEY_F3 => serializer.serialize_str("KEY_F3"),
                KEY_F4 => serializer.serialize_str("KEY_F4"),
                KEY_F5 => serializer.serialize_str("KEY_F5"),
                KEY_F6 => serializer.serialize_str("KEY_F6"),
                KEY_F7 => serializer.serialize_str("KEY_F7"),
                KEY_F8 => serializer.serialize_str("KEY_F8"),
                KEY_F9 => serializer.serialize_str("KEY_F9"),
                KEY_F10 => serializer.serialize_str("KEY_F10"),
                KEY_F11 => serializer.serialize_str("KEY_F11"),
                KEY_F12 => serializer.serialize_str("KEY_F12"),
                KEY_LEFT_SHIFT => serializer.serialize_str("KEY_LEFT_SHIFT"),
                KEY_LEFT_CONTROL => serializer.serialize_str("KEY_LEFT_CONTROL"),
                KEY_LEFT_ALT => serializer.serialize_str("KEY_LEFT_ALT"),
                KEY_LEFT_SUPER => serializer.serialize_str("KEY_LEFT_SUPER"),
                KEY_RIGHT_SHIFT => serializer.serialize_str("KEY_RIGHT_SHIFT"),
                KEY_RIGHT_CONTROL => serializer.serialize_str("KEY_RIGHT_CONTROL"),
                KEY_RIGHT_ALT => serializer.serialize_str("KEY_RIGHT_ALT"),
                KEY_RIGHT_SUPER => serializer.serialize_str("KEY_RIGHT_SUPER"),
                KEY_KB_MENU => serializer.serialize_str("KEY_KB_MENU"),
                KEY_KP_0 => serializer.serialize_str("KEY_KP_0"),
                KEY_KP_1 => serializer.serialize_str("KEY_KP_1"),
                KEY_KP_2 => serializer.serialize_str("KEY_KP_2"),
                KEY_KP_3 => serializer.serialize_str("KEY_KP_3"),
                KEY_KP_4 => serializer.serialize_str("KEY_KP_4"),
                KEY_KP_5 => serializer.serialize_str("KEY_KP_5"),
                KEY_KP_6 => serializer.serialize_str("KEY_KP_6"),
                KEY_KP_7 => serializer.serialize_str("KEY_KP_7"),
                KEY_KP_8 => serializer.serialize_str("KEY_KP_8"),
                KEY_KP_9 => serializer.serialize_str("KEY_KP_9"),
                KEY_KP_DECIMAL => serializer.serialize_str("KEY_KP_DECIMAL"),
                KEY_KP_DIVIDE => serializer.serialize_str("KEY_KP_DIVIDE"),
                KEY_KP_MULTIPLY => serializer.serialize_str("KEY_KP_MULTIPLY"),
                KEY_KP_SUBTRACT => serializer.serialize_str("KEY_KP_SUBTRACT"),
                KEY_KP_ADD => serializer.serialize_str("KEY_KP_ADD"),
                KEY_KP_ENTER => serializer.serialize_str("KEY_KP_ENTER"),
                KEY_KP_EQUAL => serializer.serialize_str("KEY_KP_EQUAL"),
                KEY_BACK => serializer.serialize_str("KEY_BACK"),
                KEY_MENU => serializer.serialize_str("KEY_MENU"),
                KEY_VOLUME_UP => serializer.serialize_str("KEY_VOLUME_UP"),
                KEY_VOLUME_DOWN => serializer.serialize_str("KEY_VOLUME_DOWN"),
            },
            Key::Mouse(mouse_button) => match mouse_button {
                MOUSE_BUTTON_LEFT => serializer.serialize_str("MOUSE_BUTTON_LEFT"),
                MOUSE_BUTTON_RIGHT => serializer.serialize_str("MOUSE_BUTTON_RIGHT"),
                MOUSE_BUTTON_MIDDLE => serializer.serialize_str("MOUSE_BUTTON_MIDDLE"),
                MOUSE_BUTTON_SIDE => serializer.serialize_str("MOUSE_BUTTON_SIDE"),
                MOUSE_BUTTON_EXTRA => serializer.serialize_str("MOUSE_BUTTON_EXTRA"),
                MOUSE_BUTTON_FORWARD => serializer.serialize_str("MOUSE_BUTTON_FORWARD"),
                MOUSE_BUTTON_BACK => serializer.serialize_str("MOUSE_BUTTON_BACK"),
            },
        }
    }
}
