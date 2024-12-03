use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FoodEntry {
    pub name: String,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub sugar: f32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FoodTracker {
    pub entries: Vec<FoodEntry>,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub total_sugar: f32,
}

impl FoodTracker {
    pub fn add_food(&mut self, food: FoodEntry) {
        self.total_calories += food.calories;
        self.total_protein += food.protein;
        self.total_carbs += food.carbs;
        self.total_fat += food.fat;
        self.total_sugar += food.sugar;
        self.entries.push(food);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.total_calories = 0.0;
        self.total_protein = 0.0;
        self.total_carbs = 0.0;
        self.total_fat = 0.0;
        self.total_sugar = 0.0;
    }
}
