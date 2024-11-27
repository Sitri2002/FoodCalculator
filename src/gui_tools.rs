use eframe::egui::{self, Color32, Stroke, Ui};
use std::{cmp::max, cmp::min};

/**
 * This function draws a box with a fancy outline and title, then runs the code inside of it
 */
pub fn gui_box(ui: &mut crate::Ui, label: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::group(ui.style())
        .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(25, 25, 25, 25)))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(label);
                ui.add_space(5.0);
                content(ui);
            });
        });
}

/**
 * This function draws a string field that the user can type in
 */
pub fn gui_string_field(ui: &mut Ui, label: &str, mut text: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(text);
    });
}

/**
 * This function draws a number field that the user can type in, as well as providing some other functionality
 */
pub fn gui_number_field(
    ui: &mut Ui,
    label: &str,
    mut number: i32,
    default: i32,
    minNumber: i32,
    maxNumber: i32,
) -> i32 {

    // layout the elements horizontally
    ui.horizontal(|ui| {
        let mut number_string = number.to_string();

        // draw the label
        ui.label(label);

        // draw the string field
        if ui.text_edit_singleline(&mut number_string).changed() {
            // parse into a number
            if let Ok(parsed) = number_string.parse::<i32>() {
                number = parsed;
            } else {
                number = default;
            }
        }
    });

    // return the number within the range
    min(max(number, minNumber), maxNumber)
}
