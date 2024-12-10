use crate::editor::*;
use crate::game::*;
use crate::status::*;

//================================================================

use raylib::prelude::*;
use std::collections::HashMap;

//================================================================

pub struct Window {
    data: HashMap<String, widget::Data>,
    font: Font,
    logo: Texture2D,
    point: Vector2,
    focus: Option<i32>,
    count: i32,
    menu: bool,
}

impl Window {
    pub const FONT: &'static [u8] = include_bytes!("asset/font.ttf");
    pub const LOGO: &'static [u8] = include_bytes!("asset/logo.png");

    pub const COLOR_PRIMARY_MAIN: Color = Color::new(3, 169, 244, 255);
    pub const COLOR_PRIMARY_SIDE: Color = Color::new(25, 118, 210, 255);
    pub const COLOR_TEXT: Color = Color::new(255, 255, 255, 255);
    pub const COLOR_TEXT_MAIN: Color = Color::new(33, 33, 33, 255);
    pub const COLOR_TEXT_SIDE: Color = Color::new(117, 117, 117, 255);

    //================================================================

    const GRADIENT_POINT_Y: f32 = 4.0;
    const GRADIENT_SHAPE_Y: i32 = 6;
    const GRADIENT_COLOR_MAX: Color = Color::new(0, 0, 0, 99);
    const GRADIENT_COLOR_MIN: Color = Color::new(0, 0, 0, 0);

    //================================================================

    const LOGO_SHAPE: f32 = 160.0;

    //================================================================

    const CARD_ROUND_SHAPE: f32 = 0.25;
    const CARD_ROUND_COUNT: i32 = 4;

    //================================================================

    const TEXT_SHAPE: f32 = 24.0;
    const TEXT_SPACE: f32 = 1.0;
    const TEXT_SHIFT: f32 = 8.0;

    //================================================================

    const BUTTON_SHAPE: Vector2 = Vector2::new(160.0, 32.0);
    const BUTTON_TEXT_SHIFT: Vector2 = Vector2::new(8.0, 4.0);
    const BUTTON_SHIFT: f32 = 8.0;

    //================================================================

    const TOGGLE_SHAPE: Vector2 = Vector2::new(24.0, 24.0);
    const TOGGLE_SHIFT: f32 = 8.0;

    const SLIDER_CIRCLE_SHAPE: f32 = 8.0;
    const SLIDER_FOCUS_POINT: Vector2 = Vector2::new(0.0, -32.0);
    const SLIDER_FOCUS_SHAPE: f32 = 20.0;
    const SLIDER_SHAPE_MAX: Vector2 = Vector2::new(160.0, 24.0);
    const SLIDER_SHAPE_MIN: Vector2 = Vector2::new(160.0, 4.0);
    const SLIDER_SHIFT: f32 = 8.0;

    const RECORD_SHAPE_MAX: Vector2 = Vector2::new(160.0, 24.0);
    const RECORD_SHAPE_MIN: Vector2 = Vector2::new(160.0, 4.0);
    const RECORD_SHAPE_CARET: Vector2 = Vector2::new(2.0, 16.0);
    const RECORD_SHIFT: f32 = 8.0;

    pub const TOOL_SHAPE: f32 = 56.0;
    pub const EDIT_SHAPE: f32 = 320.0;

    //================================================================

    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font = handle
            .load_font_from_memory(thread, ".ttf", Self::FONT, Self::TEXT_SHAPE as i32, None)
            .expect("Window::new(): Could not load default font.");
        let logo = handle
            .load_texture_from_image(
                thread,
                &Image::load_image_from_mem(".png", Self::LOGO)
                    .expect("Window::new(): Could not load texture."),
            )
            .expect("Window::new(): Could not load texture.");

