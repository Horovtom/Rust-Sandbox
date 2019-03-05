pub fn run() {
    len age = 18;

    if age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else {
        println!("Bartender: You are not old enough!");
    }

    let check_id: bool = false;
    if check_id && age >= 21 {
        println!("Bartender: What would you like to drink?");
    } else if check_id {
println!("Bartender: You are not old enough!");
    } else {
println!("Bartender: I need to see your ID!");
    }

    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    
}