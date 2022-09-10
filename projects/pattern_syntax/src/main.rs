fn main() {
    // literal matching

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // naming variabl matching
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"), // pass
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple pattern

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ... 이용한 값의 범위 매칭
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }

    // destructing struct

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // destructing reference

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    // match guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five"),
        Some(x) => println!("{}", x),
        None => (),
    }
}
