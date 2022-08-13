use std::collections::HashMap;

use super::{event::button::Button, state::direction::Direction};

pub struct Array<T: Copy> {
    arr: Box<[T]>,
    len: usize,
}

impl<T: Copy> Array<T> {
    fn from_val(val: T, len: usize) -> Array<T> {
        let arr = (0..len).map(|_| val).collect();
        Array { arr, len }
    }

    pub fn get(&self, index: usize) -> T {
        self.arr[index]
    }

    pub fn set(&mut self, val: T, index: usize) {
        assert!(index < self.len);
        self.arr[index] = val;
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.arr.into_iter()
    }
}

pub struct Requests {
    map: HashMap<Button, Array<bool>>,
    n_floors: usize,
}

impl Requests {
    pub fn new(n_floors: usize) -> Requests {
        let mut map = HashMap::new();
        for button in Button::iterator() {
            map.insert(button, Array::from_val(false, n_floors));
        }
        Requests { map, n_floors }
    }

    pub fn get(&self, button: &Button) -> &Array<bool> {
        self.map.get(button).unwrap()
    }

    pub fn get_mut(&mut self, button: &Button) -> &mut Array<bool> {
        self.map.get_mut(button).unwrap()
    }

    pub fn add_request(&mut self, button: Button, floor: usize) {
        self.get_mut(&button).set(true, floor);
    }

    pub fn request_at_floor(&mut self, current_floor: usize, direction: Direction) -> bool {
        let mut result = false;
        let buttons = [
            Button::Cab,
            match direction {
                Direction::Up => Button::HallUp,
                Direction::Down => Button::HallDown,
            },
        ];

        for button in buttons {
            if self.get(&button).get(current_floor) {
                self.get_mut(&button).set(false, current_floor);
                result = true;
            }
        }

        result
    }

    pub fn check_for_requests(&self, current_floor: usize, direction: Direction) -> bool {
        //println!("checking for requests in direction {direction:?}");
        let (floors, buttons) = match direction {
            Direction::Up => (
                (current_floor + 1)..self.n_floors,
                [Button::Cab, Button::HallUp],
            ),
            Direction::Down => (0..current_floor, [Button::Cab, Button::HallDown]),
        };
        //println!("Buttons: {buttons:?}");
        //println!("Floors: {floors:?}");
        for button in buttons {
            if self.get(&button).arr[floors.clone()].iter().any(|&x| x) {
                return true;
            }
        }
        false
    }
}

/*
pub enum Status {
    Taken{ id: usize },
    Available,
}
*/
