use amplrs::{Ampl, DataFrame, Value};

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

macro_rules! assert_near {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y).abs() < $d,
            "expected |{} - {}| < {}",
            $x,
            $y,
            $d
        );
    };
}

#[test]
fn set_value_test() {
    let df = DataFrame::new(2, 2, &["index1", "index2", "v1", "v2"]);

    let i1 = [1.0, 1.0, 2.0, 2.0];
    let i2 = ["1", "2", "1", "2"];
    let v = [0.0, 0.0, 0.0, 0.0];

    df.set_column_doubles("index1", &i1);
    df.set_column_strings("index2", &i2);
    df.set_column_doubles("v1", &v);
    df.set_column_doubles("v2", &v);

    df.set_value(0, 2, &Value::Numeric(1.0));
    df.set_value(1, 2, &Value::Numeric(2.0));
    df.set_value(0, 3, &Value::Text("1".to_string()));
    df.set_value(1, 3, &Value::Text("2".to_string()));

    let t1 = [Value::Numeric(2.0), Value::Text("1".to_string())];
    let t2 = [Value::Numeric(2.0), Value::Text("2".to_string())];

    df.set_value_at(&t1, "v1", &Value::Numeric(5.0));
    df.set_value_at(&t2, "v1", &Value::Numeric(6.0));
    df.set_value_at(&t1, "v2", &Value::Text("5".to_string()));
    df.set_value_at(&t2, "v2", &Value::Text("6".to_string()));

    assert_eq!(1.0, df.get_row_by_index(0)[2].as_f64().unwrap());
    assert_eq!(2.0, df.get_row_by_index(1)[2].as_f64().unwrap());
    assert_eq!("1", df.get_row_by_index(0)[3].as_str().unwrap());
    assert_eq!("2", df.get_row_by_index(1)[3].as_str().unwrap());

    assert_eq!(5.0, df.get_row_by_index(2)[2].as_f64().unwrap());
    assert_eq!(6.0, df.get_row_by_index(3)[2].as_f64().unwrap());
    assert_eq!("5", df.get_row_by_index(2)[3].as_str().unwrap());
    assert_eq!("6", df.get_row_by_index(3)[3].as_str().unwrap());
}

#[test]
fn add_multiple_values_to_not_indexed_table() {
    let df = DataFrame::new(0, 0, &[]);
    let c = [4.0, 5.0];
    df.add_column_doubles("D", &c);
}

#[test]
fn locale_test() {
    let df = DataFrame::new(1, 1, &["INDEX", "value"]);
    df.add_row(&[Value::Text("A".to_string()), Value::Numeric(34.5)]);
    assert!(df.to_string().contains("34.5"));
}

#[test]
fn row_iterator_test() {
    let mut ampl = init();
    ampl.eval("param p{i in 1..1000} := i;");
    let df = ampl.get_data(&["p"]);
    let mut i = 0;
    for row in df.rows() {
        i += 1;
        assert_eq!(i as f64, row[1].as_f64().unwrap());
    }
    assert_eq!(1000, i);
}

#[test]
fn get_data_multi() {
    let mut ampl = init();
    ampl.eval("param p1{i in 1..10} := 1*i;");
    ampl.eval("param p2{i in 1..10} := 2*i;");
    ampl.eval("param p3{i in 0..10: i >= 1} := 3*i;");
    let df = ampl.get_data(&["1..10", "p1", "p2", "p3"]);
    let mut i = 0;
    for row in df.rows() {
        i += 1;
        assert_eq!(i as f64, row[0].as_f64().unwrap());
        assert_eq!((1 * i) as f64, row[1].as_f64().unwrap());
        assert_eq!((2 * i) as f64, row[2].as_f64().unwrap());
        assert_eq!((3 * i) as f64, row[3].as_f64().unwrap());
    }
    assert_eq!(10, i);
}

#[test]
fn basic_test() {
    let df = DataFrame::new(1, 2, &["uno", "due", "tre"]);
    assert_eq!(1, df.num_indices());
    assert_eq!(3, df.num_cols());
    assert_eq!(0, df.num_rows());
    df.add_row_doubles(&[1.0, 4.0, 5.0]);
    assert_eq!(1, df.num_rows());
    df.add_row_doubles(&[2.0, 4.0, 5.0]);
    assert_eq!(2, df.num_rows());
}

#[test]
fn get_data_test() {
    let mut ampl = init();
    ampl.eval("param p{i in 1..1000} := i;");
    let df = ampl.get_data(&["p"]);
    assert_eq!(1000, df.num_rows());
}

