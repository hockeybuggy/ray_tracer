use crate::color;
use crate::tuple;

pub struct Light {
    pub position: tuple::Tuple,
    pub intensity: color::Color,
}

pub fn point_light(position: tuple::Tuple, intensity: color::Color) -> Light {
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
        let position = tuple::point(0.0, 0.0, 0.0);

        let light = lights::point_light(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
