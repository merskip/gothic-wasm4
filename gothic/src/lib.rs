#![no_std]

extern crate alloc;


use crate::audio::music::Music;
use crate::dispatcher::Dispatcher;
use crate::game::main_menu::make_main_menu;
use crate::music_clips::MAIN_THEME;
use crate::renderable::{Canvas, RenderContext};
use crate::platform_context::PlatformContext;
use crate::ui::geometry::{Point, Rect};
use crate::ui::navigator::Navigator;
use crate::updatable::{Controls, UpdateContext};

pub mod ui;
pub mod game;
pub mod renderable;
pub mod updatable;
pub mod dispatcher;
pub mod audio;
pub mod music_clips;
pub mod dialogue;
pub mod image_asset;
pub mod platform_context;

pub struct GothicApplication {
    dispatcher: Dispatcher,
    navigator: Navigator,
    platform: &'static dyn PlatformContext,
    music: Music,
}

impl GothicApplication {
    pub fn start(platform: &'static dyn PlatformContext) -> Self {
        let mut navigator = Navigator::new();
        navigator.push_view(make_main_menu());

        let mut music = Music::new(platform.audio_system());
        music.play_clip(&MAIN_THEME);

        Self {
            dispatcher: Dispatcher::new(),
            navigator,
            platform,
            music,
        }
    }

    pub fn update(&mut self, controls: &dyn Controls) {
        let mut context = UpdateContext::new(
            &mut self.dispatcher,
            &mut self.navigator,
            controls,
            self.platform,
        );

        if let Some(mut top_view) = context.navigator.top_view_mut() {
            top_view.update(&mut context);
            context.navigator.push_view_box(top_view);
        }
        self.music.update();
        context.dispatcher.execute(&mut context);
    }

    pub fn render(&self, canvas: &dyn Canvas) {
        let frame = Rect::new(
            Point::new(0, 0),
            canvas.get_size(),
        );
        if let Some(top_view) = self.navigator.top_view() {
            let mut context = RenderContext::new(canvas, frame, self.platform);
            top_view.render(&mut context);
        }
    }
}
