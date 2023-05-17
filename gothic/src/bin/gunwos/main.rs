#![no_std]
#![no_main]

extern crate gunwos_rs;

use gothic::audio::audio_system::AudioSystem;
use gothic::audio::music::Instrument;
use gunwos_rs::cstr;
use gunwos_rs::ipc::Ipc;

use gothic::GothicApplication;
use gothic::image_asset::ImageAsset;
use gothic::platform_context::PlatformContext;
use gothic::renderable::{Image, ImageProvider};

mod allocator;

#[no_mangle]
extern "C" fn dupa() {
    Ipc::send(cstr!("t0"), b"Gothic").unwrap();

    let application = GothicApplication::start(&GunwosSystem);
    Ipc::send(cstr!("t0"), b"Application created").unwrap();
}

struct GunwosSystem;

impl PlatformContext for GunwosSystem {
    fn image_provider(&self) -> &'static dyn ImageProvider {
        &GunwosImageProvider
    }

    fn audio_system(&self) -> &'static dyn AudioSystem {
        &GunwosAudioSystem
    }
}

struct GunwosImageProvider;

impl ImageProvider for GunwosImageProvider {
    fn get_image(&self, asset: ImageAsset) -> &dyn Image {
        todo!()
    }
}

struct GunwosAudioSystem;

impl AudioSystem for GunwosAudioSystem {
    fn stop_all(&self) {
        // Not supported
    }

    fn tone(&self, _instrument: Instrument, _frequency: u32, _duration: u32, _volume: f32) {
        // Not supported
    }
}