use image::{GenericImage,ImageBuffer, Rgb, RgbImage, DynamicImage};

use opencv::core::*;
use opencv::prelude::*;
use opencv::imgcodecs::*;

use crate::*;

#[test]
#[ignore]
fn ndarray_image_rs_conversion() {
    let img = image::open("test_images/lionfish.jpeg").expect("Could not open the image").to_rgb();
    let img_vec = &img.to_vec();
    let (_w,_h) = img.dimensions();
    let arr = rgb_image_rs_to_ndarray(img);
    let rebuilt = rgb_ndarray_to_image(arr);
    let rebuilt_array_path = "test_images/rebuilt_array.png";
    rebuilt.save(rebuilt_array_path).expect("Couldn't write rebuilt image");
    let rebuilt_vec = image::open(rebuilt_array_path).unwrap().to_rgb().to_vec();
    assert_eq!(img_vec,&rebuilt_vec);
}

#[test]
#[ignore]
fn image_rs_to_mat_conversion() {
    let img = image::open("test_images/lionfish.jpeg").expect("Could not open the image").to_rgb();
    let img_vec = &img.to_vec();
    let mat: Mat = image_rs_to_mat(img);
    let rebuilt_mat_path = "test_images/rebuilt_mat.png";
    let params = opencv::types::VectorOfint::new();
    imwrite(rebuilt_mat_path, &mat, &params);
    let mat_vec = image::open(rebuilt_mat_path).unwrap().to_rgb().to_vec();
    assert_eq!(img_vec,&mat_vec);
}

#[test]
fn mat_to_image_conversion() {
    let (w,h) = (255,255);
    let mut mat: Mat = Mat::new_rows_cols_with_default(h as i32,w as i32,CV_MAKETYPE(CV_8U,3),Scalar::new(0f64,0f64,0f64,0f64)).unwrap();
    for y in 0..h {
        for x in 0..w {
            mat.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[2] = x / 4 ;
            mat.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[1] = x / 2;
            mat.at_2d_mut::<Vec3<u8>>(y as i32,x as i32).unwrap()[0] = x;
        }
    }

    let mat_path = "test_images/mat.png";
    let params = opencv::types::VectorOfint::new();
    imwrite(mat_path, &mat, &params);
    let mat_vec = image::open(mat_path).unwrap().to_rgb().to_vec();
    let img_vec = mat_to_image(mat).to_vec();
    assert_eq!(mat_vec,img_vec);
}
