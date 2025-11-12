use std::io;

fn main() {
    loop {
        let mut p_input = String::new();
        let mut r_input = String::new();
        let mut t_input = String::new();

        println!("Enter the principal amount (P): ");
        io::stdin().read_line(&mut p_input).expect("Failed to read P");

        println!("Enter annual interest rate (R%): ");
        io::stdin().read_line(&mut r_input).expect("Failed to read R");

        println!("Enter number of years (T): ");
        io::stdin().read_line(&mut t_input).expect("Failed to read T");

        let p: f64 = p_input.trim().parse().expect("Enter a valid number for P");
        let r: f64 = r_input.trim().parse().expect("Enter a valid number for R");
        let t: f64 = t_input.trim().parse().expect("Enter a valid number for T");

        let a = p * (1.0 + r / 100.0).powf(t);
        let interest = a - p;

        println!("\n--- Loan Summary ---");
        println!("Principal (P): ₦{}", p);
        println!("Rate (R): {}%", r);
        println!("Time (T): {} years", t);
        println!("Total Amount (A): ₦{}", a);
        println!("Interest: ₦{}", interest);

        let mut again = String::new();
        println!("\nDo you want to calculate for another borrower? (y/n): ");
        io::stdin().read_line(&mut again).expect("Failed to read input");

        if again.trim().to_lowercase() == "n" {
            println!("Thank you for using the loan calculator!");
            break;
        }
        println!(); // just a space before next loop
    }
}

