use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod test_helpers {
    const SCALE: u32 = 1;

    use ray_tracer::transformation::Transform;
    use ray_tracer::{
        camera, color, lights, material, matrix, shape, transformation, tuple, world,
    };

    pub fn create_camera() -> camera::Camera {
        let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
        camera.transform = transformation::view_transform(
            &tuple::Point::new(0.0, 1.5, -5.0),
            &tuple::Point::new(0.0, 1.0, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        return camera;
    }

    pub fn create_simple_world_with_planes() -> world::World {
        let mut builder = world::WorldBuilder::new();

        // Create a floor and add it to the scene
        builder.add_shape({
            let mut floor = shape::Shape::default_plane();
            floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            floor.material = material;
            floor
        });

        // Add a sphere to the center
        builder.add_shape({
            let mut middle = shape::Shape::default_sphere();
            middle.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-0.5, 1.0, 0.5));
            let mut material = material::material();
            material.color = color::color(0.1, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            middle.material = material;
            middle
        });

        // Add a small green sphere on the right
        builder.add_shape({
            let mut right = shape::Shape::default_sphere();
            right.set_transformation_matrix(
                matrix::Matrix4::IDENTITY
                    .scaling(0.5, 0.5, 0.5)
                    .translation(1.5, 0.5, 0.5),
            );
            let mut material = material::material();
            material.color = color::color(0.1, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            right.material = material;
            right
        });

        // Add a smaller green sphere on the left
        builder.add_shape({
            let mut left = shape::Shape::default_sphere();
            left.set_transformation_matrix(
                matrix::Matrix4::IDENTITY
                    .scaling(0.3333, 0.3333, 0.3333)
                    .translation(-1.5, 0.33, -0.75),
            );
            let mut material = material::material();
            material.color = color::color(1.0, 0.8, 0.1);
            material.diffuse = 0.7;
            material.specular = 0.3;
            left.material = material;
            left
        });

        // Let there be light
        builder.add_light_source(lights::point_light(
            tuple::Point::new(-10.0, 10.0, -10.0),
            color::white(),
        ));

        return builder.world;
    }
}

fn simple_world_benchmark(c: &mut Criterion) {
    let world = test_helpers::create_simple_world_with_planes();
    let camera = test_helpers::create_camera();

    c.bench_function("render simple world", |b| {
        b.iter(|| camera.render(black_box(&world)))
    });
}

criterion_group!(benches, simple_world_benchmark);
criterion_main!(benches);
