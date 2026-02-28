use std::{fmt::Display, ops::{Add, AddAssign}};

use crate::number::sign;

#[derive(Debug, Clone)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(start:usize, end:usize) -> Self {
        Range {
            start : std::cmp::min(start, end),
            end : std::cmp::max(start, end),
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn contains(&self, value:usize) -> bool {
        value >= self.start && value <= self.end
    }

    pub fn overlaps(&self, other:&Range) -> bool {
        !(self.end < other.start || other.end < self.start)
    }

    pub fn merge(&self, other:&Range) -> Range {
        Range {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        }
    }

    pub fn length(&self) -> usize {
        if self.end >= self.start {
            self.end - self.start + 1
        }
        else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    pub fn is_zero(&self) -> bool {
        self.start == 0 && self.end == 0
    }

    pub fn offset(&self, offset:isize) -> Range {
        let new_start = if offset < 0 {
            self.start.saturating_sub((-offset) as usize)
        }
        else {
            self.start.saturating_add(offset as usize)
        };
        let new_end = if offset < 0 {
            self.end.saturating_sub((-offset) as usize)
        }
        else {
            self.end.saturating_add(offset as usize)
        };
        Range {
            start: new_start,
            end: new_end,
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range {
            start: 0,
            end: 0,
        }
    }
}

impl Add<&Range> for Range {
    type Output = Range;

    fn add(self, other: &Range) -> Range {
        let r = self.merge(other);
        r
    }
}

impl AddAssign<&Range> for Range {
    fn add_assign(&mut self, other: &Range) {
        let r = self.merge(other);
        self.start = r.start;
        self.end = r.end;
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = sign(self.start as isize - other.start as isize);
        let b = sign(self.end as isize - other.end as isize);
        match (a, b) {
            (0, 0) => Some(std::cmp::Ordering::Equal),
            (x, y) if x <= 0 && y <= 0 => Some(std::cmp::Ordering::Less),
            (x, y) if x >= 0 && y >= 0 => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Range {}

impl From<&(usize, usize)> for Range {
    fn from(t:&(usize, usize)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<(usize, usize)> for Range {
    fn from(t:(usize, usize)) -> Self {
        (&t).into()
    }
}

impl From<&Range> for (usize, usize) {
    fn from(r:&Range) -> Self {
        (r.start, r.end)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}
