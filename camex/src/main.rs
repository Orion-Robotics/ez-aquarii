// use opencv::core;
// use opencv::highgui;
// use opencv::prelude;
use opencv::{
    imgcodecs, 
    highgui, 
    core, 
    prelude, 
    imgproc,
};
use anyhow::Result;

fn main() -> Result<()> {
    highgui::named_window("nya", highgui::WINDOW_NORMAL).expect("window creation failed");
    let im = imgcodecs::imread("junior_01.png", 1).expect("read failed");
    let hsvim = {
        let mut out = prelude::Mat::default();
        imgproc::cvt_color(&im, &mut out, imgproc::COLOR_BGR2HSV, 0).expect("failed to hsv convert");
        out
    };
    let mask = {
        let mut m = prelude::Mat::default();
        let lb = core::Scalar::new(20.0, 30.0, 40.0, 50.0);
        let ub = core::Scalar::new(255.0, 255.0, 255.0, 255.0);
        core::in_range(&hsvim, &lb, &ub, &mut m).expect("failed to inrange");
        m
    };
    let masked = {
        let mut out = prelude::Mat::default();
        core::bitwise_and(&im, &im, &mut out, &mask).expect("failed to bitwise and");
        out
    };
    let tcb = highgui::TrackbarCallback::default();
    highgui::create_trackbar("HH", "nya", 255, 255, )
    highgui::imshow("nya", &masked).expect("show failed");
    highgui::wait_key(0).expect("waitkey failed");
    Ok(())
}
