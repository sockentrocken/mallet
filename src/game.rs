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
