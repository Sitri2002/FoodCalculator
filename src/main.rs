use eframe::egui::{Ui};
use your_info_box::gui_your_info_box;
use your_meals_box::gui_your_meals_box;
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, LineWriter, Write};

pub mod your_info_box;
pub mod your_meals_box;
pub mod gui_tools;

extern crate eframe;

struct User {
    name: String, // Name of person
    height: f32,  // This would be in inches which we can convert to cm
    gender: String,
    age: f32,
    weight: f32,   // Pounds, we can then convert to kg
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
    calories: i32,
    total_fat: i32,
    carbohydrates: i32,
    sugar: i32,
    protein: i32,
    category: String
}

// May need more stuff
impl User {
    fn calculate_calories(&self) -> f64 {
        let weight_in_kg: f64 = self.weight as f64 * 0.45;
        let height_in_cm: f64 = self.height as f64 * 2.54;

        // Calculate calories
        if self.gender == "male" {
            (10.0 * weight_in_kg) + (6.25 * height_in_cm) - (5.0 * self.age as f64) + 5.0
        } else {
            (10.0 * weight_in_kg) + (6.25 * height_in_cm) - (5.0 * self.age as f64) - 161.0
        }
    }

    fn calculate_exercise(&self) ->f64
    {
        0.0
    }

    fn num_of_calories(&self) -> u32 {
        self.calories + self.exercise
    }

    fn save(&self, path: &str) -> io::Result<bool> {
        let file = File::create(path)?;
        let mut writer = LineWriter::new(file);
        writer.write_all(self.name.as_bytes())?;
        writer.write_all("\n".as_bytes())?;
        writer.write_all(self.height.to_string().as_bytes())?;
        writer.write_all("\n".as_bytes())?;
        writer.write_all(self.age.to_string().as_bytes())?;
        writer.write_all("\n".as_bytes())?;
        writer.write_all(self.gender.as_bytes())?;
        writer.write_all("\n".as_bytes())?;
        writer.write_all(self.weight.to_string().as_bytes())?;
        writer.write_all("\n".as_bytes())?;
        writer.write_all(self.exercise.to_string().as_bytes())?;
        writer.write_all("\n".as_bytes())?;

        writer.flush()?;
        
        Ok(true)
    }
    
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calorie Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(FoodCalculatorApp::new(cc)))),
    )
}

#[derive(Default)]
struct FoodCalculatorApp {
    user: Option<User>,
    food: Option<Food>,
}

impl FoodCalculatorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}


impl eframe::App for FoodCalculatorApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            // draw the header
            ui.heading("Calorie Calculator");
            ui.add_space(10.0);

            // draw the info box
            gui_your_info_box(self, ui);

            ui.add_space(10.0);

            // draw the meals box
            gui_your_meals_box(self, ui);

            // draw the footer
            ui.add_space(10.0);

            let mut calories: f64 = 0.0;
            if let Some(user) = &mut self.user {
                calories = user.calculate_calories();
            }

            let mut exercise: f64 = 0.0;
            if let Some(user) = &mut self.user
            {
                exercise = user.calculate_exercise();
            }
            ui.label(format!("Your Total Calories: {calories}"));
            ui.label(format!("Your Total Exercise: {exercise}"))
        });
    }
}

pub fn build_user(name: &str, height: f32, age: f32, gender: &str, weight: f32) -> User {
    User {
        name: name.to_string(),
        height,
        gender: gender.to_string(),
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

pub fn build_food(item: &str, calories: u32, total_fat: u32, carbohydrates: u32, sugar: u32, protein: u32, category: &str) -> Food
{
    Food
    {
        item: item.to_string(),
        calories: 0,
        total_fat: 0,
        carbohydrates: 0,
        sugar: 0, 
        protein: 0,
        category: category.to_string()
    }
}

// call this inside a loop to enter as many food items as we want
fn get_food_info() -> Food {
    println!("Please enter your food item: ");
    let mut item = String::new();
    io::stdin().read_line(&mut item).unwrap().to_string();

    println!("Please enter this food's calories: ");
    let mut calories = String::new();
    io::stdin().read_line(&mut calories);
    let calories: i32 = calories.trim().parse().unwrap();

    println!("Please enter this food's total fat : ");
    let mut total_fat = String::new();
    io::stdin().read_line(&mut total_fat);
    let total_fat: i32 = total_fat.trim().parse().unwrap();

    println!("Please enter this food's carbohydrates: ");
    let mut carbohydrates = String::new();
    io::stdin().read_line(&mut carbohydrates);
    let carbohydrates: i32 = carbohydrates.trim().parse().unwrap();

    println!("Please enter this food's sugar: ");
    let mut sugar = String::new();
    io::stdin().read_line(&mut sugar);
    let sugar: i32 = sugar.trim().parse().unwrap();

    println!("Please enter this food's protein: ");
    let mut protein = String::new();
    io::stdin().read_line(&mut protein);
    let protein: i32 = protein.trim().parse().unwrap();

    println!("Please enter this food's category: ");
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap().to_string();

    Food {
        item,
        calories,
        total_fat,
        carbohydrates,
        sugar,
        protein,
        category,
    }
}

fn getExercise()
{
    println!("Please enter the amount of calories burned.");
    let mut total_exercise = String::new();
    io::stdin().read_line(&mut total_exercise);
    let total_exercise: u32 = total_exercise.trim().parse().unwrap();
}

/**
 * This function loads a user from a path
 */
fn load_user(path: &str) -> io::Result<crate::User> {
    let file = File::open(path)?;
    let content = BufReader::new(file);

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Unable to parse the file"))
        .collect();

    // read the name from line 0
    let mut name = if lines.len() > 0 {
        lines[0].as_str()
    } else {
        "No Name"
    };

    // read the height from line 1
    let mut height: f32 = if lines.len() > 1 {
        lines[1].as_str().parse::<f32>().unwrap()
    } else {
        60.0
    };

    // read the age from line 2
    let mut age: f32 = if lines.len() > 2 {
        lines[2].as_str().parse::<f32>().unwrap()
    } else {
        18.0
    };

    // read the gender from line 3
    let mut gender = if lines.len() > 3 {
        lines[3].as_str()
    } else {
        "male"
    };

    // read the weight from line 4
    let mut weight = if lines.len() > 4 {
        lines[4].as_str().parse::<f32>().unwrap()
    } else {
        200.0
    };

    // read the exercise from line 5
    let mut excercise = if lines.len() > 5 {
        lines[5].as_str().parse::<u32>().unwrap()
    } else {
        0
    };

    let mut user = crate::build_user(name, height, age, gender, weight);
    user.exercise = excercise;
    Ok(user)
}
