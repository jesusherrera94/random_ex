enum State {
    Start,
    GotA,
    Accept,
    Reject,
}

fn transition(state: State, input: char) -> State {
    match (state, input) {
        (State::Start, 'a') => State::GotA,
        (State::GotA, 'b') => State::GotA,
        (State::GotA, 'c') => State::Accept,
        (State::Accept, _) => State::Reject,
        _ => State::Reject,
    }
}


pub fn recognize_pattern(input: &str) -> bool {
    let mut state = State::Start;
    for c in input.chars() {
        state = transition(state, c);
    }
    matches!(state, State::Accept)
}

pub fn main() {
    let result = recognize_pattern("abbbc");
    assert_eq!(result, true);
    
    let result = recognize_pattern("ac");
    assert_eq!(result, true);
    
    let result = recognize_pattern("abbbd");
    assert_eq!(result, false);
    
    let result = recognize_pattern("");
    assert_eq!(result, false);
    println!("All tests passed");
}