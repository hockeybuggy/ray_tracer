use crate::matrix;

pub struct Camera {
    hsize: i32,
    vsize: i32,
    field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: matrix::Matrix4,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / f64::from(hsize);

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: matrix::Matrix4::IDENTITY,
        }
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera;
    use crate::matrix;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_camera_constructor() {
        let hsize = 160;
        let vsize = 160;
        let field_of_view = std::f64::consts::PI / 2.0;

        let camera = camera::Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
        assert_eq!(camera.field_of_view, field_of_view);
        assert_eq!(camera.transform, matrix::Matrix4::IDENTITY);
    }

    #[test]
    fn test_pixel_size_for_a_horizontal_canvas() {
        let camera = camera::Camera::new(200, 125, std::f64::consts::PI / 2.0);
        assert_approx_eq!(camera.pixel_size, 0.01);
    }

    #[test]
    fn test_pixel_size_for_a_vertical_canvas() {
        let camera = camera::Camera::new(125, 200, std::f64::consts::PI / 2.0);
        assert_approx_eq!(camera.pixel_size, 0.01);
    }
}
