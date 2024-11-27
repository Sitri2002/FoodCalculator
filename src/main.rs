use std::{cmp::max, cmp::min};
use eframe::egui::Ui;
use std::io::{self, stdin};

extern crate eframe;

struct User {
    name: String, // Name of person
    height: u32,  // This would be in inches which we can convert to cm
    gender: String,
    age: u32,
    weight: f64,   // Pounds, we can then convert to kg
    calories: u32, // how many calories the person has
    exercise: u32, // The amount of exercise

    // These can be lists of food
    breakfast: Vec<Food>,
    lunch: Vec<Food>,
    dinner: Vec<Food>,
    snacks: Vec<Food>,
}

struct Food {
    item: String,
    calories: u32,
    total_fat: u32,
    carbohydrates: u32,
    sugar: u32,
    protein: u32
}


// call this inside a loop to enter as many food items as we want
fn get_food_info() -> Food {
    
    println!("Please enter your food item: ");
    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap().to_string();

    println!("Please enter this food's calories: ");
    let mut calories = String::new();
    io::stdin().read_line(&mut calories);
    let calories: u32 = calories.trim().parse().unwrap();


    println!("Please enter this food's total fat : ");
    let mut total_fat = String::new();
    io::stdin().read_line(&mut total_fat);
    let total_fat: u32 = total_fat.trim().parse().unwrap();


    println!("Please enter this food's carbohydrates: ");
    let mut carbohydrates = String::new();
    io::stdin().read_line(&mut carbohydrates);
    let carbohydrates: u32 = carbohydrates.trim().parse().unwrap();


    println!("Please enter this food's sugar: ");
    let mut sugar = String::new();
    io::stdin().read_line(&mut sugar);
    let sugar: u32 = sugar.trim().parse().unwrap();


    println!("Please enter this food's protein: ");
    let mut protein = String::new();
    io::stdin().read_line(&mut protein);
    let protein: u32 = protein.trim().parse().unwrap();

    Food {item, calories, total_fat, carbohydrates, sugar, protein}

}

//  May need more stuff
impl User {
    fn calculate_calories(&self) -> f64 {
        let weight_in_kg: f64 = self.weight * 0.45;
        let height_in_cm: f64 = self.height as f64 * 2.54;

        // Calculate calories
        if self.gender == "male" {
            (10.0 * weight_in_kg) + (6.25 * height_in_cm) - (5.0 * self.age as f64) + 5.0
        } else {
            (10.0 * weight_in_kg) + (6.25 * height_in_cm) - (5.0 * self.age as f64) - 161.0
        }
    }

    fn num_of_calories(&self) -> u32 {
        self.calories + self.exercise
    }

    fn build_user(name: String, height: u32, age: u32, gender: String, weight: f64) -> User {
        User {
            name,
            height,
            gender,
            age,
            weight,
            calories: 0,
            exercise: 0,
            breakfast: vec![],
            lunch: vec![],
            dinner: vec![],
            snacks: vec![],
        }
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calorie Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyEguiApp {
    name: String,
    weight: u32,
    height: u32
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calorie Calculator");

            ui.add_space(10.0);

            // the user information category
            ui.vertical(|ui| {
                ui.label("Your Information...");

                place_string_field(ui, "Name:", &mut self.name);

                self.weight = place_number_field(ui, "Weight (lbs):", self.weight, 0, 0, 500);
                self.height = place_number_field(ui, "Height (in):", self.height, 0, 0, 500);
            });

            ui.add_space(10.0);

            ui.button("Calculate your Calories");
        });
    }

}

fn place_string_field(ui: &mut Ui, label: &str, mut text: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(text);
    });
}
fn place_number_field(ui: &mut Ui, label: &str, mut number: u32, default: u32, minNumber: u32, maxNumber: u32) -> u32 {
    ui.horizontal(|ui| {
        let mut number_string = number.to_string();
        ui.label(label);
        if ui.text_edit_singleline(&mut number_string).changed() {
            // parse into a number
            if let Ok(parsed) = number_string.parse::<u32>() {
                number = parsed;
            } else {
                number = default;
            }
        }
    });
    min(max(number, minNumber), maxNumber)
}