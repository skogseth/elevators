use once_cell::sync::OnceCell;

use crate::types::Floor;

pub static N_FLOORS: OnceCell<usize> = OnceCell::new();
static ERROR_MESSAGE: &'static str =
    "Floor is uninitialized, run Floor::initialize(max_floor) first.";

impl Floor {
    pub fn new() -> Self {
        Floor {
            val: 0,
            max: *N_FLOORS.get().expect(ERROR_MESSAGE) - 1,
        }
    }

    pub fn from_value(val: usize) -> Option<Self> {
        let max = *N_FLOORS.get().expect(ERROR_MESSAGE) - 1;
        if val <= max {
            Some(Floor { val, max })
        } else {
            None
        }
    }

    pub fn get(&self) -> usize {
        self.val
    }

    pub fn change(self, val: usize) -> Result<Self, usize> {
        let floor = Floor::from_value(val);
        floor.ok_or(self.max)
    } 

    pub fn initialize(n_floors: usize) {
        N_FLOORS.set(n_floors).unwrap();
    }

    pub fn get_n_floors() -> usize {
        *N_FLOORS.get().expect(ERROR_MESSAGE)
    }
}

impl From<usize> for Floor {
    fn from(val: usize) -> Floor {
        Floor::from_value(val).unwrap()
    }
}

impl From<Floor> for usize {
    fn from(val: Floor) -> usize {
        val.get()
    }
}

impl TryFrom<u8> for Floor {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Floor::from_value(val as usize).ok_or("Floor value was not within legal bounds.")
    }
}

impl From<Floor> for u8 {
    fn from(val: Floor) -> u8 {
        val.get() as u8
    }
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.val, self.max)
    }
}
