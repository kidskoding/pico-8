use image::GenericImageView;

fn convert_spritesheet_to_pixel_array(file: &str) -> Vec<u8> {
    let img = image::open(file).unwrap();
    let (width, height) = img.dimensions();
    let mut pixel_array: Vec<u8> = Vec::new();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            pixel_array.push(pixel[0]);
            pixel_array.push(pixel[1]);
            pixel_array.push(pixel[2]);
            pixel_array.push(pixel[3]);
        }
    }
    
    pixel_array
}