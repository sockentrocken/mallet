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

use crate::editor::*;
use crate::game::*;
use crate::helper::screen_shape;
use crate::status::*;

//================================================================

use raylib::prelude::*;
use serde_json::Number;
use std::collections::HashMap;

//================================================================

// window structure, responsible for drawing the interface at any given point during the program's life.
pub struct Window {
    gizmo: HashMap<String, gizmo::Data>,
    point: Vector2,
    shape: Option<Rectangle>,
    focus: Option<i32>,
    count: i32,
}

impl Window {
    const COLOR_PRIMARY_MAIN: Color = Color::new(3, 169, 244, 255);
    const COLOR_PRIMARY_SIDE: Color = Color::new(68, 138, 255, 255);
    const COLOR_TEXT_WHITE: Color = Color::new(255, 255, 255, 255);
    const COLOR_TEXT_BLACK: Color = Color::new(33, 33, 33, 255);

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

    //================================================================

    const SLIDER_CIRCLE_SHAPE: f32 = 8.0;
    const SLIDER_FOCUS_POINT: Vector2 = Vector2::new(0.0, -32.0);
    const SLIDER_FOCUS_SHAPE: f32 = 20.0;
    const SLIDER_SHAPE_MAX: Vector2 = Vector2::new(160.0, 24.0);
    const SLIDER_SHAPE_MIN: Vector2 = Vector2::new(160.0, 4.0);
    const SLIDER_SHIFT: f32 = 8.0;

    //================================================================

    const RECORD_SHAPE_MAX: Vector2 = Vector2::new(160.0, 24.0);
    const RECORD_SHAPE_MIN: Vector2 = Vector2::new(160.0, 4.0);
    const RECORD_SHAPE_CARET: Vector2 = Vector2::new(2.0, 16.0);
    const RECORD_SHIFT: f32 = 8.0;

    //================================================================

    pub const TOOL_SHAPE: f32 = 56.0;
    pub const EDIT_SHAPE: f32 = 400.0;

    //================================================================

