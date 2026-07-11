//! TOML descriptions of scenes and animations.
//!
//! A scene file describes a still image: a `[scene]` table (name and
//! resolution), a `[camera]`, one or more `[[lights]]`, and one or more
//! `[[objects]]`. An animation file is the same but with an `[animation]`
//! table in place of `[scene]`, followed by a list of `[[frames]]`, each
//! restating how that frame differs from the base scene: extra transform
//! steps appended after an object's base transform, or replacement camera
//! coordinates. An empty `[[frames]]` entry renders the base scene
//! unchanged.
//!
//! Angles (rotations and the camera field of view) are in degrees.
//! Transform steps apply in list order, each in world space after the ones
//! before it, matching the fluent `Transform` trait. See `scenes/*.toml`
//! and `animations/*.toml` for examples.

use std::collections::{BTreeMap, HashSet};

use serde::Deserialize;

use crate::camera;
use crate::color;
use crate::lights;
use crate::material;
use crate::matrix;
use crate::obj_file;
use crate::sequences;
use crate::shape;
use crate::transformation;
use crate::transformation::Transform;
use crate::tuple;
use crate::world;

/// Seed for a scene file's area-light jitter, so renders stay reproducible
/// across runs instead of drawing on OS randomness.
const AREA_LIGHT_JITTER_SEED: u64 = 0x5EED;

/// A still image: a base scene with no frames.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneFile {
    scene: RenderSettings,
    camera: CameraDescription,
    lights: Vec<LightDescription>,
    objects: Vec<ObjectDescription>,
}