#[test]
fn data_frame_from_entity_with_commas_in_name() {
    let mut ampl = init();
    ampl.eval("var x; minimize o: x;");
    let df = ampl.get_data(&["{i in 1..(_nvars)} (_varname[i], _var[i].val)"]);
    assert_eq!(1, df.num_rows());
}

#[test]
fn get_data_from_entity_test() {
    let mut ampl = init();
    ampl.eval("var x{i in 1..10} >= -i, <= i;");
    let x = ampl.get_variable("x");
    let df = x.get_values();
    assert_eq!(10, df.num_rows());
    for i in 1..=10_usize {
        let row = df.get_row_by_index(i - 1);
        assert_eq!(i as f64, row[0].as_f64().unwrap());
    }
}

#[test]
fn one_and_one() {
    let df = DataFrame::new(1, 1, &["Products", "Profit"]);
    df.add_row(&[
        Value::Text("bands".to_string()),
        Value::Text("25".to_string()),
    ]);
    assert_eq!(1, df.num_rows());
    df.add_row(&[
        Value::Text("coils".to_string()),
        Value::Text("30".to_string()),
    ]);
    assert_eq!(2, df.num_rows());

    let row = df.get_row(&[Value::Text("bands".to_string())]);
    assert_eq!("bands", row[0].as_str().unwrap());
    assert_eq!("25", row[1].as_str().unwrap());
}

#[test]
#[should_panic]
fn one_and_one_wrong_index() {
    let df = DataFrame::new(1, 1, &["Products", "Profit"]);
    df.add_row_doubles(&[1.0, 25.0]);
    df.add_row_doubles(&[2.0, 30.0]);
    df.get_row(&[Value::Text("bandas".to_string())]);
}

#[test]
#[should_panic]
fn one_and_one_same_index_two() {
    let df = DataFrame::new(1, 1, &["Products", "Profit"]);
    df.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
}

#[test]
#[should_panic]
fn multiple_wrong_index() {
    let df = DataFrame::new(2, 2, &["Products", "Products2", "Cost", "Profit"]);
    df.add_row_strings(&["bands", "bands", "0.0", "25.0"]);
    df.add_row_strings(&["coils", "bands", "0.0", "25.0"]);
    df.get_row(&[
        Value::Text("bands".to_string()),
        Value::Text("bananas".to_string()),
    ]);
}

#[test]
#[should_panic]
fn multiple_wrong_index_cardinality() {
    let df = DataFrame::new(2, 2, &["Products", "Products2", "Cost", "Profit"]);
    df.add_row_strings(&["bands", "bands", "0.0", "25.0"]);
    df.add_row_strings(&["coils", "bands", "0.0", "25.0"]);
    df.get_row(&[Value::Text("bands".to_string())]);
}

#[test]
fn scalar() {
    let df = DataFrame::new(0, 2, &["ascalar", "asecondscalar"]);
    df.add_row_doubles(&[2.0, 4.0]);
    assert_eq!(4.0, df.get_row_by_index(0)[1].as_f64().unwrap());
    assert_eq!(4.0, df.get_row(&[])[1].as_f64().unwrap());
}

#[test]
#[should_panic]
fn scalar_multiple_add() {
    let df = DataFrame::new(0, 2, &["ascalar", "asecondscalar"]);
    df.add_row_doubles(&[2.0, 4.0]);
    df.add_row_doubles(&[2.0, 4.0]);
}

#[test]
#[should_panic]
fn scalar_wrong_cardinality() {
    let df = DataFrame::new(0, 2, &["ascalar", "asecondscalar"]);
    df.add_row_doubles(&[2.0, 4.0]);
    df.get_row(&[Value::Text("noindex!".to_string())]);
}

#[test]
#[should_panic]
fn test_header_uniqueness() {
    DataFrame::new(0, 2, &["ascalar", "ascalar"]);
}

