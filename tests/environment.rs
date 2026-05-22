use amplrs::Environment;

//#[test]
//fn using_xampl() {
//    let environment = Environment::new("", "");
//    assert_eq!("x-ampl", environment.get_bin_name());
//}

#[test]
fn initializes_correctly() {
    let mut environment = Environment::new("binary_location", "");
    assert_eq!("binary_location", environment.get_bin_dir());
    environment = Environment::new("binary_location", "binary_name");
    assert_eq!("binary_location", environment.get_bin_dir());
    assert_eq!("binary_name", environment.get_bin_name());
    environment.set_bin_dir("binary_location_2");
    assert_eq!("binary_location_2", environment.get_bin_dir());
    environment.set_bin_name("binary_name_2");
    assert_eq!("binary_name_2", environment.get_bin_name());
}
