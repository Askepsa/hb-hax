// #![allow(unused)]

use std::error::Error;

use opencv::core::{self, no_array, MatExprTraitConst, MatTrait, MatTraitConst};
use opencv::highgui;
use opencv::imgcodecs::{self, imread};
use opencv::imgproc::{self, cvt_color, match_template};

use rustautogui::{RustAutoGui, Screen};

type Err = Box<dyn Error>;

fn main() -> Result<(), Err> {
    let auto_gui = RustAutoGui::new(true);
    let mut screen = Screen::new();

    //      Left         Right
    // top: 166, 232     1718, 238
    // bot: 160, 1044    1692, 1040

    // get frame
    // convert auto_gui buf to opencv's buf type
    let img_buf = screen.grab_screen_image((166, 232, 1718 - 166, 1040 - 232));
    let mut frame = core::Mat::zeros(
        img_buf.height() as i32,
        img_buf.width() as i32,
        core::CV_8UC4,
    )?
    .to_mat()?;
    cvt_color(&frame.clone(), &mut frame, imgproc::COLOR_RGBA2BGRA, 0)?;

    for pixel in img_buf.enumerate_pixels() {
        let x = pixel.0 as i32;
        let y = pixel.1 as i32;
        let color = pixel.2 .0;
        frame
            .at_2d_mut::<core::Vec4b>(y, x)?
            .copy_from_slice(&color);
    }

    // get playground coords
    while highgui::wait_key(0)? != 'q' as i32 {
        highgui::set_mouse_callback("img", Some(Box::new(on_mouse)))?;
        highgui::imshow("img", &frame)?;
    }

    Ok(())
}

fn on_mouse(event: i32, x: i32, y: i32, _: i32) {
    use highgui::MouseEventTypes;

    if let Ok(event) = MouseEventTypes::try_from(event) {
        if event == MouseEventTypes::EVENT_LBUTTONUP {
            println!("{x}, {y}");
        }
    }
}

fn get_target_coords(frame: core::Mat, templ: core::Mat) -> Result<(i32, i32), Err> {
    let mut frame = imread("playground.png", imgcodecs::IMREAD_GRAYSCALE)?;
    let templ = imread("target.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // template matching
    let mut templ_match = core::Mat::new_size_with_default(
        core::Size::new(frame.cols() - templ.cols(), frame.rows() - templ.rows()),
        core::CV_32FC1,
        core::VecN::default(),
    )?;
    match_template(
        &frame,
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
        &mut frame,
        core::Rect::new(max_loc.x, max_loc.y, templ.rows(), templ.cols()),
        core::Scalar::new(255., 255., 255., 255.),
        1,
        imgproc::LINE_8,
        0,
    )?;

    Ok((69, 420))
}
