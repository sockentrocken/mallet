/*
* BSD Zero Clause License
*
* Copyright (c) 2025 sockentrocken
*
* Permission to use, copy, modify, and/or distribute this software for any
* purpose with or without fee is hereby granted.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
* REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
* AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
* INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
* LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
* OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
* PERFORMANCE OF THIS SOFTWARE.
*/

use crate::helper::*;

//================================================================

use serde::{Deserialize, Serialize};

//================================================================

#[derive(Clone)]
pub struct Game {
    pub info: Info,
    pub data: Option<Data>,
    pub path: String,
}

impl Game {
    pub fn new(info: Info, data: Option<Data>, path: String) -> Self {
        Self { info, data, path }
    }

    pub fn new_list() -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();

        let info_mallet = InfoMallet::new();

        for file in info_mallet.path {
            result.push(Game::new(
                Info::new_from_file(&file),
                Data::new_from_file(&file),
                file,
            ));
        }

        result
    }
}

//================================================================

#[derive(Clone, Deserialize)]
pub struct Info {
    pub name: String,
}

impl Info {
    pub const FILE_NAME: &'static str = "info.json";

    pub fn new_from_file(path: &str) -> Self {
        let path = format!("{path}/{}", Self::FILE_NAME);

        let data = std::fs::read_to_string(path)
            .map_err(|e| panic(&format!("Info::new_from_file(): {e}")))
            .unwrap();
        serde_json::from_str(&data)
            .map_err(|e| panic(&format!("Info::new_from_file(): {e}")))
            .unwrap()
    }
}

//================================================================

#[derive(Clone, Deserialize)]
pub struct InfoMallet {
    pub path: Vec<String>,
}

impl InfoMallet {
    pub const FILE_NAME: &'static str = "info_mallet.json";

    pub fn new() -> Self {
        let data = std::fs::read_to_string(Self::FILE_NAME)
            .map_err(|e| panic(&format!("InfoMallet::new_from_file(): {e}")))
            .unwrap();
        serde_json::from_str(&data)
            .map_err(|e| panic(&format!("InfoMallet::new_from_file(): {e}")))
            .unwrap()
    }
}

//================================================================

#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    pub path: String,
    pub configuration: Vec<Configuration>,
}

impl Data {
    pub const FILE_NAME: &'static str = "data.json";

    pub fn new_from_file(path: &str) -> Option<Self> {
        let path = format!("{path}/{}", Self::FILE_NAME);

        if std::path::Path::new(&path).is_file() {
            let data = std::fs::read_to_string(path)
                .map_err(|e| panic(&format!("Data::new_from_file(): {e}")))
                .unwrap();
            serde_json::from_str(&data)
                .map_err(|e| panic(&format!("Data::new_from_file(): {e}")))
                .unwrap()
        } else {
            None
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub name: String,
    pub info: String,
    pub kind: serde_json::Value,
}
