use std::path::Path;

fn main() {
    
    let image_path = "../image/flower.jpg";

    // print the image current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    // check if image exists
    let path = Path::new(image_path);
    println!("image_path: {:?}", path.canonicalize().unwrap());


    //try to read the file content output
    let img_result = image::open(path);
    match img_result {
        Ok (img) => {
            println!("image loaded successfully!");
            println!("Dimension: {}x{}", img.width(), img.height());
        },
        Err(e) => {
            println!("Failed to load image: {:?}", e);
        }
        
    }
}
