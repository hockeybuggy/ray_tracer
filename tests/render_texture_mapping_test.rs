extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::{camera, color, lights, material, shape, transformation, tuple, uv, world};

const SCALE: u32 = 1;

#[test]
fn test_uv_checkered_sphere() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_material({
                let checkers = uv::UvPattern::checkers(
                    20,
                    10,
                    color::color(0.0, 0.5, 0.0),
                    color::color(1.0, 1.0, 1.0),
                );
                let pattern =
                    ray_tracer::patterns::Pattern::texture_map(checkers, uv::UvMap::Spherical);
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.ambient = 0.1;
                material.specular = 0.4;
                material.shininess = 10.0;
                material.diffuse = 0.6;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.5);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 0.0, -5.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("uv_checkered_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "uv_checkered_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `uv_checkered_sphere.png`."
        );
    }
    return Ok(());
}
