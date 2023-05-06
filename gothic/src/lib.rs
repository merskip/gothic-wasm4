#![no_std]

extern crate alloc;


use crate::dispatcher::Dispatcher;
use crate::game::main_menu::make_main_menu;
use crate::renderable::{Canvas, RenderContext};
use crate::system::{set_shared_system, System};
use crate::ui::geometry::{Point, Rect};
use crate::ui::navigator::Navigator;
use crate::updatable::{Controls, UpdateContext};

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;
// pub mod audio;
// pub mod music_clips;
pub mod dialogue;
pub mod image_asset;
pub mod system;

pub struct GothicApplication {
    dispatcher: Dispatcher,
    navigator: Navigator,
    // music: Music,
}

impl GothicApplication {
    pub fn start(system: &'static dyn System) -> Self {
        set_shared_system(system);

        let mut navigator = Navigator::new();
        navigator.push_view(make_main_menu());

        // let mut music = Music::new(THEME);Audio::shared());
        //         // music.play_clip(&music_clips::MAIN_

        Self {
            dispatcher: Dispatcher::new(),
            navigator,
            // music,
        }
    }

    pub fn update(&mut self, controls: &dyn Controls) {
        let mut context = UpdateContext::new(
            &mut self.dispatcher,
            &mut self.navigator,
            controls,
            // &mut self.music,
        );

        if let Some(mut top_view) = context.navigator.top_view_mut() {
            top_view.update(&mut context);
            context.navigator.push_view_box(top_view);
        }
        // context.music.update();
        context.dispatcher.execute(&mut context);
    }

    pub fn render(&self, canvas: &dyn Canvas) {
        let frame = Rect::new(
            Point::new(0, 0),
            canvas.get_size(),
        );
        if let Some(top_view) = self.navigator.top_view() {
            let mut context = RenderContext::new(canvas, frame);
            top_view.render(&mut context);
        }
    }
}
