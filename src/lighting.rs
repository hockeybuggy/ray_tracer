use crate::color;
use crate::lights;
use crate::material;
use crate::matrix;
use crate::tuple;

pub fn lighting(
    material: &material::Material,
    object_to_world: &matrix::Matrix4,
    light: &lights::Light,
    point: &tuple::Point,
    camerav: &tuple::Vector,
    normalv: &tuple::Vector,
    intensity: f64,
) -> color::Color {
    let color = if material.pattern.is_some() {
        material
            .pattern
            .as_ref()
            .unwrap()
            .pattern_at_object(object_to_world, &point)
    } else {
        material.color
    };

    // combine the surface color with the light's color/intensity
    let effective_color = color * light.intensity;

    // compute the ambient contribution once, outside the sample loop; it is
    // never scaled by intensity or averaged over samples
    let ambient = effective_color * material.ambient;

    let (usteps, vsteps) = match &light.kind {
        lights::LightKind::Point => (1, 1),
        lights::LightKind::Area { usteps, vsteps, .. } => (*usteps, *vsteps),
    };

    let mut sum = color::black();
    for v in 0..vsteps {
        for u in 0..usteps {
            let light_position = lights::point_on_light(light, u, v);

            // find the direction to this light sample
            let lightv = tuple::normalize(&(light_position - *point));

            // light_dot_normal represents the cosine of the angle between the
            // light vector and the normal vector. A negative number means the
            // light is on the other side of the surface.
            let light_dot_normal = tuple::dot(&lightv, normalv);
            if light_dot_normal < 0.0 {
                continue;
            }

            // accumulate the diffuse contribution
            sum = sum + effective_color * material.diffuse * light_dot_normal;

            // reflect_dot_camera represents the cosine of the angle between
            // the light reflects away from the camera.
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_camera = tuple::dot(&reflectv, camerav);
            if reflect_dot_camera <= 0.0 {
                continue;
            }

            // accumulate the specular contribution
            let factor = reflect_dot_camera.powf(material.shininess);
            sum = sum + light.intensity * material.specular * factor;
        }
    }

    let samples = (usteps * vsteps) as f64;
    return ambient + sum * (intensity / samples);
}
