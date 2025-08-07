use eframe::egui;

mod app;
mod generator;
mod model;

use crate::app::LotteryApp;

fn main() -> Result<(), eframe::Error> {
    // 配置窗口选项
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    // 启动应用
    eframe::run_native(
        "彩票分析缩水工具",
        options,
        Box::new(|cc| Ok(Box::new(LotteryApp::new(cc)))),
    )
}
