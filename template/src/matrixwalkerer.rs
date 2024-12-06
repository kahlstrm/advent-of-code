#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST,
    EAST,
    NORTHEAST,
}
pub const ALL_DIRECTIONS: &[Direction] = &[
    Direction::NORTH,
    Direction::NORTHWEST,
    Direction::WEST,
    Direction::SOUTHWEST,
    Direction::SOUTH,
    Direction::SOUTHEAST,
    Direction::EAST,
    Direction::NORTHEAST,
];
/// provides iterators for walking through 2-dimensional arrays in all 8 directions

#[derive(Clone, Copy)]
pub struct MatrixWalker<'a, T> {
    matrix: &'a [&'a [T]],
    i: usize,
    j: usize,
    cur: Option<&'a T>,
}
impl<'a, T> MatrixWalker<'a, T> {
    pub fn peek(&self) -> Option<&T> {
        self.cur
    }
    pub fn pos(&self) -> (usize, usize) {
        (self.i, self.j)
    }
    pub fn pos_toward(&self, dir: &Direction) -> Option<(usize, usize)> {
        let mut pos = (self.i, self.j);
        match dir {
            Direction::NORTH => {
                pos.1 = pos.1.checked_sub(1)?;
            }
            Direction::NORTHWEST => {
                pos.1 = pos.1.checked_sub(1)?;
                pos.0 += 1;
            }
            Direction::WEST => {
                pos.0 += 1;
            }
            Direction::SOUTHWEST => {
                pos.1 += 1;
                pos.0 += 1;
            }
            Direction::SOUTH => {
                pos.1 += 1;
            }
            Direction::SOUTHEAST => {
                pos.1 += 1;
                pos.0 = pos.0.checked_sub(1)?;
            }
            Direction::EAST => {
                pos.0 = pos.0.checked_sub(1)?;
            }
            Direction::NORTHEAST => {
                pos.1 = pos.1.checked_sub(1)?;
                pos.0 = pos.0.checked_sub(1)?;
            }
        }
        return pos.into();
    }
    pub fn peek_forward(&self, dir: &Direction) -> Option<&T> {
        return MatrixWalker::get_val_from_matrix(self.matrix, self.pos_toward(&dir)?);
    }
    /// attempts moving towards given direction for given amount of steps. If moving is succesful,
    /// returns Ok(()), otherwise Err(usize), usize being steps taken
    pub fn traverse(&mut self, direction: &Direction, steps: usize) -> Result<(), usize> {
        for n in 0..steps {
            self.get_and_step(direction).ok_or(n)?;
        }
        Ok(())
    }
    /// traverses towards the given direction, returns value or None if out of bounds.
    pub fn get_and_step(&mut self, direction: &Direction) -> Option<&'a T> {
        let tmp = self.cur;
        self.cur = (|| {
            match direction {
                Direction::NORTH => {
                    self.j = self.j.checked_sub(1)?;
                }
                Direction::NORTHWEST => {
                    self.j = self.j.checked_sub(1)?;
                    self.i += 1;
                }
                Direction::WEST => {
                    self.i += 1;
                }
                Direction::SOUTHWEST => {
                    self.j += 1;
                    self.i += 1;
                }
                Direction::SOUTH => {
                    self.j += 1;
                }
                Direction::SOUTHEAST => {
                    self.j += 1;
                    self.i = self.i.checked_sub(1)?;
                }
                Direction::EAST => {
                    self.i = self.i.checked_sub(1)?;
                }
                Direction::NORTHEAST => {
                    self.j = self.j.checked_sub(1)?;
                    self.i = self.i.checked_sub(1)?;
                }
            }
            MatrixWalker::get_val_from_matrix(self.matrix, (self.i, self.j))
        })();
        return tmp;
    }
    pub fn new(matrix: &'a [&'a [T]], coordinates: (usize, usize)) -> Self {
        Self {
            matrix,
            i: coordinates.0,
            j: coordinates.1,
            cur: MatrixWalker::get_val_from_matrix(matrix, coordinates),
        }
    }
    fn get_val_from_matrix(matrix: &'a [&'a [T]], (i, j): (usize, usize)) -> Option<&'a T> {
        matrix.get(j).and_then(|arr| arr.get(i))
    }
}
