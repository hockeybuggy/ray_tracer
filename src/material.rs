use crate::color;

pub struct Material {
    color: color::Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
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
    use crate::color;
    use crate::material;

    #[test]
    fn test_default_material_constructor() {
        let material = material::material();

        assert_eq!(material.color, color::color(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }
}
