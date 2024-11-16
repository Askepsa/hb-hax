// #![allow(unused)]
use opencv::core::{
    self, count_non_zero, in_range, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Scalar,
};
use opencv::imgproc::{self, cvt_color_def};
use rustautogui::{RustAutoGui, Screen};
use std::error;
use std::thread;
use std::time::Duration;

type Error = Box<dyn error::Error>;

// fn print_coords(event: i32, x: i32, y: i32, _: i32) {
//     if let Ok(event) = highgui::MouseEventTypes::try_from(event) {
//         match event {
//             highgui::MouseEventTypes::EVENT_LBUTTONDOWN => println!("x: {x} y: {y}"),
//             _ => (),
//         }
//     };
// }

// x: 440 y: 298
// x: 1626 y: 1014
fn screenshot(screen: &mut Screen) -> Result<Mat, Error> {
    let x = 881;
    let y = 387;
    let width = 1019 - 881; // 138
    let height = 587 - 387; // 200
    let mut frame = Mat::zeros(height, width, core::CV_8UC4)?.to_mat()?;
    let screen_region = screen.grab_screen_image((x, y, width as u32, height as u32));

    for pixel in screen_region.enumerate_pixels() {
        let x = pixel.0 as i32;
        let y = pixel.1 as i32;
        let color = pixel.2 .0;
        *frame.at_2d_mut::<core::Vec4b>(y, x)? = core::Vec4b::from_array(color);
    }

    cvt_color_def(&frame.clone(), &mut frame, imgproc::COLOR_RGB2BGR)?;
    cvt_color_def(&frame.clone(), &mut frame, imgproc::COLOR_BGR2HSV)?;

    Ok(frame)
}

fn main() -> Result<(), Error> {
    // x: 881 y: 387
    thread::sleep(Duration::from_secs(5));
    let auto_gui = RustAutoGui::new(true);
    let mut screen = Screen::new();
    auto_gui.move_mouse_to_pos(881, 387, 0.);
    auto_gui.left_click();
    loop {
        let frame = screenshot(&mut screen)?;
        if is_green(&frame)? {
            auto_gui.left_click();
            auto_gui.left_click();
        }
    }
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
