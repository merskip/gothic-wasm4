use std::fs;
use std::path::Path;
use std::process::Command;

use codegen::Scope;
use imagesize;

fn main() {
    let images = [
        Path::new("resources/Player.png"),
        Path::new("resources/King Rhobar 2.png"),
        Path::new("resources/Orc.png"),
        Path::new("resources/Crossbones.png"),
    ];

    generate_image_assets("src/image_asset.rs".as_ref(), &images);
    generate_wasm4_sprites("src/bin/wasm4/sprites.rs", &images);
    generate_windows_images("src/bin/windows/windows_images.rs".as_ref(), &images);
    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_image_assets(output: &Path, images: &[&Path]) {
    let mut scope = Scope::new();

    let image_asset = scope.new_enum("ImageAsset");
    image_asset.doc("*THIS FILE IS GENERATED, DON'T CHANGE IT*");
    image_asset.vis("pub");
    image_asset.derive("Copy");
    image_asset.derive("Clone");
    image_asset.derive("Eq");
    image_asset.derive("PartialEq");
    image_asset.derive("Debug");

    for image_path in images {
        let variant = get_enum_variant(image_path);
        image_asset.new_variant(variant);
    }

    fs::write(output, scope.to_string()).unwrap();
}

fn generate_wasm4_sprites(output: &str, images: &[&Path]) {
    let mut command = Command::new("w4");
    let command = command
        .arg("png2src")
        .arg("--rust")
        .args(&["--template", "resources/sprite.mustache"])
        .arg("--output").arg(output)
        .args(images);

    let output = command.output().unwrap();
    if !output.status.success() || !output.stderr.is_empty() {
        panic!("Failed generate sprites: {}", String::from_utf8(output.stderr).unwrap());
    }

    for image in images {
        println!("cargo:rerun-if-changed={}", image.to_str().unwrap());
    }
}

fn generate_windows_images(output: &Path, images: &[&Path]) {
    let mut scope = Scope::new();

    let image_impl = scope.new_impl("crate::windows_image_provider::WindowsImage");
    for image_path in images {
        let image_abs_path = fs::canonicalize(image_path).unwrap();
        let const_name = get_const_name(image_path);
        let image_size = imagesize::size(image_path).unwrap();

        image_impl.associate_const(
            const_name,
            "Self",
            format!("Self {{\n    path: r\"{}\",\n    size: gothic::ui::geometry::Size::new({}, {})\n}}",
                    image_abs_path.to_str().unwrap(), image_size.width, image_size.height),
            "pub",
        );
    }

    fs::write(output, scope.to_string()).unwrap();
}

fn get_enum_variant(path: &Path) -> String {
    // Obtains file name without extension (eg. "King Rhobar 2")
    let base_name = path.file_stem().unwrap().to_str().unwrap();
    base_name.replace(" ", "")
}

fn get_const_name(path: &Path) -> String {
    // Obtains file name without extension (eg. "King Rhobar 2")
    let base_name = path.file_stem().unwrap().to_str().unwrap();
    base_name.replace(" ", "_").to_uppercase()
}