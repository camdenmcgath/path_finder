use std::cmp::{Ord, Ordering, PartialOrd};
#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    pub state: (usize, usize),
    pub prev: Option<(usize, usize)>,
    pub weight: usize,
    pub path_cost: usize,
    pub estimated_cost: usize,
}
impl Cell {
    pub fn new(
        state: (usize, usize),
        prev: Option<(usize, usize)>,
        weight: usize,
        path_cost: usize,
        estimated_cost: usize,
    ) -> Cell {
        Cell {
            state,
            prev,
            weight,
            path_cost,
            estimated_cost,
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        let estimated_cmp = other.estimated_cost.cmp(&self.estimated_cost);
        match estimated_cmp {
            Ordering::Equal => other.path_cost.cmp(&self.path_cost),
            _ => estimated_cmp,
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.estimated_cost != other.estimated_cost {
            return Some(other.estimated_cost.cmp(&self.estimated_cost));
        } else {
            return Some(other.path_cost.cmp(&self.path_cost));
        }
    }
}
