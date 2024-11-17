#![allow(unused)]

use std::error::Error;
use std::process::Command;

use opencv::{
    core::{self, MatTraitConst},
    highgui, imgcodecs, imgproc,
};
use rustautogui::{RustAutoGui, Screen};

type Err = Box<dyn Error>;

struct Square {
    pub id: u32,
    pub corners: (core::Point, core::Point),
    pub center: core::Point,
    pub value: Option<u32>,
}

fn main() -> Result<(), Err> {
    startup();

    // read screen
    let auto_gui = RustAutoGui::new(true);
    let mut screen = Screen::new();

    // 28   231   top left
    // 1892 243   top right
    // 36   1051  bot left
    // 1884 1055  bot right

    let img = imgcodecs::imread("./monke.png", imgcodecs::IMREAD_GRAYSCALE)?;
    let mut edge = core::Mat::default();
    imgproc::adaptive_threshold(
        &img,
        &mut edge,
        128.,
        core::BORDER_REPLICATE,
        imgproc::THRESH_BINARY_INV,
        7,
        3.,
    )?;

    // get coords
    let mut contours: core::Vector<core::Vector<core::Point>> = core::Vector::default();
    imgproc::find_contours(
        &edge,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;

    // save one roi image
    let squares = get_squares(&contours)?;
    for square in squares.iter() {
        let (top, bot) = square.corners;
        let img = img.roi(core::Rect::new(top.x, top.y, bot.x - top.x, bot.y - top.y))?;
        let mut edges = core::Mat::default();
        imgproc::canny_def(&img, &mut edges, 128., 256.)?;
        println!("{:#?}", img);
        let _ = imgcodecs::imwrite_def(&format!("./imgs/{}.png", square.id), &edges);
    }

    // (sqr_id, value)
    let mut proccess_order: Vec<(u32, u32)> = Vec::new();
    for square in squares.iter() {
        let path = format!("./imgs/{}.png", square.id);
        let val = {
            match ocr(&path) {
                Ok(n) => n,
                _ => continue,
            }
        };
        proccess_order.push((square.id, val));
    }
    proccess_order.sort_by_key(|(_, val)| *val);
    println!("{:?}", proccess_order);

    Ok(())
}

fn get_squares(contours: &core::Vector<core::Vector<core::Point>>) -> Result<Vec<Square>, Err> {
    let mut squares = Vec::new();
    let mut id = 0;
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
            value: None,
        };
        squares.push(square);
        id += 1;
    }

    Ok(squares)
}

fn get_centroid(contour: &core::Vector<core::Point_<i32>>) -> Result<core::Point, Err> {
    let moment = imgproc::moments(&contour, false)?;
    let (x, y) = (moment.m10 / moment.m00, moment.m01 / moment.m00);
    Ok(core::Point::new(x as i32, y as i32))
}

fn get_corners(contour: &core::Vector<core::Point_<i32>>) -> (core::Point, core::Point) {
    let max_val = i32::max_value();
    let top_left = contour
        .iter()
        .fold(core::Point::new(max_val, max_val), |mut acc, point| {
            if point.x <= acc.x && point.y <= acc.y {
                acc = point;
            }
            acc
        });

    let min_val = 0;
    let bot_right = contour
        .iter()
        .fold(core::Point::new(min_val, min_val), |mut acc, point| {
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

fn get_screen(screen: &mut Screen) {
    screen.grab_screen_image((todo!()));
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
