// #![allow(unused)]

use std::error::Error;

use opencv::core::{no_array, Mat, Point, Scalar, Vector};
use opencv::highgui::{imshow, set_mouse_callback, wait_key_def, MouseEventTypes};
use opencv::imgcodecs::{self, imread};
use opencv::imgproc::{self, adaptive_threshold, draw_contours, find_contours};

// OCR
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn _do_something() -> opencv::Result<()> {
    let img = imread("./monke.png", imgcodecs::IMREAD_GRAYSCALE)?;

    let mut edge = Mat::default();
    // canny_def(&img, &mut edge, 200., 255.)?;
    adaptive_threshold(
        &img,
        &mut edge,
        128.,
        imgproc::ADAPTIVE_THRESH_MEAN_C,
        imgproc::THRESH_BINARY_INV,
        7,
        3.,
    )?;

    // get coords
    let mut contours: Vector<Vector<Point>> = Vector::default();
    find_contours(
        &edge,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;

    // make struct of img (uuid: String, contour, val: Option<u32>)
    // crop images and then save to array (with uuid) and file
    // make tesseract read each images and add corresponding uuid
    // as key to hashmap and its corresponding parsed number as value

    for contour in contours.iter() {
        println!("{:#?}\n\n\n", contour);
    }

    // get store values of hashmap to an array and sort it
    // for each img (struct) then get centroid coords and
    // make rustautogui move the mouse and click the
    // coords

    let mut img = imread("./monke.png", imgcodecs::IMREAD_COLOR)?;
    draw_contours(
        &mut img,
        &contours,
        -1,
        Scalar::new(0., 0., 255., 0.),
        2,
        imgproc::LINE_8,
        &no_array(),
        1,
        Point::new(0, 0),
    )?;

    while wait_key_def()? != 'q' as i32 {
        set_mouse_callback("Monke", Some(Box::new(_print_coords)))?;
        imshow("Monke", &img)?;
    }

    Ok(())
}

fn _print_coords(event: i32, x: i32, y: i32, _: i32) {
    if let Ok(event) = MouseEventTypes::try_from(event) {
        match event {
            MouseEventTypes::EVENT_LBUTTONDOWN => println!("x: {x} y: {y}"),
            _ => (),
        }
    };
}
