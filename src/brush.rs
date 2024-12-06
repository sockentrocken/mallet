use raylib::prelude::*;

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
