use crate::editor::*;
use crate::game::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;

//================================================================

pub struct Script {
    pub lua: Lua,
    pub meta: Meta,
}

impl Script {
    pub fn new(game: &Game) -> Self {
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new()).unwrap();

        let global = lua.globals();
        let mallet = lua.create_table().unwrap();

        Self::system(&lua, &mallet);

        global.set("mallet", mallet).unwrap();

        lua.set_app_data(Meta::default());

        let package = global.get::<mlua::Table>("package").unwrap();
        let path = package.get::<mlua::String>("path").unwrap();
        package
            .set("path", format!("{path:?};{}/?.lua", game.path))
            .unwrap();

        lua.load("require \"main\"").exec().unwrap();

        let meta = lua.remove_app_data::<Meta>().unwrap();

        Self { lua, meta }
    }

    fn system(lua: &Lua, table: &mlua::Table) {
        table
            .set("map_entity", lua.create_function(Self::map_entity).unwrap())
            .unwrap();

        table
            .set(
                "map_texture",
                lua.create_function(Self::map_texture).unwrap(),
            )
            .unwrap();

        table
            .set("model", lua.create_function(Model::new).unwrap())
            .unwrap();
    }

    fn map_entity(lua: &Lua, (meta, call): (LuaValue, Option<mlua::Function>)) -> mlua::Result<()> {
        let mut app = lua.app_data_mut::<Meta>().unwrap();

        let entity = EntityLua {
            meta: lua.from_value(meta)?,
            call,
        };

        app.entity.push(entity);
        Ok(())
    }

    fn map_texture(lua: &Lua, path: String) -> mlua::Result<()> {
        let mut app = lua.app_data_mut::<Meta>().unwrap();

        app.texture.push(path);
        Ok(())
    }
}

//================================================================

#[derive(Default)]
pub struct Meta {
    pub entity: Vec<EntityLua>,
    pub texture: Vec<String>,
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Vector2Lua {
    pub x: f32,
    pub y: f32,
}

impl Into<ffi::Vector2> for Vector2Lua {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Vector2Lua {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Vector3Lua {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3Lua {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Into<ffi::Vector3> for Vector3Lua {
    fn into(self) -> ffi::Vector3 {
        ffi::Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

/* class
{ "name": "quiver.model", "info": "The model API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
    let model = lua.create_table()?;

    model.set("new", lua.create_function(self::Model::new)?)?;

    table.set("model", model)?;

    Ok(())
}

type RLModel = ffi::Model;

/* class
{ "name": "model", "info": "An unique handle for a model in memory." }
*/
pub struct Model(pub RLModel);

impl Model {
    /* entry
    {
        "name": "quiver.model.new",
        "info": "Create a new Model resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "Model", "info": "Model resource.", "kind": "Model" }
        ]
    }
    */
    fn new(_: &Lua, path: String) -> mlua::Result<Self> {
        let name = CString::new(path.clone()).map_err(|e| mlua::Error::runtime(e.to_string()))?;

        unsafe {
            let data = ffi::LoadModel(name.as_ptr());

            if ffi::IsModelValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "Model::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadModel(self.0);
        }
    }
}

impl mlua::UserData for Model {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "name": "model:draw", "info": "Draw the model." }
        */
        method.add_method("draw", |lua, this, point: [f32; 3]| unsafe {
            ffi::DrawModel(
                this.0,
                Vector3Lua::new(point[0], point[1], point[2]).into(),
                1.0,
                Color::RED.into(),
            );
            Ok(())
        });
    }
}
