use once_cell::sync::OnceCell;

use crate::types::Floor;

static MAX_FLOORS: OnceCell<usize> = OnceCell::new();
static ERROR_MESSAGE: &'static str =
    "Floor is uninitialized, run Floor::initialize(max_floor) first.";

impl Floor {
    pub fn new() -> Self {
        Floor {
            val: 0,
            max: *MAX_FLOORS.get().expect(ERROR_MESSAGE),
        }
    }

    pub fn from_value(val: usize) -> Option<Self> {
        let max = *MAX_FLOORS.get().expect(ERROR_MESSAGE);
        if val < max {
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

    pub fn initialize(max_floor: usize) {
        MAX_FLOORS.set(max_floor).unwrap();
    }
}

impl TryFrom<u8> for Floor {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Floor::from_value(val as usize).ok_or("Floor value was not within legal bounds.")
    }
}

impl From<Floor> for u8 {
    fn from(floor: Floor) -> u8 {
        floor.val as u8
    }
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.val, self.max)
    }
}
