use std::fs;

use image::GrayImage;
use image::ImageBuffer;

fn magnitude(a: i32, b: i32) -> i32 {
    return ((a.pow(2) as f32) + (b.pow(2) as f32)).sqrt() as i32;
}

fn sobel(img: &mut image::GrayImage) {
    let (width, height) = img.dimensions();

    const KERNEL_X: [&'static [i8; 3]; 3] = [&[-1, 0, 1], &[-2, 0, 2], &[-1, 0, 1]];
    const KERNEL_Y: [&'static [i8; 3]; 3] = [&[1, 2, 1], &[0, 0, 0], &[-1, -2, 1]];

    let mut g_x: GrayImage = ImageBuffer::new(width, height);
    let mut g_y: GrayImage = ImageBuffer::new(width, height);
    let mut g: GrayImage = ImageBuffer::new(width, height);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut ky = 0;
            let mut kx;
            let mut out_pixel_x: i32 = 0;
            let mut out_pixel_y: i32 = 0;

            // TODO: make a function to handle convolutions for any kernel
            for py in y - 1..y + 2 {
                kx = 0;
                for px in x - 1..x + 2 {
                    let pixel = img.get_pixel(px, py);
                    let vx = KERNEL_X[ky][kx];
                    let vy = KERNEL_Y[ky][kx];

                    out_pixel_x = out_pixel_x + (pixel[0] as i32) * (vx as i32);
                    out_pixel_y = out_pixel_y + (pixel[0] as i32) * (vy as i32);

                    kx = kx + 1;
                }
                ky = ky + 1;
            }

            let out_pixel = magnitude(out_pixel_x, out_pixel_y) as u8;
            out_pixel_x = out_pixel_x.abs();
            out_pixel_y = out_pixel_y.abs();

            g_x[(x, y)] = image::Luma([out_pixel_x as u8]);
            g_y[(x, y)] = image::Luma([out_pixel_y as u8]);
            g[(x, y)] = image::Luma([out_pixel]);
        }
    }

    fs::create_dir_all("out/edge_detection/sobel/").unwrap();
    g_x.save("out/edge_detection/sobel/gx.png").unwrap();
    g_y.save("out/edge_detection/sobel/gy.png").unwrap();
    g.save("out/edge_detection/sobel/sobel.png").unwrap();
}

fn main() {
    let img = image::open("data/simple_example.png").unwrap();
    let img_gray: &mut image::GrayImage = &mut img.to_luma8();

    sobel(img_gray);
}
