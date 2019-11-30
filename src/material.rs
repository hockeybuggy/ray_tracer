use crate::color;

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: color::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub fn material() -> Material {
    Material {
        color: color::color(1.0, 1.0, 1.0),
        ambient: 0.1_f64,
        diffuse: 0.9_f64,
        specular: 0.9_f64,
        shininess: 200.0_f64,
    }
}

#[cfg(test)]
mod material_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::lighting;
    use crate::lights;
    use crate::material;
    use crate::tuple;

    #[test]
    fn test_default_material_constructor() {
        let material = material::material();

        assert_eq!(material.color, color::color(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    ///               ║
    ///  L⇐     C   ←-║
    ///               ║
    #[test]
    fn test_lighting_with_the_camera_between_light_and_surface() {
        let material = material::material();
        let position = tuple::Point::new(0.0, 0.0, 0.0);
        let camera = tuple::vector(0.0, 0.0, -1.0);
        let normal = tuple::vector(0.0, 0.0, -1.0);
        let light = lights::point_light(tuple::Point::new(0.0, 0.0, -10.0), color::color(1.0, 1.0, 1.0));

        let result = lighting::lighting(&material, &light, &position, &camera, &normal);

        let expected = color::color(1.9, 1.9, 1.9);
        assert_color_approx_eq!(expected, result);
    }

    ///         ║
    ///      C  ║
    ///       ╲ ║
    ///        ╲║
    ///  L⇐   ←-║
    ///         ║
    #[test]
    fn test_lighting_with_the_camera_opposite_surface_eye_offset_45() {
        let material = material::material();
        let position = tuple::Point::new(0.0, 0.0, 0.0);
        let camera = tuple::vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = tuple::vector(0.0, 0.0, -1.0);
        let light = lights::point_light(tuple::Point::new(0.0, 0.0, -10.0), color::color(1.0, 1.0, 1.0));

        let result = lighting::lighting(&material, &light, &position, &camera, &normal);

        let expected = color::color(1.0, 1.0, 1.0);
        assert_color_approx_eq!(expected, result);
    }

    ///        ║
    ///     L  ║
    ///      ╲ ║
    ///       ╲║
    ///  C   ←-║
    ///        ║
    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45() {
        let material = material::material();
        let position = tuple::Point::new(0.0, 0.0, 0.0);
        let camera = tuple::vector(0.0, 0.0, -1.0);
        let normal = tuple::vector(0.0, 0.0, -1.0);
        let light =
            lights::point_light(tuple::Point::new(0.0, 10.0, -10.0), color::color(1.0, 1.0, 1.0));

        let result = lighting::lighting(&material, &light, &position, &camera, &normal);

        let expected = color::color(0.7364, 0.7364, 0.7364);
        assert_color_approx_eq!(expected, result);
    }

    ///     L  ║
    ///      ╲ ║
    ///       ╲║
    ///      ←-║
    ///       ╱║
    ///      ╱ ║
    ///     C  ║
    #[test]
    fn test_lighting_with_the_eye_in_the_path_of_the_reflection() {
        let material = material::material();
        let position = tuple::Point::new(0.0, 0.0, 0.0);
        let camera = tuple::vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = tuple::vector(0.0, 0.0, -1.0);
        let light =
            lights::point_light(tuple::Point::new(0.0, 10.0, -10.0), color::color(1.0, 1.0, 1.0));

        let result = lighting::lighting(&material, &light, &position, &camera, &normal);

        let expected = color::color(1.6364, 1.6364, 1.6364);
        assert_color_approx_eq!(expected, result);
    }

    ///        ║
    ///  C   ←-║   ⇒ L
    ///        ║
    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let material = material::material();
        let position = tuple::Point::new(0.0, 0.0, 0.0);
        let camera = tuple::vector(0.0, 0.0, -1.0);
        let normal = tuple::vector(0.0, 0.0, -1.0);
        let light = lights::point_light(tuple::Point::new(0.0, 0.0, 10.0), color::color(1.0, 1.0, 1.0));

        let result = lighting::lighting(&material, &light, &position, &camera, &normal);

        let expected = color::color(0.1, 0.1, 0.1);
        assert_color_approx_eq!(expected, result);
    }
}
