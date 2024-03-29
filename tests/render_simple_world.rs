extern crate ray_tracer;

mod shared_test_helpers;

mod test_helpers {
    const SCALE: u32 = 1;

    use ray_tracer::transformation::Transform;
    use ray_tracer::{
        camera, color, lights, material, matrix, patterns, shape, transformation, tuple, world,
    };

    pub fn create_rectangular_camera() -> camera::Camera {
        let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
        camera.transform = transformation::view_transform(
            &tuple::Point::new(0.0, 1.5, -5.0),
            &tuple::Point::new(0.0, 1.0, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        return camera;
    }

    pub fn create_square_camera() -> camera::Camera {
        let mut camera = camera::Camera::new(50 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
        camera.transform = transformation::view_transform(
            &tuple::Point::new(0.0, 1.5, -5.0),
            &tuple::Point::new(0.0, 1.0, 0.0),
            &tuple::Vector::new(0.0, 1.0, 0.0),
        );
        return camera;
    }

    pub fn create_simple_world_with_only_spheres() -> world::World {
        let mut builder = world::WorldBuilder::new();

        // Create a floor and add it to the scene
        builder.add_shape({
            let mut floor = shape::Shape::default_sphere();
            floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            floor.material = material;
            floor
        });

        // Create a wall and add it to the scene
        builder.add_shape({
            let mut left_wall = shape::Shape::default_sphere();
            left_wall.set_transformation_matrix(
                matrix::Matrix4::IDENTITY
                    .scaling(10.0, 0.01, 10.0)
                    .rotation_x(std::f64::consts::PI / 2.0)
                    .rotation_y(-std::f64::consts::PI / 4.0)
                    .translation(0.0, 0.0, 5.0),
            );
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            left_wall.material = material;
            left_wall
        });

        // Create another wall and add it to the scene
        builder.add_shape({
            let mut right_wall = shape::Shape::default_sphere();
            right_wall.set_transformation_matrix(
                matrix::Matrix4::IDENTITY
                    .scaling(10.0, 0.01, 10.0)
                    .rotation_x(std::f64::consts::PI / 2.0)
                    .rotation_y(std::f64::consts::PI / 4.0)
                    .translation(0.0, 0.0, 5.0),
            );
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            right_wall.material = material;
            right_wall
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

    pub fn create_checker_floor_world(reflective_floor: bool) -> world::World {
        let mut builder = world::WorldBuilder::new();

        // Create a floor and add it to the scene
        builder.add_shape({
            let mut floor = shape::Shape::default_plane();
            floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));

            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            if reflective_floor {
                material.reflective = 0.8;
            }
            material.specular = 0.0;
            let mut pattern = patterns::Pattern::checkers(color::black(), color::white());
            pattern.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.1, 0.1, 0.1));
            material.pattern = Some(pattern);
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

        // Let there be light
        builder.add_light_source(lights::point_light(
            tuple::Point::new(-10.0, 10.0, -10.0),
            color::white(),
        ));

        return builder.world;
    }
}

#[test]
fn test_simple_world() -> Result<(), std::io::Error> {
    let world = test_helpers::create_simple_world_with_only_spheres();
    let camera = test_helpers::create_rectangular_camera();

    let canvas = camera.render(&world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("simple_world").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "simple_world.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `simple_world.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_world_with_planes() -> Result<(), std::io::Error> {
    let world = test_helpers::create_simple_world_with_planes();
    let camera = test_helpers::create_rectangular_camera();

    let canvas = camera.render(&world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("world_with_plane").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "world_with_plane.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `world_with_plane.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_world_with_non_reflective_checkered_floor() -> Result<(), std::io::Error> {
    let world = test_helpers::create_checker_floor_world(false);
    let camera = test_helpers::create_square_camera();

    let canvas = camera.render(&world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file(
        "world_with_non_reflective_checkered_floor",
    )
    .unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(
            &canvas,
            "world_with_non_reflective_checkered_floor.png",
        )
        .unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `world_with_non_reflective_checkered_floor.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_world_with_reflective_checkered_floor() -> Result<(), std::io::Error> {
    let world = test_helpers::create_checker_floor_world(true);
    let camera = test_helpers::create_square_camera();

    let canvas = camera.render(&world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("world_with_reflective_checkered_floor")
            .unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(
            &canvas,
            "world_with_reflective_checkered_floor.png",
        )
        .unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `world_with_reflective_checkered_floor.png`."
        );
    }

    return Ok(());
}
