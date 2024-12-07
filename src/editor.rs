use crate::asset::*;
use crate::brush::*;
use crate::entity::*;
use crate::game::*;
use crate::script::*;
use crate::user::*;
use crate::view::*;
use crate::widget::*;
use crate::window::*;

//================================================================

use raylib::prelude::*;

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

impl Editor {
    #[rustfmt::skip]
    pub fn new(handle: &mut RaylibHandle, thread: &RaylibThread, game: Game) -> Self {
        let mut asset = Asset::new(handle, thread);
        let mut script = Script::new(&game);

        asset.outer.set_texture_list(handle, thread, &script.meta.texture);

        Self {
            //brush: vec![Brush::default()],
            brush: Vec::default(),
            entity: Vec::default(),
            widget: Widget::default(),
            asset,
            view: [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
                //View::new(handle, thread, Camera3D::orthographic(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 30.0)),
            ],
            user: User::new_from_file(&game.path),
            script,
            game,
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

    #[rustfmt::skip]
    pub fn resize(&mut self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        if handle.is_window_resized() {
            self.view = [
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
                View::new(handle, thread, Camera3D::perspective(Vector3::new(4.0, 4.0, 4.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 90.0)),
            ];
        }
    }

    pub fn update(&mut self, draw: &mut RaylibDrawHandle, thread: &RaylibThread) {
        for (i, view) in self.view.iter_mut().enumerate() {
            {
                let mut draw_texture = draw.begin_texture_mode(thread, &mut view.render_texture);

                draw_texture.clear_background(Color::WHITE);

                let mut draw = draw_texture.begin_mode3D(view.camera);

                draw.draw_grid(32, 1.0);

                for brush in &self.brush {
                    brush.draw(&self.asset.inner.default);
                }

                for entity in &self.entity {
                    entity.draw(&mut draw);
                }
            }

            let shift = Vector2::new(
                (i as f32 % 2.0).floor() * view.render_texture.width() as f32,
                (i as f32 / 2.0).floor() * view.render_texture.height() as f32 + Window::TOOL_SHAPE,
            );

            draw.draw_texture_rec(
                &view.render_texture,
                Rectangle::new(
                    0.0,
                    0.0,
                    view.render_texture.width() as f32,
                    -view.render_texture.height() as f32,
                ),
                shift,
                Color::WHITE,
            );
        }
    }
}
