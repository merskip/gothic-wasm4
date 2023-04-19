use std::process::Command;

fn main() {
    png2src("src/bin/wasm4/sprites.rs", &[
        "resources/Player.png",
        "resources/King Rhobar 2.png",
        "resources/Orc.png",
        "resources/Crossbones.png",
    ]);
}

fn png2src(output: &str, images: &[&str]) {
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
        println!("cargo:rerun-if-changed={}", image);
    }
}
