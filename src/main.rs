use std::collections::HashMap;
use std::io;

struct Bill {
    name: String,
    amount: f64,
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}

fn add_bill(bills: &mut HashMap<String, Bill>) {
    println!("Bill name:");
    let name = get_input();
    println!("Amount:");
    let amount: f64 = match get_input().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid amount.");
            return;
        }
    };
    bills.insert(name.clone(), Bill { name, amount });
    println!("Bill added.");
}

fn view_bills(bills: &HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("No bills.");
        return;
    }
    for bill in bills.values() {
        println!("{}: ${:.2}", bill.name, bill.amount);
    }
}

fn remove_bill(bills: &mut HashMap<String, Bill>) {
    println!("Bill name to remove:");
    let name = get_input();
    if bills.remove(&name).is_some() {
        println!("Bill removed.");
    } else {
        println!("Bill not found.");
    }
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();
    loop {
        println!("\n1. Add bill\n2. View bills\n3. Remove bill\n4. Quit");
        match get_input().as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => break,
            _ => println!("Invalid choice."),
        }
    }
}
