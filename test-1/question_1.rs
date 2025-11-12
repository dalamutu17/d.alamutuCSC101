use std::io;

fn main() {
    let mut name = String::new();
    let mut units_input = String::new();

    println!("Enter customer name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter units consumed (kWh): ");
    io::stdin().read_line(&mut units_input).expect("Failed to read units");

    let units: f64 = units_input.trim().parse().expect("Please enter a number");
    let rate: f64;
    let mut total: f64;

    if units <= 100.0 {
        rate = 20.0;
    } else if units <= 300.0 {
        rate = 35.0;
    } else {
        rate = 50.0;
    }

    total = units * rate;

    if units > 500.0 {
        total = total + 5000.0;
    }

    println!("\n--- Electricity Bill ---");
    println!("Customer Name: {}", name.trim());
    println!("Units Consumed: {} kWh", units);
    println!("Rate per Unit: ₦{}", rate);
    println!("Total Bill: ₦{}", total);
}

	
