use amplrs::ampl::Ampl;
use amplrs::dataframe::DataFrame;

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

    // Food set and its parameters — all columns declared up front
    let foods = ["BEEF", "CHK", "FISH", "HAM", "MCH", "MTL", "SPG", "TUR"];
    let costs = [3.59, 2.59, 2.29, 2.89, 1.89, 1.99, 1.99, 2.49];
    let fmin  = [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0];
    let fmax  = [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0];

    let df_food = DataFrame::new(1, 3, &["FOOD", "cost", "f_min", "f_max"]);
    df_food.set_column_strings("FOOD", &foods);
    df_food.set_column_doubles("cost", &costs);
    df_food.set_column_doubles("f_min", &fmin);
    df_food.set_column_doubles("f_max", &fmax);
    ampl.set_data(&df_food, Some("FOOD"));

    // Nutrient set and its parameters — all columns declared up front
    let nutrients = ["A", "C", "B1", "B2", "NA", "CAL"];
    let nmin = [700.0, 700.0, 700.0, 700.0, 0.0, 16000.0];
    let nmax = [20000.0, 20000.0, 20000.0, 20000.0, 50000.0, 24000.0];

    let df_nutr = DataFrame::new(1, 2, &["NUTR", "n_min", "n_max"]);
    df_nutr.set_column_strings("NUTR", &nutrients);
    df_nutr.set_column_doubles("n_min", &nmin);
    df_nutr.set_column_doubles("n_max", &nmax);
    ampl.set_data(&df_nutr, Some("NUTR"));

    // Amount matrix: rows = nutrients (6), cols = foods (8), row-major
    #[rustfmt::skip]
    let amounts: &[f64] = &[
         60.0,    8.0,   8.0,  40.0,   15.0,   70.0,   25.0,   60.0,  // A
         20.0,    0.0,  10.0,  40.0,   35.0,   30.0,   50.0,   20.0,  // C
         10.0,   20.0,  15.0,  35.0,   15.0,   15.0,   25.0,   15.0,  // B1
         15.0,   20.0,  10.0,  10.0,   15.0,   15.0,   15.0,   10.0,  // B2
        928.0, 2180.0, 945.0, 278.0, 1182.0,  896.0, 1329.0, 1397.0,  // NA
        295.0,  770.0, 440.0, 430.0,  315.0,  400.0,  379.0,  450.0,  // CAL
    ];

    let df_amt = DataFrame::new(2, 1, &["NUTR", "FOOD", "amt"]);
    df_amt.set_matrix_doubles(&nutrients, &foods, amounts);
    ampl.set_data(&df_amt, None);

    ampl.solve("", "");

    println!("Objective: {}", ampl.get_objective("Total_Cost").value());
}
