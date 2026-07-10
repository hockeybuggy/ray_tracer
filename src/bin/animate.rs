//! Renders the frames of an animation described by a TOML file.
//!
//! The file describes a base scene followed by a list of frames, each
//! restating how that frame differs from the base scene (see
//! `src/scene_file.rs` for the format, and `animations/*.toml` for
//! examples). Frames render in parallel to numbered PNGs in an output
//! directory; assembling them into a gif is left to
//! `scripts/make_gif.sh`.
//!
//! Usage:
//!
//!     cargo run --release --bin animate -- <animation.toml> [--scale N] [--output-dir DIR]
//!
//! `--scale` multiplies the scene's base resolution, so the same file can
//! render small while iterating and large for a shareable asset.

use std::sync::atomic::{AtomicUsize, Ordering};

use ray_tracer::scene_file;

struct Arguments {
    scene_path: String,
    scale: u32,
    output_dir: Option<String>,
}

fn usage() -> ! {
    eprintln!("usage: animate <animation.toml> [--scale N] [--output-dir DIR]");
    std::process::exit(2);
}

fn fail(message: &str) -> ! {
    eprintln!("error: {}", message);
    std::process::exit(1);
}

fn parse_arguments() -> Arguments {
    let mut scene_path = None;
    let mut scale = 1;
    let mut output_dir = None;

    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--scale" => {
                scale = arguments
                    .next()
                    .and_then(|value| value.parse().ok())
                    .unwrap_or_else(|| usage());
            }
            "--output-dir" => {
                output_dir = Some(arguments.next().unwrap_or_else(|| usage()));
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
        output_dir,
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
    let animation = scene_file::AnimationFile::parse(&source).unwrap_or_else(|error| {
        fail(&format!(
            "could not parse `{}`:\n{}",
            arguments.scene_path, error
        ))
    });

    let output_dir = arguments
        .output_dir
        .unwrap_or_else(|| format!("animations/frames/{}", animation.settings().name));
    std::fs::create_dir_all(&output_dir)
        .unwrap_or_else(|error| fail(&format!("could not create `{}`: {}", output_dir, error)));

    let total = animation.frame_count();
    let threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(4)
        .min(total);
    println!(
        "Rendering {} frames at {}x{} on {} threads",
        total,
        animation.settings().width * arguments.scale,
        animation.settings().height * arguments.scale,
        threads
    );

    // Frames are independent, so render them in parallel: each worker
    // claims the next unrendered frame and builds its own world for it.
    let started = std::time::Instant::now();
    let next_frame = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                loop {
                    let index = next_frame.fetch_add(1, Ordering::Relaxed);
                    if index >= total {
                        break;
                    }
                    let world = animation
                        .build_world(index)
                        .unwrap_or_else(|error| fail(&error));
                    let camera = animation.build_camera(index, arguments.scale);
                    let canvas = camera.render(&world);

                    let path = format!("{}/frame_{:03}.png", output_dir, index);
                    canvas
                        .canvas_to_image()
                        .save(&path)
                        .unwrap_or_else(|error| {
                            fail(&format!("could not write `{}`: {}", path, error))
                        });
                    println!(
                        "  frame {:>3}/{} done ({:.0?} elapsed)",
                        index + 1,
                        total,
                        started.elapsed()
                    );
                }
            });
        }
    });

    println!("Wrote {} frames to `{}`", total, output_dir);
    println!(
        "Assemble the gif with: ./scripts/make_gif.sh {} animations/{}.gif",
        output_dir,
        animation.settings().name
    );
}
