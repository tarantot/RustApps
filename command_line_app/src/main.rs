use std::io;

// BASIC
// fn main() {
//     println!("Enter your weight (in kg): ");
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).unwrap();

//     let weight: f32 = input.trim().parse().unwrap();
//     let mars_weight = calculate_weight_on_mars(weight);
//     println!("Your weight on Mars: {}kg", mars_weight);
// }

// fn calculate_weight_on_mars(weight: f32) -> f32 {
//     (weight / 9.81) * 3.711 
// }


// ADVANCED
fn main() {
    println!("Enter your weight: ");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();

    println!("Is the weight in kg or lb? (Enter 'kg' or 'lb'): ");
    let mut unit_input = String::new();
    io::stdin().read_line(&mut unit_input).unwrap();
    let unit = unit_input.trim().to_lowercase();

    let weight: f32 = weight_input.trim().parse().unwrap();
    let mars_weight = match unit.as_str() {
        "kg" => calculate_weight_on_mars_kg(weight),
        "lb" => calculate_weight_on_mars_lb(weight),
        _ => {
            println!("Invalid unit entered. Please use 'kg' or 'lb'.");
            return;
        }
    };

    match unit.as_str() {
        "kg" => println!("Your weight on Mars: {:.2} kg", mars_weight),
        "lb" => println!("Your weight on Mars: {:.2} lb", mars_weight),
        _ => (),
    }
}

fn calculate_weight_on_mars_kg(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn calculate_weight_on_mars_lb(weight: f32) -> f32 {
    let weight_in_kg = weight * 0.453592;
    calculate_weight_on_mars_kg(weight_in_kg) / 0.453592
}