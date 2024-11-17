pub fn project_vertex(
    vertex: [f32; 3],
    width: usize,
    height: usize,
    camera_position: [f32; 3],
) -> (usize, usize) {
    let fov: f32 = 90.0;
    let aspect_ratio = width as f32 / height as f32;
    let scale = (fov.to_radians() / 2.0).tan();

    let z = vertex[2] - camera_position[2];
    if z <= 1.0 {
        return (width, height);
    }

    let x = (vertex[0] - camera_position[0]) / z * scale * aspect_ratio;
    let y = (vertex[1] - camera_position[1]) / z * scale;

    let screen_x = ((x + 1.0) * 0.5 * width as f32).clamp(0.0, width as f32 - 1.0);
    let screen_y = ((1.0 - y) * 0.5 * height as f32).clamp(0.0, height as f32 - 1.0);

    (screen_x as usize, screen_y as usize)
}
