use image::open;

pub fn render_skybox(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    texture_path: &str,
    camera_position: [f32; 3],
) {
    let img = open(texture_path)
        .unwrap_or_else(|_| panic!("No se pudo cargar el skybox en la ruta: {}", texture_path))
        .to_rgba8();

    let (img_width, img_height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let tx = ((x as f32 + camera_position[0] * 5.0) as u32 % img_width) as usize;
            let ty = ((y as f32 + camera_position[1] * 5.0) as u32 % img_height) as usize;

            let pixel = img.get_pixel(tx as u32, ty as u32);
            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);

            buffer[y * width + x] = color;
        }
    }
}