#[test]
fn victor_specs() {
    let mut ampl = init();

    // With set assignment
    ampl.eval("set Products; param Profit{Products};");
    let df = DataFrame::new(1, 1, &["Products", "Profit"]);
    df.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);
    ampl.set_data(&df, Some("Products"));
    let result = ampl.get_data(&["Products", "Profit"]);
    let row = result.get_row(&[Value::Text("bands".to_string())]);
    assert_eq!(25.0, row[1].as_f64().unwrap());
    ampl.reset();

    // Without set assignment (set already declared in model)
    ampl.eval("set Products; param Profit{Products};");
    ampl.eval("data; set Products := bands coils; model;");
    let df2 = DataFrame::new(1, 1, &["Products", "Profit"]);
    df2.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df2.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);
    ampl.set_data(&df2, None);
    let result2 = ampl.get_data(&["Products", "Profit"]);
    let row2 = result2.get_row(&[Value::Text("bands".to_string())]);
    assert_eq!(25.0, row2[1].as_f64().unwrap());
    ampl.reset();

    // With set assignment again
    ampl.eval("set Products; param Profit{Products};");
    let df3 = DataFrame::new(1, 1, &["Products", "Profit"]);
    df3.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df3.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);
    ampl.set_data(&df3, Some("Products"));
    let result3 = ampl.get_data(&["Products", "Profit"]);
    let row3 = result3.get_row(&[Value::Text("bands".to_string())]);
    assert_eq!(25.0, row3[1].as_f64().unwrap());
}

#[test]
fn get_column() {
    let ss1values: Vec<String> = (0..10).map(|i| format!("index{}", i)).collect();
    let ss1values_ptr: Vec<&str> = ss1values.iter().map(|s| s.as_str()).collect();
    let pp1values: Vec<f64> = (0..10).map(|i| i as f64).collect();
    let pp2values: Vec<String> = (0..10).map(|i| format!("value{}", i)).collect();
    let pp2values_ptr: Vec<&str> = pp2values.iter().map(|s| s.as_str()).collect();

    let df = DataFrame::new(1, 2, &["Products", "Profit", "Profit2"]);
    df.set_column_strings("Products", &ss1values_ptr);
    df.set_column_doubles("Profit", &pp1values);
    df.set_column_strings("Profit2", &pp2values_ptr);

    let col0 = df.get_column("Products");
    let col1 = df.get_column("Profit");
    let col2 = df.get_column("Profit2");

    for i in 0..10 {
        assert_eq!(ss1values_ptr[i], col0[i].as_str().unwrap());
        assert_eq!(pp1values[i], col1[i].as_f64().unwrap());
        assert_eq!(pp2values_ptr[i], col2[i].as_str().unwrap());
    }
}

#[test]
fn indexing_order() {
    let mut ampl = init();
    ampl.eval("set ASSETS; param prices{ASSETS}; data;");
    ampl.eval(
        "set ASSETS := asset2 asset1 asset3; \
         param prices := asset2 2 asset1 1 asset3 3; model;",
    );
    let df = ampl.get_data(&["prices"]);
    for i in 1..=3 {
        let row = df.get_row(&[Value::Text(format!("asset{}", i))]);
        assert_eq!(i as f64, row[1].as_f64().unwrap());
    }
}

#[test]
fn set_data_multidimensional() {
    let mut ampl = init();
    ampl.eval(
        "set NODE = {91, 366}; \
         set PIPE within {1..10000} cross NODE cross NODE; \
         param PIPE_MinFlow{PIPE}; param PIPE_MaxFlow{PIPE}; \
         param PIPE_Roughness{PIPE}; param PIPE_Length{PIPE}; param PIPE_Diameter{PIPE};",
    );
    let df = DataFrame::new(
        3,
        5,
        &[
            "index0",
            "index1",
            "index2",
            "PIPE_MinFlow",
            "PIPE_MaxFlow",
            "PIPE_Roughness",
            "PIPE_Length",
            "PIPE_Diameter",
        ],
    );
    df.add_row(&[
        Value::Numeric(370.0),
        Value::Numeric(366.0),
        Value::Numeric(91.0),
        Value::Numeric(1.0),
        Value::Numeric(10000.0),
        Value::Numeric(0.06),
        Value::Numeric(10.0),
        Value::Numeric(0.15),
    ]);

    let key = [
        Value::Numeric(370.0),
        Value::Numeric(366.0),
        Value::Numeric(91.0),
    ];
    assert_eq!(10000.0, df.get_row(&key)[4].as_f64().unwrap());

    ampl.set_data(&df, Some("PIPE"));
    let pipe_df = ampl.get_data(&["PIPE"]);
    assert_eq!(1, pipe_df.num_rows());
}

#[test]
fn add_columns() {
    let df = DataFrame::new(1, 0, &["Products"]);
    df.add_row(&[Value::Text("bands".to_string())]);
    df.add_row(&[Value::Text("coils".to_string())]);
    let to_add = [25.0, 30.0];
    df.add_column_doubles("Profit", &to_add);

    let row = df.get_row(&[Value::Text("coils".to_string())]);
    assert_eq!(30.0, row[1].as_f64().unwrap());

    // set_column version
    let df2 = DataFrame::new(1, 1, &["Products", "Profit"]);
    let index_to_add = ["bands", "coils"];
    df2.set_column_strings("Products", &index_to_add);
    df2.set_column_doubles("Profit", &to_add);

    let row2 = df2.get_row(&[Value::Text("bands".to_string())]);
    assert_eq!(25.0, row2[1].as_f64().unwrap());
}

