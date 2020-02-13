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


// unions
// use the same memory location to store different data types; seems almost completely pointless
// considering how cheap memory is. One thing to note with when borrowing a union field is that
// all fields of the union will technically be borrowed for the same lifetime.

union MassiveSaving {
    i: u8,
    f: f32
}

fn unions() {
    let mut my_union = MassiveSaving{i: 1};
    my_union.i = 200;

    let val = unsafe { my_union.i };
    println!("Val {}", val);

    // we can also pattern match against unions; though returning clojures as a result (which is a nice pattern)
    // is hacky af. See: https://github.com/rust-lang/rust/issues/24036#issuecomment-89509870

    fn match_u(x : MassiveSaving) {
        unsafe {
            match x {
                MassiveSaving {i: 123} => {println!("123!!")}
                MassiveSaving {i} => {println!("Integer!")},
                MassiveSaving {f} => {println!("Float!")},
            }
        }
    }
    match_u(MassiveSaving{i: 84});
}


// Option<T> is an enum included with the standard library and prelude; that is to say that it's
// without importing it. That goes for its constituents too, Some(T) or None.
// Option<T> is defined as
// enum Option<T> {
//     Some(T),
//     None
// }
// the <T> syntax is a generic type parameter. It means data of any type can be present in Some.
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

// n order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.
// Then, when you use that value, you are required to explicitly handle the case when the value is null.
// https://doc.rust-lang.org/std/option/enum.Option.html
fn an_option() {
    let mut y: Option<u8> = Some(5);

    fn process_y(y: Option<u8>) {
        match y {
            Some(y) => {println!("y is present and its value is {}", y)}
            None => {println!("y is None!")}
        }
    }
    
    process_y(y);
    y = None;
    process_y(y);


}

fn main() {
    age_check(15);
    loops();
    match_case_statement("order_create");
    structures();
    unions();
    an_option();

}
