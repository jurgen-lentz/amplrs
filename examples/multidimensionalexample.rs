use amplrs::ampl::Ampl;
use amplrs::dataframe::DataFrame;

fn main() {
    let mut ampl = Ampl::new();

    ampl.eval("set CITIES; set LINKS within (CITIES cross CITIES);");
    ampl.eval("param cost {LINKS} >= 0; param capacity {LINKS} >= 0;");
    ampl.eval("data; set CITIES := PITT NE SE BOS EWR BWI ATL MCO;");

    let cost = [2.5, 3.5, 1.7, 0.7, 1.3, 1.3, 0.8, 0.2, 2.1];
    let capacity = [250.0, 250.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0];
    let links_from = ["PITT", "PITT", "NE", "NE", "NE", "SE", "SE", "SE", "SE"];
    let links_to = ["NE", "SE", "BOS", "EWR", "BWI", "EWR", "BWI", "ATL", "MCO"];

    let df = DataFrame::new(2, 2, &["LINKSFrom", "LINKSTo", "cost", "capacity"]);
    df.set_column_strings("LINKSFrom", &links_from);
    df.set_column_strings("LINKSTo", &links_to);
    df.set_column_doubles("cost", &cost);
    df.set_column_doubles("capacity", &capacity);
    println!("{}", df.to_string());

    ampl.set_data(&df, Some("LINKS"));
}