#[test]
fn get_column_str() {
    let ss1values: Vec<String> = (0..10).map(|i| format!("index{}", i)).collect();
    let ss1values_ptr: Vec<&str> = ss1values.iter().map(|s| s.as_str()).collect();
    let pp1values: Vec<f64> = (0..10).map(|i| i as f64).collect();

    let df = DataFrame::new(1, 1, &["Products", "Profit"]);
    df.set_column_strings("Products", &ss1values_ptr);
    df.set_column_doubles("Profit", &pp1values);

    assert_eq!(10, df.get_column("Products").len());
}

#[test]
fn deep_equality_test() {
    let df = DataFrame::new(2, 0, &["a", "b"]);
    df.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);

    let df2 = DataFrame::new(2, 0, &["a", "b"]);
    df2.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df2.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);

    let df3 = DataFrame::new(2, 0, &["a", "b"]);
    df3.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df3.add_row(&[Value::Text("coils".to_string()), Value::Numeric(31.0)]);

    let df4 = DataFrame::new(2, 0, &["a", "c"]);
    df4.add_row(&[Value::Text("bands".to_string()), Value::Numeric(25.0)]);
    df4.add_row(&[Value::Text("coils".to_string()), Value::Numeric(30.0)]);

    assert!(df == df2);
    assert!(df != df4);
    assert!(df != df3);
}

#[test]
fn diet_test() {
    let mut ampl = init();

    let df1 = DataFrame::new(1, 2, &["NUTR", "n_min", "n_max"]);
    df1.add_row(&[Value::Text("A".to_string()), Value::Numeric(700.0), Value::Numeric(20000.0)]);
    df1.add_row(&[Value::Text("B1".to_string()), Value::Numeric(700.0), Value::Numeric(20000.0)]);
    df1.add_row(&[Value::Text("B2".to_string()), Value::Numeric(700.0), Value::Numeric(20000.0)]);
    df1.add_row(&[Value::Text("C".to_string()), Value::Numeric(700.0), Value::Numeric(20000.0)]);
    df1.add_row(&[Value::Text("CAL".to_string()), Value::Numeric(16000.0), Value::Numeric(24000.0)]);
    df1.add_row(&[Value::Text("NA".to_string()), Value::Numeric(0.0), Value::Numeric(50000.0)]);

    let foods = ["BEEF", "CHK", "FISH", "HAM", "MCH", "MTL", "SPG", "TUR"];
    let df2 = DataFrame::new(1, 3, &["FOOD", "f_min", "f_max", "cost"]);
    df2.set_column_strings("FOOD", &foods);
    df2.set_column_doubles("f_min", &[2.0; 8]);
    df2.set_column_doubles("f_max", &[10.0; 8]);
    df2.set_column_doubles("cost", &[3.19, 2.59, 2.29, 2.89, 1.89, 1.99, 1.99, 2.49]);

    let nutrients = ["A", "B1", "B2", "C", "CAL", "NA"];
    let nutr_col: Vec<&str> = nutrients
        .iter()
        .flat_map(|&n| std::iter::repeat(n).take(8))
        .collect();
    let food_col: Vec<&str> = nutrients
        .iter()
        .flat_map(|_| foods.iter().copied())
        .collect();

    #[rustfmt::skip]
    let values = [
         60.0,   8.0,   8.0,  40.0,   15.0,   70.0,   25.0,   60.0,  // A
         10.0,  20.0,  15.0,  35.0,   15.0,   15.0,   25.0,   15.0,  // B1
         15.0,  20.0,  10.0,  10.0,   15.0,   15.0,   15.0,   10.0,  // B2
         20.0,   0.0,  10.0,  40.0,   35.0,   30.0,   50.0,   20.0,  // C
        295.0, 770.0, 440.0, 430.0,  315.0,  400.0,  370.0,  450.0,  // CAL
        968.0,2180.0, 945.0, 278.0, 1182.0,  896.0, 1329.0, 1397.0,  // NA
    ];

    let df3 = DataFrame::new(2, 1, &["NUTR", "FOOD", "amt"]);
    df3.set_column_strings("NUTR", &nutr_col);
    df3.set_column_strings("FOOD", &food_col);
    df3.set_column_doubles("amt", &values);

    ampl.read("../examples/models/diet/diet.mod");
    ampl.set_data(&df1, Some("NUTR"));
    ampl.set_data(&df2, Some("FOOD"));
    ampl.set_data(&df3, None);
    ampl.solve("", "");
    ampl.solve("", "");

    assert_near!(118.0, ampl.get_objective("Total_Cost").value(), 1.0);
}

