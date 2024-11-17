use image::open;

pub fn render_skybox(buffer: &mut [u32], width: usize, height: usize, texture_path: &str, camera_position: [f32; 3]) {
    let img = open(texture_path)
        .expect("Error al cargar el skybox")
        .to_rgba8();

    let (img_width, img_height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let tx = ((x as f32 + camera_position[0]) % img_width as f32) as usize;
            let ty = ((y as f32 + camera_position[1]) % img_height as f32) as usize;

            let pixel = img.get_pixel(tx as u32, ty as u32);
            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);

            buffer[y * width + x] = color;
        }
    }
}
