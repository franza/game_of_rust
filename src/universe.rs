extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        let coin: u8 = rng.gen_range(0, 2);
        match coin {
            0 => Cell::Dead,
            1 => Cell::Alive,
            _ => unreachable!()
        }
    }
}

pub struct Universe {
    arr: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Universe {
    pub fn new(h: usize, w: usize) -> Universe {
        Universe::with_value(h, w, Cell::Dead)
    }

    pub fn with_value(h: usize, w: usize, val: Cell) -> Universe {
        Universe::with_eval(h, w, |_, _| val)
    }

    pub fn with_eval<F>(h: usize, w: usize, mut get_val: F) -> Universe
        where F: FnMut(usize, usize) -> Cell {
        let mut arr = Vec::new();
        for i in 0..h {
            arr.push(Vec::new());
            for j in 0..w {
                arr[i].push(get_val(i, j));
            }
        }

        Universe { arr, height: h, width: w }
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn surroundings(&self, i: usize, j: usize) -> Vec<Cell> {
        let mut acc = Vec::new();
        let (h, w) = self.dim();
        let h = h as isize;
        let w = w as isize;
        let i = i as isize;
        let j = j as isize;

        let sur_idxs = [-1, 0, 1];

        for &delta_i in sur_idxs.clone().iter() {
            for &delta_j in sur_idxs.clone().iter() {
                if delta_i == 0 && delta_j == 0 {
                    continue;
                }

                let sur_i = i + delta_i;
                let sur_j = j + delta_j;

                if sur_i >= 0 && sur_i < h && sur_j >= 0 && sur_j < w {
                    acc.push(self[(sur_i as usize, sur_j as usize)]);
                }
            }
        }

        acc
    }

    pub fn next_gen(self) -> Universe {
        let (h, w) = self.dim();
        let mut next_gen_uni = Universe::new(h, w);

        for i in 0..h {
            for j in 0..w {
                next_gen_uni.arr[i][j] = self.evolve(i, j);
            }
        }

        next_gen_uni
    }

    fn evolve(&self, i: usize, j: usize) -> Cell {
        let alive_count = self.surroundings(i, j).iter()
            .filter(|&&cell| cell == Cell::Alive)
            .count();

        match (self[(i, j)], alive_count) {
            (Cell::Alive, 1..=2) => Cell::Alive,
            (Cell::Alive, _) => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (Cell::Dead, _) => Cell::Dead,
        }
    }
}

impl Index<(usize, usize)> for Universe {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.arr[i][j]
    }
}
