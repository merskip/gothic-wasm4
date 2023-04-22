impl crate::windows_image_provider::WindowsImage {
    pub const PLAYER: Self = Self {
        path: r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Player.png",
        size: gothic::ui::geometry::Size::new(12, 16)
    };
    pub const KING_RHOBAR_2: Self = Self {
        path: r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\King Rhobar 2.png",
        size: gothic::ui::geometry::Size::new(16, 18)
    };
    pub const ORC: Self = Self {
        path: r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Orc.png",
        size: gothic::ui::geometry::Size::new(24, 22)
    };
    pub const CROSSBONES: Self = Self {
        path: r"\\?\C:\Users\Piotr\Workspace\gothic-wasm4\gothic\resources\Crossbones.png",
        size: gothic::ui::geometry::Size::new(16, 16)
    };
}