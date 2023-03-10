// Optimized Bresenham line algorithm
// Original code from https://github.com/mbr/bresenham-rs

/// Convenient typedef for two machines-sized integers
pub type Point = (isize, isize);

/// Line-drawing iterator
pub struct Bresenham {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    x1: isize,
    diff: isize,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline(always)]
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    #[inline(always)]
    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (p.1, -p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (-p.1, p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (-p.1, p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (p.1, -p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline(always)]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        Bresenham {
            x: start.0,
            y: start.1,
            dx,
            dy,
            x1: end.0,
            diff: dy - dx,
            octant,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            return None;
        }

        let p = (self.x, self.y);

        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.dx;
        }

        self.diff += self.dy;

        // loop inc
        self.x += 1;

        Some(self.octant.from_octant0(p))
    }
}