mod infList {
    #[derive(Debug)]
    pub struct InfList <T> {
        tape : Vec<T>,
        base : T,
        min_idx : i64,
        max_idx : i64,
    }

    impl<T : Copy> InfList<T> {

        pub fn new (b : T) -> InfList<T> {
            InfList {
                tape : vec![],
                base : b,
                min_idx : 0,
                max_idx : (-1),
            }
        }

        fn is_valid_index (&self, index : i64) -> bool {
            index < self.min_idx || index > self.max_idx
        }

        fn convert_index (&self, index : i64) -> usize {
            (index - self.min_idx) as usize
        }

        pub fn read (&self, index : i64) -> T {
            if Self::is_valid_index(&self, index) {
                return self.base;
            } else {
                return self.tape[Self::convert_index(self, index)];
            }
        }

        pub fn write (&mut self, index : i64, to_write : T) {
            if index < self.min_idx{
                let d = self.min_idx - index;

                let sl = [self.base].repeat(d as usize);
                self.tape.splice(0..0, sl);

                self.min_idx = index;
            } else if index > self.max_idx{
                let d = index - self.max_idx;

                let sl = [self.base].repeat(d as usize);
                self.tape.splice(self.tape.len()..self.tape.len(), sl);

                self.max_idx = index;
            }

            let i = Self::convert_index(&self, index);
            self.tape[i] = to_write;
        
        }
    }
}

pub struct Tape<T> {
    tape : infList::InfList<T>,
    head : i64,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Left,
    Right
}

impl<T : Copy> Tape<T> {
    pub fn new (base : T) -> Tape<T> {
        Tape {
            tape : infList::InfList::new(base),
            head : 0,
        }
    }

    pub fn read (&self) -> T {
        self.tape.read(self.head)
    }

    pub fn write (&mut self, to_write : T) {
        self.tape.write(self.head, to_write);
    }

    pub fn shift (&mut self, dir : &Direction) {
        match dir {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
        }
    }
}