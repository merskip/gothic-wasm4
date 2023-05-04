impl crate::windows_image_provider::WindowsImage {
    pub const PLAYER: Self = Self {
        bytes: include_bytes!(r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Player.png"),
        native_size: gothic::ui::geometry::Size::new(12, 16),
        size: gothic::ui::geometry::Size::new(48, 64),
    };
    pub const KING_RHOBAR_2: Self = Self {
        bytes: include_bytes!(r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\King Rhobar 2.png"),
        native_size: gothic::ui::geometry::Size::new(16, 18),
        size: gothic::ui::geometry::Size::new(64, 72),
    };
    pub const ORC: Self = Self {
        bytes: include_bytes!(r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Orc.png"),
        native_size: gothic::ui::geometry::Size::new(24, 22),
        size: gothic::ui::geometry::Size::new(96, 88),
    };
    pub const CROSSBONES: Self = Self {
        bytes: include_bytes!(r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Crossbones.png"),
        native_size: gothic::ui::geometry::Size::new(16, 16),
        size: gothic::ui::geometry::Size::new(64, 64),
    };
}