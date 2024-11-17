#![allow(unused)]

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use opencv::core::{self, no_array, MatExprTraitConst, MatTrait, MatTraitConst};
use opencv::imgcodecs::imread;
use opencv::imgproc::{self, cvt_color, match_template};
use opencv::{highgui, imgcodecs};

use rustautogui::{RustAutoGui, Screen};

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    // template matching
    let frame = imread("playground.png", imgcodecs::IMREAD_COLOR)?;
    let templ = imread("target.png", imgcodecs::IMREAD_COLOR)?;
    let mut templ_match = core::Mat::new_size_with_default(
        core::Size::new(
            frame.cols() - templ.cols() + 1,
            frame.rows() - templ.rows() + 1,
        ),
        core::CV_32FC1,
        core::Scalar::default(),
    )?;
    match_template(
        &frame,
        &templ,
        &mut templ_match,
        imgproc::TM_CCOEFF_NORMED,
        &no_array(),
    )?;

    // get min max location
    let mut min_loc = core::Point::default();
    let mut max_loc = core::Point::default();
    core::min_max_loc(
        &templ_match,
        None,
        None,
        Some(&mut min_loc),
        Some(&mut max_loc),
        &no_array(),
    )?;

    println!("Frame:\n{:#?}", frame);
    println!("Templ:\n{:#?}", templ);
    println!("Match:\n{:#?}", templ_match);

    let coords = ((max_loc.x / templ.cols(), max_loc.y / templ.cols()));
    println!("{:?}", coords);

    Ok(())
}

fn _on_mouse(event: i32, x: i32, y: i32, _: i32) {
    use highgui::MouseEventTypes;

    if let Ok(event) = MouseEventTypes::try_from(event) {
        if event == MouseEventTypes::EVENT_LBUTTONUP {
            println!("{x}, {y}");
        }
    }
}
