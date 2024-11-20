use eframe::egui;

struct User
{
    name: String,   // Name of person
    height: Integer // This would be in inches which we can convert to cm
    gender: String,
    age: Integer, 
    weight: Double, // Pounds, we can then convert to kg
    calories: Integer, // how many calories the person has
    exercise: Integer, // The amount of exercise
    //These can be arrays of food
    mut breakfast: Tuple,
    mut lunch: Tuple,
    mut dinner: Tuple,
    mut snacks: Tuple,
}

struct Food
{
    calories: Integer,
    total_fat: Integer,
    carbohydrates: Integer,
    sugar: Integer,
    protein: Integer


}

//  May need more stuff
impl User
{
    fn calculateCalories(&self) -> u32
    {
        // Calculate calories
        if(self.gender == "male")
        {
            self.calories: (10 * (self.weight * 0.45)) + (6.25 * (self.height * 2.54)) - (5 * self.age) + 5
        }
        else
        {
            self.calories: (10 * (self.weight * 0.45)) + (6.25 * (self.height * 2.54)) - (5 * self.age) - 161
        }
    }

    fn num_of_calories(&self) -> u32
    {
        calories + exercise
    }
    
    fn build_user(name: String, height:  Integer) -> User
    {
        User
        {
            name: name,
            height: height,
        }

    }   

}



fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Calorie Calculator", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))))
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Test");
       });
   }
}