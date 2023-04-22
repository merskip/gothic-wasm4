#![no_std]

extern crate alloc;

use core::mem::MaybeUninit;
use crate::dispatcher::Dispatcher;
use crate::game::main_menu::make_main_menu;
use crate::image_asset::ImageAsset;
use crate::renderable::{Canvas, Image, ImageProvider, RenderContext};
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

pub struct GothicApplication {
    dispatcher: Dispatcher,
    navigator: Navigator,
    // music: Music,
}

impl GothicApplication {
    pub fn start(image_provider: &'static dyn ImageProvider) -> Self {
        unsafe {
            SHARED_IMAGE_PROVIDER.write(image_provider);
        }

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

static mut SHARED_IMAGE_PROVIDER: MaybeUninit<&dyn ImageProvider> = MaybeUninit::uninit();

pub fn get_shared_image(image_asset: ImageAsset) -> &'static dyn Image {
    let shared_image_provider = unsafe { SHARED_IMAGE_PROVIDER.assume_init() };
    shared_image_provider.get_image(image_asset)
}