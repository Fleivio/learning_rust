#[derive(PartialEq, Clone)]
pub struct State {
    name : String,
    pub is_valid : bool
}

impl State {
    pub fn new(name : String, isValid : bool) -> State {
        State {
            name : name,
            is_valid : isValid
        }
    }
}

#[derive(Clone)]
pub enum Action <T,A> {
    Valid { to : State,
            write : T,
            direction : A,
        },
    Fail
}

pub struct Transition <T,A> {
    from : State,
    read : T,
    action : Action<T,A>
}

pub type TransitionTable<T,A> = Vec<Transition<T,A>>;

impl<T : PartialEq + Clone, A : Clone> Transition<T,A> {
    
    pub fn new(s1 : State, char_r : T, act : Action<T,A>) -> Transition<T,A> {
        Transition {
            from : s1,
            read : char_r,
            action : act
        }
    }


    pub fn search(table : &TransitionTable<T,A>, read : &T, st : &State) -> Action<T,A> {
        for tr in table.iter() {
            if tr.read == *read && tr.from == *st {
                return tr.action.clone();
            }
        }
        
        return Action::Fail;
    }
}
