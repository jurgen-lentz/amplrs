use amplrs::{Ampl, Constraint};

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

fn load_diet(ampl: &mut Ampl) {
    ampl.read("../examples/models/diet/diet.mod");
    ampl.read_data("../examples/models/diet/diet.dat");
}

#[test]
fn test_refresh_instances() {
    let mut ampl = init();
    load_diet(&mut ampl);
    let c = ampl.get_constraint("Diet");
    assert_eq!(6, c.num_instances());

    for inst in c.instances() {
        assert_eq!(0.0, inst.body());
    }

    let check = |body: f64, lb: f64, ub: f64, dual: f64, c: &Constraint, s: &str| {
        let inst = c.get(s);
        assert_eq!(dual,  inst.dual());
        assert_eq!(body,  inst.body());
        assert_eq!(lb,    inst.lb());
        assert_eq!(ub,    inst.ub());
    };
    check(0.0,     700.0, 20000.0, 0.0, &c, "A");
    check(0.0,     700.0, 20000.0, 0.0, &c, "B1");
    check(0.0,     700.0, 20000.0, 0.0, &c, "B2");
    check(0.0,     700.0, 20000.0, 0.0, &c, "C");
    check(0.0, 16000.0,  24000.0, 0.0, &c, "CAL");
    check(0.0,       0.0, 50000.0, 0.0, &c, "NA");
}

#[test]
fn test_drop_and_restore() {
    let mut ampl = init();
    ampl.eval("var x{1..5};");
    ampl.eval("subject to c{i in 1..5}: x[i] <= i;");
    ampl.eval("subject to c2{i in 1..5}: x[i] <= 500;");
    ampl.eval("maximize w: sum{i in 1..5} x[i];");

    let x = ampl.get_variable("x");
    let c = ampl.get_constraint("c");

    for inst in x.instances() {
        assert_eq!(0.0, inst.dual());
        assert_eq!(0.0, inst.value());
    }

    ampl.solve("", "");

    // x[i] hits constraint c: x[i] = i
    for inst in x.instances() {
        let key = inst.key()[0].as_f64().unwrap();
        assert_eq!(key, inst.value());
    }

    c.drop();
    ampl.solve("", "");

    // x[i] hits constraint c2: x[i] = 500
    for inst in x.instances() {
        assert_eq!(500.0, inst.value());
    }

    c.restore();
    ampl.solve("", "");

    // x[i] hits constraint c again: x[i] = i
    for inst in x.instances() {
        let key = inst.key()[0].as_f64().unwrap();
        assert_eq!(key, inst.value());
    }

    c.get_int(1).drop();
    ampl.solve("", "");

    // x[1] now hits c2 (500), others still hit c
    for inst in x.instances() {
        let key = inst.key()[0].as_f64().unwrap() as i64;
        if key != 1 {
            assert_eq!(key as f64, inst.value());
        } else {
            assert_eq!(500.0, inst.value());
        }
    }

    c.get_int(1).restore();
    ampl.solve("", "");

    // x[i] hits constraint c again: x[i] = i
    for inst in x.instances() {
        let key = inst.key()[0].as_f64().unwrap();
        assert_eq!(key, inst.value());
    }
}

#[test]
fn test_constant_constraint() {
    let mut ampl = init();
    ampl.eval("var x;");
    ampl.eval("s.t. c2{i in 1..5} : 0;");
    let c2 = ampl.get_constraint("c2");
    assert!(c2.is_logical());
    assert_eq!(5, c2.num_instances());
}

#[test]
fn test_interface() {
    let mut ampl = init();
    ampl.eval("var x;");
    ampl.eval("s.t. c: x <= 5;");
    let c = ampl.get_constraint("c");
    let ci = c.get_scalar();

    assert_eq!(c.get_scalar().astatus(), ci.astatus());
    assert_eq!(c.get_scalar().body(),    ci.body());
    assert_eq!(c.get_scalar().defvar(),  ci.defvar());
    assert_eq!(c.get_scalar().dinit(),   ci.dinit());
    assert_eq!(c.get_scalar().dinit0(),  ci.dinit0());
    assert_eq!(c.get_scalar().dual(),    ci.dual());
    assert_eq!(c.get_scalar().lb(),      ci.lb());
    assert_eq!(c.get_scalar().ub(),      ci.ub());
    assert_eq!(c.get_scalar().lbs(),     ci.lbs());
    assert_eq!(c.get_scalar().ubs(),     ci.ubs());
    assert_eq!(c.get_scalar().ldual(),   ci.ldual());
    assert_eq!(c.get_scalar().udual(),   ci.udual());
    assert_eq!(c.get_scalar().lslack(),  ci.lslack());
    assert_eq!(c.get_scalar().uslack(),  ci.uslack());
    assert_eq!(c.get_scalar().slack(),   ci.slack());
    assert_eq!(c.get_scalar().sstatus(), ci.sstatus());
    assert_eq!(c.get_scalar().status(),  ci.status());
}

#[test]
#[should_panic]
fn test_constant_constraint_body() {
    let mut ampl = init();
    ampl.eval("s.t. c: 0;");
    ampl.get_constraint("c").get_scalar().body();
}

#[test]
fn test_logical_constraint() {
    let mut ampl = init();
    ampl.eval("var x; s.t. c: (x != 0);");
    ampl.get_constraint("c").get_scalar().val();
}

#[test]
fn test_logical_val() {
    let mut ampl = init();
    ampl.eval("var x; s.t. c: (x != 0);");
    let c = ampl.get_constraint("c");
    assert_eq!(0.0, c.get_scalar().val());
    ampl.get_variable("x").get_scalar().set_value(1.0);
    assert_eq!(1.0, c.get_scalar().val());
}

#[test]
fn test_get_values() {
    let mut ampl = init();
    ampl.eval("var x{i in 1..10} := i; maximize z: sum{i in 1..10} i*x[i]; c{i in 1..10}: x[i] <= i/2;");
    let c = ampl.get_constraint("c");
    ampl.solve("", "");
    let df1 = c.get_values();
    let df2 = c.get_values_with(&["dual"]);
    assert_eq!(df1.num_rows(), df2.num_rows());
}

#[test]
fn test_set_dual() {
    let mut ampl = init();
    ampl.eval("var x; s.t. c: x >= 0;");
    let c = ampl.get_constraint("c");
    c.set_dual(42.0);
    // Presolve resets dual to 0
    assert_eq!(0.0, c.get_scalar().dual());
}
