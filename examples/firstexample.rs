use amplrs::ampl::*;
use amplrs::objective::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut ampl = Ampl::new();

    ampl.set_option("solver", &args[1]);

    ampl.read("/home/lentz/Developer/amplrs/examples/models/diet/diet.mod");
    ampl.read_data("/home/lentz/Developer/amplrs/examples/models/diet/diet.dat");

    ampl.solve("", "");

    let total_cost = ampl.get_objective("Total_Cost");

    println!("Objective is: {}", total_cost.value());
}
