extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::{canvas, scene_file};

const SCALE: u32 = 1;

// Compare the render against its fixture. While iterating on a scene there
// is no fixture yet: the render is written to the repo root instead. Inspect
// it and copy it to `tests/fixtures/<name>.png` once the scene looks right.
fn assert_matches_fixture(canvas: &canvas::Canvas, name: &str) {
    let fixture_path = format!("tests/fixtures/{}.png", name);
    let output_path = format!("{}.png", name);

    if !std::path::Path::new(&fixture_path).exists() {
        shared_test_helpers::write_image_to_file(canvas, &output_path).unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `{}`.",
            output_path
        );
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file(name).unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(canvas, &output_path).unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `{}`.",
            output_path
        );
    }
}

// The scene file recreates the `multiple_lights` render test scene, so it
// must reproduce that test's fixture exactly.
#[test]
fn test_rendering_a_scene_file() -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string("scenes/multiple_lights.toml")?;
    let scene = scene_file::SceneFile::parse(&source).unwrap();

    let world = scene.build_world().unwrap();
    let camera = scene.build_camera(SCALE);
    let canvas = camera.render(&world);

    assert_matches_fixture(&canvas, "multiple_lights");
    return Ok(());
}

// Every frame of the animation is compared against its own fixture: the
// base scene unchanged, an object change, and an object plus camera change.
#[test]
fn test_rendering_animation_frames() -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string("tests/scenes/sphere_moves.toml")?;
    let animation = scene_file::AnimationFile::parse(&source).unwrap();

    assert_eq!(animation.frame_count(), 3);
    for frame_index in 0..animation.frame_count() {
        let world = animation.build_world(frame_index).unwrap();
        let camera = animation.build_camera(frame_index, SCALE);
        let canvas = camera.render(&world);

        assert_matches_fixture(&canvas, &format!("sphere_moves_{}", frame_index));
    }
    return Ok(());
}
