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
    pub configuration: Texture2D,
    pub reload: Texture2D,
    pub import: Texture2D,
    pub export: Texture2D,
    pub exit: Texture2D,
}

impl Inner {
    const DEFAULT: &'static [u8] = include_bytes!("asset/stair.png");
    const POSITION: &'static [u8] = include_bytes!("asset/position.png");
    const ROTATION: &'static [u8] = include_bytes!("asset/rotation.png");
    const SCALE: &'static [u8] = include_bytes!("asset/scale.png");
    const VERTEX: &'static [u8] = include_bytes!("asset/vertex.png");
    const EDGE: &'static [u8] = include_bytes!("asset/edge.png");
    const FACE: &'static [u8] = include_bytes!("asset/face.png");
    const CONFIGURATION: &'static [u8] = include_bytes!("asset/configuration.png");
    const RELOAD: &'static [u8] = include_bytes!("asset/reload.png");
    const IMPORT: &'static [u8] = include_bytes!("asset/import.png");
    const EXPORT: &'static [u8] = include_bytes!("asset/export.png");
    const EXIT: &'static [u8] = include_bytes!("asset/exit.png");

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
            configuration: load_texture(handle, thread, Self::CONFIGURATION),
            reload: load_texture(handle, thread, Self::RELOAD),
            import: load_texture(handle, thread, Self::IMPORT),
            export: load_texture(handle, thread, Self::EXPORT),
            exit: load_texture(handle, thread, Self::EXIT),
        }
    }
}

//================================================================

#[derive(Default)]
pub struct Outer {
    pub texture: HashMap<String, Texture2D>,
}

impl Outer {
    pub fn set_texture(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread, path: &str) {
        let mut texture = handle.load_texture(&thread, path).unwrap();

        texture.gen_texture_mipmaps();

        texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);

        self.texture.insert(path.to_string(), texture);
    }

    pub fn set_texture_list(
        &mut self,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &[String],
    ) {
        for p in path {
            self.set_texture(handle, thread, p);
        }
    }
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
