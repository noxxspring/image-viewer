use image::{buffer, DynamicImage, Pixel};
use minifb::{Key, Window, WindowOptions};






fn main () -> Result<(), Box<dyn std::error::Error>> {

    // load the original image
    let mut img = image::open("output.png")?;
    let mut current_rotation = 0;

    // create a window with dimension matching the image 
    let mut window = Window::new(
        "Image Rotation",
        img.width() as usize,
        img.height() as usize,
        WindowOptions{
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    // set the buffer for window display
    let mut buffer: Vec<u32> = vec![0; img.width() as usize * img.height() as usize];

    println!("Controls:");
    println!("'R' - Rotate 90° clockwise");
    println!("'L' - Rotate 90° counterclockwise");
    println!("'S' - save current rotation");
    println!("'Q; - Quit");


    // update the buffer with image data
    update_buffer (&img, &mut buffer);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Q) {
        // handle rotation control
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No){
            img = img.rotate90();
            current_rotation = (current_rotation + 90) % 360;
            println!("Rotated clockwise 90° - Current rotation: {}", current_rotation);

            // create new buffer with rotated dimensions
            buffer = vec![0; img.width() as usize * img.height() as usize];
            update_buffer(&img, &mut buffer);
        }

        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No){
            img = img.rotate270();
            current_rotation = (current_rotation - 90 + 360) % 360;
            println!("Rotated counterclockwise 90° - Current rotation: {}", current_rotation);


            // create new buffer with rotated dimensions
            buffer = vec![0; img.width() as usize * img.height() as usize];
            update_buffer (&img, &mut buffer);
        }

        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            let filename = format!("rotated _{}_degree.png", current_rotation);
            img.save(&filename)?;
            println!("Saved image as: {}", filename);
        }

        // update window with current buffer
        window.update_with_buffer(&buffer, img.width() as usize, img.height() as usize)?;
    }

    Ok(())

}
        fn update_buffer(img: &DynamicImage, buffer: &mut Vec<u32>) {
            let rgba_img = img.to_rgba8();

            for (i, pixel) in rgba_img.pixels().enumerate() {
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                let a = pixel[3] as u32;


                // convert RGBA to a single u32 value (ARGB format for minifb)
                buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
            }
            
        }
