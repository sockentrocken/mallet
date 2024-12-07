use raylib::prelude::*;
use serde::Deserialize;
use serde::Serialize;

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
