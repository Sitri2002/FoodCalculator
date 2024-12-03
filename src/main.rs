// Authors: Jack Nguyen, Audrey Hall, Caleb Moore, Ethan Rees

/*
    A GUI App to calculate a person calorie profile, including their basic metabolism rate (MBR),
    their diet, and their exercise history. Then calculate if they maintain their daily calories 
    well or not (excess or deficient). 

    Users can also save/load profiles based on their names. All of these data will be maintained in a profiles
    folder.
 */

// Import modules
mod food_tracker;
mod exercise_tracker;

use food_tracker::{FoodEntry, FoodTracker};
use exercise_tracker::{Exercise, ExerciseTracker};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

/* Add Serialize and Deserialize trait to save/load files
Add Default trait to conveinently create struct instances, since we use a lot of inheritance here
Add Clone trait to interact with instances data values independently 
*/ 
#[derive(Serialize, Deserialize, Default, Clone)]

// The struct of user profile and their info and calorie in/outtake
struct UserProfile {
    name: String,
    age: u32,
    gender: String,
    height: f32,
    weight: f32,
    calorie_consumption: f32,
    calorie_burned: f32,
}

#[derive(Serialize, Deserialize, Default)]
// The data struct to save an app instance of a user profile, including their data and 
// their exercise schedule and food eaten to load back.
struct AppData {
    user_profile: UserProfile,
    food_tracker: FoodTracker,
    exercise_tracker: ExerciseTracker,
}

// The GUI instance struct, containing GUI data (App is default eframe name)
#[derive(Default)]
struct App {
    data: AppData,
    calculated_calories: Option<f32>,
    new_food: FoodEntry,
    new_exercise: Exercise,
    profile_folder: String,
    available_profiles: Vec<String>,
    selected_profile: Option<String>,
    selected_activity: String,
}

// implement App methods
impl App {
    // App constructor, when app instance is created, checks if profiles folder exists
    // if it doesnt exist, create it (with error checking)
    fn new() -> Self {
        let profile_folder = "profiles".to_string();
        fs::create_dir_all(&profile_folder).unwrap_or_else(|e| {
            println!("Error creating profile folder: {}", e);
        });
        let available_profiles = Self::get_profile_list(&profile_folder);
        App {
            profile_folder,
            available_profiles,
            selected_activity: "Running".to_string(),
            ..Default::default()
        }
    }