        Self {
            data: HashMap::default(),
            font,
            logo,
            point: Vector2::default(),
            focus: None,
            count: i32::default(),
            menu: false,
        }
    }

    pub fn initial(
        &mut self,
        draw: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        status: &mut InitialState,
        game: &[Game],
    ) -> Option<Status> {
        if draw.window_should_close() {
            return Some(Status::Closure);
        }

        self.begin();

        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );

        match status {
            InitialState::Main => {
                let logo_shape = Vector2::new(self.logo.width as f32, self.logo.height as f32);
                let logo_point = Vector2::new(
                    draw_shape.x * 0.5 - logo_shape.x * 0.5,
                    draw_shape.y * 0.5 - logo_shape.y * 0.5 - Self::LOGO_SHAPE * 0.5,
                );
                let card_shape =
                    Rectangle::new(0.0, 0.0, draw_shape.x, draw_shape.y - Self::LOGO_SHAPE);

                self.card_sharp(draw, card_shape, Window::COLOR_PRIMARY_MAIN, true);

                draw.draw_texture_v(&self.logo, logo_point, Color::WHITE);

                self.point(Vector2::new(20.0, draw_shape.y - Self::LOGO_SHAPE + 24.0));

                if self.button(draw, "New Map").0.click {
                    *status = InitialState::New;
                }
                self.button(draw, "Load Map");
                if self.button(draw, "Exit Mallet").0.click {
                    return Some(Status::Closure);
                }
            }
            InitialState::New => {
                let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, 48.0);

                self.card_sharp(draw, card_shape, Window::COLOR_PRIMARY_MAIN, true);

                self.font(
                    draw,
                    "Game Selection",
                    Vector2::new(16.0, 12.0),
                    Window::COLOR_TEXT,
                );

                self.point(Vector2::new(20.0, 72.0));

                for g in game {
                    if self.button(draw, &g.info.name).0.click {
                        return Some(Status::Success(
                            SuccessState::Main,
                            Window::new(draw, thread),
                            Editor::new(draw, thread, g.clone()),
                        ));
                    }
                }

                self.point(Vector2::new(20.0, draw_shape.y - 56.0));

                if self.button(draw, "Back").0.click {
                    *status = InitialState::Main;
                }
            }
        }

        None
    }

    pub fn success(
        &mut self,
        draw: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        status: &mut SuccessState,
        editor: &mut Editor,
    ) -> Option<Status> {
        if draw.window_should_close() {
            return Some(Status::Closure);
        }

        self.begin();

        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );

        match status {
            SuccessState::Main => {
                self.card_sharp(
                    draw,
                    Rectangle::new(
                        draw_shape.x - Self::EDIT_SHAPE,
                        0.0,
                        Self::EDIT_SHAPE,
                        draw_shape.y,
                    ),
                    Window::COLOR_PRIMARY_MAIN,
                    false,
                );

                self.card_sharp(
                    draw,
                    Rectangle::new(0.0, 0.0, draw_shape.x, Self::TOOL_SHAPE),
                    Window::COLOR_PRIMARY_MAIN,
                    true,
                );

                if self.menu {
                    self.draw_texture(draw, editor);
                } else {
                    self.draw_entity(draw, editor);
                }

                self.draw_widget(draw, thread, editor);

                if self.widget(
                    draw,
                    Vector2::new(
                        draw_shape.x - Self::EDIT_SHAPE + 12.0,
                        Self::TOOL_SHAPE + 16.0,
                    ),
                    "Texture",
                    &editor.asset.inner.texture,
                    &editor.user.texture,
                    !self.menu,
                ) {
                    self.menu = true;
                };

                if self.widget(
                    draw,
                    Vector2::new(
                        draw_shape.x - Self::EDIT_SHAPE + 56.0,
                        Self::TOOL_SHAPE + 16.0,
                    ),
                    "Entity",
                    &editor.asset.inner.entity,
                    &editor.user.entity,
                    self.menu,
                ) {
                    self.menu = false;
                };

                None
            }
            SuccessState::User => None,
        }
    }

    //================================================================

    fn begin(&mut self) {
        self.point = Vector2::default();
        self.count = i32::default();
    }

    fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    #[rustfmt::skip]
    fn draw_widget(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread, editor: &mut Editor) -> Option<Status> {
        let mut x = 0.0;
        let point = 8.0;
        let shift = 44.0;

        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Position", &editor.asset.inner.position, &editor.user.position, !matches!(editor.widget, Widget::Position)) { editor.widget = Widget::Position; }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Rotation", &editor.asset.inner.rotation, &editor.user.rotation, !matches!(editor.widget, Widget::Rotation)) { editor.widget = Widget::Rotation; }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Scale",    &editor.asset.inner.scale,    &editor.user.scale,    !matches!(editor.widget, Widget::Scale))    { editor.widget = Widget::Scale;    }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Vertex",   &editor.asset.inner.vertex,   &editor.user.vertex,   !matches!(editor.widget, Widget::Vertex))   { editor.widget = Widget::Vertex;   }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Edge",     &editor.asset.inner.edge,     &editor.user.edge,     !matches!(editor.widget, Widget::Edge))     { editor.widget = Widget::Edge;     }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Face",     &editor.asset.inner.face,     &editor.user.face,     !matches!(editor.widget, Widget::Face))     { editor.widget = Widget::Face;     };

        let mut x = 0.0;
        let point = draw.get_screen_width() as f32 - 220.0;
        let shift = 44.0;

        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Configuration", &editor.asset.inner.configuration, &editor.user.configuration, true) { println!("1"); }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Reload",        &editor.asset.inner.reload,        &editor.user.reload, true)        { editor.reload(draw, thread); }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Import",        &editor.asset.inner.import,        &editor.user.import, true)        { println!("3"); }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Export",        &editor.asset.inner.export,        &editor.user.export, true)        { println!("4"); }; x += 1.0;
        if self.widget(draw, Vector2::new(point + (shift * x), 12.0), "Exit",          &editor.asset.inner.exit,          &editor.user.exit, true)          { println!("5"); };

        None
    }

    #[rustfmt::skip]
    fn draw_entity(&mut self, draw: &mut RaylibDrawHandle, editor: &mut Editor) {
        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );
        let point = draw_shape.x - Self::EDIT_SHAPE + 12.0;

        self.point(Vector2::new(point, 120.0));

        for entity in &mut editor.entity {
            if entity.focus {
                self.drop(draw, "Pos.", |window, draw| {
                    window.record_number(draw, "X", &mut entity.position[0]);
                    window.record_number(draw, "Y", &mut entity.position[1]);
                    window.record_number(draw, "Z", &mut entity.position[2]);
                });
                self.drop(draw, "Rot.", |window, draw| {
                    window.record_number(draw, "X", &mut entity.rotation[0]);
                    window.record_number(draw, "Y", &mut entity.rotation[1]);
                    window.record_number(draw, "Z", &mut entity.rotation[2]);
                });
                self.drop(draw, "Sca.", |window, draw| {
                    window.record_number(draw, "X", &mut entity.scale[0]);
                    window.record_number(draw, "Y", &mut entity.scale[1]);
                    window.record_number(draw, "Z", &mut entity.scale[2]);
                });

                for (k, v) in &mut entity.lua.meta.data {
                    match v.kind {
                        serde_json::Value::String(ref mut kind) => {
                            self.record(draw, &v.info, kind);
                        },
                        _ => {}
                    }
                }

                break;
            }
        }

        for entity in &editor.script.meta.entity {
            if self.button(draw, &entity.meta.name).0.click {
                editor.entity.push(Entity::new_from_lua(entity.clone()));
            }
        }
    }

    #[rustfmt::skip]
    fn draw_texture(&mut self, draw: &mut RaylibDrawHandle, editor: &mut Editor) {
        let draw_shape = Vector2::new(
            draw.get_screen_width() as f32,
            draw.get_screen_height() as f32,
        );
        let draw_mouse = draw.get_mouse_position();

        let pin = Vector2::new(draw_shape.x - Self::EDIT_SHAPE, Self::TOOL_SHAPE + 64.0);
        let port = Rectangle::new(pin.x, pin.y, Self::EDIT_SHAPE, 152.0);

        {
            let mut scissor = draw.begin_scissor_mode(
                port.x as i32,
                port.y as i32,
                port.width as i32,
                port.height as i32,
            );

            scissor.draw_rectangle(
                port.x as i32,
                port.y as i32,
                port.width as i32,
                port.height as i32,
                Color::RED,
            );

            let shape = Vector2::new(
                (Self::EDIT_SHAPE / 128.0) * editor.asset.inner.default.width as f32,
                (Self::EDIT_SHAPE / 128.0) * editor.asset.inner.default.height as f32,
            );

            scissor.draw_texture_pro(
                &editor.asset.inner.default,
                Rectangle::new(0.0, 0.0, shape.x * 1.0, shape.y * 1.0),
                Rectangle::new(
                    pin.x,
                    pin.y,
                    128.0 * (Self::EDIT_SHAPE / 128.0),
                    128.0 * (Self::EDIT_SHAPE / 128.0),
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );

            scissor.draw_rectangle_lines_ex(
                Rectangle::new(pin.x, pin.y, 128.0, 128.0),
                2.0,
                Color::RED,
            );

            scissor.draw_circle_v(
                pin + Vector2::new((128.0 * 0.0) - 4.0, (128.0 * 0.0) - 4.0),
                8.0,
                Color::RED,
            );

            if port.check_collision_point_rec(draw_mouse) {
                let mut mouse = scissor.get_mouse_delta();
                let wheel = scissor.get_mouse_wheel_move();

                if scissor.is_key_down(KeyboardKey::KEY_X) {
                    mouse.y = 0.0;
                }
                if scissor.is_key_down(KeyboardKey::KEY_Y) {
                    mouse.x = 0.0;
                }

                if scissor.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    self.tool_tip(
                        &mut scissor,
                        pin,
                        &format!("0, 0"),
                        None,
                    );
                } else if scissor.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                    self.tool_tip(
                        &mut scissor,
                        pin,
                        &format!("0, 0"),
                        None,
                    );

                } else if scissor.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
                }

                if scissor.is_key_pressed(KeyboardKey::KEY_SPACE) {
                }
            }
        }

        if port.check_collision_point_rec(draw_mouse) {
            let mut y = 3.0;
            let point = Vector2::new(20.0, draw_shape.y - (256.0 + 10.0));

            self.tool_tip(draw, point + Vector2::new(0.0, 36.0 * y), "Shift", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, point + Vector2::new(0.0, 36.0 * y), "Scale", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, point + Vector2::new(0.0, 36.0 * y), "Angle", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, point + Vector2::new(0.0, 36.0 * y), "Reset", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT))));
        }

        let mut tool: Option<(Vector2, String)> = None;

        for (i, (name, texture)) in editor.asset.outer.texture.iter().enumerate() {
            let x = (i as f32 % 4.0).floor();
            let y = (i as f32 / 4.0).floor();
            let p = Vector2::new(pin.x + 20.0 + x * 72.0, pin.y + 160.0 + y * 72.0);

            self.point(p);
            let state =
                self.button_shape(draw, name, Some(Vector2::new(64.0, 64.0)), Some(texture), true);

            if state.0.hover {
                let name = name.replace(&editor.game.path, "");

                tool = Some((p + Vector2::new(0.0, 64.0 + state.1.get_point()), name));
            }

            if state.0.click {
                for brush in &mut editor.brush {
                    if brush.focus {
                        for f in &mut brush.face {
                            f.texture = Some(name.to_string());
                        }
                    }
                }
            }
        }

        if let Some(tool) = tool {
            self.tool_tip(draw, tool.0, &tool.1, None);
        }
    }

    fn measure_input(&mut self, draw: &mut RaylibDrawHandle, input: &Input) -> Vector2 {
        if let Some(modify) = &input.modify {
            let modify: &str = modify.clone().into();
            let button: &str = input.button.clone().into();

            let m_modify = self.font_measure(modify) + Vector2::new(8.0, 0.0);
            let m_button = self.font_measure(button) + Vector2::new(8.0, 0.0);

            m_modify + m_button + Vector2::new(4.0, 0.0)
        } else {
            let button: &str = input.button.clone().into();

            let m_button = self.font_measure(button) + Vector2::new(4.0, 0.0);

            m_button
        }
    }

    fn draw_input(&mut self, draw: &mut RaylibDrawHandle, point: Vector2, input: &Input) {
        if let Some(modify) = &input.modify {
            let modify: &str = modify.clone().into();
            let button: &str = input.button.clone().into();

            let m_modify = self.font_measure(modify) + Vector2::new(8.0, 0.0);
            let m_button = self.font_measure(button) + Vector2::new(8.0, 0.0);

            self.card_round(
                draw,
                Rectangle::new(point.x, point.y, m_modify.x, m_modify.y),
                Color::WHITE,
            );
            self.card_round(
                draw,
                Rectangle::new(point.x + m_modify.x + 4.0, point.y, m_button.x, m_button.y),
                Color::WHITE,
            );

            self.font(draw, modify, point + Vector2::new(4.0, 0.0), Color::BLACK);
            self.font(
                draw,
                button,
                point + Vector2::new(8.0 + m_modify.x, 0.0),
                Color::BLACK,
            );
        } else {
            let button: &str = input.button.clone().into();

            let m_button = self.font_measure(button) + Vector2::new(8.0, 0.0);

            self.card_round(
                draw,
                Rectangle::new(point.x, point.y, m_button.x, m_button.y),
                Color::WHITE,
            );

            self.font(draw, button, point + Vector2::new(4.0, 0.0), Color::BLACK);
        }
    }

    fn tool_tip(
        &mut self,
        draw: &mut RaylibDrawHandle,
        point: Vector2,
        name: &str,
        input: Option<&Input>,
    ) {
        if let Some(input) = input {
            let measure = self.font_measure(&name);
            let measure_input = self.measure_input(draw, input);

            let shift = Vector2::new(
                (draw.get_screen_width() as f32 - (point.x + measure.x + measure_input.x + 20.0))
                    .min(0.0),
                (draw.get_screen_height() as f32 - (point.y + measure.y + measure_input.y + 4.0))
                    .min(0.0),
            );

            let point = point + shift;

            self.card_round(
                draw,
                Rectangle::new(point.x, point.y, measure.x + 8.0, measure.y),
                Color::new(0, 0, 0, 255),
            );

            self.font(
                draw,
                &name,
                Vector2::new(point.x + 4.0, point.y + 0.0),
                Color::new(255, 255, 255, 255),
            );

            self.draw_input(draw, point + Vector2::new(measure.x + 12.0, 0.0), input);
        } else {
            let measure = self.font_measure(&name);

            let shift = Vector2::new(
                (draw.get_screen_width() as f32 - (point.x + measure.x + 20.0)).min(0.0),
                (draw.get_screen_height() as f32 - (point.y + measure.y + 16.0)).min(0.0),
            );

            let point = point + shift;

            self.card_round(
                draw,
                Rectangle::new(point.x, point.y, measure.x + 8.0, measure.y),
                Color::new(0, 0, 0, 255),
            );

            self.font(
                draw,
                &name,
                Vector2::new(point.x + 4.0, point.y + 0.0),
                Color::new(255, 255, 255, 255),
            );
        }
    }

    fn widget(
        &mut self,
        draw: &mut RaylibDrawHandle,
        point: Vector2,
        name: &str,
        asset: &Texture2D,
        input: &Input,
        active: bool,
    ) -> bool {
        self.point(point);
        let button = self.button_shape(
            draw,
            name,
            Some(Vector2::new(36.0, 36.0)),
            Some(asset),
            active,
        );

        if button.0.hover {
            self.tool_tip(
                draw,
                point + Vector2::new(0.0, 36.0 + button.1.get_point()),
                name,
                Some(input),
            );
        }

        button.0.click || input.get_press(draw)
    }

    fn card_sharp(
        &self,
        draw: &mut RaylibDrawHandle,
        rectangle: Rectangle,
        color: Color,
        direction: bool,
    ) {
        if direction {
            draw.draw_rectangle_gradient_v(
                rectangle.x as i32,
                (rectangle.y + rectangle.height) as i32,
                rectangle.width as i32,
                Self::GRADIENT_SHAPE_Y,
                Self::GRADIENT_COLOR_MAX,
                Self::GRADIENT_COLOR_MIN,
            );
        } else {
            draw.draw_rectangle_gradient_h(
                (rectangle.x - Self::GRADIENT_SHAPE_Y as f32) as i32,
                rectangle.y as i32,
                Self::GRADIENT_SHAPE_Y,
                rectangle.height as i32,
                Self::GRADIENT_COLOR_MIN,
                Self::GRADIENT_COLOR_MAX,
            );
        }

        draw.draw_rectangle_rec(rectangle, color);
    }

    fn card_round(&self, draw: &mut RaylibDrawHandle, rectangle: Rectangle, color: Color) {
        draw.draw_rectangle_gradient_v(
            rectangle.x as i32,
            (rectangle.y + rectangle.height - Self::GRADIENT_POINT_Y) as i32,
            rectangle.width as i32,
            Self::GRADIENT_SHAPE_Y + Self::GRADIENT_POINT_Y as i32,
            Self::GRADIENT_COLOR_MAX,
            Self::GRADIENT_COLOR_MIN,
        );

        draw.draw_rectangle_rounded(
            rectangle,
            Self::CARD_ROUND_SHAPE,
            Self::CARD_ROUND_COUNT,
            color,
        );
    }

    fn text(&mut self, draw: &mut RaylibDrawHandle, text: &str, color: Color) {
        self.font(draw, text, self.point, color);

        self.point.y += self.font_measure(text).y + Self::TEXT_SHIFT;
    }

    fn button(&mut self, draw: &mut RaylibDrawHandle, text: &str) -> (widget::State, widget::Data) {
        self.button_shape(draw, text, None, None, true)
    }

    fn button_image(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        texture: &Texture2D,
        active: bool,
    ) -> (widget::State, widget::Data) {
        self.button_shape(draw, text, None, Some(texture), active)
    }

    fn button_shape(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        shape: Option<Vector2>,
        image: Option<&Texture2D>,
        active: bool,
    ) -> (widget::State, widget::Data) {
        let shape = shape.unwrap_or(Self::BUTTON_SHAPE);

        let rectangle = Rectangle::new(self.point.x, self.point.y, shape.x, shape.y);

        let mut state = widget::State::get(self, draw, rectangle);
        let data = widget::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus && active);
        let data = widget::Data::get(self, text).clone();

        let text_point = Vector2::new(
            rectangle.x + Self::BUTTON_TEXT_SHIFT.x,
            rectangle.y + Self::BUTTON_TEXT_SHIFT.y - data.get_point(),
        );

        let color = {
            if active {
                data.get_color(&Window::COLOR_PRIMARY_SIDE)
            } else {
                let c = data.get_color(&Window::COLOR_PRIMARY_SIDE);
                Color::new(
                    (c.r as f32 * 0.5) as u8,
                    (c.g as f32 * 0.5) as u8,
                    (c.b as f32 * 0.5) as u8,
                    c.a,
                )
            }
        };

        self.card_round(draw, data.get_shape(&rectangle), color);

        if let Some(image) = image {
            draw.draw_texture_pro(
                image,
                Rectangle::new(0.0, 0.0, image.width as f32, image.height as f32),
                Rectangle::new(
                    rectangle.x + 4.0,
                    rectangle.y + 4.0 - data.get_point(),
                    shape.x - 8.0,
                    shape.y - 8.0,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        } else {
            self.font(draw, text, text_point, data.get_color(&Self::COLOR_TEXT));
        }

        let result = {
            if !active {
                state.click = false;
            }

            (state, data)
        };

        self.point.y += shape.y + Self::BUTTON_SHIFT;
        self.count += 1;

        if result.0.click {
            println!("Click!");
        }

        result
    }

    /*
    fn toggle(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut bool) {
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::TOGGLE_SHAPE.x,
            Self::TOGGLE_SHAPE.y,
        );
        let rectangle_min = Rectangle::new(
            self.point.x + (Self::TOGGLE_SHAPE.x * 0.25),
            self.point.y + (Self::TOGGLE_SHAPE.y * 0.25),
            Self::TOGGLE_SHAPE.x - (Self::TOGGLE_SHAPE.x * 0.5),
            Self::TOGGLE_SHAPE.y - (Self::TOGGLE_SHAPE.y * 0.5),
        );

        let state = widget::State::get(self, draw, rectangle_max);
        let data = widget::Data::get_mutable(self);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self);

        let text_point = Vector2::new(
            rectangle_max.x + rectangle_max.width + Self::TOGGLE_SHIFT,
            rectangle_max.y - data.get_point(),
        );

        if state.click {
            *value = !*value;
        }

        self.card_round(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_PRIMARY_MAIN),
        );

        if *value {
            self.card_round(
                draw,
                data.get_shape(&rectangle_min),
                data.get_color(&Window::COLOR_PRIMARY_SIDE),
            );
        }

        self.font(
            draw,
            text,
            text_point,
            data.get_color(&Self::COLOR_TEXT_MAIN),
        );

        self.point.y += Self::TOGGLE_SHAPE.y + Self::TOGGLE_SHIFT;
        self.count += 1;
    }
    */

    fn slider(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        value: &mut f32,
        min: f32,
        max: f32,
    ) {
        let percent = (*value - min) / (max - min);
        let rectangle_hit = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::SLIDER_SHAPE_MAX.x,
            Self::SLIDER_SHAPE_MAX.y,
        );
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y + (Self::SLIDER_SHAPE_MAX.y - Self::SLIDER_SHAPE_MIN.y) * 0.5,
            Self::SLIDER_SHAPE_MIN.x,
            Self::SLIDER_SHAPE_MIN.y,
        );
        let rectangle_min = Rectangle::new(
            self.point.x,
            self.point.y + (Self::SLIDER_SHAPE_MAX.y - Self::SLIDER_SHAPE_MIN.y) * 0.5,
            Self::SLIDER_SHAPE_MIN.x * percent,
            Self::SLIDER_SHAPE_MIN.y,
        );

        let state = widget::State::get(self, draw, rectangle_hit);
        let data = widget::Data::get(self, text);
        data.set_hover(draw, state.hover || state.focus);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self, text).clone();

        let text_point = Vector2::new(
            self.point.x + Self::SLIDER_SHAPE_MAX.x + Self::SLIDER_SHIFT,
            self.point.y - data.get_point(),
        );

        if state.focus {
            let mouse = (draw.get_mouse_x() as f32 - rectangle_hit.x)
                / ((rectangle_hit.x + rectangle_hit.width) - rectangle_hit.x);

            let mouse = mouse.clamp(0.0, 1.0);

            *value = mouse * (max - min) + min;
        }

        self.card_sharp(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_PRIMARY_SIDE),
            true,
        );
        self.card_sharp(
            draw,
            data.get_shape(&rectangle_min),
            data.get_color(&Window::COLOR_PRIMARY_MAIN),
            true,
        );

        if state.focus {
            let pin = Vector2::new(
                self.point.x + Self::SLIDER_FOCUS_POINT.x + rectangle_max.width * percent,
                self.point.y + Self::SLIDER_FOCUS_POINT.y - data.get_point(),
            );

            draw.draw_circle_v(
                pin,
                Self::SLIDER_FOCUS_SHAPE,
                data.get_color(&Self::COLOR_PRIMARY_MAIN),
            );

            draw.draw_triangle(
                pin + Vector2::new(-Self::SLIDER_FOCUS_SHAPE, 0.0),
                pin + Vector2::new(0.0, -Self::SLIDER_FOCUS_POINT.y),
                pin + Vector2::new(Self::SLIDER_FOCUS_SHAPE, 0.0),
                data.get_color(&Self::COLOR_PRIMARY_MAIN),
            );

            let value = &format!("{value:.0}");

            let measure = self.font_measure(value);

            self.font(draw, value, pin - measure * 0.5, Self::COLOR_TEXT_MAIN);
        }

        let pin = Vector2::new(
            self.point.x + Self::SLIDER_SHAPE_MAX.x * percent,
            self.point.y + Self::SLIDER_SHAPE_MAX.y * 0.5 - data.get_point(),
        );

        draw.draw_circle_v(
            pin,
            Self::SLIDER_CIRCLE_SHAPE,
            data.get_color(&Self::COLOR_PRIMARY_MAIN),
        );

        self.font(draw, text, text_point, Self::COLOR_TEXT_MAIN);

        self.point.y += Self::SLIDER_SHAPE_MAX.y + Self::SLIDER_SHIFT;
        self.count += 1;
    }

    fn record_number(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut f32) {
        let rectangle_hit = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::RECORD_SHAPE_MAX.x,
            Self::RECORD_SHAPE_MAX.y,
        );
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y + Self::RECORD_SHAPE_MAX.y - Self::RECORD_SHAPE_MIN.y,
            Self::RECORD_SHAPE_MIN.x,
            Self::RECORD_SHAPE_MIN.y,
        );

        let state = widget::State::get(self, draw, rectangle_hit);
        let data = widget::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self, text).clone();

        let text_max_point = Vector2::new(
            self.point.x,
            self.point.y - data.get_point() - Self::RECORD_SHAPE_MIN.y,
        );
        let text_min_point = Vector2::new(
            self.point.x + Self::RECORD_SHAPE_MAX.x + Self::RECORD_SHIFT,
            self.point.y - data.get_point(),
        );

        self.card_sharp(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_PRIMARY_MAIN),
            true,
        );
        self.font(
            draw,
            &format!("{value}"),
            text_max_point,
            data.get_color(&Self::COLOR_TEXT_SIDE),
        );
        self.font(
            draw,
            text,
            text_min_point,
            data.get_color(&Self::COLOR_TEXT_MAIN),
        );

        unsafe {
            if state.hover {
                let key = ffi::GetCharPressed();

                if draw.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || ffi::IsKeyPressedRepeat(KeyboardKey::KEY_BACKSPACE as i32)
                {
                    let mut work = value.to_string();
                    work.pop();
                    *value = work.parse().unwrap_or(0.0);
                } else if key != 0 {
                    let mut work = value.to_string();
                    work.push(key as u8 as char);
                    *value = work.parse().unwrap_or(0.0);
                }

                let measure = self.font_measure(&format!("{value}"));

                self.card_sharp(
                    draw,
                    data.get_shape(&Rectangle::new(
                        self.point.x + measure.x + 1.0,
                        self.point.y,
                        Self::RECORD_SHAPE_CARET.x,
                        Self::RECORD_SHAPE_CARET.y,
                    )),
                    data.get_color(&Color::BLACK),
                    true,
                );
            }
        }

        self.point.y += Self::RECORD_SHAPE_MAX.y + Self::RECORD_SHIFT;
        self.count += 1;
    }

    fn record(&mut self, draw: &mut RaylibDrawHandle, text: &str, value: &mut String) {
        let rectangle_hit = Rectangle::new(
            self.point.x,
            self.point.y,
            Self::RECORD_SHAPE_MAX.x,
            Self::RECORD_SHAPE_MAX.y,
        );
        let rectangle_max = Rectangle::new(
            self.point.x,
            self.point.y + Self::RECORD_SHAPE_MAX.y - Self::RECORD_SHAPE_MIN.y,
            Self::RECORD_SHAPE_MIN.x,
            Self::RECORD_SHAPE_MIN.y,
        );

        let state = widget::State::get(self, draw, rectangle_hit);
        let data = widget::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = widget::Data::get(self, text).clone();

        let text_max_point = Vector2::new(
            self.point.x,
            self.point.y - data.get_point() - Self::RECORD_SHAPE_MIN.y,
        );
        let text_min_point = Vector2::new(
            self.point.x + Self::RECORD_SHAPE_MAX.x + Self::RECORD_SHIFT,
            self.point.y - data.get_point(),
        );

        self.card_sharp(
            draw,
            data.get_shape(&rectangle_max),
            data.get_color(&Window::COLOR_PRIMARY_MAIN),
            true,
        );
        self.font(
            draw,
            value,
            text_max_point,
            data.get_color(&Self::COLOR_TEXT_SIDE),
        );
        self.font(
            draw,
            text,
            text_min_point,
            data.get_color(&Self::COLOR_TEXT_MAIN),
        );

        unsafe {
            if state.hover {
                let key = ffi::GetCharPressed();

                if draw.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                    || ffi::IsKeyPressedRepeat(KeyboardKey::KEY_BACKSPACE as i32)
                {
                    value.pop();
                } else if key != 0 {
                    value.push(key as u8 as char);
                }

                let measure = self.font_measure(value);

                self.card_sharp(
                    draw,
                    data.get_shape(&Rectangle::new(
                        self.point.x + measure.x,
                        self.point.y,
                        Self::RECORD_SHAPE_CARET.x,
                        Self::RECORD_SHAPE_CARET.y,
                    )),
                    data.get_color(&Window::COLOR_PRIMARY_MAIN),
                    true,
                );
            }
        }

        self.point.y += Self::RECORD_SHAPE_MAX.y + Self::RECORD_SHIFT;
        self.count += 1;
    }

    fn drop<F: FnOnce(&mut Window, &mut RaylibDrawHandle)>(
        &mut self,
        draw: &mut RaylibDrawHandle,
        text: &str,
        call: F,
    ) {
        let shape = Self::BUTTON_SHAPE;

        let rectangle = Rectangle::new(self.point.x, self.point.y, shape.x, shape.y);

        let state = widget::State::get(self, draw, rectangle);
        let data = widget::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        if state.click {
            data.active = !data.active;
        }
        let data = widget::Data::get(self, text).clone();

        let text_point = Vector2::new(
            rectangle.x + Self::BUTTON_TEXT_SHIFT.x,
            rectangle.y + Self::BUTTON_TEXT_SHIFT.y - data.get_point(),
        );

        self.card_round(
            draw,
            data.get_shape(&rectangle),
            data.get_color(&Window::COLOR_PRIMARY_SIDE),
        );

        self.font(draw, text, text_point, data.get_color(&Self::COLOR_TEXT));

        self.point.y += shape.y + Self::BUTTON_SHIFT;
        self.count += 1;

        println!("{}", data.active);

        if data.active {
            call(self, draw);
        }
    }

    fn font(&self, draw: &mut RaylibDrawHandle, text: &str, point: Vector2, color: Color) {
        draw.draw_text_ex(
            &self.font,
            text,
            point,
            Self::TEXT_SHAPE,
            Self::TEXT_SPACE,
            color,
        );
    }

    fn font_measure(&self, text: &str) -> Vector2 {
        self.font
            .measure_text(text, Self::TEXT_SHAPE, Self::TEXT_SPACE)
    }
}

