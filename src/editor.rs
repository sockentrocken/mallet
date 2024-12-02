use crate::helper::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;

//================================================================

pub struct Active {
    button: Button,
    mouse: Vector2,
    view: usize,
    port: Rectangle,
}

impl Active {
    pub fn new(button: Button, mouse: Vector2, view: usize, port: Rectangle) -> Self {
        Self {
            button,
            mouse,
            view,
            port,
        }
    }
}

pub struct Editor {
    pub brush: Vec<Brush>,
    pub active: Option<Active>,
    pub widget: Widget,
    pub asset: Asset,
    pub view: [View; 4],
}

impl Editor {
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            brush: vec![Brush::default()],
            active: None,
            widget: Widget::default(),
            asset: Asset::new(handle, thread),
            view: [
                View::new(handle, thread, Vector3::new(4.00, 4.00, 4.00),    Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.00, 0.00, 0.00), Quaternion::new(0.0, 1.0, 0.0, 90.0), false),
                View::new(handle, thread, Vector3::new(1000.00, 0.00, 0.00), Vector3::new(1.0, 0.0, 0.0), Vector3::new(90.00, 90.0, 0.00), Quaternion::new(0.0, 0.0, 1.0, 90.0), true),
                View::new(handle, thread, Vector3::new(0.00, 1000.00, 0.00), Vector3::new(0.0, 1.0, 0.0), Vector3::new(90.0, 90.0, 0.00), Quaternion::new(0.0, 1.0, 0.0, 90.0), true),
                View::new(handle, thread, Vector3::new(0.00, 0.00, 1000.00), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.00, 0.00, 0.00), Quaternion::new(1.0, 0.0, 0.0, 90.0), true),
            ],
        }
    }

    #[rustfmt::skip]
    pub fn resize(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        if handle.is_window_resized() {
            self.view = [
                View::new(handle, thread, Vector3::new(4.00, 4.00, 4.00), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.00, 0.00, 0.00), Quaternion::new(0.0, 1.0, 0.0, 90.0), false),
                View::new(handle, thread, Vector3::new(1.01, 0.00, 0.00), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.00, 90.0, 0.00), Quaternion::new(0.0, 0.0, 1.0, 90.0), true),
                View::new(handle, thread, Vector3::new(0.00, 1.01, 0.00), Vector3::new(0.0, 1.0, 0.0), Vector3::new(90.0, 90.0, 0.00), Quaternion::new(0.0, 1.0, 0.0, 90.0), true),
                View::new(handle, thread, Vector3::new(0.00, 0.00, 1.01), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.00, 0.00, 0.00), Quaternion::new(1.0, 0.0, 0.0, 90.0), true),
            ];
        }
    }

    fn dirty(&mut self) {
        for view in &mut self.view {
            view.dirty = true;
        }
    }

    pub fn update(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread) {
        self.render(draw, thread);
        self.button(draw, thread);
    }

    fn render(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread) {
        for (i, view) in self.view.iter_mut().enumerate() {
            if view.dirty {
                view.dirty = false;
            } else {
                continue;
            }

            view.button.clear();

            let size = Vector2::new(
                view.render_texture.width() as f32,
                view.render_texture.height() as f32,
            );

            let mut draw_texture = draw.begin_texture_mode(thread, &mut view.render_texture);

            draw_texture.clear_background(Color::WHITE);

            {
                let mut draw = draw_texture.begin_mode3D(view.camera);

                draw_grid(128, 1.0, view.grid);

                for brush in &self.brush {
                    brush.draw(&self.asset.icon_position, Color::WHITE);
                }
            }
            {
                for (i, brush) in self.brush.iter().enumerate() {
                    for (j, index) in brush.index.iter().enumerate() {
                        match self.widget {
                            Widget::Position => {}
                            Widget::Rotation => {}
                            Widget::Scale => {}
                            Widget::Vertex => {
                                for x in 0..4 {
                                    let scr_point = draw_texture.get_world_to_screen_ex(
                                        brush.vertex[index[x]],
                                        view.camera,
                                        size.x as i32,
                                        size.y as i32,
                                    );

                                    let current_button = Button::new(
                                        vec![brush.vertex[index[x]]],
                                        brush.vertex[index[x]],
                                        Rectangle::new(scr_point.x, scr_point.y, 16.0, 16.0),
                                        i,
                                        vec![index[x]],
                                    );

                                    let mut check = true;

                                    for button in &mut view.button {
                                        // current button and another button are occupying the same place in 2D space
                                        if current_button.shape == button.shape {
                                            let length_1 = view
                                                .camera
                                                .position
                                                .distance_to(current_button.point);
                                            let length_2 =
                                                view.camera.position.distance_to(button.point);

                                            // current button is farther away than other button, skip
                                            if length_1 > length_2 {
                                                check = false;
                                            }

                                            // current button is closer than other button, replace
                                            if length_2 > length_1 {
                                                *button = current_button.clone();
                                                check = false;
                                            }
                                        }
                                    }

                                    if check {
                                        view.button.push(current_button);
                                    } else {
                                        println!("Skipping button!");
                                    }
                                }
                            }
                            Widget::Edge => {
                                // EDGE EDITOR

                                for x in 0..4 {
                                    let t_idx = x;
                                    let h_idx = {
                                        if x == 3 {
                                            0
                                        } else {
                                            x + 1
                                        }
                                    };

                                    let tail = brush.vertex[index[t_idx]];
                                    let head = brush.vertex[index[h_idx]];

                                    let which = Vector3::new(
                                        (head.x + tail.x) / 2.0,
                                        (head.y + tail.y) / 2.0,
                                        (head.z + tail.z) / 2.0,
                                    );

                                    let scr_point = draw_texture.get_world_to_screen_ex(
                                        which,
                                        view.camera,
                                        size.x as i32,
                                        size.y as i32,
                                    );

                                    let current_button = Button::new(
                                        [tail, head].to_vec(),
                                        which,
                                        Rectangle::new(scr_point.x, scr_point.y, 16.0, 16.0),
                                        i,
                                        vec![index[t_idx], index[h_idx]],
                                    );

                                    let mut check = true;

                                    for button in &mut view.button {
                                        // current button and another button are occupying the same place in 2D space
                                        if current_button.shape == button.shape {
                                            let length_1 = view
                                                .camera
                                                .position
                                                .distance_to(current_button.point);
                                            let length_2 =
                                                view.camera.position.distance_to(button.point);

                                            // current button is farther away than other button, skip
                                            if length_1 > length_2 {
                                                check = false;
                                            }

                                            // current button is closer than other button, replace
                                            if length_2 > length_1 {
                                                *button = current_button.clone();
                                                check = false;
                                            }
                                        }
                                    }

                                    if check {
                                        view.button.push(current_button);
                                    } else {
                                        println!("Skipping button!");
                                    }
                                }
                            }
                            Widget::Face => {
                                let mut point = [Vector3::default(); 4];
                                let mut which = Vector3::default();

                                for x in 0..4 {
                                    point[x] = brush.vertex[index[x]];

                                    which += point[x];
                                }

                                which /= 4.0;

                                let scr_point = draw_texture.get_world_to_screen_ex(
                                    which,
                                    view.camera,
                                    size.x as i32,
                                    size.y as i32,
                                );

                                let current_button = Button::new(
                                    point.to_vec(),
                                    which,
                                    Rectangle::new(scr_point.x, scr_point.y, 16.0, 16.0),
                                    i,
                                    vec![index[0], index[1], index[2], index[3]],
                                );

                                let mut check = true;

                                for button in &mut view.button {
                                    // current button and another button are occupying the same place in 2D space
                                    if current_button.shape == button.shape {
                                        let length_1 =
                                            view.camera.position.distance_to(current_button.point);
                                        let length_2 =
                                            view.camera.position.distance_to(button.point);

                                        // current button is farther away than other button, skip
                                        if length_1 > length_2 {
                                            check = false;
                                        }

                                        // current button is closer than other button, replace
                                        if length_2 > length_1 {
                                            *button = current_button.clone();
                                            check = false;
                                        }
                                    }
                                }

                                if check {
                                    view.button.push(current_button);
                                } else {
                                    println!("Skipping button!");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn button(&mut self, draw: &mut RaylibDrawHandle, _: &RaylibThread) {
        let mouse = draw.get_mouse_position();
        let mut dirty = false;

        for (i, view) in self.view.iter_mut().enumerate() {
            let x = (i as f32 / 2.0).floor();
            let y = (i as f32 % 2.0).floor();
            let shift = Vector2::new(
                x * view.render_texture.width() as f32,
                48.0 + y * view.render_texture.height() as f32,
            );
            let size = Vector2::new(
                view.render_texture.width() as f32,
                view.render_texture.height() as f32,
            );
            let view_port = Rectangle::new(shift.x, shift.y, size.x, size.y);

            draw.draw_texture_rec(
                &view.render_texture,
                Rectangle::new(0.0, 0.0, size.x, -size.y),
                Vector2::new(shift.x, shift.y),
                Color::WHITE,
            );

            if view_port.check_collision_point_rec(mouse) {
                if draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                    unsafe {
                        ffi::DisableCursor();
                    }

                    view.mouse = true;
                }

                let delta = draw.get_mouse_wheel_move();

                if delta != 0.0 {
                    match view.camera.camera_type() {
                        CameraProjection::CAMERA_PERSPECTIVE => {
                            view.camera.fovy -= delta * 8.0;
                            view.camera.fovy = view.camera.fovy.clamp(60.0, 120.0)
                        }
                        CameraProjection::CAMERA_ORTHOGRAPHIC => {
                            view.camera.fovy -= delta;
                            view.camera.fovy = view.camera.fovy.clamp(5.0, 30.0)
                        }
                    }

                    view.dirty = true;
                }
            }

            if view.mouse && draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
                unsafe {
                    ffi::EnableCursor();
                }

                view.mouse = false;
            }

            if view.mouse {
                match view.camera.camera_type() {
                    CameraProjection::CAMERA_PERSPECTIVE => {
                        view.angle.x += draw.get_mouse_delta().y * 0.25;
                        view.angle.y -= draw.get_mouse_delta().x * 0.25;

                        let direction = direction_from_euler(view.angle.x, view.angle.y);

                        let mut key = Vector3::default();

                        if draw.is_key_down(KeyboardKey::KEY_W) {
                            key.x = 1.0;
                        }
                        if draw.is_key_down(KeyboardKey::KEY_S) {
                            key.x = -1.0;
                        }

                        if draw.is_key_down(KeyboardKey::KEY_A) {
                            key.z = 1.0;
                        }
                        if draw.is_key_down(KeyboardKey::KEY_D) {
                            key.z = -1.0;
                        }

                        key *= draw.get_frame_time() * 10.0;

                        view.camera.position += direction.0 * key.x + direction.2 * key.z;

                        view.camera.target = view.camera.position + direction.0;
                        view.camera.up = direction.1;
                    }
                    CameraProjection::CAMERA_ORTHOGRAPHIC => {
                        let delta = draw.get_mouse_delta() * 0.05;

                        let direction =
                            direction_from_euler(view.angle.x + 0.01, view.angle.y + 0.01);

                        let shift = direction.1 * delta.y + direction.2 * -delta.x;

                        view.camera.position += shift
                            * Vector3::new(
                                1.0 - view.orient.x,
                                1.0 - view.orient.y,
                                1.0 - view.orient.z,
                            );

                        /*
                        view.camera.position.x += delta.y;
                        view.camera.position.z += delta.x;

                        view.camera.target.x += delta.y;
                        view.camera.target.z += delta.x;
                        */

                        view.camera.target = view.camera.position
                            * Vector3::new(
                                1.0 - view.orient.x,
                                1.0 - view.orient.y,
                                1.0 - view.orient.z,
                            );
                        view.camera.up = direction.1;
                    }
                }

                view.dirty = true;
            }

            if view.camera.camera_type() == CameraProjection::CAMERA_PERSPECTIVE {
                //continue;
            }

            for button in &view.button {
                let rectangle = Rectangle::new(
                    button.shape.x + x * size.x - 8.0,
                    button.shape.y + y * size.y - 8.0 + 48.0,
                    button.shape.width,
                    button.shape.height,
                );

                let point = Vector2::new(rectangle.x + 8.0, rectangle.y + 8.0);

                if rectangle.check_collision_point_rec(mouse) {
                    if draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        let port = Rectangle::new(
                            shift.x,
                            shift.y,
                            view.render_texture.width() as f32,
                            view.render_texture.height() as f32,
                        );

                        self.active = Some(Active::new(
                            button.clone(),
                            draw.get_mouse_position(),
                            i,
                            port,
                        ));

                        draw.draw_circle_v(point, 8.0, Color::BLUE);
                    } else {
                        draw.draw_circle_v(point, 8.0, Color::GREEN);
                    }
                } else {
                    draw.draw_circle_lines(point.x as i32, point.y as i32, 12.0, Color::GREEN);

                    draw.draw_circle_v(point, 8.0, Color::RED);
                }
            }
        }

        if draw.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.widget = Widget::Vertex;
            dirty = true;
        }

        if draw.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.widget = Widget::Edge;
            dirty = true;
        }

        if draw.is_key_pressed(KeyboardKey::KEY_THREE) {
            self.widget = Widget::Face;
            dirty = true;
        }

        if dirty {
            self.dirty();
        }

        if let Some(active) = &self.active {
            if active.port.check_collision_point_rec(mouse) {
                let view = self.view.get(active.view).unwrap();

                let mut old_ray = get_screen_to_world_ray_ex(
                    Vector2::new(
                        active.mouse.x - active.port.x,
                        active.mouse.y - active.port.y,
                    ),
                    view.camera,
                    ((draw.get_screen_width() - Window::EDIT_SHAPE as i32) as f32 / 2.0) as i32,
                    ((draw.get_screen_height() - Window::TOOL_SHAPE as i32) as f32 / 2.0) as i32,
                );
                old_ray.position = Self::snap(&old_ray.position, 1.0);

                let mut new_ray = get_screen_to_world_ray_ex(
                    Vector2::new(mouse.x - active.port.x, mouse.y - active.port.y),
                    view.camera,
                    ((draw.get_screen_width() - Window::EDIT_SHAPE as i32) as f32 / 2.0) as i32,
                    ((draw.get_screen_height() - Window::TOOL_SHAPE as i32) as f32 / 2.0) as i32,
                );
                new_ray.position = Self::snap(&new_ray.position, 1.0);

                //println!("{:?}", new_ray.position - old_ray.position);

                let brush = self.brush.get_mut(active.button.brush).unwrap();

                for (i, vertex) in active.button.index.iter().enumerate() {
                    let point = brush.vertex.get_mut(*vertex).unwrap();
                    println!("moving");

                    let screen = draw.get_world_to_screen_ex(
                        *point,
                        view.camera,
                        view.render_texture.width(),
                        view.render_texture.height(),
                    );

                    *point = active.button.vertex[i];

                    draw.draw_circle_v(Vector2::new(screen.x, screen.y + 48.0), 16.0, Color::PINK);

                    point.x += (new_ray.position.x - old_ray.position.x);
                    point.y += (new_ray.position.y - old_ray.position.y);
                    point.z += (new_ray.position.z - old_ray.position.z);

                    *point = Self::snap(point, 1.0);
                }

                self.dirty();
            }

            if draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                self.active = None;
            }
        }
    }

    pub fn snap(vector: &Vector3, grid: f32) -> Vector3 {
        Vector3::new(
            (vector.x / grid).round() * grid,
            (vector.y / grid).round() * grid,
            (vector.z / grid).round() * grid,
        )
    }
}

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

pub struct Asset {
    pub icon_position: Texture2D,
    pub icon_rotation: Texture2D,
    pub icon_scale: Texture2D,
}

impl Asset {
    const ICON_POSITION: &'static [u8] =
        include_bytes!("../resource/game_1/texture/texture_10.png");
    const ICON_ROTATION: &'static [u8] = include_bytes!("asset/rotation.png");
    const ICON_SCALE: &'static [u8] = include_bytes!("asset/scale.png");

    fn load_texture(handle: &mut RaylibHandle, thread: &RaylibThread, data: &[u8]) -> Texture2D {
        let mut texture = handle
            .load_texture_from_image(
                thread,
                &Image::load_image_from_mem(".png", data)
                    .expect("Asset::load_texture(): Could not load texture."),
            )
            .expect("Asset::load_texture(): Could not load texture.");

        texture.gen_texture_mipmaps();

        texture.set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);

        texture
    }

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            icon_position: Self::load_texture(handle, thread, Self::ICON_POSITION),
            icon_rotation: Self::load_texture(handle, thread, Self::ICON_ROTATION),
            icon_scale: Self::load_texture(handle, thread, Self::ICON_SCALE),
        }
    }
}

