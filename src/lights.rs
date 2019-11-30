use crate::color;
use crate::tuple;

pub struct Light {
    pub position: tuple::Point,
    pub intensity: color::Color,
}

pub fn point_light(position: tuple::Point, intensity: color::Color) -> Light {
    Light {
        position,
        intensity,
    }
}

#[cfg(test)]
mod lights_tests {
    use crate::color;
    use crate::lights;
    use crate::tuple;

    #[test]
    fn test_point_light_has_position_an_instensity() {
        let intensity = color::color(1.0, 1.0, 1.0);
        let position = tuple::Point::new(0.0, 0.0, 0.0);

        let light = lights::point_light(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
