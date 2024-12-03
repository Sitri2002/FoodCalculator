use eframe::egui::{Color32, Ui};

use crate::FoodCalculatorApp;

pub fn gui_your_info_box(app: &mut crate::FoodCalculatorApp, ui: &mut Ui) {
    // the user information category
    crate::gui_tools::gui_box(ui, "Your Information...", |ui| {
        // if the user is currently set inside of the app, show the user fields
        if let Some(user) = &mut app.user {
            // user fields
            crate::gui_tools::gui_string_field(ui, "Name:", &mut user.name);

            // age field
            user.age = crate::gui_tools::gui_number_field(
                ui, 
                "Age (years):", 
                user.age as i32, 
                0,
                0, 
                500
            ) as f32;

            // weight field
            user.weight = crate::gui_tools::gui_number_field(
                ui,
                "Weight (lbs):",
                user.weight as i32,
                0,
                0,
                500,
            ) as f32;

            // height field
            user.height = crate::gui_tools::gui_number_field(
                ui,
                "Height (in):",
                user.height as i32,
                0,
                0,
                500,
            ) as f32;

            // gender field
            crate::gui_tools::gui_string_field(ui, "Gender:", &mut user.gender);

            // exercise field
            /*
            user.exercise = crate::gui_tools::gui_number_field(
                ui,
                "Previous Excecise (idk):",
                user.exercise as i32,
                0,
                0,
                500,
            ) as u32; */

            // layout the following buttons horizontally
            // save the user data to a file
            if ui.button("Save User To File").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("User File", &["ccuser"])
                    .save_file()
                {
                    if let Some(path_str) = path.to_str() {
                        user.save(path_str);
                    }
                }
            }

            ui.add_space(10.0);
            // display the rest of the options
            ui.horizontal(|ui| {
                gui_create_load_user(app, ui);
            });
        } else {
            // otherwise, show a warning message
            ui.colored_label(Color32::LIGHT_RED, "No user created / loaded!");

            ui.horizontal(|ui| {
                gui_create_load_user(app, ui);
            });
        }

        fn gui_create_load_user(app: &mut FoodCalculatorApp, ui: &mut Ui) {
            if ui.button("Create New User").clicked() {
                app.user = Some(crate::build_user("", 60.0, 18.0, "male", 180.0));
            }

            if ui.button("Load Existing User").clicked() {
                // open the file dialog to choose a user file
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("User File", &["ccuser"])
                    .pick_file()
                {
                    // get the string version of the path
                    if let Some(path_str) = path.to_str() {

                        // load in the user
                        match crate::load_user(path_str) {
                            Ok(user) => {
                                app.user = Some(user);
                            }
                            Err(e) => {
                                println!("FAILED TO READ IN USER! {e}");
                            }
                        }
                    }
                }
            }
        }
    });
}

