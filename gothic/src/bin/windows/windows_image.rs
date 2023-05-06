use std::any::Any;
use std::ptr::null;

use windows::core::Result;
use windows::Win32::Graphics::Direct2D::{ID2D1Bitmap, ID2D1RenderTarget};
use windows::Win32::Graphics::Imaging::{GUID_WICPixelFormat32bppPRGBA, IWICImagingFactory, WICBitmapDitherTypeNone, WICBitmapPaletteTypeMedianCut, WICDecodeMetadataCacheOnDemand};
use windows::Win32::System::Com::StructuredStorage::CreateStreamOnHGlobal;
use windows::Win32::System::Memory::{GlobalAlloc, GMEM_FIXED};

use gothic::image_asset::ImageAsset;
use gothic::renderable::Image;
use gothic::ui::geometry::Size;

pub struct WindowsImage {
    pub image_asset: ImageAsset,
    pub bytes: &'static [u8],
    pub native_size: Size,
    pub size: Size,
    pub cached_bitmap: Option<ID2D1Bitmap>
}

impl Image for WindowsImage {
    fn image_asset(&self) -> ImageAsset {
        self.image_asset
    }

    fn size(&self) -> Size {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WindowsImage {

    pub fn bitmap(&mut self, imaging_factory: &IWICImagingFactory, target: &ID2D1RenderTarget) -> ID2D1Bitmap {
        let bitmap = self.cached_bitmap.as_ref();
        return if let Some(bitmap) = bitmap {
            bitmap.clone()
        } else {
            let bitmap = self.load_bitmap(imaging_factory, target)
                .expect("Failed load bitmap");
            self.cached_bitmap.replace(bitmap.clone());
            bitmap
        }
    }

    fn load_bitmap(&self, imaging_factory: &IWICImagingFactory, target: &ID2D1RenderTarget) -> Result<ID2D1Bitmap> {
        unsafe {
            let global_alloc = GlobalAlloc(GMEM_FIXED, self.bytes.len())?;
            let stream = CreateStreamOnHGlobal(global_alloc, false)?;
            stream.Write(self.bytes.as_ptr() as *const _, self.bytes.len() as u32, None).unwrap();

            let decoder = imaging_factory.CreateDecoderFromStream(&stream, null(), WICDecodeMetadataCacheOnDemand)?;

            let source = decoder.GetFrame(0)?;
            let image = imaging_factory.CreateFormatConverter()?;
            image.Initialize(
                &source,
                &GUID_WICPixelFormat32bppPRGBA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            ).unwrap();

            let bitmap = target.CreateBitmapFromWicBitmap(&image, None)?;
            Ok(bitmap)
        }
    }
}