use grid::*;

fn calculate_scenic_score(grid: &Grid<(usize, bool)>, start: (usize, usize)) -> usize {
    let current_tree = grid.get(start.0, start.1).unwrap();
    let height = current_tree.0;

    let mut scores: Vec<usize> = Vec::new();

    // Left
    if start.1 == 0 {
        scores.push(0);
    } else {
        let distance_to_edge = start.1;
        let trees = grid
            .iter_row(start.0)
            .rev()
            .skip(grid.cols() - start.1)
            .take_while(|t| t.0 < height)
            .count();

        if trees == distance_to_edge {
            scores.push(trees)
        } else {
            scores.push(trees + 1);
        }
    }

    // Right
    if start.1 == grid.cols() - 1 {
        scores.push(0);
    } else {
        let distance_to_edge = grid.cols() - start.1 - 1;
        let trees = grid.iter_row(start.0)
                .skip(start.1 + 1)
                .take_while(|t| t.0 < height)
                .count();

                if trees == distance_to_edge {
                    scores.push(trees);
                } else {
                    scores.push(trees + 1);
                }
    }

    // Up
    if start.0 == 0 {
        scores.push(0);
    } else {
        let distance_to_edge = start.0;
        let trees = grid.iter_col(start.1).rev().skip(grid.rows() - start.0).take_while(|t| t.0 < height).count();

        if trees == distance_to_edge {
            scores.push(trees);
        } else {
            scores.push(trees + 1);
        }
    }

    // Down
    if start.0 == grid.rows() - 1 {
        scores.push(0);
    } else {
        let distance_to_edge = grid.rows() - start.0 - 1;
        let trees = grid.iter_col(start.1).skip(start.0 + 1).take_while(|t| t.0 < height).count();

        if trees == distance_to_edge {
            scores.push(trees);
        } else {
            scores.push(trees + 1);
        }
    }

    return scores.iter().product();
}

fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("Error Opening File");

    let mut grid: grid::Grid<(usize, bool)> = Grid::new(0, 0);

    for line in input.lines().map(|l| {
        l.chars()
            .map(|c| (c.to_digit(10).unwrap() as usize, true))
            .collect::<Vec<(usize, bool)>>()
    }) {
        grid.push_row(line);
    }

    println!("Starting Grid:");
    for row in 0..grid.rows() {
        println!("{:?}", grid.iter_row(row));
    }

    println!("\nNew Grid:");
    for col in 0..grid.cols() {
        for row in 0..grid.rows() {
            let mut score: Vec<usize> = Vec::new();
            // All border trees are visible
            if row == 0 || row == grid.rows() - 1 || col == 0 || col == grid.cols() - 1 {
                // grid.get_mut(row, col).unwrap().1 = true;
                continue;
            }

            let height = grid.get(row, col).unwrap().0;

            let mut left = true;
            let mut right = true;
            let mut up = true;
            let mut down = true;

            println!("Evaluating tree at ({},{})", row, col);
            // Check Left
            for x in (0..col).rev() {
                let other_tree = grid.get(row, x).unwrap();

                if other_tree.0 >= height {
                    left = false;
                    score.push(x);
                    break;
                }

                if x == 0 {
                    score.push(col);
                }
            }

            // Check Right
            for x in col + 1..grid.cols() - 1 {
                let other_tree = grid.get(row, x).unwrap();

                if other_tree.0 >= height {
                    right = false;
                    score.push(x);
                    break;
                }

                if x == grid.cols() {
                    score.push(col);
                }
            }

            // Check Up
            for y in (0..row).rev() {
                let other_tree = grid.get(y, col).unwrap();

                if other_tree.0 >= height {
                    up = false;
                    score.push(y);
                    break;
                }

                if y == 0 {
                    score.push(row);
                }
            }

            // Check Down
            for y in row + 1..grid.rows() {
                let other_tree = grid.get(y, col).unwrap();

                if other_tree.0 >= height {
                    down = false;
                    score.push(y);
                    break;
                }

                if y == grid.rows() - 1 {
                    score.push(row);
                }
            }

            // Set visible or not
            if !left && !right && !up && !down {
                grid.get_mut(row, col).unwrap().1 = false;
            }
        }
    }

    println!(
        "{} trees are visible",
        grid.iter().filter(|t| t.1 == true).count()
    );

    let mut scores: Vec<usize> = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            scores.push(calculate_scenic_score(&grid, (row, col)));
        }
    }

    println!("{}", scores.iter().max().unwrap());
}

#[cfg(test)]
mod tests {
    use grid::*;

    use crate::calculate_scenic_score;

    #[test]
    fn scenic_score_calculates_correctly() {
        let input =
            std::fs::read_to_string("./input/example_input.txt").expect("Error Opening File");

        let mut grid: grid::Grid<(usize, bool)> = Grid::new(0, 0);

        for line in input.lines().map(|l| {
            l.chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, true))
                .collect::<Vec<(usize, bool)>>()
        }) {
            grid.push_row(line);
        }

        let foo = calculate_scenic_score(&grid, (1, 2));
        assert_eq!(foo, 4);

        let bar = calculate_scenic_score(&grid, (3,2));
        assert_eq!(bar, 8);
    }
}
