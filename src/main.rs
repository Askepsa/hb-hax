#![allow(unused)]

use std::error::Error;
use std::thread::sleep;
use std::time;

use opencv::core::{self, MatExprTraitConst, MatTrait, MatTraitConst};
use opencv::highgui::MouseEventTypes;
use opencv::imgproc;

use rustautogui::Screen;

type Err = Box<dyn Error>;

// timer
// check kung ano yung maximum color ng tile (sa bgra)
fn main() -> Result<(), Err> {
    // zzz
    sleep(time::Duration::from_secs(2));

    // hanapin kung saan ang coords lol
    let mut screen = Screen::new();
    let mut timer = time::Instant::now();
    let mut sequence: Vec<(core::Point, Vec<u8>)> = Vec::new();
    loop {
        let mut img = screenshot(&mut screen)?;
        let mut frame = screenshot(&mut screen)?;
        imgproc::cvt_color(&img, &mut frame, imgproc::COLOR_BGRA2GRAY, 0);

        // alamin kung saan ang area ng rectangles
        let mut edge = core::Mat::default();
        imgproc::adaptive_threshold(
            &frame,
            &mut edge,
            128.,
            core::BORDER_REPLICATE,
            imgproc::THRESH_BINARY_INV,
            7,
            3.,
        )?;

        // get contours
        let mut contours: core::Vector<core::Vector<core::Point>> =
            core::Vector::default();
        imgproc::find_contours(
            &edge,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
            core::Point::new(0, 0),
        )?;

        // get center of contours
        let mut rect_pts_center = Vec::new();
        for contour in contours.iter() {
            let moment = imgproc::moments_def(&contour)?;
            let center = core::Point::new(
                (moment.m10 / moment.m00) as i32,
                (moment.m01 / moment.m00) as i32,
            );
            rect_pts_center.push(center);
        }

        // get center color
        for &point in rect_pts_center.iter() {
            let color = img.at_2d::<core::Vec4b>(point.y, point.x)?;
            if color[0] == 255
                && color[1] == 255
                && color[2] == 255
                && color[3] == 255
            {
                sequence.push((
                    point,
                    vec![color[0], color[1], color[2], color[3]],
                ));
            }
        }

        println!("{:#?}", sequence);
        // sleep(time::Duration::from_millis(500));
    }

    Ok(())
}

fn screenshot(screen: &mut Screen) -> Result<core::Mat, Err> {
    // area
    // 616, 356
    // 1330, 1007
    let screen_buf =
        screen.grab_screen_image((616, 356, 1330 - 616, 1007 - 356));
    let mut frame = core::Mat::zeros(
        screen_buf.height() as i32,
        screen_buf.width() as i32,
        core::CV_8UC4,
    )?
    .to_mat()?;

    for pixel in screen_buf.enumerate_pixels() {
        let (x, y) = (pixel.0, pixel.1);
        let color = pixel.2 .0;
        frame
            .at_2d_mut::<core::Vec4b>(y as i32, x as i32)?
            .copy_from_slice(&[color[2], color[1], color[0], color[3]]);
    }

    Ok(frame)
}

fn _on_mouse(event: i32, x: i32, y: i32, _: i32) {
    if let Ok(event) = MouseEventTypes::try_from(event) {
        match event {
            MouseEventTypes::EVENT_LBUTTONDOWN => {
                println!("{x}, {y}")
            }
            _ => (),
        }
    }
}
