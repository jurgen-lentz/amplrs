use amplrs::ampl::*;
use amplrs::environment::*;
use amplrs::suffix::Numericsuffix::Ub;

fn main() {
    let mut ampl = Ampl::new();
    let mut environment = Environment::new("binary_location", "");
    println!("Environment: {}", environment.get_bin_dir());

    ampl.eval("set NUTR;
               set FOOD;
    
               param cost {FOOD} > 0;
               param f_min {FOOD} >= 0;
               param f_max {j in FOOD} >= f_min[j];
    
               param n_min {NUTR} >= 0;
               param n_max {i in NUTR} >= n_min[i];
    
               param amt {NUTR,FOOD} >= 0;
    
               var Buy {j in FOOD} >= f_min[j], <= f_max[j];
    
               minimize Total_Cost:  sum {j in FOOD} cost[j] * Buy[j];
    
               subject to Diet {i in NUTR}:
                   n_min[i] <= sum {j in FOOD} amt[i,j] * Buy[j] <= n_max[i];

               data;
               set NUTR := A C B1 B2 NA CAL;
               set FOOD := BEEF CHK FISH HAM MCH MTL SPG TUR ;
               param : cost f_min f_max :=
                            BEEF 3.19 2 10
                            CHK 2.59 2 10
                            FISH 2.29 2 10
                            HAM 2.89 2 10
                            MCH 1.89 2 10
                            MTL 1.99 2 10
                            SPG 1.99 2 10
                            TUR 2.49 2 10 ;
               param : n_min n_max :=
                            A 700 20000
                            C 700 20000
                            B1 700 20000
                            B2 700 20000
                            NA 0 50000
                            CAL 16000 24000 ;
               param amt (tr):
                            A C B1 B2 NA CAL :=
                            BEEF 60 20 10 15 938 295
                            CHK 8 0 20 20 2180 770
                            FISH 8 10 15 10 945 440
                            HAM 40 40 35 10 278 430
                            MCH 15 35 15 15 1182 315
                            MTL 70 30 15 15 896 400
                            SPG 25 50 25 15 1329 370
                            TUR 60 20 15 10 1397 450 ;");

    ampl.set_option("solver","cplex");
    let _obj = ampl.get_current_objective();
    let solver = ampl.get_option("solver");

    println!("Solver: {}", solver);

    let vars = ampl.get_variables();

    for var in vars {
        var.print();
        println!("Value: {}", var.indexarity());
        let var_instances = var.instances();
        for var_instance in var_instances {
            println!("Instance: {}", var_instance.to_string());
            println!("Instance Name: {}", var_instance.dbl_suffix(Ub));
        }
    }

    let params = ampl.get_parameters();
    for param in params {
        param.print();
        println!("Value: {}", param.indexarity());
    }

    let objs = ampl.get_objectives();
    for obj in objs {
        obj.print();
        println!("Value: {}", obj.indexarity());
    }

    let sets = ampl.get_sets();
    for set in sets {
        set.print();
        println!("Value: {}", set.indexarity());
    }

    let conss = ampl.get_constraints();

    for cons in conss {
        cons.print();
        println!("Value: {}", cons.indexarity());
    }

    ampl.solve("","gurobi");
}
