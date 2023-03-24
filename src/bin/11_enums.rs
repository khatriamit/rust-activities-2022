// ------------------------------------------------------
//     - Enums
//          - General Syntax
//          - Eum with attached data
//
// ------------------------------------------------------

enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
            Conveyance::Train(miles) => *miles as f32 * 24.0 * 2.0,
            Conveyance::Air(miles) => *miles as f32 * 54.0 * 2.0,
        };
        allowance
    }
}

fn main() {
    let participant1 = Conveyance::Train(60);
    // println!("The value of option is {}", participant1 as i32);

    let participant2 = Conveyance::Car(60);
    // println!("The value of option is {}", participant2 as i32);

    let participant3 = Conveyance::Air(30);
    // println!("The value of option is {}", participant3 as i32);

    println!(
        "The participant 1 has travel allowance of {}",
        participant1.travel_allowance()
    );
    println!(
        "The participant 2 has travel allowance of {}",
        participant2.travel_allowance()
    );
    println!(
        "The participant 3 has travel allowance of {}",
        participant3.travel_allowance()
    );
}
