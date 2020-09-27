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
        let mut world = world::world();

        // Create a floor and add it to the scene
        {
            let mut floor = shape::Shape::default_sphere();
            floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            floor.material = material;
            world.shapes.push(floor);
        }

        // Create a wall and add it to the scene
        {
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
            world.shapes.push(left_wall);
        }

        // Create another wall and add it to the scene
        {
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
            world.shapes.push(right_wall);
        }

        // Add a sphere to the center
        {
            let mut middle = shape::Shape::default_sphere();
            middle.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-0.5, 1.0, 0.5));
            let mut material = material::material();
            material.color = color::color(0.1, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            middle.material = material;
            world.shapes.push(middle);
        }

        // Add a small green sphere on the right
        {
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
            world.shapes.push(right);
        }

        // Add a smaller green sphere on the left
        {
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
            world.shapes.push(left);
        }

        // Let there be light
        let white_point_light =
            lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
        world.light = Some(white_point_light);

        return world;
    }

    pub fn create_simple_world_with_planes() -> world::World {
        let mut world = world::world();

        // Create a floor and add it to the scene
        {
            let mut floor = shape::Shape::default_plane();
            floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
            let mut material = material::material();
            material.color = color::color(1.0, 0.9, 0.9);
            material.specular = 0.0;
            floor.material = material;
            world.shapes.push(floor);
        }

        // Add a sphere to the center
        {
            let mut middle = shape::Shape::default_sphere();
            middle.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-0.5, 1.0, 0.5));
            let mut material = material::material();
            material.color = color::color(0.1, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            middle.material = material;
            world.shapes.push(middle);
        }

        // Add a small green sphere on the right
        {
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
            world.shapes.push(right);
        }

        // Add a smaller green sphere on the left
        {
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
            world.shapes.push(left);
        }

        // Let there be light
        let white_point_light =
            lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
        world.light = Some(white_point_light);

        return world;
    }

    pub fn create_checker_floor_world(reflective_floor: bool) -> world::World {
        let mut world = world::world();

        // Create a floor and add it to the scene
        {
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
            world.shapes.push(floor);
        }

        // Add a sphere to the center
        {
            let mut middle = shape::Shape::default_sphere();
            middle.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-0.5, 1.0, 0.5));
            let mut material = material::material();
            material.color = color::color(0.1, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            middle.material = material;
            world.shapes.push(middle);
        }

        // Let there be light
        let white_point_light =
            lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
        world.light = Some(white_point_light);

        return world;
    }
}

#[test]
fn test_simple_world() -> Result<(), std::io::Error> {
    let world = test_helpers::create_simple_world_with_only_spheres();
    let camera = test_helpers::create_rectangular_camera();

    let canvas = camera.render(&world);

    let output_path = "output_simple_world.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(&canvas, "output_simple_world.png").unwrap();

    let expected_str = include_str!("fixtures/simple_world_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}

#[test]
fn test_world_with_planes() -> Result<(), std::io::Error> {
    let world = test_helpers::create_simple_world_with_planes();
    let camera = test_helpers::create_rectangular_camera();

    let canvas = camera.render(&world);

    let output_path = "output_world_with_plane.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(&canvas, "world_with_plane.png").unwrap();

    let expected_str = include_str!("fixtures/world_with_plane.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}

#[test]
fn test_world_with_non_reflective_checkered_floor() -> Result<(), std::io::Error> {
    let world = test_helpers::create_checker_floor_world(false);
    let camera = test_helpers::create_square_camera();

    let canvas = camera.render(&world);

    let output_path = "output_world_with_non_reflective_checkered_floor.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(
        &canvas,
        "world_with_non_reflective_checkered_floor.png",
    )
    .unwrap();

    let expected_str = include_str!("fixtures/world_with_non_reflective_checkered_floor.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}

#[test]
fn test_world_with_reflective_checkered_floor() -> Result<(), std::io::Error> {
    let world = test_helpers::create_checker_floor_world(true);
    let camera = test_helpers::create_square_camera();

    let canvas = camera.render(&world);

    let output_path = "output_world_with_reflective_checkered_floor.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(&canvas, "world_with_reflective_checkered_floor.png")
        .unwrap();

    let expected_str = include_str!("fixtures/world_with_reflective_checkered_floor.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}
