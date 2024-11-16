// #![allow(unused)]
use opencv::core::{
    self, count_non_zero, in_range, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Scalar,
};
use opencv::highgui::{self, imshow, set_mouse_callback, wait_key_ex};
use opencv::imgproc::{self, cvt_color_def};
use rustautogui::{RustAutoGui, Screen};
use std::error;

type Error = Box<dyn error::Error>;

fn print_coords(event: i32, x: i32, y: i32, _: i32) {
    if let Ok(event) = highgui::MouseEventTypes::try_from(event) {
        match event {
            highgui::MouseEventTypes::EVENT_LBUTTONDOWN => println!("x: {x} y: {y}"),
            _ => (),
        }
    };
}

fn main() -> Result<(), Error> {
    let mut auto_gui = RustAutoGui::new(true);
    let mut screen = Screen::new();

    let x = 881;
    let y = 387;
    let width = 1019 - 881; // 138
    let height = 587 - 387; // 200
    let mut img = Mat::zeros(height, width, core::CV_8UC4)?.to_mat()?;
    let screen_region = screen.grab_screen_image((x, y, width as u32, height as u32));
    for pixel in screen_region.enumerate_pixels() {
        let x = pixel.0 as i32;
        let y = pixel.1 as i32;
        let color = pixel.2 .0;
        *img.at_2d_mut::<core::Vec4b>(y, x)? = core::Vec4b::from_array(color);
    }

    cvt_color_def(&img.clone(), &mut img, imgproc::COLOR_BGRA2BGR)?;
    while wait_key_ex(0)? != 'q' as i32 {
        set_mouse_callback("img", Some(Box::new(print_coords)))?;
        imshow("img", &img)?;
    }

    Ok(())
}

fn process(frame: &Mat) -> Result<(), Error> {
    if is_green(frame)? {
        println!("green");
    } else {
        println!("not green");
    }

    imshow("Color", &frame)?;

    Ok(())
}

fn is_green(frame: &Mat) -> Result<bool, Error> {
    let low = Scalar::new(18., 0., 0., 0.);
    let upper = Scalar::new(75., 255., 255., 0.);
    let mut mask = Mat::default();
    in_range(&frame, &low, &upper, &mut mask)?;

    let threshold = 0.5;
    let mask_pixels = count_non_zero(&mask)? as f32;
    let frame_pixels = (frame.rows() + frame.cols()) as f32;

    Ok((mask_pixels / frame_pixels) > threshold)
}
