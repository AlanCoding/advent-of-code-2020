use super::*;

/*
 * rules:
 * If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
 * If a seat is occupied (#) and five or more seats adjacent to it are also occupied, the seat becomes empty.
 * Otherwise, the seat's state does not change.
*/

fn change_seat_state(x: usize, y: usize, seat_data: &SeatMap) -> Option<Seat> {
    match seat_data[y][x] {
        Some(Seat::Empty) => {
            if count_adjacent_seats(x, y, seat_data) == 0 {
                Some(Seat::Taken)
            } else {
                Some(Seat::Empty)
            }
        }
        Some(Seat::Taken) => {
            if count_adjacent_seats(x, y, seat_data) >= 5 {
                Some(Seat::Empty)
            } else {
                Some(Seat::Taken)
            }
        }
        None => None,
    }
}

fn count_adjacent_seats(x: usize, y: usize, seat_data: &SeatMap) -> usize {
    let mut count = 0;
    let mut tmp_x = x;
    while tmp_x > 0 {
        tmp_x = tmp_x - 1;
        match seat_data[y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_x = x;
    while tmp_x < seat_data[y].len() - 1 {
        tmp_x = tmp_x + 1;
        match seat_data[y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_y = y;
    while tmp_y > 0 {
        tmp_y = tmp_y - 1;
        match seat_data[tmp_y][x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_y = y;
    while tmp_y < seat_data.len() - 1 {
        tmp_y = tmp_y + 1;
        match seat_data[tmp_y][x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_x = x;
    let mut tmp_y = y;
    while tmp_y > 0 && tmp_x > 0 {
        tmp_y = tmp_y - 1;
        tmp_x = tmp_x - 1;
        match seat_data[tmp_y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_x = x;
    let mut tmp_y = y;
    while tmp_y < seat_data.len() - 1 && tmp_x < seat_data[tmp_y].len() - 1 {
        tmp_y = tmp_y + 1;
        tmp_x = tmp_x + 1;
        match seat_data[tmp_y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_x = x;
    let mut tmp_y = y;
    while tmp_y > 0 && tmp_x < seat_data[tmp_y].len() - 1 {
        tmp_y = tmp_y - 1;
        tmp_x = tmp_x + 1;
        match seat_data[tmp_y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    let mut tmp_x = x;
    let mut tmp_y = y;
    while tmp_y < seat_data.len() - 1 && tmp_x > 0 {
        tmp_y = tmp_y + 1;
        tmp_x = tmp_x - 1;
        match seat_data[tmp_y][tmp_x] {
            Some(Seat::Taken) => {
                count += 1;
                break;
            }
            Some(Seat::Empty) => break,
            _ => continue,
        }
    }
    count
}

fn step_seat_map(seat_data: &SeatMap) -> SeatMap {
    let mut new_seat_data = seat_data.clone();
    for y in 0..seat_data.len() {
        for x in 0..seat_data[y].len() {
            new_seat_data[y][x] = change_seat_state(x, y, &seat_data);
        }
    }
    new_seat_data
}

fn count_taken_seats(seat_data: &SeatMap) -> usize {
    let mut count = 0;
    for y in 0..seat_data.len() {
        for x in 0..seat_data[y].len() {
            if seat_data[y][x] == Some(Seat::Taken) {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_p2(data: &SeatMap) -> usize {
    let mut old_seat_data = data.clone();
    let mut seat_data = step_seat_map(&old_seat_data);
    while seat_data != old_seat_data {
        old_seat_data = seat_data;
        seat_data = step_seat_map(&old_seat_data);
    }
    count_taken_seats(&seat_data)
}
