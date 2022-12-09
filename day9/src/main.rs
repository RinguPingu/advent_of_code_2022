use core::panic;

fn move_knot(direction: &str, knot: &mut (i32, i32)) {
    let mut axis;
    let dir;

    match direction {
        "U" => {
            axis = &mut knot.1;
            dir = 1
        }
        "D" => {
            axis = &mut knot.1;
            dir = -1
        }
        "R" => {
            axis = &mut knot.0;
            dir = 1
        }
        "L" => {
            axis = &mut knot.0;
            dir = -1
        }
        s => panic!("move_head received invalid input: {}", s),
    };

    *axis += dir;
}

fn update_tail_position(head: &mut (i32, i32), tail: &mut (i32, i32)) {
    let distance_x = head.0 - tail.0;
    let distance_y = head.1 - tail.1;

    // Head should never be 2 steps away (or more) in both directions at once
    // if distance_x.abs() >= 2 && distance_y.abs() >= 2 {
    //     panic!(
    //         "Head and tail was 2 or more in both directions!\nDist_X: {}\nDist_Y: {}",
    //         distance_x, distance_y
    //     );
    // }

    // Head should never be more than 2 steps away in any directions
    if distance_x.abs() > 2 || distance_y.abs() > 2 {
        panic!(
            "Distance in one direction was greater than 2!\nDist_X: {}\nDist_Y: {}",
            distance_x, distance_y
        );
    }

    // If head and tail are touching, do nothing.
    if distance_x.abs() <= 1 && distance_y.abs() <= 1 {
        return;
    }

    // If head and tail are not on the same row OR column, move diagonally
    if head.0 != tail.0 && head.1 != tail.1 {
        if distance_x.is_positive() {
            move_knot("R", tail);
        } else {
            move_knot("L", tail);
        }

        if distance_y.is_positive() {
            move_knot("U", tail);
        } else {
            move_knot("D", tail);
        }
        return;
    }

    // If head and tail are on the same row OR column, move only horizontally or vertically
    if distance_x == 0 {
        match distance_y {
            2 => move_knot("U", tail),
            -2 => move_knot("D", tail),
            _ => (),
        }
        return;
    } else if distance_y == 0 {
        match distance_x {
            2 => move_knot("R", tail),
            -2 => move_knot("L", tail),
            _ => (),
        }
        return;
    }
}

fn main() {
    let input =
        std::fs::read_to_string("./input/input.txt").expect("Error opening input file");

    let commands: Vec<(&str, i32)> = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut x| (x.next().unwrap(), x.next().unwrap().parse::<i32>().unwrap()))
        .collect();

    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];

    // let mut first = knots.first();
    // let mut last = knots.last();

    // let head = first.as_mut().unwrap();
    // let tail = last.as_mut().unwrap();

    let mut tail_positions: Vec<(i32, i32)> = Vec::new();

    tail_positions.push(knots.last().unwrap().clone());

    for command in &commands {
        for step in 0..command.1 {
            // println!("Step {} of command {:?}", step + 1, command);
            // println!("Head is at:\t{:?}", knots.first().unwrap());
            move_knot(command.0, knots.first_mut().unwrap());
            // println!("Moved head to:\t{:?}", knots.first().unwrap());
            let mut previous_knot = knots.first().unwrap().clone();
            for knot in knots.iter_mut().skip(1).enumerate() {
                // println!("Updating knot: {}", knot.0 + 1);
                // println!(
                //     "Previous knot: {:?} | Current knot: {:?}",
                //     previous_knot, knot.1
                // );
                update_tail_position(&mut previous_knot, knot.1);
                // println!("Moved current knot to:\t{:?}", knot.1);
                previous_knot = knot.1.clone();
            }
            tail_positions.push(knots.last().unwrap().clone());
        }
    }

    tail_positions.sort();
    tail_positions.dedup();

    println!("{}", tail_positions.len());
}

#[cfg(test)]
mod tests {
    use crate::{move_knot, update_tail_position};

    #[test]
    fn move_head_moves_head_correctly() {
        let head = &mut (0, 0);

        move_knot("R", head);
        move_knot("U", head);

        assert_eq!(head, &mut (1, 1));

        move_knot("L", head);
        move_knot("D", head);

        assert_eq!(head, &mut (0, 0));
    }

    #[test]
    fn move_head_produces_correct_final_head_position_from_example_input() {
        let head = &mut (0, 0);

        let input =
            std::fs::read_to_string("./input/example_input.txt").expect("Error opening input file");

        let commands: Vec<(&str, i32)> = input
            .lines()
            .map(|l| l.split_whitespace())
            .map(|mut x| (x.next().unwrap(), x.next().unwrap().parse::<i32>().unwrap()))
            .collect();

        for command in commands {
            for _step in 0..command.1 {
                move_knot(command.0, head)
            }
        }

        assert_eq!(head, &mut (2, 2));
    }

    #[test]
    fn tail_position_updates_correctly_horizontal() {
        let head = &mut (0, 0);
        let tail = &mut (0, 0);

        move_knot("R", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (0, 0));

        move_knot("R", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (1, 0));
    }

    #[test]
    fn tail_position_updates_correctly_vertical() {
        let head = &mut (0, 0);
        let tail = &mut (0, 0);

        move_knot("U", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (0, 0));

        move_knot("U", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (0, 1));
    }

    #[test]
    fn tail_position_updates_correctly_diagonal() {
        let head = &mut (0, 0);
        let tail = &mut (0, 0);

        move_knot("R", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (0, 0));

        move_knot("U", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (0, 0));

        move_knot("U", head);
        update_tail_position(head, tail);
        assert_eq!(tail, &mut (1, 1));
    }

    #[test]
    #[should_panic]
    fn tail_position_panics_when_distance_too_far() {
        let head = &mut (3, 0);
        let tail = &mut (0, 0);

        update_tail_position(head, tail);
    }
}
