use serde::{Deserialize, Serialize};

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

impl ExerciseTracker {
    pub fn new() -> Self {
        ExerciseTracker {
            exercises: Vec::new(),
        }
    }

    pub fn add_exercise(&mut self, activity: &str, time_minutes: f32, user_weight: f32) {
        let met = match activity.to_lowercase().as_str() {
            "running" => 9.8,
            "weight lifting" => 6.0,
            "biking" => 7.5,
            "regular exercise" => 4.0,
            "swimming" => 8.0,
            _ => 4.0, // Default for unknown activities
        };

        let calories_burned = (met * user_weight * time_minutes) / 60.0;

        self.exercises.push(Exercise {
            activity: activity.to_string(),
            time_minutes,
            calories_burned,
        });
    }

    pub fn total_calories_burned(&self) -> f32 {
        self.exercises.iter().map(|e| e.calories_burned).sum()
    }
}