/// A base scene plus the list of frames that vary it.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationFile {
    animation: RenderSettings,
    camera: CameraDescription,
    lights: Vec<LightDescription>,
    objects: Vec<ObjectDescription>,
    frames: Vec<FrameDescription>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderSettings {
    pub name: String,
    pub width: u32,
    pub height: u32,
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

/// A `[[lights]]` block is a point light if it has `position`, or an area
/// light if it has `corner`/`uvec`/`usteps`/`vvec`/`vsteps` instead.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LightDescription {
    intensity: [f64; 3],
    position: Option<[f64; 3]>,
    corner: Option<[f64; 3]>,
    uvec: Option<[f64; 3]>,
    usteps: Option<usize>,
    vvec: Option<[f64; 3]>,
    vsteps: Option<usize>,
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

/// One step of a transform chain, e.g. `{ rotate_y = 90.0 }`.
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

/// How a single frame differs from the base scene.
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

impl SceneFile {
    pub fn parse(source: &str) -> Result<SceneFile, String> {
        return toml::from_str(source).map_err(|error| error.to_string());
    }

    pub fn settings(&self) -> &RenderSettings {
        return &self.scene;
    }

    pub fn build_world(&self) -> Result<world::World, String> {
        return build_world(&self.objects, &self.lights, &BTreeMap::new());
    }

    pub fn build_camera(&self, scale: u32) -> camera::Camera {
        return build_camera(&self.camera, None, &self.scene, scale);
    }
}

impl AnimationFile {
    pub fn parse(source: &str) -> Result<AnimationFile, String> {
        let animation: AnimationFile = toml::from_str(source).map_err(|error| error.to_string())?;
        animation.validate()?;
        return Ok(animation);
    }

    pub fn settings(&self) -> &RenderSettings {
        return &self.animation;
    }

    pub fn frame_count(&self) -> usize {
        return self.frames.len();
    }

    pub fn build_world(&self, frame_index: usize) -> Result<world::World, String> {
        let frame = &self.frames[frame_index];
        return build_world(&self.objects, &self.lights, &frame.objects);
    }

    pub fn build_camera(&self, frame_index: usize, scale: u32) -> camera::Camera {
        let frame = &self.frames[frame_index];
        return build_camera(&self.camera, frame.camera.as_ref(), &self.animation, scale);
    }

    fn validate(&self) -> Result<(), String> {
        let mut names = HashSet::new();
        for object in &self.objects {
            if !names.insert(&object.name) {
                return Err(format!("duplicate object name `{}`", object.name));
            }
        }
        if self.frames.is_empty() {
            return Err("the animation has no frames".to_string());
        }
        for (index, frame) in self.frames.iter().enumerate() {
            for name in frame.objects.keys() {
                if !names.contains(name) {
                    return Err(format!("frame {} changes unknown object `{}`", index, name));
                }
            }
        }
        return Ok(());
    }
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

fn build_shape(
    description: &ObjectDescription,
    change: Option<&ObjectChange>,
) -> Result<shape::Shape, String> {
    let mut shape = match description.kind {
        ObjectKind::Plane => shape::Shape::default_plane(),
        ObjectKind::Sphere => shape::Shape::default_sphere(),
        ObjectKind::Cube => shape::Shape::default_cube(),
        ObjectKind::Obj => {
            let path = description.file.as_ref().ok_or_else(|| {
                format!(
                    "object `{}` has kind \"obj\" but no `file`",
                    description.name
                )
            })?;
            let source = std::fs::read_to_string(path)
                .map_err(|error| format!("could not read `{}`: {}", path, error))?;
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

    return Ok(shape);
}

fn build_light(description: &LightDescription) -> Result<lights::Light, String> {
    let intensity = to_color(description.intensity);

    if let Some(position) = description.position {
        return Ok(lights::point_light(point(position), intensity));
    }

    let missing = |field: &str| format!("light needs `position`, or `{}` for an area light", field);
    let corner = description.corner.ok_or_else(|| missing("corner"))?;
    let uvec = description.uvec.ok_or_else(|| missing("uvec"))?;
    let usteps = description.usteps.ok_or_else(|| missing("usteps"))?;
    let vvec = description.vvec.ok_or_else(|| missing("vvec"))?;
    let vsteps = description.vsteps.ok_or_else(|| missing("vsteps"))?;

    let mut light = lights::area_light(
        point(corner),
        vector(uvec),
        usteps,
        vector(vvec),
        vsteps,
        intensity,
    );
    light.set_jitter(sequences::Sequence::random(256, AREA_LIGHT_JITTER_SEED));
    return Ok(light);
}

fn build_world(
    objects: &[ObjectDescription],
    lights: &[LightDescription],
    changes: &BTreeMap<String, ObjectChange>,
) -> Result<world::World, String> {
    let mut builder = world::WorldBuilder::new();
    for object in objects {
        builder.add_shape(build_shape(object, changes.get(&object.name))?);
    }
    for light in lights {
        builder.add_light_source(build_light(light)?);
    }
    return Ok(builder.world);
}

fn build_camera(
    base: &CameraDescription,
    change: Option<&CameraChange>,
    settings: &RenderSettings,
    scale: u32,
) -> camera::Camera {
    let from = change.and_then(|camera| camera.from).unwrap_or(base.from);
    let to = change.and_then(|camera| camera.to).unwrap_or(base.to);
    let up = change.and_then(|camera| camera.up).unwrap_or(base.up);

    let mut camera = camera::Camera::new(
        settings.width * scale,
        settings.height * scale,
        base.field_of_view.to_radians(),
    );
    camera.transform = transformation::view_transform(&point(from), &point(to), &vector(up));
    return camera;
}

#[cfg(test)]
mod scene_file_tests {
    use super::*;

    const MINIMAL_ANIMATION: &str = r#"
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
    fn test_parsing_an_animation_file() {
        let animation = AnimationFile::parse(MINIMAL_ANIMATION).unwrap();

        assert_eq!(animation.settings().name, "test");
        assert_eq!(animation.objects.len(), 2);
        assert_eq!(animation.frame_count(), 3);
        assert!(animation.frames[0].objects.is_empty());
        assert!(animation.frames[1].objects.contains_key("ball"));
        assert_eq!(
            animation.frames[2].camera.as_ref().unwrap().from,
            Some([4.0, 2.0, 0.0])
        );
    }

    #[test]
    fn test_parsing_a_scene_file() {
        // The same document with a `[scene]` table and no frames.
        let source = MINIMAL_ANIMATION
            .replace("[animation]", "[scene]")
            .split("[[frames]]")
            .next()
            .unwrap()
            .to_string();

        let scene = SceneFile::parse(&source).unwrap();

        assert_eq!(scene.settings().name, "test");
        assert_eq!(scene.objects.len(), 2);
    }

    #[test]
    fn test_a_scene_file_rejects_frames() {
        let source = MINIMAL_ANIMATION.replace("[animation]", "[scene]");

        let error = SceneFile::parse(&source).err().unwrap();
        assert!(error.contains("frames"), "{}", error);
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
        let animation = AnimationFile::parse(MINIMAL_ANIMATION).unwrap();
        let frame = &animation.frames[1];

        let ball = build_shape(&animation.objects[1], frame.objects.get("ball")).unwrap();

        let expected = matrix::Matrix4::IDENTITY
            .scaling(0.5, 0.5, 0.5)
            .translation(0.0, 0.5, 0.0)
            .rotation_y(std::f64::consts::PI / 2.0);
        assert_eq!(*ball.transformation_matrix(), expected);
    }

    #[test]
    fn test_a_frame_camera_change_overrides_only_the_given_fields() {
        let animation = AnimationFile::parse(MINIMAL_ANIMATION).unwrap();

        let base_camera = animation.build_camera(0, 1);
        let moved_camera = animation.build_camera(2, 1);

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
    fn test_parse_rejects_a_frame_changing_an_unknown_object() {
        let source = format!(
            "{}\n[[frames]]\nobjects.teacup.transform = [{{ rotate_y = 10.0 }}]\n",
            MINIMAL_ANIMATION
        );

        let error = AnimationFile::parse(&source).err().unwrap();
        assert!(error.contains("unknown object `teacup`"), "{}", error);
    }

    #[test]
    fn test_a_light_block_with_area_light_fields_builds_an_area_light() {
        let source = MINIMAL_ANIMATION
            .replace("[animation]", "[scene]")
            .replace(
                "position = [-6.0, 8.0, -8.0]",
                "corner = [-1.0, 2.0, 4.0]\n        uvec = [2.0, 0.0, 0.0]\n        usteps = 4\n        vvec = [0.0, 2.0, 0.0]\n        vsteps = 2",
            )
            .split("[[frames]]")
            .next()
            .unwrap()
            .to_string();

        let scene = SceneFile::parse(&source).unwrap();
        let world = scene.build_world().unwrap();

        assert_eq!(world.lights.len(), 1);
        match world.lights[0].kind {
            lights::LightKind::Area { usteps, vsteps, .. } => {
                assert_eq!(usteps, 4);
                assert_eq!(vsteps, 2);
            }
            lights::LightKind::Point => panic!("expected an area light"),
        }
    }

    #[test]
    fn test_a_light_block_missing_position_and_area_fields_is_an_error() {
        let source = MINIMAL_ANIMATION
            .replace("[animation]", "[scene]")
            .replace("position = [-6.0, 8.0, -8.0]", "")
            .split("[[frames]]")
            .next()
            .unwrap()
            .to_string();

        let scene = SceneFile::parse(&source).unwrap();
        let error = scene.build_world().err().unwrap();
        assert!(error.contains("corner"), "{}", error);
    }
}
