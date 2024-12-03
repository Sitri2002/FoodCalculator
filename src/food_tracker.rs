// Authors: Jack Nguyen, Audrey Hall, Caleb Moore, Ethan Rees

use serde::{Deserialize, Serialize};

/* Add Serialize and Deserialize trait to save/load files
Add Default trait to conveinently create struct instances, since we use a lot of inheritance here
Add Clone trait to interact with instances data values independently 
*/ 
#[derive(Serialize, Deserialize, Default, Clone)]
// Struct of single food entry nutrients
pub struct FoodEntry {
    pub name: String,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub sugar: f32,
}

// Struct of total diet
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FoodTracker {
    pub entries: Vec<FoodEntry>,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub total_sugar: f32,
}

// implement add_food method to calculate total diet
impl FoodTracker {
    pub fn add_food(&mut self, food: FoodEntry) {
        self.total_calories += food.calories;
        self.total_protein += food.protein;
        self.total_carbs += food.carbs;
        self.total_fat += food.fat;
        self.total_sugar += food.sugar;
        self.entries.push(food);
    }
}
