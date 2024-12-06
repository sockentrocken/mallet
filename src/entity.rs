use serde::Deserialize;
use serde::Serialize;

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Entity {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub meta: EntityMeta,
}

#[derive(Deserialize, Serialize)]
pub struct EntityMeta {
    pub name: String,
    pub info: String,
    pub data: Vec<EntityData>,
    pub model: String,
    pub shape: [[f32; 3]; 2],
    pub color: [f32; 4],
}

#[derive(Deserialize, Serialize)]
pub struct EntityData {
    pub name: String,
    pub info: String,
    pub kind: serde_json::Value,
}
