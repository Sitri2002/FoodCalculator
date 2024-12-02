use eframe::egui::{Color32, Ui};


use crate::FoodCalculatorApp;

/**
 * This function will draw the meals box
 */
pub fn gui_your_meals_box(app: &mut crate::FoodCalculatorApp, ui: &mut Ui) {
    crate::gui_tools::gui_box(ui, "Your Meals...", |ui| {

        if let Some(food) = &mut app.food {
            // user fields
            crate::gui_tools::gui_string_field(ui, "Food:", &mut food.item);

            // age field
            food.calories = crate::gui_tools::gui_number_field(
                ui, 
                "Calories: ", 
                food.calories as i32, 
                0,
                0, 
                500
            ) as i32;

            // weight field
            food.total_fat = crate::gui_tools::gui_number_field(
                ui,
                "Total Fat : ",
                food.total_fat as i32,
                0,
                0,
                500,
            ) as i32;
         

            ui.add_space(10.0);
            // display the rest of the options
            ui.horizontal(|ui| {
                gui_create_food(app, ui);
            });
        } else {
            // otherwise, show a warning message
            ui.colored_label(Color32::LIGHT_RED, "No user created / loaded!");

            ui.horizontal(|ui| {
                gui_create_food(app, ui);
            });
        }

        fn gui_create_food(app: &mut FoodCalculatorApp, ui: &mut Ui)
        {
            if ui.button("Add Breakfast").clicked() {
                app.food = Some(crate::build_food("", 0, 0, 0, 0, 0, "Breakfast"));
    
            }
            if ui.button("Add Lunch").clicked() {}
            if ui.button("Add Dinner").clicked() {}
            if ui.button("Add Snack").clicked() {}
            if ui.button("Add Exercise").clicked() {}
        }

    });
}