pub mod widget {
    use super::*;

    #[derive(Default, Debug)]
    pub struct State {
        pub hover: bool,
        pub focus: bool,
        pub click: bool,
    }

    impl State {
        pub fn get(window: &mut Window, draw: &RaylibDrawHandle, rectangle: Rectangle) -> Self {
            let mut state = Self::default();
            let hover = rectangle.check_collision_point_rec(draw.get_mouse_position());

            if hover {
                if window.focus.is_none()
                    && draw.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    window.focus = Some(window.count);
                }

                state.hover = true;
            }

            if let Some(focus) = window.focus {
                if focus == window.count {
                    if draw.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                        if hover {
                            state.click = true;
                        }

                        window.focus = None;
                    }

                    state.focus = true;
                }
            }

            state
        }
    }

    #[derive(Copy, Clone, Default)]
    pub struct Data {
        pub hover: f32,
        pub focus: f32,
        pub active: bool,
    }

    impl Data {
        const POINT_SHIFT: f32 = 4.0;
        const COLOR_UPPER: f32 = 0.25;
        const COLOR_LOWER: f32 = 0.75;
        const HOVER_SPEED: f32 = 16.0;
        const FOCUS_SPEED: f32 = 16.0;

        pub fn get<'a>(window: &'a mut Window, hash: &str) -> &'a mut Self {
            window.data.entry(hash.to_string()).or_default()
        }

        pub fn get_point(&self) -> f32 {
            ((self.hover - 1.0) + (1.0 - self.focus)) * Self::POINT_SHIFT
        }

        pub fn get_shape(&self, rectangle: &Rectangle) -> Rectangle {
            Rectangle::new(
                rectangle.x,
                rectangle.y - self.get_point(),
                rectangle.width,
                rectangle.height,
            )
        }

        pub fn get_color(&self, color: &Color) -> Color {
            Color::new(
                (color.r as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.g as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                (color.b as f32 * ((self.hover * Self::COLOR_UPPER) + Self::COLOR_LOWER)) as u8,
                color.a,
            )
        }

        pub fn set_hover(&mut self, draw: &RaylibDrawHandle, value: bool) {
            if value {
                self.hover += draw.get_frame_time() * Self::HOVER_SPEED;
            } else {
                self.hover -= draw.get_frame_time() * Self::HOVER_SPEED;
            }

            self.hover = self.hover.clamp(0.0, 1.0);
        }

        pub fn set_focus(&mut self, draw: &RaylibDrawHandle, value: bool) {
            if value {
                self.focus += draw.get_frame_time() * Self::FOCUS_SPEED;
            } else {
                self.focus -= draw.get_frame_time() * Self::FOCUS_SPEED;
            }

            self.focus = self.focus.clamp(0.0, 1.0);
        }
    }
}
