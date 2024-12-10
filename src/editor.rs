use crate::game::*;
use crate::helper::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::{ffi::KeyboardKey::*, ffi::MouseButton::*, prelude::*};
use serde::{de, de::Visitor, Deserialize, Serialize};
use std::{collections::HashMap, ffi::CString, fmt};

//================================================================

pub struct Editor {
    pub brush: Vec<Brush>,
    pub entity: Vec<Entity>,
    pub widget: Widget,
    pub asset: Asset,
    pub view: [View; 4],
    pub game: Game,
    pub user: User,
    pub script: Script,
}

pub fn select() {}

impl Editor {
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, game: Game) -> Self {
        let mut asset = Asset::new(handle, thread);
        let mut script = Script::new(&game);

        asset.outer.set_texture_list(handle, thread, &script.meta.texture);

        Self {
            brush: vec![Brush::default()],
            //brush: Vec::default(),
            entity: Vec::default(),
            widget: Widget::default(),
            asset,
            view: [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(256.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 256.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 15.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 256.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
            ],
            user: User::new(),
            script,
            game,
        }
    }

    pub fn select(
        user: &User,
        brush: &mut [Brush],
        entity: &mut [Entity],
        widget: &Widget,
        draw: &mut RaylibDrawHandle,
        render_view: Rectangle,
        view: &Camera3D,
    ) {
        if user.interact.get_press(draw) {
            // get ray from camera.
            let ray = draw.get_screen_to_world_ray_ex(
                draw.get_mouse_position() - Vector2::new(render_view.x, render_view.y),
                view,
                render_view.width as i32,
                render_view.height as i32,
            );

            enum Picker {
                Brush(usize),
                Entity(usize),
            }

            let mut hit: Option<(Picker, f32)> = None;

            // for each brush...
            for (i, brush) in brush.iter().enumerate() {
                for face in &brush.face {
                    // generate quad.
                    let point = [
                        brush.vertex[face.index[0]],
                        brush.vertex[face.index[1]],
                        brush.vertex[face.index[2]],
                        brush.vertex[face.index[3]],
                    ];

                    let point = [
                        Vector3::new(point[0][0], point[0][1], point[0][2]),
                        Vector3::new(point[1][0], point[1][1], point[1][2]),
                        Vector3::new(point[2][0], point[2][1], point[2][2]),
                        Vector3::new(point[3][0], point[3][1], point[3][2]),
                    ];

                    // check for collision.
                    let ray = get_ray_collision_quad(ray, point[0], point[1], point[2], point[3]);

                    // collision hit; check if the entity is closer than the hit entity, or if there is no hit entity, set it as such.
                    if ray.hit {
                        if let Some((_, distance)) = hit {
                            if ray.distance < distance {
                                hit = Some((Picker::Brush(i), ray.distance));
                            }
                        } else {
                            hit = Some((Picker::Brush(i), ray.distance));
                        }
                    }
                }
            }

            // for each entity...
            for (i, entity) in entity.iter().enumerate() {
                // generate a bound-box.
                let shape = entity.lua.meta.shape;
                let min = Vector3::new(
                    entity.position[0] + shape[0][0],
                    entity.position[1] + shape[0][1],
                    entity.position[2] + shape[0][2],
                );
                let max = Vector3::new(
                    entity.position[0] + shape[1][0],
                    entity.position[1] + shape[1][1],
                    entity.position[2] + shape[1][2],
                );
                let shape = BoundingBox::new(min, max);

                // check for collision.
                let ray = shape.get_ray_collision_box(ray);

                // collision hit; check if the entity is closer than the hit entity, or if there is no hit entity, set it as such.
                if ray.hit {
                    if let Some((_, distance)) = hit {
                        if ray.distance < distance {
                            hit = Some((Picker::Entity(i), ray.distance));
                        }
                    } else {
                        hit = Some((Picker::Entity(i), ray.distance));
                    }
                }
            }

            // collision!
            if let Some(hit) = hit {
                if draw.is_key_up(KeyboardKey::KEY_LEFT_SHIFT) {
                    for e in &mut *brush {
                        e.focus = false;
                    }

                    for e in &mut *entity {
                        e.focus = false;
                    }
                }

                match hit.0 {
                    Picker::Brush(i) => {
                        let brush = brush.get_mut(i).unwrap();
                        brush.focus = !brush.focus;
                    }
                    Picker::Entity(i) => {
                        let entity = entity.get_mut(i).unwrap();
                        entity.focus = !entity.focus;
                    }
                }
            } else {
                for brush in &mut *brush {
                    brush.focus = false;
                }

                for entity in &mut *entity {
                    entity.focus = false;
                }
            }
        }

        let cross = view.up.cross((view.position - view.target).normalized());

        let x = {
            if user.move_y_a.get_press(draw) {
                -1.0
            } else if user.move_y_b.get_press(draw) {
                1.0
            } else {
                0.0
            }
        };
        let y = {
            if user.move_x_a.get_press(draw) {
                1.0
            } else if user.move_x_b.get_press(draw) {
                -1.0
            } else {
                0.0
            }
        };

        let cross = Vector3::new(
            cross.x * x + view.up.x * y,
            cross.y * x + view.up.y * y,
            cross.z * x + view.up.z * y,
        );

        let zero = cross == Vector3::zero();

        for entity in &mut *entity {
            if !entity.focus {
                continue;
            }

            if zero {
                return;
            }

            match widget {
                Widget::Position => entity.position(cross),
                Widget::Rotation => entity.rotation(cross),
                Widget::Scale => entity.scale(cross),
                _ => {}
            }
        }

        for brush in &mut *brush {
            if !brush.focus {
                continue;
            }

            if zero {
                return;
            }

            match widget {
                Widget::Position => brush.position(cross),
                Widget::Rotation => brush.rotation(cross),
                Widget::Scale => brush.scale(cross),
                _ => {}
            }
        }
    }

    #[rustfmt::skip]
    pub fn update(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread) {
        if draw.is_window_resized() {
            self.view = [
                View::new(draw, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(256.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(0.0, 256.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 15.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 256.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
            ];
        }

        for (i, view) in self.view.iter_mut().enumerate() {
            let render_view = Rectangle::new(
                (i as f32 % 2.0).floor() * view.render_texture.width() as f32,
                (i as f32 / 2.0).floor() * view.render_texture.height() as f32 + Window::TOOL_SHAPE,
                view.render_texture.width() as f32,
                view.render_texture.height() as f32,
            );

            if draw.is_key_pressed(KeyboardKey::KEY_DELETE) {
                let mut index: Vec<usize> = Vec::new();

                for (j, entity) in self.entity.iter().enumerate() {
                    if entity.focus {
                        index.push(j);
                    }
                }

                for (k, j) in index.iter().enumerate() {
                    self.entity.remove(j - k);
                }
            }

            if render_view.check_collision_point_rec(draw.get_mouse_position()) {
                Self::select(
                    &self.user,
                    &mut self.brush,
                    &mut self.entity,
                    &self.widget,
                    draw,
                    render_view,
                    &mut view.camera,
                );

                if self.user.look.get_down(draw) {
                    let cross = view
                        .camera
                        .up
                        .cross((view.camera.position - view.camera.target).normalized());

                    let delta = draw.get_mouse_delta() * 0.05;
                    let x = cross * delta.x;
                    let y = view.camera.up * -delta.y;

                    view.camera.position += x + y;
                    view.camera.target += x + y;
                }
            }

            {
                let mut draw_texture = draw.begin_texture_mode(thread, &mut view.render_texture);

                draw_texture.clear_background(Color::WHITE);

                let mut draw = draw_texture.begin_mode3D(view.camera);

                let angle = {
                    if view.camera.camera_type() == CameraProjection::CAMERA_PERSPECTIVE {
                        Vector4::default()
                    } else {
                        let cross = view
                            .camera
                            .up
                            .cross(view.camera.position - view.camera.target);

                        let angle = {
                            if view.camera.up == Vector3::up() {
                                90.0
                            } else {
                                180.0
                            }
                        };

                        Vector4::new(cross.x, cross.y, cross.z, angle)
                    }
                };

                draw_grid(1000, 1.0, angle);

                let mut x = Ray::default();
                x.direction = Vector3::new(1.0, 0.0, 0.0);
                let mut y = Ray::default();
                y.direction = Vector3::new(0.0, 1.0, 0.0);
                let mut z = Ray::default();
                z.direction = Vector3::new(0.0, 0.0, 1.0);

                draw.draw_ray(x, Color::RED);
                draw.draw_ray(y, Color::GREEN);
                draw.draw_ray(z, Color::BLUE);

                for brush in &self.brush {
                    brush.draw(&self.asset);

                    if brush.focus {
                        match self.widget {
                            Widget::Vertex => {
                                for v in brush.vertex {
                                    draw.draw_cube(
                                        Vector3::new(v[0], v[1], v[2]),
                                        0.5,
                                        0.5,
                                        0.5,
                                        Color::RED,
                                    );
                                }
                            }
                            Widget::Edge => {}
                            Widget::Face => {}
                            _ => {}
                        }
                    }
                }

                for entity in &self.entity {
                    entity.draw(&self.script.lua, &mut draw);
                }
            }

            draw.draw_texture_rec(
                &view.render_texture,
                Rectangle::new(
                    0.0,
                    0.0,
                    view.render_texture.width() as f32,
                    -view.render_texture.height() as f32,
                ),
                Vector2::new(render_view.x, render_view.y),
                Color::WHITE,
            );
        }
    }

    pub fn reload(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        self.entity.clear();
        self.asset.outer.texture.clear();
        self.script = Script::new(&self.game);
        self.asset
            .outer
            .set_texture_list(handle, thread, &self.script.meta.texture);
    }
}

//================================================================

pub struct Brush {
    pub vertex: [[f32; 3]; 8],
    pub face: [Face; 6],
    pub focus: bool,
}

impl Brush {
    pub const DEFAULT_SHAPE: f32 = 1.0;

    pub fn position(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            let vector = Vector3::new(v[0], v[1], v[2]);
            let vector = vector.transform_with(Matrix::translate(value.x, value.y, value.z));
            v[0] = vector.x;
            v[1] = vector.y;
            v[2] = vector.z;
        }
    }

    pub fn rotation(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            let vector = Vector3::new(v[0], v[1], v[2]);
            let vector = vector.transform_with(Matrix::rotate_xyz(value * DEG2RAD as f32 * 10.0));
            v[0] = vector.x;
            v[1] = vector.y;
            v[2] = vector.z;
        }
    }

    pub fn scale(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            let vector = Vector3::new(v[0], v[1], v[2]);
            let vector = vector.transform_with(Matrix::scale(value.x, value.y, value.z));
            v[0] = vector.x;
            v[1] = vector.y;
            v[2] = vector.z;
        }
    }

    pub fn draw(&self, asset: &Asset) {
        unsafe {
            // begin quad draw.
            ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());

            if self.focus {
                ffi::rlColor3f(1.0, 0.0, 0.0);
            } else {
                ffi::rlColor3f(1.0, 1.0, 1.0);
            }

            // for each vertex index, draw the corresponding face.
            for f in &self.face {
                // if we have a texture for this face, use it. otherwise, use the default.
                if let Some(texture) = &f.texture {
                    let texture = asset.outer.texture.get(texture).unwrap();
                    ffi::rlSetTexture(texture.id);
                } else {
                    ffi::rlSetTexture(asset.inner.default.id);
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
            face: Face::new_list(),
            focus: false,
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

//================================================================

pub struct Entity {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub focus: bool,
    pub lua: EntityLua,
}

impl Entity {
    pub fn new_from_lua(lua: EntityLua) -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            focus: false,
            lua,
        }
    }

    pub fn position(&mut self, value: Vector3) {
        self.position[0] += value.x;
        self.position[1] += value.y;
        self.position[2] += value.z;
    }

    pub fn rotation(&mut self, value: Vector3) {
        self.rotation[0] += value.x;
        self.rotation[1] += value.y;
        self.rotation[2] += value.z;
    }

    pub fn scale(&mut self, value: Vector3) {
        self.scale[0] += value.x;
        self.scale[1] += value.y;
        self.scale[2] += value.z;
    }

    pub fn draw(&self, lua: &Lua, draw: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        let shape = self.lua.meta.shape;
        let min = Vector3::new(
            self.position[0] + shape[0][0],
            self.position[1] + shape[0][1],
            self.position[2] + shape[0][2],
        );
        let max = Vector3::new(
            self.position[0] + shape[1][0],
            self.position[1] + shape[1][1],
            self.position[2] + shape[1][2],
        );

        draw.draw_bounding_box(
            BoundingBox::new(min, max),
            if self.focus { Color::GREEN } else { Color::RED },
        );

        let data = lua.to_value(&self.lua.meta.data).unwrap();

        if let Some(call) = &self.lua.call {
            call.call::<()>((self.position, self.rotation, self.scale, data))
                .unwrap();
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
    pub data: HashMap<String, EntityData>,
    pub shape: [[f32; 3]; 2],
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EntityData {
    pub info: String,
    pub kind: serde_json::Value,
}

//================================================================

#[derive(Default)]
pub enum Widget {
    #[default]
    Position = 0,
    Rotation = 1,
    Scale = 2,
    Vertex = 3,
    Edge = 4,
    Face = 5,
}

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

pub struct Inner {
    pub default: Texture2D,
    pub texture: Texture2D,
    pub entity: Texture2D,
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
    const DEFAULT: &'static [u8] = include_bytes!("asset/default.png");
    const TEXTURE: &'static [u8] = include_bytes!("asset/texture.png");
    const ENTITY: &'static [u8] = include_bytes!("asset/entity.png");
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

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            default: load_texture(handle, thread, Self::DEFAULT),
            texture: load_texture(handle, thread, Self::TEXTURE),
            entity: load_texture(handle, thread, Self::ENTITY),
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

//================================================================

pub struct View {
    pub render_texture: RenderTexture2D,
    pub camera: Camera3D,
}

impl View {
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, camera: Camera3D) -> Self {
        Self {
            render_texture: handle
                .load_render_texture(
                    &thread,
                    ((handle.get_screen_width() - Window::EDIT_SHAPE as i32) as f32 / 2.0) as u32,
                    ((handle.get_screen_height() - Window::TOOL_SHAPE as i32) as f32 / 2.0) as u32,
                )
                .unwrap(),
            camera,
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct User {
    pub mouse_speed: [f32; 2],
    pub move_x_a: Input,
    pub move_x_b: Input,
    pub move_y_a: Input,
    pub move_y_b: Input,
    pub interact: Input,
    pub look: Input,
    pub texture: Input,
    pub entity: Input,
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

    pub fn new() -> Self {
        // check if file does exist, otherwise, return default.
        if std::path::Path::new(Self::FILE_NAME).is_file() {
            // read file.
            let user = std::fs::read_to_string(Self::FILE_NAME)
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
            interact:      Input::new(None, Key::Mouse(MOUSE_BUTTON_LEFT)),
            look:          Input::new(None, Key::Mouse(MOUSE_BUTTON_RIGHT)),
            texture:       Input::new(None, Key::Keyboard(KEY_SEVEN)),
            entity:        Input::new(None, Key::Keyboard(KEY_EIGHT)),
            position:      Input::new(None, Key::Keyboard(KEY_ONE)),
            rotation:      Input::new(None, Key::Keyboard(KEY_TWO)),
            scale:         Input::new(None, Key::Keyboard(KEY_THREE)),
            vertex:        Input::new(None, Key::Keyboard(KEY_FOUR)),
            edge:          Input::new(None, Key::Keyboard(KEY_FIVE)),
            face:          Input::new(None, Key::Keyboard(KEY_SIX)),
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

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "Null" => Key::Keyboard(KEY_NULL),
            "Apostrophe" => Key::Keyboard(KEY_APOSTROPHE),
            "Comma" => Key::Keyboard(KEY_COMMA),
            "Minus" => Key::Keyboard(KEY_MINUS),
            "Period" => Key::Keyboard(KEY_PERIOD),
            "Slash" => Key::Keyboard(KEY_SLASH),
            "Zero" => Key::Keyboard(KEY_ZERO),
            "1" => Key::Keyboard(KEY_ONE),
            "2" => Key::Keyboard(KEY_TWO),
            "3" => Key::Keyboard(KEY_THREE),
            "4" => Key::Keyboard(KEY_FOUR),
            "5" => Key::Keyboard(KEY_FIVE),
            "6" => Key::Keyboard(KEY_SIX),
            "7" => Key::Keyboard(KEY_SEVEN),
            "8" => Key::Keyboard(KEY_EIGHT),
            "9" => Key::Keyboard(KEY_NINE),
            "Semicolon" => Key::Keyboard(KEY_SEMICOLON),
            "Equal" => Key::Keyboard(KEY_EQUAL),
            "A" => Key::Keyboard(KEY_A),
            "B" => Key::Keyboard(KEY_B),
            "C" => Key::Keyboard(KEY_C),
            "D" => Key::Keyboard(KEY_D),
            "E" => Key::Keyboard(KEY_E),
            "F" => Key::Keyboard(KEY_F),
            "G" => Key::Keyboard(KEY_G),
            "H" => Key::Keyboard(KEY_H),
            "I" => Key::Keyboard(KEY_I),
            "J" => Key::Keyboard(KEY_J),
            "K" => Key::Keyboard(KEY_K),
            "L" => Key::Keyboard(KEY_L),
            "M" => Key::Keyboard(KEY_M),
            "N" => Key::Keyboard(KEY_N),
            "O" => Key::Keyboard(KEY_O),
            "P" => Key::Keyboard(KEY_P),
            "Q" => Key::Keyboard(KEY_Q),
            "R" => Key::Keyboard(KEY_R),
            "S" => Key::Keyboard(KEY_S),
            "T" => Key::Keyboard(KEY_T),
            "U" => Key::Keyboard(KEY_U),
            "V" => Key::Keyboard(KEY_V),
            "W" => Key::Keyboard(KEY_W),
            "X" => Key::Keyboard(KEY_X),
            "Y" => Key::Keyboard(KEY_Y),
            "Z" => Key::Keyboard(KEY_Z),
            "Left Bracket" => Key::Keyboard(KEY_LEFT_BRACKET),
            "Backslash" => Key::Keyboard(KEY_BACKSLASH),
            "Right Bracket" => Key::Keyboard(KEY_RIGHT_BRACKET),
            "Grave" => Key::Keyboard(KEY_GRAVE),
            "Space" => Key::Keyboard(KEY_SPACE),
            "Escape" => Key::Keyboard(KEY_ESCAPE),
            "Enter" => Key::Keyboard(KEY_ENTER),
            "Tabulator" => Key::Keyboard(KEY_TAB),
            "Backspace" => Key::Keyboard(KEY_BACKSPACE),
            "Insert" => Key::Keyboard(KEY_INSERT),
            "Delete" => Key::Keyboard(KEY_DELETE),
            "Right" => Key::Keyboard(KEY_RIGHT),
            "Left" => Key::Keyboard(KEY_LEFT),
            "Down" => Key::Keyboard(KEY_DOWN),
            "Up" => Key::Keyboard(KEY_UP),
            "Page Up" => Key::Keyboard(KEY_PAGE_UP),
            "Page Down" => Key::Keyboard(KEY_PAGE_DOWN),
            "Home" => Key::Keyboard(KEY_HOME),
            "End" => Key::Keyboard(KEY_END),
            "Caps Lock" => Key::Keyboard(KEY_CAPS_LOCK),
            "Scroll Lock" => Key::Keyboard(KEY_SCROLL_LOCK),
            "Number Lock" => Key::Keyboard(KEY_NUM_LOCK),
            "Print Screen" => Key::Keyboard(KEY_PRINT_SCREEN),
            "Pause" => Key::Keyboard(KEY_PAUSE),
            "F1" => Key::Keyboard(KEY_F1),
            "F2" => Key::Keyboard(KEY_F2),
            "F3" => Key::Keyboard(KEY_F3),
            "F4" => Key::Keyboard(KEY_F4),
            "F5" => Key::Keyboard(KEY_F5),
            "F6" => Key::Keyboard(KEY_F6),
            "F7" => Key::Keyboard(KEY_F7),
            "F8" => Key::Keyboard(KEY_F8),
            "F9" => Key::Keyboard(KEY_F9),
            "F10" => Key::Keyboard(KEY_F10),
            "F11" => Key::Keyboard(KEY_F11),
            "F12" => Key::Keyboard(KEY_F12),
            "L. Shift" => Key::Keyboard(KEY_LEFT_SHIFT),
            "L. Control" => Key::Keyboard(KEY_LEFT_CONTROL),
            "L. Alt" => Key::Keyboard(KEY_LEFT_ALT),
            "L. Super" => Key::Keyboard(KEY_LEFT_SUPER),
            "R. Shift" => Key::Keyboard(KEY_RIGHT_SHIFT),
            "R. Control" => Key::Keyboard(KEY_RIGHT_CONTROL),
            "R. Alt" => Key::Keyboard(KEY_RIGHT_ALT),
            "R. Super" => Key::Keyboard(KEY_RIGHT_SUPER),
            "Keyboard Menu" => Key::Keyboard(KEY_KB_MENU),
            "Pad 0" => Key::Keyboard(KEY_KP_0),
            "Pad 1" => Key::Keyboard(KEY_KP_1),
            "Pad 2" => Key::Keyboard(KEY_KP_2),
            "Pad 3" => Key::Keyboard(KEY_KP_3),
            "Pad 4" => Key::Keyboard(KEY_KP_4),
            "Pad 5" => Key::Keyboard(KEY_KP_5),
            "Pad 6" => Key::Keyboard(KEY_KP_6),
            "Pad 7" => Key::Keyboard(KEY_KP_7),
            "Pad 8" => Key::Keyboard(KEY_KP_8),
            "Pad 9" => Key::Keyboard(KEY_KP_9),
            "Pad Decimal" => Key::Keyboard(KEY_KP_DECIMAL),
            "Pad Divide" => Key::Keyboard(KEY_KP_DIVIDE),
            "Pad Multiply" => Key::Keyboard(KEY_KP_MULTIPLY),
            "Pad Subtract" => Key::Keyboard(KEY_KP_SUBTRACT),
            "Pad Add" => Key::Keyboard(KEY_KP_ADD),
            "Pad Enter" => Key::Keyboard(KEY_KP_ENTER),
            "Pad Equal" => Key::Keyboard(KEY_KP_EQUAL),
            "Back" => Key::Keyboard(KEY_BACK),
            "Menu" => Key::Keyboard(KEY_MENU),
            "Volume Up" => Key::Keyboard(KEY_VOLUME_UP),
            "Volume Down" => Key::Keyboard(KEY_VOLUME_DOWN),
            "Mouse Left" => Key::Mouse(MOUSE_BUTTON_LEFT),
            "Mouse Right" => Key::Mouse(MOUSE_BUTTON_RIGHT),
            "Mouse Middle" => Key::Mouse(MOUSE_BUTTON_MIDDLE),
            "Mouse Side" => Key::Mouse(MOUSE_BUTTON_SIDE),
            "Mouse Extra" => Key::Mouse(MOUSE_BUTTON_EXTRA),
            "Mouse Forward" => Key::Mouse(MOUSE_BUTTON_FORWARD),
            "Mouse Back" => Key::Mouse(MOUSE_BUTTON_BACK),
            _ => Key::Keyboard(KEY_NULL),
        }
    }
}

impl From<Key> for &str {
    fn from(value: Key) -> Self {
        match value {
            Key::Keyboard(keyboard_key) => match keyboard_key {
                KEY_NULL => "Null",
                KEY_APOSTROPHE => "Apostrophe",
                KEY_COMMA => "Comma",
                KEY_MINUS => "Minus",
                KEY_PERIOD => "Period",
                KEY_SLASH => "Slash",
                KEY_ZERO => "Zero",
                KEY_ONE => "1",
                KEY_TWO => "2",
                KEY_THREE => "3",
                KEY_FOUR => "4",
                KEY_FIVE => "5",
                KEY_SIX => "6",
                KEY_SEVEN => "7",
                KEY_EIGHT => "8",
                KEY_NINE => "9",
                KEY_SEMICOLON => "Semicolon",
                KEY_EQUAL => "Equal",
                KEY_A => "A",
                KEY_B => "B",
                KEY_C => "C",
                KEY_D => "D",
                KEY_E => "E",
                KEY_F => "F",
                KEY_G => "G",
                KEY_H => "H",
                KEY_I => "I",
                KEY_J => "J",
                KEY_K => "K",
                KEY_L => "L",
                KEY_M => "M",
                KEY_N => "N",
                KEY_O => "O",
                KEY_P => "P",
                KEY_Q => "Q",
                KEY_R => "R",
                KEY_S => "S",
                KEY_T => "T",
                KEY_U => "U",
                KEY_V => "V",
                KEY_W => "W",
                KEY_X => "X",
                KEY_Y => "Y",
                KEY_Z => "Z",
                KEY_LEFT_BRACKET => "Left Bracket",
                KEY_BACKSLASH => "Backslash",
                KEY_RIGHT_BRACKET => "Right Bracket",
                KEY_GRAVE => "Grave",
                KEY_SPACE => "Space",
                KEY_ESCAPE => "Escape",
                KEY_ENTER => "Enter",
                KEY_TAB => "Tabulator",
                KEY_BACKSPACE => "Backspace",
                KEY_INSERT => "Insert",
                KEY_DELETE => "Delete",
                KEY_RIGHT => "Right",
                KEY_LEFT => "Left",
                KEY_DOWN => "Down",
                KEY_UP => "Up",
                KEY_PAGE_UP => "Page Up",
                KEY_PAGE_DOWN => "Page Down",
                KEY_HOME => "Home",
                KEY_END => "End",
                KEY_CAPS_LOCK => "Caps Lock",
                KEY_SCROLL_LOCK => "Scroll Lock",
                KEY_NUM_LOCK => "Number Lock",
                KEY_PRINT_SCREEN => "Print Screen",
                KEY_PAUSE => "Pause",
                KEY_F1 => "F1",
                KEY_F2 => "F2",
                KEY_F3 => "F3",
                KEY_F4 => "F4",
                KEY_F5 => "F5",
                KEY_F6 => "F6",
                KEY_F7 => "F7",
                KEY_F8 => "F8",
                KEY_F9 => "F9",
                KEY_F10 => "F10",
                KEY_F11 => "F11",
                KEY_F12 => "F12",
                KEY_LEFT_SHIFT => "L. Shift",
                KEY_LEFT_CONTROL => "L. Control",
                KEY_LEFT_ALT => "L. Alt",
                KEY_LEFT_SUPER => "L. Super",
                KEY_RIGHT_SHIFT => "R. Shift",
                KEY_RIGHT_CONTROL => "R. Control",
                KEY_RIGHT_ALT => "R. Alt",
                KEY_RIGHT_SUPER => "R. Super",
                KEY_KB_MENU => "Keyboard Menu",
                KEY_KP_0 => "Pad 0",
                KEY_KP_1 => "Pad 1",
                KEY_KP_2 => "Pad 2",
                KEY_KP_3 => "Pad 3",
                KEY_KP_4 => "Pad 4",
                KEY_KP_5 => "Pad 5",
                KEY_KP_6 => "Pad 6",
                KEY_KP_7 => "Pad 7",
                KEY_KP_8 => "Pad 8",
                KEY_KP_9 => "Pad 9",
                KEY_KP_DECIMAL => "Pad Decimal",
                KEY_KP_DIVIDE => "Pad Divide",
                KEY_KP_MULTIPLY => "Pad Multiply",
                KEY_KP_SUBTRACT => "Pad Subtract",
                KEY_KP_ADD => "Pad Add",
                KEY_KP_ENTER => "Pad Enter",
                KEY_KP_EQUAL => "Pad Equal",
                KEY_BACK => "Back",
                KEY_MENU => "Menu",
                KEY_VOLUME_UP => "Volume Up",
                KEY_VOLUME_DOWN => "Volume Down",
            },
            Key::Mouse(mouse_button) => match mouse_button {
                MOUSE_BUTTON_LEFT => "Mouse Left",
                MOUSE_BUTTON_RIGHT => "Mouse Right",
                MOUSE_BUTTON_MIDDLE => "Mouse Middle",
                MOUSE_BUTTON_SIDE => "Mouse Side",
                MOUSE_BUTTON_EXTRA => "Mouse Extra",
                MOUSE_BUTTON_FORWARD => "Mouse Forward",
                MOUSE_BUTTON_BACK => "Mouse Back",
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
pub enum Key {
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

//================================================================

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
        method.add_method_mut(
            "draw",
            |_lua, this, (position, rotation, scale): ([f32; 3], [f32; 3], [f32; 3])| unsafe {
                this.0.transform = (Matrix::rotate_xyz(Vector3::new(
                    rotation[0] * DEG2RAD as f32,
                    rotation[1] * DEG2RAD as f32,
                    rotation[2] * DEG2RAD as f32,
                )) * Matrix::scale(scale[0], scale[1], scale[2]))
                .into();

                ffi::DrawModel(
                    this.0,
                    Vector3::new(position[0], position[1], position[2]).into(),
                    1.0,
                    Color::RED.into(),
                );

                this.0.transform = Matrix::identity().into();
                Ok(())
            },
        );
    }
}
