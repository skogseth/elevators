use std::collections::HashMap;

use interface::types::{Button, Direction, Floor};

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

    pub fn add_request(&mut self, button: Button, floor: Floor) {
        self.get_mut(&button).set(true, floor.into());
    }

    pub fn request_at_floor(&mut self, current_floor: Floor, direction: Direction) -> Vec<Button> {
        let mut results = Vec::new();
        let buttons = [Button::Cab, Button::Hall(direction)];
        let index = usize::from(current_floor);

        for button in buttons {
            if self.get(&button).get(index) {
                self.get_mut(&button).set(false, index);
                results.push(button);
            }
        }

        results
    }

    pub fn check_in_direction(&self, current_floor: Floor, direction: Direction) -> bool {
        let buttons = [Button::Cab, Button::Hall(direction)];
        let floors = match direction {
            Direction::Up => (usize::from(current_floor) + 1)..self.n_floors,
            Direction::Down => 0..usize::from(current_floor),
        };

        for button in buttons {
            if self.get(&button).arr[floors.clone()].iter().any(|&x| x) {
                return true;
            }
        }

        false
    }

    pub fn check_for_any(&self) -> Option<(Floor, Button)> {
        for button in Button::iterator() {
            for floor in 0..self.n_floors {
                if self.get(&button).arr[floor] {
                    return Some((Floor::from(floor), button));
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

    pub fn get_active_buttons(&self, button: Button) -> Vec<Floor> {
        self.active_buttons
            .get(&button)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|&x| *x.1 == true)
            .map(|x| Floor::from(x.0))
            .collect::<Vec<Floor>>()
    }

    pub fn update_active_button(&mut self, button: Button, floor: Floor, active: bool) {
        self.active_buttons
            .get_mut(&button)
            .unwrap()
            .set(active, floor.into());
    }
}

/*
pub enum Status {
    Taken{ id: usize },
    Available,
}
*/
