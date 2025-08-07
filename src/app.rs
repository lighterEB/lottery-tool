use eframe::egui;
use std::sync::Arc;
use egui::FontFamily;
use egui::FontDefinitions;
use crate::model::Number;
use crate::generator::generate_numbers;

// 定义应用状态
pub struct LotteryApp {
    numbers: Vec<Number>, // 存储生成的号码
}

impl LotteryApp {
    // 构造函数
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "cryy".to_owned(),
            Arc::new(egui::FontData::from_owned(include_bytes!("../fonts/cryy.ttf").to_vec())
            ),
        );
        fonts.families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "cryy".to_owned());

    cc.egui_ctx.set_fonts(fonts);
        Self {
            numbers: Vec::new(),
        }
    }
}

impl eframe::App for LotteryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // 标题
            ui.heading("彩票分析缩水工具");

            // 按钮：生成号码
            if ui.button("生成号码").clicked() {
                // 调用生成号码的函数
                self.numbers = generate_numbers();
            }

            // 统计信息
            ui.label(format!("共生成 {} 组号码", self.numbers.len()));

            // 滚动区域显示号码
            egui::ScrollArea::vertical().show(ui, |ui| {
                for num in &self.numbers {
                    ui.label(format!("{}-{}-{}", num.hundreds, num.tens, num.units));
                }
            });
        });
    }
}