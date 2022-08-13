use std::collections::HashMap;

use super::event::button::Button;

pub struct Array<T: Copy> {
    arr: Box<[T]>,
    len: usize,
}

impl<T: Copy> Array<T> {
    fn from_val(val: T, len: usize) -> Array<T> {
        let arr = (0..len).map(|_| val).collect();
        Array { arr, len }
    }

    pub fn set(&mut self, val: T, index: usize) {
        assert!(index < self.len);
        self.arr[index] = val;
    }
}

pub struct Requests(HashMap<Button, Array<bool>>);

impl Requests {
    pub fn new(n_floors: usize) -> Requests {
        let mut map = HashMap::new();
        for button in Button::iterator() {
            map.insert(button, Array::from_val(false, n_floors));
        }
        Requests(map)
    }

    pub fn add_request(&mut self, button: Button, floor: usize) {
        self.0.get_mut(&button).unwrap().set(true, floor);
    }

    //pub fn check_for_requests(current_floor: usize, direction: Direction) {}
}


/*
pub enum Status {
    Taken{ id: usize },
    Available,
}
*/