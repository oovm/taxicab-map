use super::*;

impl<'i, T> IntoIterator for &'i TaxicabMap<T> {
    type Item = (isize, isize, &'i T);
    type IntoIter = GetTaxicabPoints<'i, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.points_all()
    }
}

pub struct GetTaxicabPoints<'i, T> {
    map: &'i TaxicabMap<T>,
    cartesian: Product<Range<usize>, Range<usize>>,
}

pub struct MutGetTaxicabPoints<'i, T> {
    map: &'i mut TaxicabMap<T>,
    cartesian: Product<Range<usize>, Range<usize>>,
}

impl<'i, T> Iterator for GetTaxicabPoints<'i, T> {
    type Item = (isize, isize, &'i T);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.cartesian.next()?;
        let (x, y) = relative_to_absolute(i, j, self.map.origin_x, self.map.origin_y);
        let v = self.map.dense.get((i, j))?;
        Some((x, y, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cartesian.size_hint()
    }
}

impl<'i, T> Iterator for MutGetTaxicabPoints<'i, T> {
    type Item = (isize, isize, &'i mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.cartesian.next()?;
        let (x, y) = relative_to_absolute(i, j, self.map.origin_x, self.map.origin_y);
        // SAFETY:
        let v = unsafe { &mut *self.map.dense.get_mut_ptr((i, j))? };
        Some((x, y, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cartesian.size_hint()
    }
}

impl<T> TaxicabMap<T> {
    pub fn points_all(&self) -> GetTaxicabPoints<'_, T> {
        let (w, h) = self.get_size();
        GetTaxicabPoints { map: self, cartesian: (0..w).cartesian_product(0..h) }
    }
    pub fn points_mut(&mut self) -> MutGetTaxicabPoints<T> {
        let (w, h) = self.get_size();
        MutGetTaxicabPoints { map: self, cartesian: (0..w).cartesian_product(0..h) }
    }
}

/// A diamond shaped area around a point.
pub struct GetTaxicabPointsAround {
    points: DiamondPoints,
    origin_x: isize,
    origin_y: isize,
    w: usize,
    h: usize,
    cycle_x: bool,
    cycle_y: bool,
}

impl Iterator for GetTaxicabPointsAround {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.points.next()?;
        match absolute_to_relative(x, y, self.origin_x, self.origin_y, self.w, self.h, self.cycle_x, self.cycle_y) {
            Some(_) => Some((x, y)),
            None => self.next(),
        }
    }
}

/// A diamond shaped area around a point.
pub struct DiamondPoints {
    x: isize,
    y: isize,
    n: isize,
    index: isize,
}

impl DiamondPoints {
    pub fn new(x: isize, y: isize, n: isize) -> Self {
        Self { x, y, n, index: 0 }
    }
}

//      0: (x + n, y)
//      1: (x + n - 1, y + 1)
//      2: (x + n - 2, y + 2)
//  n    : (x, y + n)
//  n + 1: (x - 1, y + n - 1)
//  n + 2: (x - 2, y + n - 2)
// 2n    : (x - n, y)
// 2n + 1: (x - n + 1, y - 1)
// 2n + 2: (x - n + 2, y - 2)
// 3n    : (x, y - n)
// 3n + 1: (x + 1, y - n + 1)
// 3n + 2: (x + 2, y - n + 2)
// 4n - 1: (x + n - 1, y - 1)
impl Iterator for DiamondPoints {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = None;
        if self.n == 0 && self.index == 0 {
            out = Some((self.x, self.y))
        }
        else {
            if self.index < self.n {
                let k = self.index;
                out = Some((self.x + self.n - k, self.y + k))
            }
            else if self.index < 2 * self.n {
                let k = self.index - self.n;
                out = Some((self.x - k, self.y + self.n - k))
            }
            else if self.index < 3 * self.n {
                let k = self.index - 2 * self.n;
                out = Some((self.x - self.n + k, self.y - k))
            }
            else if self.index < 4 * self.n {
                let k = self.index - 3 * self.n;
                out = Some((self.x + k, self.y - self.n + k))
            }
        }
        self.index += 1;
        out
    }
}

impl<T> TaxicabMap<T> {
    /// Find at most 4 points that are exists and adjacent to a direction.
    pub fn points_nearby(&self, x: isize, y: isize) -> GetTaxicabPointsAround {
        self.points_around(x, y, 1)
    }
    /// Find all points that are within a certain distance of a direction.
    pub fn points_around(&self, x: isize, y: isize, steps: usize) -> GetTaxicabPointsAround {
        let (w, h) = self.get_size();
        GetTaxicabPointsAround {
            points: DiamondPoints::new(x, y, steps as isize),
            origin_x: self.origin_x,
            origin_y: self.origin_y,
            w,
            h,
            cycle_x: self.cycle_x,
            cycle_y: self.cycle_y,
        }
    }
}