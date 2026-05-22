use amplrs::ampl::Ampl;

fn main() {
    let mut ampl = Ampl::new();

    // Get the value of the option presolve and print
    let presolve = ampl.get_int_option("presolve");
    println!("AMPL presolve is {}", presolve);

    // Set the value to false (maps to 0)
    ampl.set_bool_option("presolve", false);

    // Get the value of the option presolve and print
    let presolve = ampl.get_int_option("presolve");
    println!("AMPL presolve is now {}", presolve);

    // Check whether an option with a specified name exists
    let value = ampl.get_option("solver");
    if !value.is_empty() {
        println!("Option solver exists and has value: {}", value);
    }

    // Check again, this time failing
    let value = ampl.get_option("s_o_l_v_e_r");
    if value.is_empty() {
        println!("Option s_o_l_v_e_r does not exist");
    }
}
