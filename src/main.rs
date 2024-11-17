// #![allow(unused)]

use opencv::core::{
    self, min_max_loc, no_array, Mat, MatTraitConst, Point, Rect, Scalar, Size,
};
use opencv::highgui::{imshow, wait_key_def};
use opencv::imgcodecs::{self, imread};
use opencv::imgproc;

use std::error::Error;

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    let mut img = imread("playground.png", imgcodecs::IMREAD_GRAYSCALE)?;
    let templ = imread("target.png", imgcodecs::IMREAD_GRAYSCALE)?;
    let mut result = Mat::new_size_with_default(
        Size::new(img.cols() - templ.cols() + 1, img.rows() - templ.rows() + 1),
        core::CV_32FC1,
        Scalar::default(),
    )?;

    imgproc::match_template(
        &img,
        &templ,
        &mut result,
        imgproc::TM_CCOEFF_NORMED, // da best daw pag masyadong maliwanag ang img
        &no_array(),
    )?;

    let mut min_loc = Point::default();
    let mut max_loc = Point::default();
    min_max_loc(
        &result,
        None,
        None,
        Some(&mut min_loc),
        Some(&mut max_loc),
        &core::no_array(),
    )?;

    println!("{:#?}", min_loc);
    println!("{:#?}", max_loc);

    imgproc::rectangle(
        &mut img,
        Rect::new(max_loc.x, max_loc.y, templ.rows(), templ.cols()),
        Scalar::new(0., 0., 0., 0.),
        2,
        imgproc::LINE_8,
        0,
    )?;

    for i in 0..result.rows() {
        for j in 0..result.cols() {
            let res = result.at_2d::<f32>(i, j)?;
            if *res >= 0.8 {
                println!("{j} {i} {:?}", res);
            }
        }
    }

    println!("{:#?}", result);

    while wait_key_def()? != 'q' as i32 {
        imshow("img", &img)?;
    }

    Ok(())
}
