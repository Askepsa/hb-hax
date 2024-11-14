// #![allow(unused)]

use opencv::{
    core::{
        self, bitwise_and, in_range, no_array, type_to_str, Mat, MatExprTraitConst, MatTraitConst,
        Point, Scalar, CV_8UC3,
    },
    highgui::{imshow, set_mouse_callback, wait_key, MouseEventTypes},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        canny, circle_def, cvt_color, draw_contours, find_contours, moments, CHAIN_APPROX_SIMPLE,
        COLOR_BGRA2GRAY, LINE_8, RETR_EXTERNAL,
    },
    Result,
};

fn print_coords(event: i32, x: i32, y: i32, _: i32) {
    if let Ok(event) = MouseEventTypes::try_from(event) {
        match event {
            MouseEventTypes::EVENT_LBUTTONDOWN => println!("x: {x} y: {y}"),
            _ => (),
        }
    };
}

// bilangin kung ilang green rectangle ang meron
// at lagyan ng label sa original image
// at lagyan ng label ang sentro ng object
fn main() -> Result<()> {
    // read image
    let mut img = imread("./photo.png", IMREAD_COLOR)?; // Mat uchar3
    let (col, row) = (img.cols(), img.rows());

    println!("channels: {}", img.channels());
    println!("depth: {}", img.depth());
    println!("type: {}", type_to_str(img.typ()).unwrap());

    // make mask
    // 47, 158, 68 color target
    // filter img according to color

    // BGR
    let color = Scalar::new(75., 151., 57., 1.);
    let mut mask = Mat::default();
    in_range(&img, &color, &color, &mut mask)?;

    // perform AND bitwise operation at img
    // and pass the result img to "output" variable/Mat
    let mut output = Mat::zeros(row, col, CV_8UC3)?.to_mat()?;

    bitwise_and(&img, &img, &mut output, &mask)?;
    cvt_color(&output.clone(), &mut output, COLOR_BGRA2GRAY, 0)?;
    canny(&output.clone(), &mut output, 150., 175., 5, true)?;

    // find contours
    let mut contours: core::Vector<core::Vector<Point>> = core::Vector::default();
    find_contours(
        &output,
        &mut contours,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point { x: 0, y: 0 },
    )?;

    for contour in contours.iter() {
        println!("----------------------------");
        // get moment
        let moment = moments(&contour, true)?;
        let x = (moment.m10 / moment.m00) as i32;
        let y = (moment.m01 / moment.m00) as i32;

        let max_val = i32::max_value();
        let top = contour
            .iter()
            .fold(Point::new(max_val, max_val), |acc, point| {
                let acc_sum = acc.x.saturating_add(acc.y);
                if point.x + point.y <= acc_sum {
                    Point::new(point.x, point.y)
                } else {
                    acc
                }
            });

        let min_val = i32::min_value();
        let bot = contour
            .iter()
            .fold(Point::new(min_val, min_val), |acc, point| {
                let acc_sum = acc.y.saturating_add(acc.x);
                if point.y + point.x >= acc_sum {
                    Point::new(point.x, point.y)
                } else {
                    acc
                }
            });

        circle_def(&mut img, Point::new(x, y), 5, Scalar::new(0., 0., 255., 1.))?;
        circle_def(
            &mut img,
            Point::new(top.x, top.y),
            10,
            Scalar::new(0., 0., 255., 1.),
        )?;
        circle_def(
            &mut img,
            Point::new(bot.x, bot.y),
            10,
            Scalar::new(0., 0., 255., 1.),
        )?;
    }

    draw_contours(
        &mut img,
        &contours,
        -1,
        Scalar::new(0., 0., 255., 0.),
        1,
        LINE_8,
        &no_array(),
        1,
        Point::new(0, 0),
    )?;

    // render image
    while wait_key(0)? != 'q' as i32 {
        set_mouse_callback("Image", Some(Box::new(print_coords)))?;
        imshow("Image", &img)?;
    }

    Ok(())
}
