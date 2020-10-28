use std::fs;

const MAX_GRID: usize = 100;

struct Grid {
    grid: [[bool; MAX_GRID]; MAX_GRID],
    corners_on: bool,
}

impl Grid {
    fn init(data: &str, corners_on: bool) -> Grid {
        let mut g = Grid {
            grid: [[false; MAX_GRID]; MAX_GRID],
            corners_on,
        };

        for (x, l) in data.trim().lines().enumerate() {
            for (y, c) in l.char_indices() {
                if g.corners_on && g.is_corner(x, y) {
                    g.set_point(x, y, true);
                    continue;
                }

                g.set_point(x, y, c == '#');
            }
        }

        g
    }

    fn is_corner(&self, x: usize, y: usize) -> bool {
        (x == 0 || x == self.grid.len() - 1) && (y == 0 || y == self.grid[0].len() - 1)
    }

    fn valid_point(&self, x: usize, y: usize) -> bool {
        x < self.grid.len() && y < self.grid[0].len()
    }

    fn set_point(&mut self, x: usize, y: usize, value: bool) {
        if !self.valid_point(x, y) {
            return;
        }

        self.grid[x][y] = value;
    }

    fn get_neighbor(&self, x: usize, y: usize) -> usize {
        if !self.valid_point(x, y) {
            return 0;
        }

        if self.corners_on && self.is_corner(x, y) {
            return 1;
        }

        if self.grid[x][y] {
            return 1;
        }

        0
    }

    fn animate(&mut self) {
        let mut changes: Vec<(usize, usize, bool)> = Vec::new();
        for (x, row) in self.grid.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if self.corners_on && self.is_corner(x, y) {
                    continue;
                }

                let mut neighbor_count = 0;
                if x > 0 && y > 0 {
                    neighbor_count += self.get_neighbor(x - 1, y - 1);
                }

                if x > 0 {
                    neighbor_count += self.get_neighbor(x - 1, y) + self.get_neighbor(x - 1, y + 1);
                }
                if y > 0 {
                    neighbor_count += self.get_neighbor(x, y - 1) + self.get_neighbor(x + 1, y - 1)
                }

                neighbor_count += self.get_neighbor(x + 1, y);
                neighbor_count += self.get_neighbor(x + 1, y + 1);
                neighbor_count += self.get_neighbor(x, y + 1);

                if *col && neighbor_count != 2 && neighbor_count != 3 {
                    changes.push((x, y, false));
                }

                if !*col && neighbor_count == 3 {
                    changes.push((x, y, true));
                }
            }
        }

        for (x, y, val) in changes {
            self.set_point(x, y, val);
        }
    }

    fn on_count(&self) -> usize {
        let mut count = 0;

        for row in &self.grid {
            for col in row {
                if *col {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day18.txt").unwrap();

    let mut grid = Grid::init(&input, false);
    for _ in 0..100 {
        grid.animate();
    }
    println!("number of on lights: {}", grid.on_count());

    let mut grid_v2 = Grid::init(&input, true);
    for _ in 0..100 {
        grid_v2.animate();
    }
    println!("number of on lights (v2): {}", grid_v2.on_count());
}
