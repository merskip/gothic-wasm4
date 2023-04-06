use std::process::Command;

fn main() {
    png2src("src/sprites.rs", &[
        "resources/character.png",
        "resources/cinematic_intro/king_rhobar_2.png",
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
    command.status().unwrap();
}
