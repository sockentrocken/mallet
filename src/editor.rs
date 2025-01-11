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

use crate::game::*;
use crate::helper::*;
use crate::window::*;

//================================================================

use mlua::prelude::*;
use raylib::{ffi::KeyboardKey::*, ffi::MouseButton::*, prelude::*};
use serde::{de, de::Visitor, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::CString,
    fmt,
};

//================================================================

pub struct Editor {
    pub asset: Asset,
    pub world: World,
    pub widget: Widget,
    pub view: [View; 4],
    pub game: Game,
    pub user: User,
    pub script: Script,
    pub search_ent: String,
    pub search_tex: String,
    pub menu: bool,
}

impl Editor {
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, game: Game) -> Self {
        let mut asset = Asset::new(handle, thread);
        let script = Script::new(&game)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();

        asset.outer.set_texture_list(handle, thread, &script.meta.texture);

        Self {
            world: World::default(),
            widget: Widget::default(),
            asset,
            view: [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(512.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 512.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 15.0)),
                View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 512.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
            ],
            user: User::new(),
            script,
            game,
            search_ent: String::default(),
            search_tex: String::default(),
            menu: bool::default(),
        }
    }

    pub fn select(
        user: &User,
        world: &mut World,
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
                Vertex(usize, usize),
            }

            let mut hit: Option<(Picker, f32)> = None;

            // for each brush...
            for (i, brush) in world.brush.iter().enumerate() {
                if brush.focus {
                    // based on which widget is selected, do per vertex/per edge/per face picking.
                    for (j, vertex) in brush.vertex.iter().enumerate() {
                        // generate a bound-box.
                        let shape = BoundingBox::new(
                            -(Vector3::one() * 0.5) + vertex.point,
                            (Vector3::one() * 0.5) + vertex.point,
                        );

                        // check for collision.
                        let ray = shape.get_ray_collision_box(ray);

                        // collision hit; check if the entity is closer than the hit entity, or if there is no hit entity, set it as such.
                        if ray.hit {
                            if let Some((_, distance)) = hit {
                                if ray.distance < distance {
                                    hit = Some((Picker::Vertex(i, j), ray.distance));
                                }
                            } else {
                                hit = Some((Picker::Vertex(i, j), ray.distance));
                            }
                        }
                    }
                } else {
                    for face in &brush.face {
                        // generate quad.
                        let point = [
                            brush.vertex[face.index[0]].point,
                            brush.vertex[face.index[1]].point,
                            brush.vertex[face.index[2]].point,
                            brush.vertex[face.index[3]].point,
                        ];

                        // check for collision.
                        let ray =
                            get_ray_collision_quad(ray, point[0], point[1], point[2], point[3]);

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
            }

            // for each entity...
            for (i, entity) in world.entity.iter().enumerate() {
                // generate a bound-box.
                let shape = entity.bound_box();

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
                if !draw.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                    world.select_all(false);
                }

                match hit.0 {
                    Picker::Brush(i) => {
                        world.select_all(false);
                        let brush = world.brush.get_mut(i).unwrap();
                        brush.focus = !brush.focus;
                    }
                    Picker::Entity(i) => {
                        world.select_all(false);

                        let entity = world.entity.get_mut(i).unwrap();
                        entity.focus = !entity.focus;
                    }
                    Picker::Vertex(i, j) => {
                        let brush = world.brush.get_mut(i).unwrap();
                        brush.focus = true;
                        brush.vertex[j].focus = !brush.vertex[j].focus;
                    }
                }
            } else {
                world.select_all(false);
            }
        }

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

        let cross = {
            match view.camera_type() {
                CameraProjection::CAMERA_PERSPECTIVE => {
                    let cross = view.up.cross((view.position - view.target).normalized());

                    let v_x = Vector3::new(1.0, 0.0, 0.0)
                        .dot((view.position - view.target).normalized())
                        .signum();
                    let v_y = Vector3::new(0.0, 0.0, 1.0).dot(cross).signum();

                    if user.look.get_down(draw) {
                        Vector3::zero()
                    } else {
                        Vector3::new(
                            (Vector3::new(1.0, 0.0, 0.0).x * x * v_x)
                                + (Vector3::new(0.0, 0.0, 1.0).x * y * v_y),
                            (Vector3::new(1.0, 0.0, 0.0).y * x * v_x)
                                + (Vector3::new(0.0, 0.0, 1.0).y * y * v_y),
                            (Vector3::new(1.0, 0.0, 0.0).z * x * v_x)
                                + (Vector3::new(0.0, 0.0, 1.0).z * y * v_y),
                        )
                    }
                }
                CameraProjection::CAMERA_ORTHOGRAPHIC => {
                    let cross = view.up.cross((view.position - view.target).normalized());

                    if user.look.get_down(draw) {
                        Vector3::zero()
                    } else {
                        Vector3::new(
                            cross.x * x + view.up.x * y,
                            cross.y * x + view.up.y * y,
                            cross.z * x + view.up.z * y,
                        )
                    }
                }
            }
        };

        let zero = cross == Vector3::zero();

        for entity in &mut world.entity {
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

        for brush in &mut world.brush {
            if !brush.focus {
                continue;
            }

            if zero {
                return;
            }

            for vertex in &mut brush.vertex {
                if !vertex.focus {
                    continue;
                }

                vertex.point += cross;
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
    pub fn update(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread, asset: &Asset) {
        if draw.is_window_resized() {
            self.view = [
                View::new(draw, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(512.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(0.0, 512.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 15.0)),
                View::new(draw, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 512.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 15.0)),
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

                for (j, entity) in self.world.entity.iter().enumerate() {
                    if entity.focus {
                        index.push(j);
                    }
                }

                for (k, j) in index.iter().enumerate() {
                    self.world.entity.remove(j - k);
                }
            }

            if view.mouse {
                match view.camera.camera_type() {
                    CameraProjection::CAMERA_PERSPECTIVE => {
                        draw.update_camera(&mut view.camera, CameraMode::CAMERA_FIRST_PERSON);
                    },
                    CameraProjection::CAMERA_ORTHOGRAPHIC => {
                        let cross = view
                            .camera
                            .up
                            .cross((view.camera.position - view.camera.target).normalized());

                        let delta = draw.get_mouse_delta() * 0.05;
                        let x = cross * delta.x;
                        let y = view.camera.up * -delta.y;

                        view.camera.position += x + y;
                        view.camera.target   += x + y;
                    },
                }
            }

            if self.user.look.get_release(draw) && view.mouse {
                view.mouse = false;
            }

            if render_view.check_collision_point_rec(draw.get_mouse_position()) {
                Self::select(
                    &self.user,
                    &mut self.world,
                    &self.widget,
                    draw,
                    render_view,
                    &mut view.camera,
                );

                if self.user.look.get_press(draw) {
                    view.mouse = true;
                };
            }

            {
                let port = Vector2::new(
                    view.render_texture.width() as f32,
                    view.render_texture.height() as f32,
                );

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

                for brush in &self.world.brush {
                    brush.draw(&self.asset);

                    if brush.focus {
                        match self.widget {
                            Widget::Vertex => {
                                for v in &brush.vertex {
                                    draw.draw_cube(
                                        v.point,
                                        0.5,
                                        0.5,
                                        0.5,
                                        if v.focus {
                                            Color::GREEN
                                        } else {
                                            Color::RED
                                        }
                                        ,
                                    );
                                }
                            }
                            Widget::Edge => {}
                            Widget::Face => {}
                            _ => {}
                        }
                    }
                }

                for entity in &self.world.entity {
                    entity.draw_3d(&self.script.lua, &mut draw);
                }

                drop(draw);

                let mut draw = draw_texture.begin_mode2D(Camera2D {
                    offset: Vector2::default(),
                    target: Vector2::default(),
                    rotation: 0.0,
                    zoom: 1.0,
                });

                for entity in &self.world.entity {
                    entity.draw_2d(&mut draw, asset, &view.camera, port);
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
        self.asset.outer.texture.clear();
        self.script = Script::new(&self.game)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();
        self.asset
            .outer
            .set_texture_list(handle, thread, &self.script.meta.texture);
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct World {
    pub brush: Vec<Brush>,
    pub entity: Vec<Entity>,
}

impl World {
    pub fn select_all(&mut self, value: bool) {
        for brush in &mut self.brush {
            brush.focus = value;

            for vertex in &mut brush.vertex {
                vertex.focus = value;
            }

            for face in &mut brush.face {
                face.focus = value;
            }
        }

        for entity in &mut self.entity {
            entity.focus = value;
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            brush: vec![Brush::default()],
            entity: vec![],
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Brush {
    pub vertex: [Vertex; 8],
    pub face: [Face; 6],
    pub focus: bool,
}

impl Brush {
    pub const DEFAULT_SHAPE: f32 = 1.0;

    pub fn position(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            v.point = v
                .point
                .transform_with(Matrix::translate(value.x, value.y, value.z));
        }
    }

    pub fn rotation(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            v.point = v
                .point
                .transform_with(Matrix::rotate_xyz(value * DEG2RAD as f32 * 10.0));
        }
    }

    pub fn scale(&mut self, value: Vector3) {
        for v in &mut self.vertex {
            v.point = v
                .point
                .transform_with(Matrix::scale(value.x, value.y, value.z));
        }
    }

    pub fn draw(&self, asset: &Asset) {
        unsafe {
            // begin quad draw.
            ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());

            if self.focus {
                ffi::rlColor3f(1.00, 0.75, 0.75);
            } else {
                ffi::rlColor3f(1.00, 1.00, 1.00);
            }

            // for each vertex index, draw the corresponding face.
            for f in &self.face {
                // if we have a texture for this face, use it. otherwise, use the default.
                if let Some(texture) = &f.texture {
                    if let Some(texture) = asset.outer.texture.get(texture) {
                        // texture does exist, use it.
                        ffi::rlSetTexture(texture.id);
                    } else {
                        // we are pointing to a texture that does not exist...use the default texture.
                        ffi::rlSetTexture(asset.inner.default.id);
                    }
                } else {
                    ffi::rlSetTexture(asset.inner.default.id);
                }

                ffi::rlTexCoord2f(f.scale.x * (f.shift.x + 0.0), f.scale.y * (f.shift.y + 1.0));
                ffi::rlVertex3f(
                    self.vertex[f.index[0]].point.x,
                    self.vertex[f.index[0]].point.y,
                    self.vertex[f.index[0]].point.z,
                );
                ffi::rlTexCoord2f(f.scale.x * (f.shift.x + 1.0), f.scale.y * (f.shift.y + 1.0));
                ffi::rlVertex3f(
                    self.vertex[f.index[1]].point.x,
                    self.vertex[f.index[1]].point.y,
                    self.vertex[f.index[1]].point.z,
                );
                ffi::rlTexCoord2f(f.scale.x * (f.shift.x + 1.0), f.scale.y * (f.shift.y + 0.0));
                ffi::rlVertex3f(
                    self.vertex[f.index[2]].point.x,
                    self.vertex[f.index[2]].point.y,
                    self.vertex[f.index[2]].point.z,
                );
                ffi::rlTexCoord2f(f.scale.x * (f.shift.x + 0.0), f.scale.y * (f.shift.y + 0.0));
                ffi::rlVertex3f(
                    self.vertex[f.index[3]].point.x,
                    self.vertex[f.index[3]].point.y,
                    self.vertex[f.index[3]].point.z,
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
                Vertex::new(-Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE),
                Vertex::new( Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE),
                Vertex::new( Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE),
                Vertex::new(-Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE),
                Vertex::new(-Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE),
                Vertex::new( Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE),
                Vertex::new( Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE),
                Vertex::new(-Self::DEFAULT_SHAPE,  Self::DEFAULT_SHAPE, -Self::DEFAULT_SHAPE),
            ],
            face: Face::new_list(),
            focus: false,
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Vertex {
    pub focus: bool,
    pub point: Vector3,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            focus: false,
            point: Vector3::new(x, y, z),
        }
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Face {
    pub focus: bool,
    pub index: [usize; 4],
    pub shift: Vector2,
    pub scale: Vector2,
    pub color: Color,
    pub texture: Option<String>,
}

impl Face {
    pub fn new(index: [usize; 4]) -> Self {
        Self {
            focus: false,
            index,
            shift: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            color: Color::WHITE,
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

#[derive(Deserialize, Serialize)]
pub struct Entity {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub focus: bool,
    pub meta: EntityMeta,
}

impl Entity {
    pub fn new_from_lua(meta: EntityMeta) -> Self {
        Self {
            position: Vector3::default(),
            rotation: Vector3::default(),
            scale: Vector3::one(),
            focus: false,
            meta,
        }
    }

    pub fn position(&mut self, value: Vector3) {
        self.position += value;
    }

    pub fn rotation(&mut self, value: Vector3) {
        self.rotation += value;
    }

    pub fn scale(&mut self, value: Vector3) {
        self.scale += value;
    }

    pub fn bound_box(&self) -> BoundingBox {
        BoundingBox::new(
            self.meta.shape.min + self.position,
            self.meta.shape.max + self.position,
        )
    }

    pub fn draw_3d(&self, lua: &Lua, draw: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        draw.draw_bounding_box(
            self.bound_box(),
            if self.focus { Color::GREEN } else { Color::RED },
        );

        let data = lua
            .to_value(&self)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();

        if let Some(call) = &self.meta.call {
            let g = lua.globals();
            let g = g
                .get::<mlua::Function>(&**call)
                .map_err(|e| panic(&e.to_string()))
                .unwrap();

            g.call::<()>(data)
                .map_err(|e| panic(&e.to_string()))
                .unwrap();
        }
    }

    pub fn draw_2d(
        &self,
        draw: &mut RaylibMode2D<RaylibTextureMode<RaylibDrawHandle>>,
        asset: &Asset,
        view: &Camera3D,
        port: Vector2,
    ) {
        let text = draw.get_world_to_screen_ex(
            self.position + Vector3::new(0.0, self.meta.shape.max.y + 1.0, 0.0),
            view,
            port.x as i32,
            port.y as i32,
        );
        let font = asset.inner.font.measure_text(&self.meta.name, 24.0, 1.0);

        draw.draw_rectangle_rounded(
            Rectangle::new(
                text.x - font.x * 0.5,
                text.y - font.y * 0.5,
                font.x + 8.0,
                font.y,
            ),
            0.25,
            4,
            Color::new(0, 0, 0, if self.focus { 255 } else { 127 }),
        );

        draw.draw_text_ex(
            &asset.inner.font,
            &self.meta.name,
            Vector2::new((text.x - font.x * 0.5) + 4.0, text.y - font.y * 0.5),
            24.0,
            1.0,
            Color::WHITE,
        );
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EntityMeta {
    pub name: String,
    pub info: String,
    pub data: HashMap<String, EntityData>,
    pub shape: BoundingBox,
    pub call: Option<String>,
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
    Position,
    Rotation,
    Scale,
    Vertex,
    Edge,
    Face,
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
    pub font: Font,
    pub logo: Texture2D,
    pub default: Texture2D,
    pub drop_a: Texture2D,
    pub drop_b: Texture2D,
    pub texture: Texture2D,
    pub entity: Texture2D,
    pub position: Texture2D,
    pub rotation: Texture2D,
    pub scale: Texture2D,
    pub vertex: Texture2D,
    pub edge: Texture2D,
    pub face: Texture2D,
    pub user: Texture2D,
    pub reload: Texture2D,
    pub import: Texture2D,
    pub export: Texture2D,
    pub exit: Texture2D,
}

#[rustfmt::skip]
impl Inner {
    pub const ICON: &'static [u8] = include_bytes!("asset/icon.png");
    const FONT:     &'static [u8] = include_bytes!("asset/font.ttf");
    const LOGO:     &'static [u8] = include_bytes!("asset/logo.png");
    const DEFAULT:  &'static [u8] = include_bytes!("asset/default.png");
    const DROP_A:   &'static [u8] = include_bytes!("asset/drop-a.png");
    const DROP_B:   &'static [u8] = include_bytes!("asset/drop-b.png");
    const TEXTURE:  &'static [u8] = include_bytes!("asset/texture.png");
    const ENTITY:   &'static [u8] = include_bytes!("asset/entity.png");
    const POSITION: &'static [u8] = include_bytes!("asset/position.png");
    const ROTATION: &'static [u8] = include_bytes!("asset/rotation.png");
    const SCALE:    &'static [u8] = include_bytes!("asset/scale.png");
    const VERTEX:   &'static [u8] = include_bytes!("asset/vertex.png");
    const EDGE:     &'static [u8] = include_bytes!("asset/edge.png");
    const FACE:     &'static [u8] = include_bytes!("asset/face.png");
    const USER:     &'static [u8] = include_bytes!("asset/user.png");
    const RELOAD:   &'static [u8] = include_bytes!("asset/reload.png");
    const IMPORT:   &'static [u8] = include_bytes!("asset/import.png");
    const EXPORT:   &'static [u8] = include_bytes!("asset/export.png");
    const EXIT:     &'static [u8] = include_bytes!("asset/exit.png");

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            font:     load_font(handle, thread, Self::FONT),
            logo:     load_texture(handle, thread, Self::LOGO),
            default:  load_texture(handle, thread, Self::DEFAULT),
            drop_a:   load_texture(handle, thread, Self::DROP_A),
            drop_b:   load_texture(handle, thread, Self::DROP_B),
            texture:  load_texture(handle, thread, Self::TEXTURE),
            entity:   load_texture(handle, thread, Self::ENTITY),
            position: load_texture(handle, thread, Self::POSITION),
            rotation: load_texture(handle, thread, Self::ROTATION),
            scale:    load_texture(handle, thread, Self::SCALE),
            vertex:   load_texture(handle, thread, Self::VERTEX),
            edge:     load_texture(handle, thread, Self::EDGE),
            face:     load_texture(handle, thread, Self::FACE),
            user:     load_texture(handle, thread, Self::USER),
            reload:   load_texture(handle, thread, Self::RELOAD),
            import:   load_texture(handle, thread, Self::IMPORT),
            export:   load_texture(handle, thread, Self::EXPORT),
            exit:     load_texture(handle, thread, Self::EXIT),
        }
    }
}

#[derive(Default)]
pub struct Outer {
    pub texture: BTreeMap<String, Texture2D>,
}

impl Outer {
    // load a texture from disk into the hash-map.
    pub fn set_texture(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread, path: &str) {
        let texture = handle
            .load_texture(thread, path)
            .map_err(|e| panic(&e.to_string()))
            .unwrap();

        self.texture.insert(path.to_string(), texture);
    }

    // load a texture from disk into the hash-map, using a path list instead.
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

// a representation of a 3D view-port.
pub struct View {
    pub render_texture: RenderTexture2D,
    pub camera: Camera3D,
    pub mouse: bool,
}

impl View {
    // create a new view-port.
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, camera: Camera3D) -> Self {
        // convert screen X/Y to a f32 vector.
        let shape = Vector2::new(
            handle.get_screen_width() as f32 * 0.5,
            handle.get_screen_height() as f32 * 0.5,
        );

        // load render texture.
        let render_texture = handle
            .load_render_texture(
                &thread,
                (shape.x - Window::EDIT_SHAPE / 2.0) as u32,
                (shape.y - Window::TOOL_SHAPE / 2.0) as u32,
            )
            .map_err(|e| panic(&e.to_string()))
            .unwrap();

        Self {
            render_texture,
            camera,
            mouse: false,
        }
    }
}

//================================================================

// a representation of user configuration data.
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
    pub user: Input,
    pub reload: Input,
    pub import: Input,
    pub export: Input,
    pub exit: Input,
}

impl User {
    pub const FILE_NAME: &'static str = "user.json";

    // create a new user, using existing user data from disk, or the default data.
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

#[rustfmt::skip]
impl Default for User {
    fn default() -> Self {
        Self {
            mouse_speed: [1.0, 1.0],
            move_x_a: Input::new(None, Key::Keyboard(KEY_W)),
            move_x_b: Input::new(None, Key::Keyboard(KEY_S)),
            move_y_a: Input::new(None, Key::Keyboard(KEY_A)),
            move_y_b: Input::new(None, Key::Keyboard(KEY_D)),
            interact: Input::new(None, Key::Mouse(MOUSE_BUTTON_LEFT)),
            look:     Input::new(None, Key::Mouse(MOUSE_BUTTON_RIGHT)),
            texture:  Input::new(None, Key::Keyboard(KEY_SEVEN)),
            entity:   Input::new(None, Key::Keyboard(KEY_EIGHT)),
            position: Input::new(None, Key::Keyboard(KEY_ONE)),
            rotation: Input::new(None, Key::Keyboard(KEY_TWO)),
            scale:    Input::new(None, Key::Keyboard(KEY_THREE)),
            vertex:   Input::new(None, Key::Keyboard(KEY_FOUR)),
            edge:     Input::new(None, Key::Keyboard(KEY_FIVE)),
            face:     Input::new(None, Key::Keyboard(KEY_SIX)),
            user:     Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_Z)),
            reload:   Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_X)),
            import:   Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_C)),
            export:   Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_V)),
            exit:     Input::new(Some(Key::Keyboard(KEY_LEFT_CONTROL)), Key::Keyboard(KEY_B)),
        }
    }
}

//================================================================

// an abstraction of input, wherein "key" can be a keyboard button or a mouse button.
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

// button abstraction.
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

pub struct Script {
    pub lua: Lua,
    pub meta: Meta,
}

impl Script {
    const FILE_NAME: &'static str = "main";

    pub fn new(game: &Game) -> mlua::Result<Self> {
        // initialize lua, get the global table, and create the mallet table.
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new())?;
        let global = lua.globals();
        let mallet = lua.create_table()?;

        // add every built-in function to the mallet table.
        Self::system(&lua, &mallet)?;

        // set the mallet table at the global level.
        global.set("mallet", mallet)?;

        // set the Meta app data for the script to push to.
        lua.set_app_data(Meta::default());

        // get the package loader table, and append the game path to the end, so lua can also search the game directory for it.
        let package = global.get::<mlua::Table>("package")?;
        package.set(
            "path",
            format!(
                "{:?};{}/?.lua",
                package.get::<mlua::String>("path")?,
                game.path
            ),
        )?;

        // load the main.lua script.
        lua.load(format!("require \"{}\"", Self::FILE_NAME))
            .exec()?;

        // get the Meta app data, to retrieve every all Lua data such as the map entity list.
        let meta = lua.remove_app_data::<Meta>().unwrap();

        Ok(Self { lua, meta })
    }

    // push every built-in function to the lua space.
    fn system(lua: &Lua, table: &mlua::Table) -> mlua::Result<()> {
        table.set("map_entity", lua.create_function(Self::map_entity)?)?;
        table.set("map_texture", lua.create_function(Self::map_texture)?)?;
        set_global(lua, table)?;

        Ok(())
    }

    // load a map entity.
    fn map_entity(lua: &Lua, entity: LuaValue) -> mlua::Result<()> {
        if let Some(mut app) = lua.app_data_mut::<Meta>() {
            // push!
            app.entity.push(lua.from_value(entity)?);
        }

        Ok(())
    }

    // load a map texture.
    fn map_texture(lua: &Lua, path: String) -> mlua::Result<()> {
        if let Some(mut app) = lua.app_data_mut::<Meta>() {
            // push!
            app.texture.push(path);
        }

        Ok(())
    }
}

//================================================================

#[derive(Default)]
pub struct Meta {
    pub entity: Vec<EntityMeta>,
    pub texture: Vec<String>,
}

//================================================================

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
            |lua, this, (position, rotation, scale): (LuaValue, LuaValue, LuaValue)| unsafe {
                let position: Vector3 = lua.from_value(position)?;
                let rotation: Vector3 = lua.from_value(rotation)?;
                let scale: Vector3 = lua.from_value(scale)?;

                this.0.transform = (Matrix::rotate_xyz(Vector3::new(
                    rotation.x * DEG2RAD as f32,
                    rotation.y * DEG2RAD as f32,
                    rotation.z * DEG2RAD as f32,
                )) * Matrix::scale(scale.x, scale.y, scale.z))
                .into();

                ffi::DrawModel(
                    this.0,
                    Vector3::new(position.x, position.y, position.z).into(),
                    1.0,
                    Color::RED.into(),
                );

                this.0.transform = Matrix::identity().into();
                Ok(())
            },
        );
    }
}
