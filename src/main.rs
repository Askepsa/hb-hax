// #![allow(unused)]

use std::cell::RefCell;
use std::error::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use opencv::core::{self, MatExprTraitConst, MatTrait, MatTraitConst, Point, Point_};
use opencv::highgui;
use opencv::imgcodecs;
use opencv::imgproc::{self, cvt_color};

use rustautogui::{RustAutoGui, Screen};

type Err = Box<dyn Error>;

#[derive(Debug)]
struct Square {
    pub id: u32,
    pub corners: (Point, Point),
    pub center: Point,
    pub value: RefCell<Option<u32>>,
}

// offset:
// x: 28 + 1892?  y: 231 + 1055?

// play area:
// 28    231   top left
// 1892  243   top right
// 36    1051  bot left
// 1884  1055  bot right

const AREA: (Point_<u32>, Point_<u32>) = (Point_::new(28, 231), Point_::new(1884, 1055));
const OFFSET: Point = Point::new(AREA.0.x as i32, AREA.0.y as i32);
const BUTTON_POS: Point = Point::new(930, 645);

fn main() -> Result<(), Err> {
    // init
    startup();

    // sleep
    sleep(Duration::from_secs(2));

    let auto_gui = RustAutoGui::new(true);
    let mut screen = Screen::new(); // read screen

    loop {
        // read frames, get contours and coords
        let screenshot = screenshot(&mut screen)?;
        let (frame, edge) = process_screenshot(&screenshot)?;
        let mut contours: core::Vector<core::Vector<Point>> = core::Vector::default();
        imgproc::find_contours(
            &edge,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
            Point::new(0, 0),
        )?;

        // save roi images
        let mut squares = get_squares(&contours)?;
        parse_imgs(&frame, &mut squares)?;
        for square in squares.iter() {
            let pos = square.center;
            auto_gui.move_mouse_to_pos(pos.x + OFFSET.x, pos.y + OFFSET.y, 0.);
            auto_gui.left_click();
        }

        println!("{:#?}", squares);

        if !squares.is_empty() {
            auto_gui.move_mouse_to_pos(BUTTON_POS.x + OFFSET.x, BUTTON_POS.y + OFFSET.y, 0.);
            let _ = Command::new("rm").arg("imgs/*.png").output();
        }
    }
}

fn process_screenshot(screenshot: &core::Mat) -> Result<(core::Mat, core::Mat), Err> {
    let mut frame = core::Mat::default();
    cvt_color(&screenshot, &mut frame, imgproc::COLOR_BGRA2GRAY, 0)
        .expect("sumabog ang conversion mula bgra to grayscale");

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

    Ok((frame, edge))
}

fn parse_imgs(frame: &core::Mat, squares: &mut Vec<Square>) -> Result<(), Err> {
    for square in squares.iter() {
        let (top, bot) = square.corners;
        let frame = frame.roi(core::Rect::new(top.x, top.y, bot.x - top.x, bot.y - top.y))?;
        let mut edges = core::Mat::default();
        imgproc::canny_def(&frame, &mut edges, 128., 256.)?;
        imgproc::resize(
            &frame,
            &mut edges,
            core::Size::default(),
            0.2,
            0.2,
            imgproc::INTER_LINEAR,
        )?;
        let _ = imgcodecs::imwrite_def(&format!("./imgs/{}.png", square.id), &edges);
    }

    // save img, run ocr, update square and sort squares
    for square in squares.iter() {
        let path = format!("./imgs/{}.png", square.id);
        let val = {
            match ocr(&path) {
                Ok(n) => n,
                _ => continue,
            }
        };
        *square.value.borrow_mut() = Some(val);
    }

    // println!("{:#?}", squares);

    squares.sort_by_key(|sqr| *sqr.value.borrow());

    Ok(())
}

fn get_squares(contours: &core::Vector<core::Vector<Point>>) -> Result<Vec<Square>, Err> {
    let mut squares = Vec::new();
    let mut id = 1;
    for contour in contours.iter() {
        let area = imgproc::contour_area_def(&contour)?;
        if area < 10000. {
            continue;
        }

        let corners = get_corners(&contour);
        let center = get_centroid(&contour)?;
        let square = Square {
            id,
            corners,
            center,
            value: RefCell::new(None),
        };
        squares.push(square);
        id += 1;
    }

    Ok(squares)
}

fn get_centroid(contour: &core::Vector<core::Point_<i32>>) -> Result<Point, Err> {
    let moment = imgproc::moments(&contour, false)?;
    let (x, y) = (moment.m10 / moment.m00, moment.m01 / moment.m00);
    Ok(Point::new(x as i32, y as i32))
}

fn get_corners(contour: &core::Vector<core::Point_<i32>>) -> (Point, Point) {
    let max_val = i32::max_value();
    let top_left = contour
        .iter()
        .fold(Point::new(max_val, max_val), |mut acc, point| {
            if point.x <= acc.x && point.y <= acc.y {
                acc = point;
            }
            acc
        });

    let min_val = 0;
    let bot_right = contour
        .iter()
        .fold(Point::new(min_val, min_val), |mut acc, point| {
            if point.x >= acc.x && point.y >= acc.y {
                acc = point;
            }
            acc
        });

    (top_left, bot_right)
}

fn _on_mouse(event: i32, x: i32, y: i32, _: i32) {
    if let Ok(event) = highgui::MouseEventTypes::try_from(event) {
        match event {
            highgui::MouseEventTypes::EVENT_LBUTTONUP => println!("{x} {y}"),
            _ => (),
        }
    }
}

fn startup() {
    let _ = Command::new("rm")
        .arg("-rf")
        .arg("imgs")
        .output()
        .expect("kaboom");

    let _ = Command::new("mkdir")
        .arg("-p")
        .arg("imgs")
        .output()
        .expect("sumabog");
}

// offset:
// x: 28  y: 231

// play area:
// 28    231   top left
// 1892  243   top right
// 36    1051  bot left
// 1884  1055  bot right
fn screenshot(screen: &mut Screen) -> Result<core::Mat, Err> {
    let screenshot =
        screen.grab_screen_image((AREA.0.x, AREA.0.y, AREA.1.x - AREA.0.x, AREA.1.y - AREA.0.y));
    let mut frame = core::Mat::zeros(
        screenshot.height() as i32,
        screenshot.width() as i32,
        core::CV_8UC4,
    )?
    .to_mat()?;

    for pixel in screenshot.enumerate_pixels() {
        let (x, y) = (pixel.0 as i32, pixel.1 as i32);
        let color = pixel.2 .0;
        frame
            .at_2d_mut::<core::Vec4b>(y, x)?
            .copy_from_slice(&[color[2], color[1], color[0], color[3]]);
    }

    Ok(frame)
}

fn ocr(src: &str) -> Result<u32, Err> {
    let output = Command::new("ocrs")
        .arg(src)
        .output()
        .expect("sumabog ang ocr");
    let num: u32 = output
        .stdout
        .into_iter()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b as char)
        .collect::<String>()
        .parse()?;

    Ok(num)
}
