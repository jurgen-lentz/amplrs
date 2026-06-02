use amplrs::Ampl;

macro_rules! assert_near {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) { panic!(); }
    }
}

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    //ampl.set_bool_option("times", true);
    //ampl.set_bool_option("gentimes", true);
    ampl.set_option("solver", "gurobi");
    ampl
}

#[test]
fn export_model() {
    let mut ampl = init();
    ampl.eval(
        "set A; set B{A}; param c{A}; param d{a in A, B[a]}; var x; var y; maximize z: y; cc: x<=5;");
    let s = ampl.export_model("");
    ampl.reset();
    ampl.eval(s.as_str());
    assert_eq!(2, ampl.get_variables().len());
    assert_eq!(2, ampl.get_parameters().len());
    assert_eq!(2, ampl.get_sets().len());
    assert_eq!(1, ampl.get_objectives().len());
    assert_eq!(1, ampl.get_constraints().len());
}

#[test]
fn export_model_to_file_test() {
    let mut ampl = init();
    ampl.eval(
        "set A; set B{A}; param c{A}; param d{a in A, B[a]}; var x; var y;
        maximize z: y; cc: x<=5;");
    ampl.export_model("output.mod");
    ampl.reset();
    ampl.read("output.mod");
    assert_eq!(2, ampl.get_variables().len());
    assert_eq!(2, ampl.get_parameters().len());
    assert_eq!(2, ampl.get_sets().len());
    assert_eq!(1, ampl.get_objectives().len());
    assert_eq!(1, ampl.get_constraints().len());
}

#[test]
fn read_file_test() {
    let mut ampl = init();
    ampl.read("../models/diet/diet.mod");
    ampl.read_data("../models/diet/diet.dat");
    ampl.solve("", "");
    ampl.solve("", "");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);
}

#[test]
fn reset_test() {
    let mut ampl = init();
    ampl.read("../models/diet/diet.mod");
    ampl.read_data("../models/diet/diet.dat");

    ampl.solve("", "");
    ampl.solve("", "");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);

    ampl.eval("reset;");
    ampl.reset();
    ampl.read("../models/diet/diet.mod");
    ampl.read_data("../models/diet/diet.dat");
    ampl.solve("", "");
    ampl.solve("", "");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);

    ampl.reset();
    ampl.eval("var x;");
}

#[test]
fn solve_arguments_test() {
    let mut ampl = init();
    ampl.read("../models/diet/diet.mod");
    ampl.read_data("../models/diet/diet.dat");

    ampl.solve("", "");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);

    ampl.solve("Total_Cost", "highs");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);
    assert_eq!("highs", ampl.get_option("solver"));

    ampl.solve("", "cbc");
    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);
    assert_eq!("cbc", ampl.get_option("solver"));
}

#[test]
fn snapshot_arguments_test() {
    let mut ampl = init();
    ampl.read("../models/diet/diet.mod");
    ampl.read_data("../models/diet/diet.dat");
    let snapshot_model = ampl.snapshot("", true, false, false);
    let export_model = ampl.export_model("");
    assert_eq!(snapshot_model, export_model);
    let snapshot_data = ampl.snapshot("", false, true, false);
    let export_data = ampl.export_data("");
    assert_eq!(snapshot_data, export_data);
}
