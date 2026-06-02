use amplrs::ampl::Ampl;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut ampl = Ampl::new();

    if args.len() > 1 {
        ampl.set_option("solver", &args[1]);
    }

    let model_dir = format!(
        "{}/tracking",
        if args.len() == 3 { args[2].as_str() } else { "../../models" }
    );

    ampl.read(&format!("{}/tracking.mod", model_dir));
    ampl.read_data(&format!("{}/tracking.dat", model_dir));
    ampl.read(&format!("{}/trackingbit.run", model_dir));

    // Set the scalar string parameter used by the table declarations
    ampl.eval(&format!("let data_dir := '{}';", model_dir));

    ampl.read_table("assets");
    ampl.read_table("indret");
    ampl.read_table("returns");

    let hold = ampl.get_variable("hold");
    let ifinuniverse = ampl.get_parameter("ifinuniverse");

    // Relax integrality and solve the QP relaxation
    ampl.set_bool_option("relax_integrality", true);
    ampl.solve("", "");
    println!("QP objective value {}", ampl.get_objectives()[0].value());

    let low_cutoff = 0.04;
    let high_cutoff = 0.1;

    // For each asset: force in (2), leave free (1), or force out (0)
    let hold_df = hold.get_values();
    let to_hold: Vec<f64> = hold_df
        .get_column("hold.val")
        .iter()
        .map(|v| {
            let x = v.as_f64().unwrap();
            if x < low_cutoff {
                0.0
            } else if x > high_cutoff {
                2.0
            } else {
                1.0
            }
        })
        .collect();

    ifinuniverse.set_all_double_values(&to_hold);

    // Solve the integer problem
    ampl.set_bool_option("relax_integrality", false);
    ampl.solve("", "");
    println!("QMIP objective value {}", ampl.get_objectives()[0].value());
}
