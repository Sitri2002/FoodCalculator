use eframe::egui::{Color32, Ui};




/**
 * This function will draw the meals box
 */
pub fn gui_your_meals_box(app: &mut crate::FoodCalculatorApp, ui: &mut Ui) {
    crate::gui_tools::gui_box(ui, "Your Meals...", |ui| {
        if ui.button("Add Breakfast").clicked() {
            
        }
        if ui.button("Add Lunch").clicked() {}
        if ui.button("Add Dinner").clicked() {}
        if ui.button("Add Snack").clicked() {}
        if ui.button("Add Exercise").clicked() {}
    });
}