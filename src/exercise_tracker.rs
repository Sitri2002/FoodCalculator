// Authors: Jack Nguyen, Audrey Hall, Caleb Moore, Ethan Rees

use serde::{Deserialize, Serialize};

/* Add Serialize and Deserialize trait to save/load files
Add Default trait to conveinently create struct instances, since we use a lot of inheritance here
Add Clone trait to interact with instances data values independently 
*/ 
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Exercise {
    pub activity: String,
    pub time_minutes: f32,
    pub calories_burned: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExerciseTracker {
    pub exercises: Vec<Exercise>,
}

// Methods implement for exercise tracker struct, calculate calories burned through exercise type
// and time spent working out
impl ExerciseTracker {
    pub fn add_exercise(&mut self, activity: &str, time_minutes: f32, user_weight: f32) {
        let met = match activity.to_lowercase().as_str() {
            "running" => 9.8,
            "weight lifting" => 6.0,
            "biking" => 7.5,
            "regular exercise" => 4.0,
            "swimming" => 8.0,
            _ => 0.0, // Default for unknown activities is 0
        };

        let calories_burned = (met * user_weight * time_minutes) / 60.0;

        self.exercises.push(Exercise {
            activity: activity.to_string(),
            time_minutes,
            calories_burned,
        });
    }

    // Calculate total calories burned by calculating the sum of all the calories
    // burned of each exercise stored in the vector
    pub fn total_calories_burned(&self) -> f32 {
        self.exercises.iter().map(|e| e.calories_burned).sum()
    }
}
