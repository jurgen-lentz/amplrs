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
fn set_string_option() {
    let mut ampl = init();
    ampl.set_option("solver", "afortmp");
    let s = ampl.get_option("solver");
    assert_eq!("afortmp", s);
    ampl.set_option("solver", "c:\\bin\\afortmp.exe");
    let s = ampl.get_option("solver");
    assert_eq!("c:\\bin\\afortmp.exe", s);
}

#[test]
fn set_int_option() {
    let mut ampl = init();
    ampl.set_int_option("anumber", 55);
    let i = ampl.get_int_option("anumber");
    assert_eq!(i, 55);
}

#[test]
fn set_not_so_int_option() {
    let mut ampl = init();
    let precision = 0.000000001;
    ampl.set_dbl_option("test", 55.0 - precision);
    assert_eq!(55.0 - precision, ampl.get_dbl_option("test"));
    assert_eq!(55, ampl.get_int_option("test"));
}

#[test]
fn set_dbl_option() {
    let mut ampl = init();
    ampl.set_dbl_option("anumber", 55.6);
    let i = ampl.get_dbl_option("anumber");
    assert_eq!(55.6, i);
}

#[test]
fn set_bool_option() {
    let mut ampl = init();
    ampl.set_bool_option("anumber", false);
    let i = ampl.get_bool_option("anumber");
    assert_eq!(false, i);
}

#[test]
fn get_not_existent_bool_option() {
    let mut ampl = init();
    assert_eq!(false, ampl.get_bool_option("notExistent"));
}

#[test]
fn test_regression() {
    let mut ampl = init();
    let option = "abc\n";
    ampl.set_option("test", option);
    assert_eq!(option, ampl.get_option("test"));

    let option = "\nabc\n";
    ampl.set_option("test", option);
    assert_eq!(option, ampl.get_option("test"));

    let option = "abc\"abc";
    ampl.set_option("test", option);
    assert_eq!(option, ampl.get_option("test"));

    let option = "abc\"\nabc";
    ampl.set_option("test", option);
    assert_eq!(option, ampl.get_option("test"));

}

#[test]
fn test_small_numbers() {
    let mut ampl = init();
    ampl.set_dbl_option("presolve_epsmax", -1e-5);
    assert_eq!(-1.0E-5, ampl.get_dbl_option("presolve_epsmax"));
}
