#![allow(dead_code)]
mod test;
pub mod stereo;

use image::{ImageBuffer, Rgb, RgbImage};
use opencv::core::*;
use opencv::prelude::*;
use ndarray::{prelude::*,Array3};

use std::convert::TryInto;

#[no_mangle]
pub unsafe extern "C" fn parse_float_data(ptr: *const f32, length: u32) {
    let v = std::slice::from_raw_parts(ptr, length.try_into().unwrap());
    println!("The length of the read-in slice is: {}", v.len());
    for i in 0..v.len() {
        println!("{}",v[i]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn process_rgb_image_rs(ptr: *const u8, width: u32, height: u32) {
    let length: usize = (width * height * 3) as usize; // This is multiplied by 3 since it's an RGB pixel
    let v = std::slice::from_raw_parts(ptr, length);
    let img = parse_image(v, width, height);
    img.save("rebuilt.png").expect("Couldn't save the image");

}

fn parse_image(v: &[u8], w: u32, h: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: RgbImage = ImageBuffer::new(w, h);
    println!("The length of the read-in slice is: {}", v.len());
    for y in 0..h {
        for x in 0..w {
            let pixel_num = ((w * y + x) * 3) as usize;
            // Just making sure that we don't accidentally try to read past the end of the array
            if pixel_num < ((w * h * 3) as usize - 2) {
                let b = v[pixel_num] as u8;
                let g = v[pixel_num + 1] as u8;
                let r = v[pixel_num + 2] as u8;
                //println!("bgr #{} at ({},{}): [{},{},{}]",pixel_num,x,y,b,g,r);
                img.put_pixel(x, y, Rgb([r, g, b]));
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }
    img
}

#[no_mangle]
pub extern "C" fn write_small_image_rs() {
    let mut img: RgbImage = ImageBuffer::new(10, 5);
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let position_u8: u8 = (w * y + x) as u8;
            img.put_pixel(x, y, Rgb([position_u8, position_u8, position_u8]));
        }
    }
    img.save("small_image.png")
        .expect("Couldn't save the small image");
}

// Credit to Stack Overflow answer from `E_net4 the harmed SO member`
// Pulled from https://stackoverflow.com/questions/56762026/how-to-save-ndarray-in-rust-as-image
fn rgb_ndarray_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    let img: RgbImage = RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions");
    img
}

fn rgb_image_rs_to_ndarray(img: RgbImage) -> Array3<u8> {
    let (w,h) = img.dimensions();
    //let mut dim = Dimension::new(u32;3);
    let mut arr = Array3::<u8>::zeros((h as usize,w as usize,3));
    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x,y);
            arr[[y as usize,x as usize,0usize]] = pixel[0];
            arr[[y as usize,x as usize,1usize]] = pixel[1];
            arr[[y as usize,x as usize,2usize]] = pixel[2];
        }
    }
    arr
}

fn image_rs_to_mat(img: RgbImage) -> Mat {
    let (w,h) = img.dimensions();
    let mut mat_rgb: Mat = Mat::new_rows_cols_with_default(h as i32,w as i32,CV_MAKETYPE(CV_8U,3),Scalar::new(0f64,0f64,0f64,0f64)).unwrap();
    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x,y);
            // OpenCV pixels are BGR
            mat_rgb.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[2] = pixel[0];
            mat_rgb.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[1] = pixel[1];
            mat_rgb.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[0] = pixel[2];
        }
    }
    // mat_rgb.at_2d_mut::<Vec3<u8>>(0,0).unwrap()[0] += 1;
    // let pixel = mat_rgb.at_2d::<Vec3<u8>>(0, 0).unwrap();
    // println!("[B,G,R]= [{},{},{}]", pixel.0[0], pixel.0[1], pixel.0[2]);
    mat_rgb
}


fn mat_to_image(mat: Mat) -> RgbImage {
    let (w,h) = (mat.cols().unwrap(),mat.rows().unwrap());
    let mut img: RgbImage = ImageBuffer::new(w as u32,h as u32);
    for y in 0..h {
        for x in 0..w {
            let r = mat.at_2d::<Vec3<u8>>(y as i32,x as i32).unwrap()[2];
            let g = mat.at_2d::<Vec3<u8>>(y as i32,x as i32).unwrap()[1];
            let b = mat.at_2d::<Vec3<u8>>(y as i32,x as i32).unwrap()[0];
            img.put_pixel(x as u32,y as u32,image::Rgb([r,g,b]));
        }
    }
    img
}
