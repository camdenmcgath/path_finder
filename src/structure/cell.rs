use crate::structure::direction::Direction;

#[derive(Clone)]
pub struct Cell {
    pub value: char,
    pub state: (usize, usize),
    pub prev: Option<(usize, usize)>,
    pub in_path: bool,
    pub prev_direction: Option<Direction>,
    pub weight: usize,
    pub path_cost: usize,
}
impl Cell {
    pub fn new(
        value: char,
        state: (usize, usize),
        prev: Option<(usize, usize)>,
        in_path: bool,
        prev_direction: Option<Direction>,
        weight: usize,
        path_cost: usize,
    ) -> Cell {
        Cell {
            value,
            state,
            prev,
            in_path,
            prev_direction,
            weight,
            path_cost,
        }
    }
    pub fn set_val(&mut self, val: char) -> () {
        self.value = val;
    }
    pub fn set_state(&mut self, coords: (usize, usize)) -> () {
        self.state = coords;
    }
}
