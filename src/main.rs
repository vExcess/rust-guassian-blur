use image::{GenericImage, GenericImageView};
mod blur;

struct ImageData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>
}

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("./input.png").unwrap();

    let (img_width, img_height) = img.dimensions();

    let mut img_data = ImageData {
        width: img_width as i32,
        height: img_height as i32,
        data: vec![0; (img_width * img_height * 4) as usize]
    };

    {
        let mut x: u32 = 0;
        let mut y: u32;
        while x < img_width {
            y = 0;
            while y < img_height {
                let pixel = img.get_pixel(x, y).0;
                let idx = (x + y * img_width) << 2;
                
                img_data.data[(idx) as usize] = pixel[0];
                img_data.data[(idx+1) as usize] = pixel[1];
                img_data.data[(idx+2) as usize] = pixel[2];
                img_data.data[(idx+3) as usize] = pixel[3];
                
                y += 1;
            }
            x += 1;
        }
    }

    img_data.data = blur::blur(img_data.width, img_data.height, &mut img_data.data, 10.0);

    {
        let mut x: u32 = 0;
        let mut y: u32;
        while x < img_width {
            y = 0;
            while y < img_height {
                let mut pixel = img.get_pixel(x, y);
                let idx = (x + y * img_width) << 2;
                
                pixel.0[0] = img_data.data[(idx) as usize];
                pixel.0[1] = img_data.data[(idx + 1) as usize];
                pixel.0[2] = img_data.data[(idx + 2) as usize];
                pixel.0[3] = img_data.data[(idx + 3) as usize];
                img.put_pixel(x, y, pixel);
                
                y += 1;
            }
            x += 1;
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    img.save("./output.png").unwrap();

    println!("success");
}