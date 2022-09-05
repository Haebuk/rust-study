fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1
    }

    println!("LIFTOFF!!");

    println!();

    // while indexing
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    println!();

    // for
    for element in a.iter() {
        println!("the value is: {}", element)
    }

    println!();

    // 역순
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
