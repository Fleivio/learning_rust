pub mod transition;
pub mod tape;

use transition::{Transition, TransitionTable, Action, State};
use tape::{Tape, Direction};

pub struct TuringMachine <T,D> {
    tr_table : TransitionTable <T,D>,
    tape : Tape<T>,
    current_state : State,
    halt : bool
}

pub type Turing2D <T> = TuringMachine <T, Direction>;
type TransTab2D <T> = TransitionTable<T, Direction>;
type Trans2D <T> = Transition<T, Direction>;
type Action2D <T> = Action<T, Direction>;

impl<T : Copy + PartialEq + Clone> Turing2D<T> {

    pub fn new( base : T, tr_table : TransTab2D<T>, initial_st : State ) -> Turing2D<T> {
        Turing2D {
            tr_table : tr_table,
            tape : Tape::new(base),
            current_state : initial_st,
            halt : false
        }
    }

    fn perform_action ( &mut self, act : &Action2D<T> ) {
        match act {
            Action::Valid { to, write, direction } => {
                self.tape.write(*write);
                self.current_state = to.clone();
                self.tape.shift(direction);
            },
            Action::Fail => {
                self.halt = true;
            }
        }
    }

    pub fn success (&self) -> bool {
        self.halt && self.current_state.is_valid
    }

    fn step(&mut self) {
        let read = self.tape.read();
        let action = Trans2D::search(&self.tr_table, &read, &self.current_state);
        Self::perform_action(self, &action);
    }

    pub fn run(&mut self) {
        while !self.halt {
            Self::step(self);
        }
    }

    pub fn controlled_run( &mut self, max_steps : u64) {
        let mut steps = 0;

        while !self.halt && steps < max_steps {
            Self::step(self);
            steps += 1;
        }
    }

}
