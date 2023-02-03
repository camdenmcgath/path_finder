use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct CostPoint {
    pub cost: usize,
    pub pt: (usize, usize),
}

impl Ord for CostPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pt.cmp(&other.pt))
    }
}

impl PartialOrd for CostPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