    // create a new window.
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            gizmo: HashMap::default(),
            point: Vector2::default(),
            shape: None,
            focus: None,
            count: i32::default(),
        }
    }

    // draw the initial window.
    pub fn initial(
        &mut self,
        draw: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        status: &mut InitialState,
        asset: &Asset,
        game: &[Game],
    ) -> Option<Status> {
        self.begin();

        let draw_shape = screen_shape(draw);

        match status {
            InitialState::Main => {
                let logo_shape = Vector2::new(
                    asset.inner.logo.width as f32,
                    asset.inner.logo.height as f32,
                );
                let logo_point = Vector2::new(
                    draw_shape.x * 0.5 - logo_shape.x * 0.5,
                    draw_shape.y * 0.5 - logo_shape.y * 0.5 - Self::LOGO_SHAPE * 0.5,
                );
                let card_shape =
                    Rectangle::new(0.0, 0.0, draw_shape.x, draw_shape.y - Self::LOGO_SHAPE);

                self.card_sharp(draw, card_shape, Window::COLOR_PRIMARY_MAIN, true);

                draw.draw_texture_v(&asset.inner.logo, logo_point, Color::WHITE);

                self.point(Vector2::new(20.0, draw_shape.y - Self::LOGO_SHAPE + 24.0));

                if self.button(draw, asset, "New Map").0.click {
                    *status = InitialState::New;
                }
                self.button(draw, asset, "Load Map");
                if self.button(draw, asset, "Exit Mallet").0.click {
                    return Some(Status::Closure);
                }
            }
            InitialState::New => {
                let card_shape = Rectangle::new(0.0, 0.0, draw_shape.x, 48.0);

                self.card_sharp(draw, card_shape, Window::COLOR_PRIMARY_MAIN, true);

                self.font(
                    draw,
                    asset,
                    "Game Selection",
                    Vector2::new(16.0, 12.0),
                    Window::COLOR_TEXT_WHITE,
                );

                self.point(Vector2::new(20.0, 72.0));

                for g in game {
                    if self.button(draw, asset, &g.info.name).0.click {
                        return Some(Status::Success(
                            SuccessState::Main,
                            Asset::new(draw, thread),
                            Window::new(draw, thread),
                            Editor::new(draw, thread, g.clone()),
                        ));
                    }
                }

                self.point(Vector2::new(20.0, draw_shape.y - 56.0));

                if self.button(draw, asset, "Back").0.click {
                    *status = InitialState::Main;
                }
            }
        }

        if draw.window_should_close() {
            Some(Status::Closure)
        } else {
            None
        }
    }

    pub fn success(
        &mut self,
        draw: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        status: &mut SuccessState,
        asset: &Asset,
        editor: &mut Editor,
    ) -> Option<Status> {
        if draw.window_should_close() {
            return Some(Status::Closure);
        }

        self.begin();

        let draw_shape = screen_shape(draw);

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

                if editor.menu {
                    self.draw_texture(draw, asset, editor);
                } else {
                    self.draw_entity(draw, asset, editor);
                }

                self.draw_widget(draw, asset, thread, editor);

                if self.widget(
                    draw,
                    asset,
                    Vector2::new(
                        draw_shape.x - Self::EDIT_SHAPE + 12.0,
                        Self::TOOL_SHAPE + 16.0,
                    ),
                    "Texture",
                    &editor.asset.inner.texture,
                    &editor.user.texture,
                    !editor.menu,
                ) {
                    editor.menu = true;
                };

                if self.widget(
                    draw,
                    asset,
                    Vector2::new(
                        draw_shape.x - Self::EDIT_SHAPE + 56.0,
                        Self::TOOL_SHAPE + 16.0,
                    ),
                    "Entity",
                    &editor.asset.inner.entity,
                    &editor.user.entity,
                    editor.menu,
                ) {
                    editor.menu = false;
                };

                None
            }
            SuccessState::User => None,
        }
    }

    //================================================================

    // reset the state of the window before drawing.
    fn begin(&mut self) {
        self.point = Vector2::default();
        self.count = i32::default();
    }

    // set the draw point.
    fn point(&mut self, point: Vector2) {
        self.point = point;
    }

    // set the draw shape. if not None, then every widget will check if they are inside of the shape.
    fn shape(&mut self, shape: Option<Rectangle>) {
        self.shape = shape;
    }

    fn check_draw(&self, rectangle: Rectangle) -> bool {
        if let Some(shape) = self.shape {
            shape.check_collision_recs(&rectangle)
        } else {
            true
        }
    }

    fn check_mouse(&self, draw: &RaylibDrawHandle, rectangle: Rectangle) -> bool {
        // get the mouse position.
        let mouse = draw.get_mouse_position();
        // check if the mouse is currently over the widget.
        let mouse_widget = rectangle.check_collision_point_rec(mouse);

        if let Some(shape) = self.shape {
            // return mouse <-> widget check AND view-port <-> widget check AND mouse <-> view-port check.
            mouse_widget && shape.check_collision_recs(&rectangle) && shape.check_collision_point_rec(mouse)
        } else {
            // return mouse <-> widget check.
            mouse_widget
        }
    }

    #[rustfmt::skip]
    fn draw_widget(&mut self, draw: &mut RaylibDrawHandle, asset: &Asset, thread: &RaylibThread, editor: &mut Editor) -> Option<Status> {
        let screen_shape = screen_shape(draw);
        let mut x = 0.0;
        let point = 8.0;
        let shift = 44.0;

        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Position", &editor.asset.inner.position, &editor.user.position, !matches!(editor.widget, Widget::Position)) { editor.widget = Widget::Position; }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Rotation", &editor.asset.inner.rotation, &editor.user.rotation, !matches!(editor.widget, Widget::Rotation)) { editor.widget = Widget::Rotation; }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Scale",    &editor.asset.inner.scale,    &editor.user.scale,    !matches!(editor.widget, Widget::Scale))    { editor.widget = Widget::Scale;    }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Vertex",   &editor.asset.inner.vertex,   &editor.user.vertex,   !matches!(editor.widget, Widget::Vertex))   { editor.widget = Widget::Vertex;   }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Edge",     &editor.asset.inner.edge,     &editor.user.edge,     !matches!(editor.widget, Widget::Edge))     { editor.widget = Widget::Edge;     }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Face",     &editor.asset.inner.face,     &editor.user.face,     !matches!(editor.widget, Widget::Face))     { editor.widget = Widget::Face;     };

        let mut x = 0.0;
        let point = screen_shape.x - 220.0;
        let shift = 44.0;

        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "User",   &editor.asset.inner.user,   &editor.user.user, true)   { println!("1"); }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Reload", &editor.asset.inner.reload, &editor.user.reload, true) { editor.reload(draw, thread); }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Import", &editor.asset.inner.import, &editor.user.import, true) { println!("3"); }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Export", &editor.asset.inner.export, &editor.user.export, true) { println!("4"); }; x += 1.0;
        if self.widget(draw, asset, Vector2::new(point + (shift * x), 12.0), "Exit",   &editor.asset.inner.exit,   &editor.user.exit, true)   { };

        None
    }


    #[rustfmt::skip]
    fn draw_entity(&mut self, draw: &mut RaylibDrawHandle, asset: &Asset, editor: &mut Editor) {
        let draw_shape = screen_shape(draw);
        let point = draw_shape.x - Self::EDIT_SHAPE + 12.0;

        self.point(Vector2::new(point, 120.0));

        for entity in &mut editor.world.entity {
            if entity.focus {
                self.scroll(asset, draw, "##Entity Data",  Rectangle::new(self.point.x, self.point.y, Self::EDIT_SHAPE - 24.0, (draw_shape.y * 0.5) - self.point.y - 32.0), |window, draw, scroll| {
                    window.text(draw, asset, &entity.meta.info, Self::COLOR_TEXT_WHITE);

                    window.drop(&editor.asset, draw, "Position", |window, draw| {
                        window.record_number(draw, asset, "X", &mut entity.position.x);
                        window.record_number(draw, asset, "Y", &mut entity.position.y);
                        window.record_number(draw, asset, "Z", &mut entity.position.z);
                    });
                    window.drop(&editor.asset, draw, "Rotation", |window, draw| {
                        window.record_number(draw, asset, "X", &mut entity.rotation.x);
                        window.record_number(draw, asset, "Y", &mut entity.rotation.y);
                        window.record_number(draw, asset, "Z", &mut entity.rotation.z);
                    });
                    window.drop(&editor.asset, draw, "Scale", |window, draw| {
                        window.record_number(draw, asset, "X", &mut entity.scale.x);
                        window.record_number(draw, asset, "Y", &mut entity.scale.y);
                        window.record_number(draw, asset, "Z", &mut entity.scale.z);
                    });

                    for (_, v) in &mut entity.meta.data {
                        match &mut v.kind {
                            serde_json::Value::Bool(ref mut value) => {
                                window.toggle(draw, asset, &v.info, value);
                            },
                            serde_json::Value::Number(ref mut value) => {
                                let mut cast = value.as_f64().unwrap() as f32;

                                window.record_number(draw, asset, &v.info, &mut cast);

                                *value = Number::from_f64(cast as f64).unwrap();
                            },
                            serde_json::Value::String(ref mut value) => {
                                window.record(draw, asset, &v.info, value);
                            },
                            _ => {},
                        }
                    }
                });

                self.separator(draw, Vector2::new(Self::EDIT_SHAPE - 24.0, 2.0));

                break;
            }
        }

        self.record(draw, asset, "Search Ent.", &mut editor.search_ent);

        self.scroll(asset, draw, "##Entity Scroll", Rectangle::new(self.point.x, self.point.y, Self::EDIT_SHAPE - 24.0, draw_shape.y - self.point.y - 16.0), |window, draw, scroll| {
            for entity in &editor.script.meta.entity {
                if !entity.name.starts_with(&editor.search_ent) {
                    continue;
                }

                if window.button_shape(draw, asset, &entity.name, Some(Vector2::new(Self::EDIT_SHAPE - 24.0, 32.0)), None, true).0.click {
                    editor.world.entity.push(Entity::new_from_lua(entity.clone()));
                }
            }
        });
    }

    fn separator(&mut self, draw: &mut RaylibDrawHandle, shape: Vector2) {
        self.card_sharp(
            draw,
            Rectangle::new(self.point.x, self.point.y + shape.y, shape.x, shape.y),
            Self::COLOR_PRIMARY_SIDE,
            true,
        );

        self.point.y += shape.y + 12.0;
        self.count += 1;
    }

    #[rustfmt::skip]
    fn draw_texture(&mut self, draw: &mut RaylibDrawHandle, asset: &Asset, editor: &mut Editor) {
        let draw_shape = screen_shape(draw);
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
                        asset, 
                        pin,
                        &format!("0, 0"),
                        None,
                    );
                } else if scissor.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
                    self.tool_tip(
                        &mut scissor,
                        asset, 
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

            self.tool_tip(draw, asset, point + Vector2::new(0.0, 36.0 * y), "Shift", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, asset, point + Vector2::new(0.0, 36.0 * y), "Scale", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, asset, point + Vector2::new(0.0, 36.0 * y), "Angle", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT)))); y += 1.0;
            self.tool_tip(draw, asset, point + Vector2::new(0.0, 36.0 * y), "Reset", Some(&Input::new(None, Key::Mouse(MouseButton::MOUSE_BUTTON_LEFT))));
        }

        let p = Vector2::new(pin.x + 12.0, pin.y + 160.0);

        self.point(p);
        self.separator(draw, Vector2::new(Self::EDIT_SHAPE - 24.0, 2.0));
        self.record(draw, asset, "Search Tex.", &mut editor.search_tex);

        let mut tool: Option<(Vector2, String)> = None;

        self.scroll(asset, draw, "##Texture", Rectangle::new(self.point.x, self.point.y, Self::EDIT_SHAPE - 24.0, draw_shape.y - self.point.y - 16.0), |window, draw, scroll| {
            let mut j = 0;

            for (name, texture) in &editor.asset.outer.texture {
                let name = name.replace(&editor.game.path, "");

                if !name.starts_with(&editor.search_tex) {
                    continue;
                }

                let s = (Self::EDIT_SHAPE / 72.0).floor();
                let x = (j as f32 % s).floor();
                let y = (j as f32 / s).floor();
                let p = scroll + Vector2::new(x * 72.0, y * 72.0);

                window.point(p);
                let state =
                    window.button_image(draw, asset, &name, Vector2::new(64.0, 64.0), texture, true);

                if state.0.hover {
                    tool = Some((p + Vector2::new(0.0, 64.0 + state.1.get_point()), name.clone()));
                }

                if state.0.click {
                    for brush in &mut editor.world.brush {
                        if brush.focus {
                            for f in &mut brush.face {
                                f.texture = Some(name.clone());
                            }
                        }
                    }
                }

                j += 1;
            }
        });

        if let Some(tool) = tool {
            self.tool_tip(draw, asset, tool.0, &tool.1, None);
        }
    }

    fn measure_input(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        input: &Input,
    ) -> Vector2 {
        if let Some(modify) = &input.modify {
            let modify: &str = modify.clone().into();
            let button: &str = input.button.clone().into();

            let m_modify = self.font_measure(asset, modify) + Vector2::new(8.0, 0.0);
            let m_button = self.font_measure(asset, button) + Vector2::new(8.0, 0.0);

            m_modify + m_button + Vector2::new(4.0, 0.0)
        } else {
            let button: &str = input.button.clone().into();

            let m_button = self.font_measure(asset, button) + Vector2::new(4.0, 0.0);

            m_button
        }
    }

    fn draw_input(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        point: Vector2,
        input: &Input,
    ) {
        if let Some(modify) = &input.modify {
            let modify: &str = modify.clone().into();
            let button: &str = input.button.clone().into();

            let m_modify = self.font_measure(asset, modify) + Vector2::new(8.0, 0.0);
            let m_button = self.font_measure(asset, button) + Vector2::new(8.0, 0.0);

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

            self.font(
                draw,
                asset,
                modify,
                point + Vector2::new(4.0, 0.0),
                Color::BLACK,
            );
            self.font(
                draw,
                asset,
                button,
                point + Vector2::new(8.0 + m_modify.x, 0.0),
                Color::BLACK,
            );
        } else {
            let button: &str = input.button.clone().into();

            let m_button = self.font_measure(asset, button) + Vector2::new(8.0, 0.0);

            self.card_round(
                draw,
                Rectangle::new(point.x, point.y, m_button.x, m_button.y),
                Color::WHITE,
            );

            self.font(draw, asset, button, point + Vector2::new(4.0, 0.0), Color::BLACK);
        }
    }

    fn tool_tip(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        point: Vector2,
        text: &str,
        input: Option<&Input>,
    ) {
        // measure the size of the tool-tip.
        let measure = self.font_measure(asset, text);

        let measure_input = {
            if let Some(input) = input {
                self.measure_input(draw, asset, input)
            } else {
                Vector2::default()
            }
        };

        let shape = screen_shape(draw);
        let shift = Vector2::new(
            (shape.x - (point.x + measure.x + measure_input.x + 20.0)).min(0.0),
            (shape.y - (point.y + measure.y + measure_input.y + 4.0)).min(0.0),
        );

        let point = point + shift;

        self.card_round(
            draw,
            Rectangle::new(point.x, point.y, measure.x + 8.0, measure.y),
            Color::new(0, 0, 0, 255),
        );

        self.font(
            draw,
            asset,
            text,
            Vector2::new(point.x + 4.0, point.y + 0.0),
            Color::new(255, 255, 255, 255),
        );

        if let Some(input) = input {
            self.draw_input(draw, asset, point + Vector2::new(measure.x + 12.0, 0.0), input);
        }
    }

    // draw a widget, using an icon and a hot-key.
    fn widget(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        point: Vector2,
        name: &str,
        icon: &Texture2D,
        input: &Input,
        active: bool,
    ) -> bool {
        // set widget point.
        self.point(point);

        // draw button, hiding the text for the button. ("##{name}")
        let button = self.button_shape(
            draw,
            asset,
            &format!("##{name}"),
            Some(Vector2::new(36.0, 36.0)),
            Some(icon),
            active,
        );

        // if the mouse is currently over the mouse, draw tool-tip.
        if button.0.hover {
            self.tool_tip(
                draw,
                asset,
                point + Vector2::new(0.0, 36.0 + button.1.get_point()),
                name,
                Some(input),
            );
        }

        // return true if a click event or if the corresponding hot-key has been set off.
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

    fn text(&mut self, draw: &mut RaylibDrawHandle, asset: &Asset, text: &str, color: Color) {
        self.font(draw, asset, text, self.point, color);

        self.point.y += self.font_measure(asset, text).y + Self::TEXT_SHIFT;
    }

    fn button(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
    ) -> (gizmo::State, gizmo::Data) {
        self.button_shape(draw, asset, text, None, None, true)
    }

    fn button_image(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
        shape: Vector2,
        texture: &Texture2D,
        active: bool,
    ) -> (gizmo::State, gizmo::Data) {
        self.button_shape(
            draw,
            asset,
            &format!("##{text}"),
            Some(shape),
            Some(texture),
            active,
        )
    }

    fn button_shape(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
        shape: Option<Vector2>,
        image: Option<&Texture2D>,
        active: bool,
    ) -> (gizmo::State, gizmo::Data) {
        let shape = shape.unwrap_or(Self::BUTTON_SHAPE);
        let rectangle = Rectangle::new(self.point.x, self.point.y, shape.x, shape.y);

        let mut state = gizmo::State::get(self, draw, rectangle);
        let data = gizmo::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus && active);
        let data = *gizmo::Data::get(self, text);

        if self.check_draw(rectangle) {
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

                self.font(
                    draw,
                    asset,
                    Self::text_hash(text),
                    Vector2::new(
                        rectangle.x + shape.x + Self::BUTTON_TEXT_SHIFT.x,
                        rectangle.y - data.get_point(),
                    ),
                    data.get_color(&Self::COLOR_TEXT_WHITE),
                );
            } else {
                self.font(
                    draw,
                    asset,
                    text,
                    text_point,
                    data.get_color(&Self::COLOR_TEXT_WHITE),
                );
            }
        }

        let result = {
            if !active {
                state.click = false;
            }

            (state, data)
        };

        self.point.y += shape.y + Self::BUTTON_SHIFT;
        self.count += 1;

        result
    }

    fn toggle(&mut self, draw: &mut RaylibDrawHandle, asset: &Asset, text: &str, value: &mut bool) {
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

        let state = gizmo::State::get(self, draw, rectangle_max);
        let data = gizmo::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = *gizmo::Data::get(self, text);

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
            asset,
            text,
            text_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
        );

        self.point.y += Self::TOGGLE_SHAPE.y + Self::TOGGLE_SHIFT;
        self.count += 1;
    }

    fn slider(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
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

        let state = gizmo::State::get(self, draw, rectangle_hit);
        let data = gizmo::Data::get(self, text);
        data.set_hover(draw, state.hover || state.focus);
        data.set_focus(draw, state.focus);
        let data = gizmo::Data::get(self, text).clone();

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

            let measure = self.font_measure(asset, value);

            self.font(draw, asset, value, pin - measure * 0.5, Self::COLOR_TEXT_WHITE);
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

        self.font(draw, asset, text, text_point, Self::COLOR_TEXT_WHITE);

        self.point.y += Self::SLIDER_SHAPE_MAX.y + Self::SLIDER_SHIFT;
        self.count += 1;
    }

    fn record_number(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
        value: &mut f32,
    ) {
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

        let state = gizmo::State::get(self, draw, rectangle_hit);
        let data = gizmo::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = gizmo::Data::get(self, text).clone();

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
            asset,
            &format!("{value}"),
            text_max_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
        );
        self.font(
            draw,
            asset,
            text,
            text_min_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
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

                let measure = self.font_measure(asset, &format!("{value}"));

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

    fn record(
        &mut self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
        value: &mut String,
    ) {
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

        let state = gizmo::State::get(self, draw, rectangle_hit);
        let data = gizmo::Data::get(self, text);
        data.set_hover(draw, state.hover);
        data.set_focus(draw, state.focus);
        let data = gizmo::Data::get(self, text).clone();

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
            data.get_color(&Window::COLOR_PRIMARY_SIDE),
            true,
        );
        self.font(
            draw,
            asset,
            value,
            text_max_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
        );
        self.font(
            draw,
            asset,
            text,
            text_min_point,
            data.get_color(&Self::COLOR_TEXT_WHITE),
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

                let measure = self.font_measure(asset, value);

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

    fn scroll<F: FnOnce(&mut Window, &mut RaylibDrawHandle, Vector2)>(
        &mut self,
        asset: &Asset,
        draw: &mut RaylibDrawHandle,
        text: &str,
        shape: Rectangle,
        call: F,) {
        let data = *gizmo::Data::get(self, text);

        unsafe {
            ffi::BeginScissorMode(shape.x as i32, shape.y as i32, shape.width as i32, shape.height as i32);
        }

        let difference = (data.scroll_shape - shape.height).max(0.0);

        let scroll = Vector2::new(shape.x, shape.y - difference * data.scroll_shift);

        self.point(scroll);
        self.shape(Some(shape));

        let old = self.point.y;

        call(self, draw, scroll);

        //draw.draw_rectangle_rec(shape, Color::new(255, 0, 0, 127));

        self.shape = None;

        unsafe {
            ffi::EndScissorMode();
        }

        let old = self.point.y - old;
    
        let data = gizmo::Data::get(self, text);
        data.scroll_shape = old;


        let mouse = draw.get_mouse_position();

        if shape.check_collision_point_rec(mouse) {
            data.scroll_shift -= draw.get_mouse_wheel_move() * 0.1;
            data.scroll_shift = data.scroll_shift.clamp(0.0, 1.0);   
        }

        self.point.y -= old - difference * data.scroll_shift;
        self.point.y += shape.height + 4.0;
        self.count += 1;
    }

    fn drop<F: FnOnce(&mut Window, &mut RaylibDrawHandle)>(
        &mut self,
        asset: &Asset,
        draw: &mut RaylibDrawHandle,
        text: &str,
        call: F,
    ) {
        let data = gizmo::Data::get(self, text);
        let icon = {
            if data.active {
                Some(&asset.inner.drop_b)
            } else {
                Some(&asset.inner.drop_a)
            }
        };

        let button = self.button_shape(draw, asset, text, Some(Vector2::new(24.0, 24.0)), icon, true);

        let data = gizmo::Data::get(self, text);

        if button.0.click {
            data.active = !data.active;
        }

        if data.active {
            call(self, draw);
        }
    }

    fn font(
        &self,
        draw: &mut RaylibDrawHandle,
        asset: &Asset,
        text: &str,
        point: Vector2,
        color: Color,
    ) {
        draw.draw_text_ex(
            &asset.inner.font,
            text,
            point,
            Self::TEXT_SHAPE,
            Self::TEXT_SPACE,
            color,
        );
    }

    fn font_measure(&self, asset: &Asset, text: &str) -> Vector2 {
        asset
            .inner
            .font
            .measure_text(text, Self::TEXT_SHAPE, Self::TEXT_SPACE)
    }

    fn text_hash(text: &str) -> &str {
        let text: Vec<&str> = text.split("##").collect();

        text.first().unwrap_or(&"")
    }
}

pub mod gizmo {
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
            let check = window.check_mouse(draw, rectangle);

            if check {
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
                        if check {
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
        pub scroll_shift: f32,
        pub scroll_shape: f32,
    }

    impl Data {
        const POINT_SHIFT: f32 = 4.0;
        const COLOR_UPPER: f32 = 0.25;
        const COLOR_LOWER: f32 = 0.75;
        const HOVER_SPEED: f32 = 16.0;
        const FOCUS_SPEED: f32 = 16.0;

        pub fn get<'a>(window: &'a mut Window, hash: &str) -> &'a mut Self {
            window.gizmo.entry(hash.to_string()).or_default()
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
