//! Renders a single scene described by a TOML file to a PNG.
//!
//! The file uses the same format as the animation files (see
//! `src/scene_file.rs`), but with a `[scene]` table in place of
//! `[animation]` and no `[[frames]]` list. See `scenes/*.toml` for
//! examples.
//!
//! Usage:
//!
//!     cargo run --release --bin render -- <scene.toml> [--scale N] [--output PATH]
//!
//! `--scale` multiplies the scene's base resolution, so the same file can
//! render small while iterating and large for a shareable asset.

use ray_tracer::scene_file;

struct Arguments {
    scene_path: String,
    scale: u32,
    output: Option<String>,
}

fn usage() -> ! {
    eprintln!("usage: render <scene.toml> [--scale N] [--output PATH]");
    std::process::exit(2);
}

fn fail(message: &str) -> ! {
    eprintln!("error: {}", message);
    std::process::exit(1);
}

fn parse_arguments() -> Arguments {
    let mut scene_path = None;
    let mut scale = 1;
    let mut output = None;

    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--scale" => {
                scale = arguments
                    .next()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or_else(|| usage());
            }
            "--output" => {
                output = Some(arguments.next().unwrap_or_else(|| usage()));
            }
            "--help" | "-h" => usage(),
            _ if scene_path.is_none() && !argument.starts_with('-') => {
                scene_path = Some(argument);
            }
            _ => usage(),
        }
    }

    return Arguments {
        scene_path: scene_path.unwrap_or_else(|| usage()),
        scale,
        output,
    };
}

fn main() {
    let arguments = parse_arguments();

    let source = std::fs::read_to_string(&arguments.scene_path).unwrap_or_else(|error| {
        fail(&format!(
            "could not read `{}`: {}",
            arguments.scene_path, error
        ))
    });
    let scene = scene_file::SceneFile::parse(&source).unwrap_or_else(|error| {
        fail(&format!(
            "could not parse `{}`:\n{}",
            arguments.scene_path, error
        ))
    });

    println!(
        "Rendering `{}` at {}x{}",
        scene.settings().name,
        scene.settings().width * arguments.scale,
        scene.settings().height * arguments.scale
    );
    let world = scene.build_world().unwrap_or_else(|error| fail(&error));
    let camera = scene.build_camera(arguments.scale);
    let canvas = camera.render(&world);

    let output = arguments
        .output
        .unwrap_or_else(|| format!("{}.png", scene.settings().name));
    canvas
        .canvas_to_image()
        .save(&output)
        .unwrap_or_else(|error| fail(&format!("could not write `{}`: {}", output, error)));
    println!("Wrote `{}`", output);
}
