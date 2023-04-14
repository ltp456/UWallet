use eframe::emath::Vec2;
use log::debug;

use coreui::{eframe, egui, egui::Ui};

pub fn left_menu(ctx: &egui::Context) -> (bool, bool, bool) {
    let mut home = false;
    let mut transfer = false;
    let mut setting = false;
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        five_space(ui);
        ui.heading(egui::RichText::new("Menu").size(18.0).color(egui::Color32::WHITE));
        five_space(ui);
        ui.separator();
        five_space(ui);
        if ui.button(egui::RichText::new("Home").size(15.0).color(egui::Color32::LIGHT_BLUE)).clicked() {
            home = true;
        }
        five_space(ui);
        ui.separator();
        five_space(ui);
        if ui.button(egui::RichText::new("Transfer").size(15.0).color(egui::Color32::LIGHT_BLUE)).clicked() {
            transfer = true;
        }
        five_space(ui);
        ui.separator();
        five_space(ui);
        if ui.button(egui::RichText::new("Setting").size(15.0).color(egui::Color32::LIGHT_BLUE)).clicked() {
            setting = true;
        }
        five_space(ui);
        ui.separator();
    });
    (home, transfer, setting)
}


pub fn label(ui: &mut Ui, msg: &str) {
    ui.label(egui::RichText::new(msg).size(15.0).color(egui::Color32::LIGHT_GRAY));
}

pub fn hyperlink_to(ui: &mut Ui, msg: &str) {
    ui.hyperlink_to(
        msg,
        "https://github.com/emilk/egui",
    );
}


pub fn single_label(ui: &mut Ui, title: &str, text: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(title).size(15.0).color(egui::Color32::GRAY));
        ui.label(egui::RichText::new(text).size(13.0).color(egui::Color32::GRAY));
    });
}


pub fn single_input_label(ui: &mut Ui, title: &str, hit: &str, text: &mut String) {
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(title).size(18.0).color(egui::Color32::LIGHT_GRAY));
        ui.add(egui::TextEdit::singleline(text).password(true).text_color(egui::Color32::LIGHT_GRAY).lock_focus(true).hint_text(hit));
    });
}


pub fn simple_input_label(ui: &mut Ui, title: &str, hit: &str, text: &mut String) {
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(title).size(15.0).color(egui::Color32::GRAY));
        ui.add(egui::TextEdit::singleline(text).password(false).text_color(egui::Color32::GRAY).min_size(egui::emath::Vec2::ZERO).lock_focus(true).hint_text(hit));
    });
}


pub fn title(ui: &mut Ui, title: &str) {
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new(title).size(25.0).color(egui::Color32::WHITE));
    });
}


pub fn center(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        let id = ui.id().with("_centerer");
        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        if let Some(last_width) = last_width {
            ui.add_space((ui.available_width() - last_width) / 2.0);
        }
        let res = ui
            .scope(|ui| {
                add_contents(ui);
            })
            .response;
        let width = res.rect.width();
        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}


pub fn right_bottom_button(ui: &mut Ui, text: &str) -> bool {
    let mut clicked = false;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
        five_space(ui);
        if button(ui, "Submit").clicked() {
            clicked = true;
        }
    });
    clicked
}


pub fn button(ui: &mut Ui, text: &str) -> egui::Response {
    ui.button(egui::RichText::new(text).size(20.0).color(egui::Color32::LIGHT_BLUE))
}

pub fn small_button(ui: &mut Ui, text: &str) -> egui::Response {
    ui.button(egui::RichText::new(text).size(16.0).color(egui::Color32::WHITE))
}

pub fn ssmall_button(ui: &mut Ui, text: &str) -> egui::Response {
    ui.button(egui::RichText::new(text).size(14.0).color(egui::Color32::GRAY))
}


pub fn five_space(ui: &mut Ui) {
    ui.add_space(5.0);
}

pub fn ten_space(ui: &mut Ui) {
    ui.add_space(10.0);
}

pub fn fifteen_space(ui: &mut Ui) {
    ui.add_space(15.0);
}

pub fn twenty_space(ui: &mut Ui) {
    ui.add_space(20.0);
}

pub fn thirty_space(ui: &mut Ui) {
    ui.add_space(30.0);
}

pub fn forty_space(ui: &mut Ui) {
    ui.add_space(40.0);
}

pub fn fifty_space(ui: &mut Ui) {
    ui.add_space(50.0);
}


pub fn space(ui: &mut Ui, value: i32) {
    ui.add_space(value as f32);
}