    // Grab the list of profiles from the profile, include checks if folder doesnt exist
    fn get_profile_list(folder: &str) -> Vec<String> {
        if let Err(e) = fs::create_dir_all(folder) {
            println!("Folder doesnt exist: {}", e);
            return vec![];
        }

        fs::read_dir(folder)
            .unwrap()
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.is_file() {
                        path.file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    // Calculate calories based on sex, height, weight (I got these values from BMR equation)
    fn calculate_calories(&self) -> f32 {
        let bmr = match self.data.user_profile.gender.to_lowercase().as_str() {
            "male" => {
                10.0 * self.data.user_profile.weight
                    + 6.25 * self.data.user_profile.height
                    - 5.0 * self.data.user_profile.age as f32
                    + 5.0
            }
            "female" => {
                10.0 * self.data.user_profile.weight
                    + 6.25 * self.data.user_profile.height
                    - 5.0 * self.data.user_profile.age as f32
                    - 161.0
            }
            _ => 0.0,
        };

        bmr * 1.2 // The default no exercise multiplier, since we will calculate exercise seperately
    }

    // Save data (check if there is a name so it is possible to save json with user name to get profiles)
    fn save_data(&mut self) {
        if self.data.user_profile.name.is_empty() {
            println!("User name is required to save data!");
            return;
        }

        let file_name = format!("{}/{}.json", self.profile_folder, self.data.user_profile.name);
        if let Ok(json) = serde_json::to_string(&self.data) {
            if fs::write(&file_name, json).is_ok() {
                println!("Data saved to {}", file_name);
            }
        }
        self.update_profile_list();
    }

    // Load data based on String selection in GUI
    fn load_data(&mut self) {
        if let Some(profile) = &self.selected_profile {
            let file_name = format!("{}/{}.json", self.profile_folder, profile);
            if let Ok(json) = fs::read_to_string(&file_name) {
                if let Ok(data) = serde_json::from_str(&json) {
                    self.data = data;
                    println!("Data loaded from {}", file_name);
                }
            }
        }
    }

    // Update profile list every time there is a change
    fn update_profile_list(&mut self) {
        self.available_profiles = Self::get_profile_list(&self.profile_folder);
    }
}

// Implementing the GUI appearance
impl eframe::App for App {
    /*
        Here I am adding labels and text boxes for the user to fill in
     */
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Daily Calorie Tracker");

            // User Profile Input
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.data.user_profile.name);
            });

            // DragValue lets you drag the cell up and down and increases the counter, rather than
            // clicking one by one such as a slider (speed is how much the value changes per pixel)
            ui.horizontal(|ui| {
                ui.label("Age:");
                ui.add(egui::DragValue::new(&mut self.data.user_profile.age).speed(1));
            });

            ui.horizontal(|ui| {
                ui.label("Gender:");
                egui::ComboBox::from_label("Select Gender")
                    .selected_text(&self.data.user_profile.gender)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.data.user_profile.gender, "Male".to_string(), "Male");
                        ui.selectable_value(&mut self.data.user_profile.gender, "Female".to_string(), "Female");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Height (cm):");
                ui.add(egui::DragValue::new(&mut self.data.user_profile.height).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Weight (kg):");
                ui.add(egui::DragValue::new(&mut self.data.user_profile.weight).speed(1.0));
            });

            self.calculated_calories = Some(self.calculate_calories());
            
            // Calculate BMR and put it in the label
            if let Some(calories) = self.calculated_calories {
                ui.label(format!("Daily Calorie Needs: {:.2} Calories", calories));
            }

            // Food tracker, user can add nutrients and calories and that will be added
            // to their diet profile
            ui.separator();
            ui.label("Add Food:");

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_food.name);
            });

            ui.horizontal(|ui| {
                ui.label("Calories:");
                ui.add(egui::DragValue::new(&mut self.new_food.calories).speed(10.0));
            });

            ui.horizontal(|ui| {
                ui.label("Protein (g):");
                ui.add(egui::DragValue::new(&mut self.new_food.protein).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Carbs (g):");
                ui.add(egui::DragValue::new(&mut self.new_food.carbs).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Fat (g):");
                ui.add(egui::DragValue::new(&mut self.new_food.fat).speed(1.0));
            });

            ui.horizontal(|ui| {
                ui.label("Sugar (g):");
                ui.add(egui::DragValue::new(&mut self.new_food.sugar).speed(1.0));
            });

            if ui.button("Add Food").clicked() {
                self.data.food_tracker.add_food(self.new_food.clone());
                self.new_food = FoodEntry::default();
            }

            ui.separator();
            ui.label("Total Nutrients:");
            ui.label(format!("Calories: {:.2} Calories", self.data.food_tracker.total_calories));
            ui.label(format!("Protein: {:.2} g", self.data.food_tracker.total_protein));
            ui.label(format!("Carbs: {:.2} g", self.data.food_tracker.total_carbs));
            ui.label(format!("Fat: {:.2} g", self.data.food_tracker.total_fat));
            ui.label(format!("Sugar: {:.2} g", self.data.food_tracker.total_sugar));

            // Exercise tracker, user can choose what exercise they did and for how long, and it will be added to their exercise data
            // The calories burned will be calculated by exercise type calories burned/minute * minutes (the data is in exercise_tracker.rs)
            ui.separator();
            ui.label("Add Exercise:");

            ui.horizontal(|ui| {
                ui.label("Activity:");
                egui::ComboBox::from_label("Select Activity")
                    .selected_text(&self.selected_activity)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_activity, "Running".to_string(), "Running");
                        ui.selectable_value(&mut self.selected_activity, "Weight Lifting".to_string(), "Weight Lifting");
                        ui.selectable_value(&mut self.selected_activity, "Biking".to_string(), "Biking");
                        ui.selectable_value(&mut self.selected_activity, "Regular Exercise".to_string(), "Regular Exercise");
                        ui.selectable_value(&mut self.selected_activity, "Swimming".to_string(), "Swimming");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Time (minutes):");
                ui.add(egui::DragValue::new(&mut self.new_exercise.time_minutes).speed(1.0));
            });

            if ui.button("Add Exercise").clicked() {
                self.data.exercise_tracker.add_exercise(
                    &self.selected_activity,
                    self.new_exercise.time_minutes,
                    self.data.user_profile.weight,
                );
                self.new_exercise = Exercise::default();
            }

            // This display each exercise session done, how long, and how much calories is burned
            ui.separator();
            ui.label("Exercises:");
            for exercise in &self.data.exercise_tracker.exercises {
                ui.label(format!(
                    "{}: {:.2} minutes, {:.2} Calories burned",
                    exercise.activity, exercise.time_minutes, exercise.calories_burned
                ));
            }

            ui.label(format!(
                "Total Calories Burned: {:.2} Calories",
                self.data.exercise_tracker.total_calories_burned()
            ));

            // Summary of how much calories is needed for BMR, how much is added in through food, 
            // and how much is burned through exercise
            ui.separator();
            ui.label("Summary:");
            if let Some(calories_needed) = self.calculated_calories {
                let calories_eaten = self.data.food_tracker.total_calories;
                let calories_burned = self.data.exercise_tracker.total_calories_burned();
                let excess_calories = calories_eaten - (calories_needed + calories_burned);

                ui.label(format!("Calories Needed: {:.2} Calories", calories_needed));
                ui.label(format!("Calories Eaten: {:.2} Calories", calories_eaten));
                ui.label(format!("Calories Burned: {:.2} Calories", calories_burned));
                ui.label(format!("Excess Calories: {:.2} Calories", excess_calories));
            }

            // Save, load, select profile buttons
            ui.separator();
            if ui.button("Save All Data").clicked() {
                self.save_data();
            }

            ui.horizontal(|ui| {
                ui.label("Select Profile:");
                egui::ComboBox::from_label("")
                    .selected_text(
                        self.selected_profile
                            .clone()
                            .unwrap_or_else(|| "Select a profile".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for profile in &self.available_profiles {
                            ui.selectable_value(&mut self.selected_profile, Some(profile.clone()), profile);
                        }
                    });
            });

            if ui.button("Load Selected Profile").clicked() {
                self.load_data();
            }
        });
    }
}

// Main function to run GUI (can just type cargo run in CLI), default resolution is 800 by 850, but can change if you want to
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 850.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Calorie Tracker",
        options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
