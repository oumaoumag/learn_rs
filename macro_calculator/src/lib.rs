use json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins:f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins =0.0;
    let mut total_fats =0.0;

    for food in foods {
        // Get the calorie value in kcal from string
        let calories_str = &food.calories[1];
        let calories_value: f64 = calories_str
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);

        // Get totals by multiplying by number of portions
        total_calories += calories_value * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // return a JSON object
    json::object!{
        "cals": format_value(total_calories),
        "carbs": format_value(total_carbs),
        "proteins": format_value(total_proteins),
        "fats": format_value(total_fats)
    }
}
    // function to format values according to requirements
    fn format_value(value: f64) -> f64 {
        let rounded = (value * 100.0).round() / 100.0;

        if (rounded * 10.0).round() == (rounded * 100.0).round() / 10.0 {
            (rounded * 10.0).round() / 10.0
        } else {
            rounded
        }
    }