//================================================================

#[derive(Clone)]
pub struct Button {
    vertex: Vec<Vector3>,
    point: Vector3,
    shape: Rectangle,
    brush: usize,
    index: Vec<usize>,
}

impl Button {
    pub fn new(
        vertex: Vec<Vector3>,
        point: Vector3,
        shape: Rectangle,
        brush: usize,
        index: Vec<usize>,
    ) -> Self {
        Self {
            vertex,
            point,
            shape,
            brush,
            index,
        }
    }
}

//================================================================

pub struct View {
    pub render_texture: RenderTexture2D,
    pub camera: Camera3D,
    pub angle: Vector3,
    pub orient: Vector3,
    pub grid: Quaternion,
    pub button: Vec<Button>,
    pub dirty: bool,
    pub mouse: bool,
}

impl View {
    pub fn new(
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        point: Vector3,
        orient: Vector3,
        angle: Vector3,
        grid: Quaternion,
        orthographic: bool,
    ) -> Self {
        Self {
            render_texture: handle
                .load_render_texture(
                    &thread,
                    ((handle.get_screen_width() - Window::EDIT_SHAPE as i32) as f32 / 2.0) as u32,
                    ((handle.get_screen_height() - Window::TOOL_SHAPE as i32) as f32 / 2.0) as u32,
                )
                .unwrap(),
            camera: {
                if orthographic {
                    Camera3D::orthographic(
                        point,
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(1.0, 0.0, 0.0),
                        15.0,
                    )
                } else {
                    Camera3D::perspective(
                        point,
                        Vector3::new(0.0, 0.0, 0.0),
                        Vector3::new(0.0, 1.0, 0.0),
                        90.0,
                    )
                }
            },
            orient,
            angle,
            grid,
            button: Vec::default(),
            dirty: true,
            mouse: false,
        }
    }
}

