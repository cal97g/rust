// control flows
use std::fmt;

fn age_check(age: i32) {
    if age >= 18 {
        println!("Can buy Beer")
    } else {
        println!("Can buy Coke Zero!")
    }
}

fn loops() {
    let mut number = 1;

    while number < 1000 {
        number *= 2;
        println!("number = {}", number);
    }

    // for x in range.. 100
    for x in 1..101 {
        println!("{}", x);

        if x == 8 {
            break;
        }
    }

    let mut age = 0;
    // while
    loop {
        age += 1;
        if age > 88 {
            println!("rip");
            break;
        }
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}, {}", pos, y)
    }
}

fn match_case_statement(event: &str) {
    // apparently rust also implements erlang inspired pattern matching 👀
    let translated_type: &str = match event {
        "order_create" => "OrderCreate",
        "order_update" => "OrderUpdate",
        _ => "UnkownEvent"
    };

    // error[E0658]: exclusive range pattern syntax is experimental; (have to enable nightly)
    // let templos : &str = match number {
    //     1..665 => "HolyRng",
    //     666 => "EvilRng",
    //     1..999 => "BlessedRng",
    //     _ => "ScrambledRng"
    // };

    println!("{}", translated_type);
    // println!("{}", templos);
}

// I don't really understand what's going on here, I guess I'm inheriting this functionality for
// my struct? Need to understand why this is required, e.g why p1 can't be referenced in two places.
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 1, y: 1 };
    let p3 = Point { x: 2, y: 2 };

    let myline = Line { start: p1, end: p2 };

    println!("{}", myline.start);
    println!("<Point {}, {}>", p1.x, p1.y);
}

// enums

enum FootballTeams {
    Wolves,
    ManUtd,
    Liverpool,
    Tottenham
}


fn main() {
    age_check(15);
    loops();
    match_case_statement("order_create");
    structures();
}
