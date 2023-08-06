use turing::transition::*;
use turing::tape::*;
use turing::*;
mod turing;



fn main() {
    let q0: State = State::new(String::from("0"), false);
    let q1: State = State::new(String::from("1"), true);

    let mut tr_table: TransitionTable<char, Direction> = Vec::new();

    tr_table.push(Transition::new(
    q0.clone(), '0', Action::Valid {
        to: q1.clone(),
        write: '1',
        direction: Direction::Right,
    }));

    let mut turing = Turing2D::new('1', tr_table, q0); 

    turing.run();

    println!("Success: {}", turing.success());

}
