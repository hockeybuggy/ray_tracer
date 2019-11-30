use crate::color;
use crate::lights;
use crate::material;
use crate::tuple;

pub fn lighting(
    material: &material::Material,
    light: &lights::Light,
    point: &tuple::Point,
    camerav: &tuple::Vector,
    normalv: &tuple::Vector,
) -> color::Color {
    let ambient: color::Color;
    let diffuse: color::Color;
    let specular: color::Color;
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;

    // find the direction to the light source
    let lightv = tuple::normalize(&(light.position - *point));

    // compute the ambient contribution
    ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the light vector
    // and the normal vector. A negative number means the light is on the other side
    // of the surface.
    let light_dot_normal = tuple::dot(&lightv, normalv);
    if light_dot_normal < 0.0 {
        diffuse = color::black();
        specular = color::black();
    } else {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_camera represents the cosine of the angle between the
        // light reflects away from the camera.
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_camera = tuple::dot(&reflectv, camerav);

        if reflect_dot_camera <= 0.0 {
            specular = color::black();
        } else {
            // compute the specular contribution
            let factor = reflect_dot_camera.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    return ambient + diffuse + specular;
}
