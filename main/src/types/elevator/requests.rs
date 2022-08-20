use std::collections::HashMap;

use interface::types::{Button, Direction};

use super::Requests;

pub struct Array<T: Copy> {
    arr: Box<[T]>,
    len: usize,
}

impl<T: Copy> Array<T> {
    /*
    fn with_size(len: usize) -> Array<T> {
        let arr = [T; len];
        Array { arr, len }
    }
    */

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

    pub fn len(&self) -> usize {
        self.arr.len()
    }
}

impl Requests {
    pub fn new(n_floors: usize) -> Requests {
        let mut map = HashMap::new();
        let mut active_buttons = HashMap::new();
        for button in Button::iterator() {
            map.insert(button, Array::from_val(false, n_floors));
            active_buttons.insert(button, Array::from_val(true, n_floors));
        }

        Requests {
            map,
            active_buttons,
            n_floors,
        }
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

    pub fn request_at_floor(&mut self, current_floor: usize, direction: Direction) -> Vec<Button> {
        let mut results = Vec::new();
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
                results.push(button);
            }
        }

        results
    }

    pub fn check_in_direction(&self, current_floor: usize, direction: Direction) -> bool {
        let (floors, buttons) = match direction {
            Direction::Up => (
                (current_floor + 1)..self.n_floors,
                [Button::Cab, Button::HallUp],
            ),
            Direction::Down => (0..current_floor, [Button::Cab, Button::HallDown]),
        };

        for button in buttons {
            if self.get(&button).arr[floors.clone()].iter().any(|&x| x) {
                return true;
            }
        }

        false
    }

    pub fn check_for_any(&self) -> Option<(usize, Button)> {
        let buttons = [Button::Cab, Button::HallUp, Button::HallDown];
        for button in buttons {
            for floor in 0..self.n_floors {
                if self.get(&button).arr[floor] {
                    return Some((floor, button));
                }
            }
        }
        None
    }

    pub fn number_of_requests(&self) -> usize {
        let mut n_requests = 0;
        for button in Button::iterator() {
            n_requests += self.get(&button).len();
        }
        n_requests
    }

    pub fn get_active_buttons(&self, button: Button) -> Vec<usize> {
        self.active_buttons
            .get(&button)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|&x| *x.1 == true)
            .map(|x| x.0)
            .collect::<Vec<usize>>()
    }

    pub fn update_active_button(&mut self, button: Button, floor: usize, active: bool) {
        self.active_buttons
            .get_mut(&button)
            .unwrap()
            .set(active, floor);
    }
}

/*
pub enum Status {
    Taken{ id: usize },
    Available,
}
*/
