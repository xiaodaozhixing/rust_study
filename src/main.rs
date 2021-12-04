fn main() {
    print_test()
}

fn print_test() {
    let value0 = "value0";
    let value1 = "value1";
    print!("print {}", value0);
    println!("println value0: {}", value0);
    println!("println value0: {} value0: {0}", value0);
    println!("println value0: {0} value0: {}", value0);
    println!("println value0: {0} value1: {1}", value0, value1);
    println!("println value0: {} value1: {1}", value0, value1);
    println!("println value1: {1} value0: {}", value0, value1);
}
