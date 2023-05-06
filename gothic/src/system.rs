use core::mem::MaybeUninit;
use crate::image_asset::ImageAsset;

use crate::renderable::{Image, ImageProvider};

pub trait System {
    fn image_provider(&self) -> &'static dyn ImageProvider;
}

static mut SHARED_SYSTEM: MaybeUninit<&dyn System> = MaybeUninit::uninit();

pub(crate) fn set_shared_system(system: &'static dyn System) {
    unsafe {
        SHARED_SYSTEM.write(system);
    }
}

pub fn shared_system() -> &'static dyn System {
    unsafe {
        SHARED_SYSTEM.assume_init()
    }
}

pub fn get_image(image: ImageAsset) -> &'static dyn Image {
    shared_system().image_provider().get_image(image)
}