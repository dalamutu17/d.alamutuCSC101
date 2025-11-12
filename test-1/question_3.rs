use std::io;

fn main() {
    println!("--- Welcome to PAU Campus Café ---");
    println!("Code\tItem\t\tPrice(₦)");
    println!("T\tTea\t\t800");
    println!("C\tCoffee\t\t1200");
    println!("S\tSandwich\t2000");
    println!("J\tJuice\t\t1500");
    println!("Type 'exit' anytime to quit.\n");

    let mut total = 0.0;

    loop {
        let mut code = String::new();
        println!("Enter item code (T/C/S/J) or 'exit' to finish:");
        io::stdin().read_line(&mut code).expect("Failed to read input");
        let code = code.trim().to_uppercase();

        if code == "EXIT" {
            break;
        }

        let mut qty_input = String::new();
        println!("Enter quantity:");
        io::stdin().read_line(&mut qty_input).expect("Failed to read quantity");

        let qty: f64 = qty_input.trim().parse().unwrap_or(0.0);
        let price: f64;

        if code == "T" {
            price = 800.0;
        } else if code == "C" {
            price = 1200.0;
        } else if code == "S" {
            price = 2000.0;
        } else if code == "J" {
            price = 1500.0;
        } else {
            println!("Invalid code! Try again.\n");
            continue;
        }

        let cost = price * qty;
        total += cost;

        println!("Added {} x {} = ₦{}", code, qty, cost);
        println!("Current total: ₦{}\n", total);
    }

    println!("\n--- Bill Summary ---");
    println!("Total before discount: ₦{}", total
