use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

mod test_helpers {
    use ray_tracer::transformation::Transform;
    use ray_tracer::{
        camera, color, lights, material, matrix, obj_file, sequences, shape, transformation, tuple,
        world,
    };

    pub fn create_camera() -> camera::Camera {
        let mut camera = camera::Camera::new(100, 75, std::f64::consts::PI / 3.0);
        camera.transform = transformation::view_transform(
            &tuple::Point::new(0.0, 2.0, -4.0),
            &tuple::Point::new(0.0, 0.7, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        return camera;
    }

    // A teapot standing on a matte floor, the same scene as the OBJ model
    // render tests. The light is a parameter so the soft shadow bench can
    // swap the point light for an area light.
    pub fn create_teapot_world(model_path: &str, light: lights::Light) -> world::World {
        let mut builder = world::WorldBuilder::new();

        builder.add_shape(
            shape::ShapeBuilder::plane()
                .set_material({
                    let mut material = material::material();
                    material.color = color::color(0.55, 0.6, 0.65);
                    material.specular = 0.0;
                    material
                })
                .build(),
        );

        // The model is built z-up and roughly 32 units wide, so stand it up
        // on the y axis and scale it down to about three units across.
        let source = std::fs::read_to_string(model_path).unwrap();
        builder.add_shape(
            shape::ShapeBuilder::from(obj_file::parse_obj(&source).into_group())
                .set_transform(
                    matrix::Matrix4::IDENTITY
                        .rotation_x(-std::f64::consts::PI / 2.0)
                        .scaling(0.1, 0.1, 0.1),
                )
                .build(),
        );

        builder.add_light_source(light);

        return builder.world;
    }

    pub fn point_light() -> lights::Light {
        return lights::point_light(tuple::Point::new(-6.0, 8.0, -8.0), color::white());
    }

    // An 8x8 area light: every shaded point casts 64 shadow rays instead
    // of the point light's one.
    pub fn area_light() -> lights::Light {
        let mut light = lights::area_light(
            tuple::Point::new(-7.0, 5.0, -2.0),
            tuple::Vector::new(3.0, 0.0, 0.0),
            8,
            tuple::Vector::new(0.0, 0.0, 3.0),
            8,
            color::color(1.5, 1.5, 1.5),
        );
        light.set_jitter(sequences::Sequence::random(256, 0x5EED));
        return light;
    }
}

// The low-poly teapot (240 triangles): mostly a measure of BVH traversal
// and smooth triangle intersection on a small mesh.
fn low_poly_teapot_benchmark(c: &mut Criterion) {
    let world = test_helpers::create_teapot_world(
        "object_files/teapot-low.obj",
        test_helpers::point_light(),
    );
    let camera = test_helpers::create_camera();

    c.bench_function("render low poly teapot", |b| {
        b.iter(|| camera.render(black_box(&world)))
    });
}

// The high-poly teapot (6,320 triangles): the same image, but the BVH is
// several levels deeper.
fn high_poly_teapot_benchmark(c: &mut Criterion) {
    let world =
        test_helpers::create_teapot_world("object_files/teapot.obj", test_helpers::point_light());
    let camera = test_helpers::create_camera();

    c.bench_function("render high poly teapot", |b| {
        b.iter(|| camera.render(black_box(&world)))
    });
}

// The low-poly teapot under an 8x8 area light: measures the cost of soft
// shadows, which multiply the shadow rays per shaded point by 64.
fn soft_shadow_teapot_benchmark(c: &mut Criterion) {
    let world = test_helpers::create_teapot_world(
        "object_files/teapot-low.obj",
        test_helpers::area_light(),
    );
    let camera = test_helpers::create_camera();

    c.bench_function("render soft shadow teapot", |b| {
        b.iter(|| camera.render(black_box(&world)))
    });
}

criterion_group! {
    name = benches;
    // Renders are slow (hundreds of milliseconds each), so take fewer
    // samples than criterion's default of 100.
    config = Criterion::default().sample_size(10);
    targets = low_poly_teapot_benchmark, high_poly_teapot_benchmark, soft_shadow_teapot_benchmark
}
criterion_main!(benches);
