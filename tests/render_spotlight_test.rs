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

// A spotlight over three spheres: only the middle one sits fully inside
// the cone, its neighbours catch the fade band, and the beam leaves a
// pool of light on the floor.
#[test]
fn test_spotlight() -> Result<(), std::io::Error> {
    let source = std::fs::read_to_string("scenes/spotlight.toml")?;
    let scene = scene_file::SceneFile::parse(&source).unwrap();

    let world = scene.build_world().unwrap();
    let camera = scene.build_camera(SCALE);
    let canvas = camera.render(&world);

    assert_matches_fixture(&canvas, "spotlight");
    return Ok(());
}
