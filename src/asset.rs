use crate::helper::*;

//================================================================

use raylib::prelude::*;
use std::collections::HashMap;

//================================================================

pub struct Asset {
    pub inner: Inner,
    pub outer: Outer,
}

impl Asset {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            inner: Inner::new(handle, thread),
            outer: Outer::default(),
        }
    }
}

//================================================================

pub struct Inner {
    pub default: Texture2D,
    pub position: Texture2D,
    pub rotation: Texture2D,
    pub scale: Texture2D,
    pub vertex: Texture2D,
    pub edge: Texture2D,
    pub face: Texture2D,
}

impl Inner {
    const DEFAULT: &'static [u8] = include_bytes!("asset/default.png");
    const POSITION: &'static [u8] = include_bytes!("asset/position.png");
    const ROTATION: &'static [u8] = include_bytes!("asset/rotation.png");
    const SCALE: &'static [u8] = include_bytes!("asset/scale.png");
    const VERTEX: &'static [u8] = include_bytes!("asset/vertex.png");
    const EDGE: &'static [u8] = include_bytes!("asset/edge.png");
    const FACE: &'static [u8] = include_bytes!("asset/face.png");

    //================================================================

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            default: load_texture(handle, thread, Self::DEFAULT),
            position: load_texture(handle, thread, Self::POSITION),
            rotation: load_texture(handle, thread, Self::ROTATION),
            scale: load_texture(handle, thread, Self::SCALE),
            vertex: load_texture(handle, thread, Self::VERTEX),
            edge: load_texture(handle, thread, Self::EDGE),
            face: load_texture(handle, thread, Self::FACE),
        }
    }
}

//================================================================

#[derive(Default)]
pub struct Outer {
    pub model: HashMap<String, Model>,
    pub texture: HashMap<String, Model>,
}

//================================================================

fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Texture2D {
    let mut texture = handle
        .load_texture_from_image(
            thread,
            &Image::load_image_from_mem(".png", data)
                .map_err(|e| panic(&e.to_string()))
                .unwrap(),
        )
        .map_err(|e| panic(&e.to_string()))
        .unwrap();

    texture.gen_texture_mipmaps();

    texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);

    texture
}
