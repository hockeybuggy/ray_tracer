//! Renders the frames of an animation described by a TOML file.
//!
//! The file describes a base scene (camera, lights, objects) followed by a
//! list of frames, each restating how that frame differs from the base
//! scene. Frames render in parallel to numbered PNGs in an output
//! directory; assembling them into a gif is left to
//! `scripts/make_gif.sh`. See `animations/*.toml` for examples.
//!
//! Usage:
//!
//!     cargo run --release --bin animate -- <scene.toml> [--scale N] [--output-dir DIR]
//!
//! `--scale` multiplies the scene's base resolution, so the same file can
//! render small while iterating and large for a shareable asset.

use std::collections::{BTreeMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::Deserialize;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, color, lights, material, matrix, obj_file, shape, transformation, tuple, world,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneDescription {
    animation: AnimationDescription,
    camera: CameraDescription,
    lights: Vec<LightDescription>,
    objects: Vec<ObjectDescription>,
    frames: Vec<FrameDescription>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnimationDescription {
    name: String,
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CameraDescription {
    /// Field of view in degrees.
    field_of_view: f64,
    from: [f64; 3],
    to: [f64; 3],
    up: [f64; 3],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LightDescription {
    position: [f64; 3],
    intensity: [f64; 3],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObjectDescription {
    name: String,
    kind: ObjectKind,
    /// Path to a Wavefront OBJ model, required when `kind = "obj"`.
    file: Option<String>,
    #[serde(default)]
    transform: Vec<TransformOp>,
    material: Option<MaterialDescription>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ObjectKind {
    Plane,
    Sphere,
    Cube,
    Obj,
}

/// One step of a transform chain, e.g. `{ rotate_y = 90.0 }`. Steps apply
/// in list order, each in world space after the ones before it (matching
/// the fluent `Transform` trait). Angles are in degrees.
#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum TransformOp {
    RotateX(f64),
    RotateY(f64),
    RotateZ(f64),
    Scale([f64; 3]),
    Translate([f64; 3]),
}

/// Overrides applied on top of the default material.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MaterialDescription {
    color: Option<[f64; 3]>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
    reflective: Option<f64>,
    transparency: Option<f64>,
    refractive_index: Option<f64>,
}

/// How a single frame differs from the base scene. An empty `[[frames]]`
/// entry renders the base scene unchanged.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct FrameDescription {
    camera: Option<CameraChange>,
    /// Changes to base objects, keyed by object name.
    #[serde(default)]
    objects: BTreeMap<String, ObjectChange>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CameraChange {
    from: Option<[f64; 3]>,
    to: Option<[f64; 3]>,
    up: Option<[f64; 3]>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObjectChange {
    /// Extra transform steps applied after the object's base transform.
    #[serde(default)]
    transform: Vec<TransformOp>,
}

fn point(coordinates: [f64; 3]) -> tuple::Point {
    return tuple::Point::new(coordinates[0], coordinates[1], coordinates[2]);
}

fn vector(components: [f64; 3]) -> tuple::Vector {
    return tuple::Vector::new(components[0], components[1], components[2]);
}

fn to_color(components: [f64; 3]) -> color::Color {
    return color::color(components[0], components[1], components[2]);
}

fn transform_matrix(steps: &[TransformOp]) -> matrix::Matrix4 {
    return steps
        .iter()
        .fold(matrix::Matrix4::IDENTITY, |matrix, step| match *step {
            TransformOp::RotateX(degrees) => matrix.rotation_x(degrees.to_radians()),
            TransformOp::RotateY(degrees) => matrix.rotation_y(degrees.to_radians()),
            TransformOp::RotateZ(degrees) => matrix.rotation_z(degrees.to_radians()),
            TransformOp::Scale([x, y, z]) => matrix.scaling(x, y, z),
            TransformOp::Translate([x, y, z]) => matrix.translation(x, y, z),
        });
}

fn build_material(description: &MaterialDescription) -> material::Material {
    let mut material = material::material();
    if let Some(components) = description.color {
        material.color = to_color(components);
    }
    if let Some(value) = description.ambient {
        material.ambient = value;
    }
    if let Some(value) = description.diffuse {
        material.diffuse = value;
    }
    if let Some(value) = description.specular {
        material.specular = value;
    }
    if let Some(value) = description.shininess {
        material.shininess = value;
    }
    if let Some(value) = description.reflective {
        material.reflective = value;
    }
    if let Some(value) = description.transparency {
        material.transparency = value;
    }
    if let Some(value) = description.refractive_index {
        material.refractive_index = value;
    }
    return material;
}

fn build_shape(description: &ObjectDescription, change: Option<&ObjectChange>) -> shape::Shape {
    let mut shape = match description.kind {
        ObjectKind::Plane => shape::Shape::default_plane(),
        ObjectKind::Sphere => shape::Shape::default_sphere(),
        ObjectKind::Cube => shape::Shape::default_cube(),
        ObjectKind::Obj => {
            let path = description.file.as_ref().unwrap_or_else(|| {
                fail(&format!(
                    "object `{}` has kind \"obj\" but no `file`",
                    description.name
                ))
            });
            let source = std::fs::read_to_string(path)
                .unwrap_or_else(|error| fail(&format!("could not read `{}`: {}", path, error)));
            obj_file::parse_obj(&source).into_group()
        }
    };

    let mut steps = description.transform.clone();
    if let Some(change) = change {
        steps.extend_from_slice(&change.transform);
    }
    shape.set_transformation_matrix(transform_matrix(&steps));

    if let Some(material_description) = &description.material {
        shape.material = build_material(material_description);
    }

    return shape;
}

fn build_world(description: &SceneDescription, frame: &FrameDescription) -> world::World {
    let mut builder = world::WorldBuilder::new();
    for object in &description.objects {
        builder.add_shape(build_shape(object, frame.objects.get(&object.name)));
    }
    for light in &description.lights {
        builder.add_light_source(lights::point_light(
            point(light.position),
            to_color(light.intensity),
        ));
    }
    return builder.world;
}

fn build_camera(
    description: &SceneDescription,
    frame: &FrameDescription,
    scale: u32,
) -> camera::Camera {
    let base = &description.camera;
    let change = frame.camera.as_ref();
    let from = change.and_then(|camera| camera.from).unwrap_or(base.from);
    let to = change.and_then(|camera| camera.to).unwrap_or(base.to);
    let up = change.and_then(|camera| camera.up).unwrap_or(base.up);

    let mut camera = camera::Camera::new(
        description.animation.width * scale,
        description.animation.height * scale,
        base.field_of_view.to_radians(),
    );
    camera.transform = transformation::view_transform(&point(from), &point(to), &vector(up));
    return camera;
}

fn validate(description: &SceneDescription) -> Result<(), String> {
    let mut names = HashSet::new();
    for object in &description.objects {
        if !names.insert(&object.name) {
            return Err(format!("duplicate object name `{}`", object.name));
        }
    }
    if description.frames.is_empty() {
        return Err("the scene has no frames".to_string());
    }
    for (index, frame) in description.frames.iter().enumerate() {
        for name in frame.objects.keys() {
            if !names.contains(name) {
                return Err(format!("frame {} changes unknown object `{}`", index, name));
            }
        }
    }
    return Ok(());
}

struct Arguments {
    scene_path: String,
    scale: u32,
    output_dir: Option<String>,
}

fn usage() -> ! {
    eprintln!("usage: animate <scene.toml> [--scale N] [--output-dir DIR]");
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
    let description: SceneDescription = toml::from_str(&source).unwrap_or_else(|error| {
        fail(&format!(
            "could not parse `{}`:\n{}",
            arguments.scene_path, error
        ))
    });
    validate(&description).unwrap_or_else(|message| fail(&message));

    let output_dir = arguments
        .output_dir
        .unwrap_or_else(|| format!("animations/frames/{}", description.animation.name));
    std::fs::create_dir_all(&output_dir)
        .unwrap_or_else(|error| fail(&format!("could not create `{}`: {}", output_dir, error)));

    let total = description.frames.len();
    let threads = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(4)
        .min(total);
    println!(
        "Rendering {} frames at {}x{} on {} threads",
        total,
        description.animation.width * arguments.scale,
        description.animation.height * arguments.scale,
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
                    let frame = &description.frames[index];
                    let world = build_world(&description, frame);
                    let camera = build_camera(&description, frame, arguments.scale);
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
        output_dir, description.animation.name
    );
}

#[cfg(test)]
mod animate_tests {
    use super::*;

    const MINIMAL_SCENE: &str = r#"
        [animation]
        name = "test"
        width = 10
        height = 5

        [camera]
        field_of_view = 60.0
        from = [0.0, 2.0, -4.0]
        to = [0.0, 0.7, 0.0]
        up = [0.0, 1.0, 0.0]

        [[lights]]
        position = [-6.0, 8.0, -8.0]
        intensity = [1.0, 1.0, 1.0]

        [[objects]]
        name = "floor"
        kind = "plane"
        material = { color = [0.55, 0.6, 0.65], specular = 0.0 }

        [[objects]]
        name = "ball"
        kind = "sphere"
        transform = [{ scale = [0.5, 0.5, 0.5] }, { translate = [0.0, 0.5, 0.0] }]

        [[frames]]

        [[frames]]
        objects.ball.transform = [{ rotate_y = 90.0 }]

        [[frames]]
        camera.from = [4.0, 2.0, 0.0]
    "#;

    #[test]
    fn test_parsing_a_scene_description() {
        let description: SceneDescription = toml::from_str(MINIMAL_SCENE).unwrap();

        assert_eq!(description.animation.name, "test");
        assert_eq!(description.objects.len(), 2);
        assert_eq!(description.frames.len(), 3);
        assert!(description.frames[0].objects.is_empty());
        assert!(description.frames[1].objects.contains_key("ball"));
        assert_eq!(
            description.frames[2].camera.as_ref().unwrap().from,
            Some([4.0, 2.0, 0.0])
        );
        assert!(validate(&description).is_ok());
    }

    #[test]
    fn test_transform_steps_apply_in_order() {
        let steps = [
            TransformOp::RotateX(-90.0),
            TransformOp::Scale([0.1, 0.2, 0.3]),
            TransformOp::Translate([1.0, 2.0, 3.0]),
        ];

        let expected = matrix::Matrix4::IDENTITY
            .rotation_x(-std::f64::consts::PI / 2.0)
            .scaling(0.1, 0.2, 0.3)
            .translation(1.0, 2.0, 3.0);
        assert_eq!(transform_matrix(&steps), expected);
    }

    #[test]
    fn test_frame_transform_steps_apply_after_the_base_transform() {
        let description: SceneDescription = toml::from_str(MINIMAL_SCENE).unwrap();
        let frame = &description.frames[1];

        let ball = build_shape(&description.objects[1], frame.objects.get("ball"));

        let expected = matrix::Matrix4::IDENTITY
            .scaling(0.5, 0.5, 0.5)
            .translation(0.0, 0.5, 0.0)
            .rotation_y(std::f64::consts::PI / 2.0);
        assert_eq!(*ball.transformation_matrix(), expected);
    }

    #[test]
    fn test_a_frame_camera_change_overrides_only_the_given_fields() {
        let description: SceneDescription = toml::from_str(MINIMAL_SCENE).unwrap();

        let base_camera = build_camera(&description, &description.frames[0], 1);
        let moved_camera = build_camera(&description, &description.frames[2], 1);

        let expected_base = transformation::view_transform(
            &tuple::Point::new(0.0, 2.0, -4.0),
            &tuple::Point::new(0.0, 0.7, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        let expected_moved = transformation::view_transform(
            &tuple::Point::new(4.0, 2.0, 0.0),
            &tuple::Point::new(0.0, 0.7, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        assert_eq!(base_camera.transform, expected_base);
        assert_eq!(moved_camera.transform, expected_moved);
    }

    #[test]
    fn test_validate_rejects_a_frame_changing_an_unknown_object() {
        let mut description: SceneDescription = toml::from_str(MINIMAL_SCENE).unwrap();
        description.frames[1]
            .objects
            .insert("teacup".to_string(), ObjectChange { transform: vec![] });

        let error = validate(&description).unwrap_err();
        assert!(error.contains("unknown object `teacup`"), "{}", error);
    }
}
