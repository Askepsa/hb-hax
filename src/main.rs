// #![allow(unused)]

use std::error::Error;

use opencv::core::{self, no_array, MatTraitConst};
use opencv::highgui;
use opencv::imgcodecs::{self, imread};
use opencv::imgproc::{self, match_template};

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    let mut img = imread("playground.png", imgcodecs::IMREAD_GRAYSCALE)?;
    let templ = imread("target.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // template matching
    let mut templ_match = core::Mat::new_size_with_default(
        core::Size::new(img.cols() - templ.cols(), img.rows() - templ.rows()),
        core::CV_32FC1,
        core::VecN::default(),
    )?;
    match_template(
        &img,
        &templ,
        &mut templ_match,
        imgproc::TM_CCOEFF_NORMED,
        &no_array(),
    )?;

    // get min max location
    let mut min_loc = Default::default();
    let mut max_loc = Default::default();
    core::min_max_loc(
        &templ_match,
        None,
        None,
        Some(&mut min_loc),
        Some(&mut max_loc),
        &no_array(),
    )?;

    // draw rectangle
    imgproc::rectangle(
        &mut img,
        core::Rect::new(max_loc.x, max_loc.y, templ.rows(), templ.cols()),
        core::Scalar::new(255., 255., 255., 255.),
        1,
        imgproc::LINE_8,
        0,
    )?;

    while highgui::wait_key_def()? != 'q' as i32 {
        highgui::imshow("Img", &img)?;
    }

    Ok(())
}
