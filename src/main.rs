use opencv::core::{count_non_zero, in_range, Mat, MatTraitConst, Scalar};
use opencv::highgui::{imshow, wait_key_ex};
use opencv::imgcodecs::{self, imread};
use opencv::imgproc::{self, cvt_color_def};
use std::error;

type Error = Box<dyn error::Error>;

fn main() -> Result<(), Error> {
    let mut _blue = imread("blue.jpeg", imgcodecs::IMREAD_COLOR)?;
    cvt_color_def(&_blue.clone(), &mut _blue, imgproc::COLOR_BGR2HSV)?;

    let mut _red = imread("red.png", imgcodecs::IMREAD_COLOR)?;
    cvt_color_def(&_red.clone(), &mut _red, imgproc::COLOR_BGR2HSV)?;

    let mut _green = imread("green.png", imgcodecs::IMREAD_COLOR)?;
    cvt_color_def(&_green.clone(), &mut _green, imgproc::COLOR_BGR2HSV)?;

    while wait_key_ex(0)? != 'q' as i32 {
        print!("green: ");
        process(&_green)?;
        print!("blue: ");
        process(&_red)?;
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