//================================================================

pub struct Brush {
    pub vertex: [Vector3; 8],
    pub index: [[usize; 4]; 6],
}

pub unsafe fn DrawCubeC(
    texture: &Texture2D,
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
) {
    let x = position.x;
    let y = position.y;
    let z = position.z;

    // Set desired texture to be enabled while drawing following vertex data
    ffi::rlSetTexture(texture.id);

    // Vertex data transformation can be defined with the commented lines,
    // but in this example we calculate the transformed vertex data directly when calling ffi::rlVertex3f()
    //rlPushMatrix();
    // NOTE: Transformation is applied in inverse order (scale -> rotate -> translate)
    //rlTranslatef(2.0, 0.0, 0.0);
    //rlRotatef(45, 0, 1, 0);
    //rlScalef(2.0, 2.0, 2.0);

    ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());
    ffi::rlColor4ub(color.r, color.g, color.b, color.a);
    // Front Face
    //ffi::rlNormal3f(0.0, 0.0, 1.0); // Normal Pointing Towards Viewer
    //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Of The Texture and Quad
                                                                          // Back Face
    ffi::rlNormal3f(0.0, 0.0, -1.0); // Normal Pointing Away From Viewer
                                     //    ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          //  ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          // Top Face
                                                                          //ffi::rlNormal3f(0.0, 1.0, 0.0); // Normal Pointing Up
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
                                                                          // Bottom Face
                                                                          //ffi::rlNormal3f(0.0, -1.0, 0.0); // Normal Pointing Down
                                                                          //ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          // Right face
                                                                          //ffi::rlNormal3f(1.0, 0.0, 0.0); // Normal Pointing Right
                                                                          //ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z - length / 2.0); // Top Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x + width / 2.0, y + height / 2.0, z + length / 2.0); // Top Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x + width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          // Left Face
                                                                          //ffi::rlNormal3f(-1.0, 0.0, 0.0); // Normal Pointing Left
                                                                          //ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z - length / 2.0); // Bottom Left Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(x - width / 2.0, y - height / 2.0, z + length / 2.0); // Bottom Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z + length / 2.0); // Top Right Of The Texture and Quad
                                                                          //ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(x - width / 2.0, y + height / 2.0, z - length / 2.0); // Top Left Of The Texture and Quad
    ffi::rlEnd();
    //rlPopMatrix();

    ffi::rlSetTexture(0);
}

