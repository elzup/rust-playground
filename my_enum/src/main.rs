enum Heating {
    Weak,
    Strong,
}
enum Running {
    Cooling,
    Heating(Heating),
    Dry,
}
enum State {
    Off,
    Running(Running),
}

fn main() {
    // dbg!("{:?}", Disjoint::Alpha(Alpha::A));
    let state = State::Running(Running::Heating(Heating::Strong));

    match state {
        State::Off => println!("a"),
        State::Running(Running::Cooling) => println!("a"),
        State::Running(Running::Heating(Heating::Strong)) => println!("a"),
        State::Running(Running::Heating(_)) => println!("a"),
        State::Running(_) => println!("a"),
        _ => {}
    };
}
