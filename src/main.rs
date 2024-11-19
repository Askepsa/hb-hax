#![allow(unused)]

use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use opencv::core::{self, MatExprTraitConst, MatTrait, MatTraitConst};
use opencv::highgui::MouseEventTypes;
use opencv::imgproc;

use rustautogui::{RustAutoGui, Screen};

type Err = Box<dyn Error>;

// threshold
const WHITE: core::VecN<u8, 4> = core::VecN::new(255, 255, 255, 255);
const OFFSET: (i32, i32) = (646, 389);

// area
// 646, 389     - offset
// 1274, 1005

// timer
// check kung ano yung maximum color ng tile (sa bgra)
// may performance issue kaya di gumagana ng maayos
fn main() -> Result<(), Err> {
    // zzz
    sleep(Duration::from_secs(2));

    let auto_gui = RustAutoGui::new(true);
    auto_gui.move_mouse_to_pos(350 + 646, 507 + 389, 0.);
    auto_gui.left_click();

    let mut screen = Screen::new();
    let mut timer = Instant::now();
    let mut sequence: Vec<core::Point> = Vec::new();
    let mut prev_active_sqr: Option<core::Point_<i32>> = None;

    let mut hardcoded_shet = hardcoded_shet_get_circle()?;

    // let mut foo_img = screenshot(&mut screen)?;
    // black_magic(&mut screen, &mut timer, &mut sequence, &mut prev_active_sqr);
    // while opencv::highgui::wait_key_def()? != 'q' as i32 {
    //     opencv::highgui::set_mouse_callback("img", Some(Box::new(_on_mouse)));
    //     opencv::highgui::imshow("img", &foo_img);
    // }

    loop {
        black_magic(
            &mut screen,
            &mut timer,
            &mut sequence,
            &mut prev_active_sqr,
            &mut hardcoded_shet,
        );

        for point in sequence.iter() {
            println!("{:#?}", point);
            auto_gui.move_mouse_to_pos(
                point.x + OFFSET.0,
                point.y + OFFSET.1,
                0.,
            );
            auto_gui.left_click();
        }

        if !sequence.is_empty() {
            println!("{:#?}", sequence);
        }

        sequence.clear();
    }

    Ok(())
}

fn black_magic(
    screen: &mut Screen,
    timer: &mut Instant,
    sequence: &mut Vec<core::Point_<i32>>,
    prev_active_sqr: &mut Option<core::Point_<i32>>,
    rect_pts_center: &mut Vec<core::Point_<i32>>,
) -> Result<(), Err> {
    loop {
        if Instant::now().duration_since(*timer) >= Duration::from_secs(3) {
            return Ok(());
        }

        let mut img = screenshot(screen)?;
        let mut frame = screenshot(screen)?;
        imgproc::cvt_color(&img, &mut frame, imgproc::COLOR_BGRA2GRAY, 0);

        // let mut edge = core::Mat::default();
        // imgproc::adaptive_threshold(
        //     &frame,
        //     &mut edge,
        //     128.,
        //     core::BORDER_REPLICATE,
        //     imgproc::THRESH_BINARY_INV,
        //     7,
        //     3.,
        // )?;

        // let mut contours: core::Vector<core::Vector<core::Point>> =
        //     core::Vector::default();
        // imgproc::find_contours(
        //     &edge,
        //     &mut contours,
        //     imgproc::RETR_EXTERNAL,
        //     imgproc::CHAIN_APPROX_SIMPLE,
        //     core::Point::new(0, 0),
        // )?;

        // let mut rect_pts_center = Vec::new();
        // for contour in contours.iter() {
        //     let moment = imgproc::moments_def(&contour)?;
        //     let center = core::Point::new(
        //         (moment.m10 / moment.m00) as i32,
        //         (moment.m01 / moment.m00) as i32,
        //     );
        //     rect_pts_center.push(center);
        // }

        // println!("{:#?}", rect_pts_center);

        // get sequence
        // check if color does not match anymore
        // basta pipili tayo ng bagong point
        // kung ang prev point ay di na puti
        // at kung ang cur point ay puti

        for &point in rect_pts_center.iter() {
            if let Some(prev_point) = prev_active_sqr {
                let prev_point_color =
                    img.at_2d::<core::Vec4b>(prev_point.y, prev_point.x)?;
                if *prev_point == point && *prev_point_color != WHITE {
                    *prev_active_sqr = None;
                    *timer = Instant::now();
                    continue;
                }
            }

            let cur_point_color = img.at_2d::<core::Vec4b>(point.y, point.x)?;
            if *cur_point_color == WHITE && prev_active_sqr.is_none() {
                sequence.push(core::Point::new(point.x, point.y));
                *prev_active_sqr = Some(point);
                break;
            }
        }
    }
}

fn hardcoded_shet_get_circle() -> Result<Vec<core::Point_<i32>>, Err> {
    let p1 = core::Point_ { x: 521, y: 522 };
    let p2 = core::Point_ { x: 313, y: 522 };
    let p3 = core::Point_ { x: 104, y: 522 };
    let p4 = core::Point_ { x: 521, y: 313 };
    let p5 = core::Point_ { x: 313, y: 313 };
    let p6 = core::Point_ { x: 104, y: 313 };
    let p7 = core::Point_ { x: 521, y: 105 };
    let p8 = core::Point_ { x: 313, y: 105 };
    let p9 = core::Point_ { x: 104, y: 105 };

    Ok(vec![p1, p2, p3, p4, p5, p6, p7, p8, p9])
}

fn screenshot(screen: &mut Screen) -> Result<core::Mat, Err> {
    // area
    // 646, 389
    // 1274, 1005
    let screen_buf =
        screen.grab_screen_image((646, 389, 1274 - 646, 1005 - 389));
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