impl Brush {
    pub fn draw(&self, texture: &Texture2D, color: Color) {
        unsafe {
            // set texture to draw with.
            ffi::rlSetTexture(texture.id);

            // begin quad draw.
            ffi::rlBegin(ffi::RL_QUADS.try_into().unwrap());

            // set color.
            ffi::rlColor4ub(color.r, color.g, color.b, color.a);

            let x = 0.0;
            let y = 0.0;

            let s_x = 1.0;
            let s_y = 1.0;

            // for each vertex index, draw the corresponding face.
            for j in &self.index {
                ffi::rlTexCoord2f(s_x * (x + 0.0), s_y * (y + 1.0));
                ffi::rlVertex3f(
                    self.vertex[j[0]].x,
                    self.vertex[j[0]].y,
                    self.vertex[j[0]].z,
                );
                ffi::rlTexCoord2f(s_x * (x + 1.0), s_y * (y + 1.0));
                ffi::rlVertex3f(
                    self.vertex[j[1]].x,
                    self.vertex[j[1]].y,
                    self.vertex[j[1]].z,
                );
                ffi::rlTexCoord2f(s_x * (x + 1.0), s_y * (y + 0.0));
                ffi::rlVertex3f(
                    self.vertex[j[2]].x,
                    self.vertex[j[2]].y,
                    self.vertex[j[2]].z,
                );
                ffi::rlTexCoord2f(s_x * (x + 0.0), s_y * (y + 0.0));
                ffi::rlVertex3f(
                    self.vertex[j[3]].x,
                    self.vertex[j[3]].y,
                    self.vertex[j[3]].z,
                );
            }

            // end quad draw.
            ffi::rlEnd();

            ffi::rlSetTexture(0);
        }
    }
}

impl Default for Brush {
    #[rustfmt::skip]
    fn default() -> Self {
        let shape = 1.0;

        Self {
            vertex: [
                Vector3::new(-shape, -shape,  shape), // 0 bl
                Vector3::new( shape, -shape,  shape), // 1 br
                Vector3::new( shape,  shape,  shape), // 2 tr
                Vector3::new(-shape,  shape,  shape), // 3 tl
                // back
                Vector3::new(-shape, -shape, -shape), // 4 bl
                Vector3::new( shape, -shape, -shape), // 5 br
                Vector3::new( shape,  shape, -shape), // 6 tr
                Vector3::new(-shape,  shape, -shape), // 7 tl
            ],
            index: [
                [0, 1, 2, 3],
                [5, 4, 7, 6],
                [3, 2, 6, 7],
                [1, 0, 4, 5],
                [1, 5, 6, 2],
                [4, 0, 3, 7],
            ],
        }
    }
}
