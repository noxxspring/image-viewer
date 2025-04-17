use std::f32::consts::PI;

use image::{Rgb, RgbImage};
use minifb::{Key, Window, WindowOptions};




fn main () -> Result<(), Box<dyn std::error::Error>> {

    let size: usize = 256; // increased size for smoother curve
    let mut img = RgbImage::new(size as u32, size as u32);


    // fill with black background
    for x in 0..size as u32 {
        for y in 0..size as u32 {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }


    //Draw a solid circle 
    for x in 0..size as u32 {
        for y in 0..size as u32 {
            let center_x = x as f32 - size as f32 / 2.0;
            let center_y = y as f32 - size as f32 / 2.0;

            // convert to polar coordinates
            let r = (center_x.powi(2) + center_y.powi(2)).sqrt();
            let mut theta = center_y.atan2(center_x);

            //Normalize angle to 0-2m range
            if theta < 0.0 {
                theta += 2.0 * PI;
            }

            // Parameters for the circle shape
            let arm_length = size as f32 * 0.4;
            let center_width = size as f32 * 0.08;  // width at center
            let edge_width = size as f32 * 0.25;  // width at edges

            // calculate normalized angle to nearest arm
            let normalized_arm = (theta * 4.0 / PI).round() * PI /4.0;

            // calculate width at current radius
            let width_at_r = if r < arm_length {
                let t = r/ arm_length;

                //Smooth interpolation between center and edge width 
                center_width + (edge_width - center_width) * (t * t * (3.0 - 2.0 * t))
            }else{
                0.0
            };

            //check if point is present
            let angle_diff = (theta - normalized_arm).abs();
            if r < arm_length && angle_diff < width_at_r / r {
                img.put_pixel(x, y, Rgb([225, 0, 0]));
            }
        }
    }

    // create window with scaling for better visibility
    let scale: usize = 2;
    let window_size: usize = size * scale;
    let mut window = Window::new(
        "Solid Circle", 
        window_size, 
        window_size,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;


    // create scaled buffer
    let mut buffer = vec![0u32; window_size * window_size];

    // Scale and convert image to window buffer
    for (i , pixel) in img.pixels().enumerate(){
        let img_x = (i as usize % size) * scale;
        let img_y = (i as usize / size) * scale;

        for dy in 0..scale {
            for dx in 0..scale {
                let buf_idx = (img_y + dy) * window_size + (img_x + dx);
                if buf_idx < buffer.len() {
                    buffer[buf_idx] = 
                    (0xFF << 24) | // Alpha
                    (pixel[0] as u32) << 16 | 
                    (pixel[1] as u32) << 8 |
                    (pixel[2] as u32);
                }
            }
        }
    } 

    // Save image
    img.save("solid_circle.png")?;

    // Display image until closed 
    println!("Press ESC to exit");
    println!("Image saved as solid_cirle.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, window_size, window_size)?;
    }

    Ok(())
}