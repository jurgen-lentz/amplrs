use amplrs::ampl::Ampl;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut ampl = Ampl::new();

    if args.len() > 1 {
        ampl.set_option("solver", &args[1]);
    }

    let model_dir = if args.len() == 3 {
        args[2].clone()
    } else {
        "examples/models".to_string()
    };

    ampl.read(&format!("{}/diet/diet.mod", model_dir));
    ampl.read_data(&format!("{}/diet/diet.dat", model_dir));

    ampl.solve("", "");

    let total_cost = ampl.get_objective("Total_Cost");
    println!("Objective is: {}", total_cost.value());

    // Reassign data - specific instances
    let cost = ampl.get_parameter("cost");
    cost.set_some_double_values(&["BEEF", "HAM"], &[5.01, 4.55]);
    println!("Increased costs of beef and ham.");

    ampl.solve("", "");
    println!("New objective value: {}", ampl.get_objective("Total_Cost").value());

    // Reassign data - all instances
    cost.set_all_double_values(&[3.0, 5.0, 5.0, 6.0, 1.0, 2.0, 5.01, 4.55]);
    println!("Updated all costs.");

    ampl.solve("", "");
    println!("New objective value: {}", ampl.get_objective("Total_Cost").value());

    // Get the values of the variable Buy in a DataFrame
    let buy = ampl.get_variable("Buy");
    let df = buy.get_values();
    println!("{}", df.to_string());

    // Get the values of an expression into a DataFrame
    let df2 = ampl.get_data(&["{j in FOOD} 100*Buy[j]/Buy[j].ub"]);
    println!("{}", df2.to_string());
}
