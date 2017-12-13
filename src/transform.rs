use cgmath;
use cgmath::Deg;
use cgmath::Matrix;
use cgmath::SquareMatrix;
use glium::Frame;
use glium::Surface;
use prelude::*;

pub fn transform_normals(total_transform: &Trans4d) -> Trans3d {
    let without_projective_part = Trans3d::from_cols(total_transform[0].truncate(), total_transform[1].truncate(), total_transform[2].truncate());
    without_projective_part.invert().unwrap().transpose()
}

pub fn reasonable_perspective(z_near: f32, z_far: f32) -> Trans4d {
    cgmath::perspective(Deg(45.0), 1.0, z_near, z_far)
}

pub fn fix_aspect_ratio_x(frame: &Frame, total_transform: Trans4d) -> Trans4d {
    let (width, height) = frame.get_dimensions();
    Trans4d::from_nonuniform_scale(height as f32 / width as f32, 1.0, 1.0) * total_transform
}

pub fn fix_aspect_ratio_y(frame: &Frame, total_transform: Trans4d) -> Trans4d {
    let (width, height) = frame.get_dimensions();
    Trans4d::from_nonuniform_scale(1.0, width as f32 / height as f32, 1.0) * total_transform
}