#[test]
fn set_array_from_double() {
    let df = DataFrame::new(1, 1, &["NUTR", "Amount"]);
    let nutrients = ["A", "C", "B1", "B2", "NA", "CAL"];
    let amounts = [60.0, 8.0, 8.0, 40.0, 15.0, 70.0];
    df.set_array(&nutrients, &amounts);
    for j in 0..6 {
        let row = df.get_row(&[Value::Text(nutrients[j].to_string())]);
        assert_eq!(amounts[j], row[1].as_f64().unwrap());
    }
}

#[test]
fn multi_dimensional_bug() {
    let mut ampl = init();
    ampl.eval("var SPOT_Status{1..2, 1..2};");
    let df = ampl.get_data(&["{j in 1..2} SPOT_Status[1,j]"]);
    assert_eq!(0.0, df.get_row(&[Value::Numeric(1.0)])[1].as_f64().unwrap());
    assert_eq!(0.0, df.get_row(&[Value::Numeric(2.0)])[1].as_f64().unwrap());
}

#[test]
fn set_matrix_from_double() {
    let df = DataFrame::new(2, 1, &["NUTR", "FOOD", "Amount"]);
    let foods = ["BEEF", "CHK", "FISH", "HAM", "MCH", "MTL", "SPG", "TUR"];
    let nutrients = ["A", "C", "B1", "B2", "NA", "CAL"];
    #[rustfmt::skip]
    let amounts: &[f64] = &[
         60.0,  8.0,  8.0,  40.0,  15.0,  70.0,  25.0,  60.0,  // A
         20.0,  0.0, 10.0,  40.0,  35.0,  30.0,  50.0,  20.0,  // C
         10.0, 20.0, 15.0,  35.0,  15.0,  15.0,  25.0,  15.0,  // B1
         15.0, 20.0, 10.0,  10.0,  15.0,  15.0,  15.0,  10.0,  // B2
        928.0,2180.0,945.0, 278.0,1182.0, 896.0,1329.0,1397.0,  // NA
        295.0, 770.0,440.0, 430.0, 315.0, 400.0, 379.0, 450.0,  // CAL
    ];
    df.set_matrix_doubles(&nutrients, &foods, amounts);
    for (i, &nutr) in nutrients.iter().enumerate() {
        for (j, &food) in foods.iter().enumerate() {
            let row = df.get_row(&[Value::Text(nutr.to_string()), Value::Text(food.to_string())]);
            assert_eq!(amounts[i * foods.len() + j], row[2].as_f64().unwrap());
        }
    }
}

#[test]
fn set_matrix_from_string() {
    let df = DataFrame::new(2, 1, &["NUTR", "FOOD", "Amount"]);
    let foods = ["BEEF", "CHK", "FISH", "HAM", "MCH", "MTL", "SPG", "TUR"];
    let nutrients = ["A", "C", "B1", "B2", "NA", "CAL"];
    #[rustfmt::skip]
    let amounts: &[&str] = &[
        "60",  "8",   "8",  "40",  "15",  "70",  "25",  "60",
        "20",  "0",  "10",  "40",  "35",  "30",  "50",  "20",
        "10", "20",  "15",  "35",  "15",  "15",  "25",  "15",
        "60",  "8",   "8",  "40",  "15",  "70",  "25",  "60",
        "20",  "0",  "10",  "40",  "35",  "30",  "50",  "20",
        "10", "20",  "15",  "35",  "15",  "15",  "25",  "15",
    ];
    df.set_matrix_strings(&nutrients, &foods, amounts);
    for (i, &nutr) in nutrients.iter().enumerate() {
        for (j, &food) in foods.iter().enumerate() {
            let row = df.get_row(&[Value::Text(nutr.to_string()), Value::Text(food.to_string())]);
            assert_eq!(amounts[i * foods.len() + j], row[2].as_str().unwrap());
        }
    }
}

#[test]
fn to_string_with_empty_items() {
    let df = DataFrame::new(1, 1, &["one", "two"]);
    let _ = df.to_string();
    df.add_row_doubles(&[1.0, 2.0]);
    let _ = df.to_string();
    df.add_empty_column("three");
    let _ = df.to_string();
}
