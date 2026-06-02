use amplrs::ampl::Ampl;
use amplrs::dataframe::DataFrame;
use amplrs::error::catch_ampl_error;

fn make_ampl_model(ampl: &mut Ampl, numvars: usize, unfeasible: bool, unbounded: bool) {
    assert!(!(unfeasible && unbounded));
    ampl.eval(
        "set varIndexes within Integers; set constraintIndexes within Integers; \
         param varCosts{ varIndexes }; \
         param rightHandSides{ constraintIndexes }; \
         var x{ i in varIndexes } >= 0; \
         maximize objectiveFunction : sum{ i in varIndexes } varCosts[i] * x[i];",
    );

    let indexes: Vec<f64> = (0..numvars).map(|i| i as f64).collect();
    let costs: Vec<f64>   = (0..numvars).map(|i| i as f64).collect();
    let d = DataFrame::new(1, 1, &["varIndexes", "varCosts"]);
    d.set_column_doubles("varIndexes", &indexes);
    d.set_column_doubles("varCosts",   &costs);
    ampl.set_data(&d, Some("varIndexes"));

    if !unbounded {
        ampl.eval("mainConstraints{j in constraintIndexes}: x[j] + x[j+1] <= rightHandSides[j];");
        let ci:  Vec<f64> = (0..numvars - 1).map(|i| i as f64).collect();
        let rhs: Vec<f64> = (0..numvars - 1).map(|_| if unfeasible { -1.0 } else { 1.0 }).collect();
        let d2 = DataFrame::new(1, 1, &["constraintIndexes", "rightHandSides"]);
        d2.set_column_doubles("constraintIndexes", &ci);
        d2.set_column_doubles("rightHandSides",    &rhs);
        ampl.set_data(&d2, Some("constraintIndexes"));
    }
}

/// Try to write the model; print and return `true` if AMPL raises an error.
fn try_write(ampl: &mut Ampl, filename: &str, auxfiles: &str) -> bool {
    match catch_ampl_error(|| ampl.write(filename, auxfiles)) {
        Some(msg) => { println!("{}", msg); true }
        None => false,
    }
}

fn main() {
    let mut ampl = Ampl::new();

    make_ampl_model(&mut ampl, 10, true, false);

    // Infeasible model — presolver detects infeasibility during write
    let caught = try_write(&mut ampl, "gtest", "");
    assert!(caught, "expected an error for infeasible model");

    // Wrong file type — AMPL rejects the format
    let caught = try_write(&mut ampl, "htest", "");
    assert!(caught, "expected an error for wrong file type");

    // Disable presolve so the infeasible model is written as-is — this works
    ampl.set_int_option("presolve", 0);
    ampl.write("gtest", "cr");

    // Trivial model — presolver raises an error
    ampl.reset();
    ampl.set_int_option("presolve", 10);
    ampl.eval("var x; maximize z : x; c: x = 5;");
    let caught = try_write(&mut ampl, "gtest", "");
    assert!(caught, "expected an error for trivial model");
}
