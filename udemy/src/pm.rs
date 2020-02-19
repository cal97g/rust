fn how_many(count: u16) -> &'static str {
    match count {
        0 => "No",
        1 | 2 => "one or two",
        6 => "half a dozen",
        12 => "a dozen",
        //_ if (x % == 0) => "an even number", //(ths should work according to udemy guy)
        7..=11 => "inbedozen",
        _ => "an unkown number of"
    }
}


pub fn patternmatching() -> &'static str {
    for x in 0..15 {
        println!("I have {} Oranges!", how_many(x));
    }

    let point = (0,0);

    match point {
        (0,0) => "You haven't moved!",
        (1..=100, 1..=100) => "Noob adventurer",
        (101..=1000, 101..=1000) => "Good adventurer",
        (_,_) => "This minecraft autogenration is lit",
    }
}
