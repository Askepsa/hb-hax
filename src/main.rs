// #![allow(unused)]

use std::error::Error;

use opencv::core::{no_array, Mat, Point, Point_, Scalar, Vector};
use opencv::highgui::{imshow, set_mouse_callback, wait_key_def, MouseEventTypes};
use opencv::imgcodecs::{self, imread};
use opencv::imgproc::adaptive_threshold;
use opencv::imgproc::{self, contour_area_def, draw_contours, find_contours, moments_def};

struct Square {
    area: f64,
    contours: Vector<Point>,
    center: Point,
    num: Option<i32>,
}

impl Square {
    fn new(contours: Vector<Point_<i32>>) -> Result<Self, Box<dyn Error>> {
        let area = contour_area_def(&contours)?;
        let moment = moments_def(&contours)?;
        let center = Point::new(
            (moment.m10 / moment.m00) as i32,
            (moment.m01 / moment.m00) as i32,
        );

        Ok(Self {
            area,
            contours,
            center,
            num: None,
        })
    }
}

// OCR
fn main() -> Result<(), Box<dyn Error>> {
    _do_something()?;

    Ok(())
}

fn _do_something() -> opencv::Result<()> {
    let img = imread("./monke.png", imgcodecs::IMREAD_GRAYSCALE)?;

    let mut edge = Mat::default();
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
        let _square = Square::new(contour);

        // crop
        // println!("{:#?}\n\n\n", contour);
    }

    // copy this to new img
    // x: 528 y: 639
    // x: 668 y: 779

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

    // for row in 0..=500 {
    //     for col in 0..=500 {
    //         img.at_2d_mut::<core::Vec3b>(row, col)?
    //             .copy_from_slice(&[255, 255, 255]);
    //     }
    // }

    while wait_key_def()? != 'q' as i32 {
        println!("hoy");
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
