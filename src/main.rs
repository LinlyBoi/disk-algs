use std::collections::VecDeque;

fn main() {
    let requests: Vec<i32> = vec![176, 79, 34, 60, 92, 11, 41, 114];
    let init_head = 50;
    println!(
        "FIFO seek distance is {}",
        fcfs(init_head, 200, requests.clone())
    );
    println!(
        "SSTF seek distance is {}",
        sstf(init_head, 200, requests.clone())
    );
    println!(
        "Scan seek distance is {}",
        scan(init_head, 200, requests.clone(), &mut Direction::LEFT)
    );
    // println!(
    //     "FScan seek distance is {}",
    //     fscan(init_head, 200, requests.clone(), Direction::RIGHT, 4)
    // );
}

fn fcfs(head: i32, seek_distance: i32, mut requests: Vec<i32>) -> i32 {
    match requests.is_empty() {
        true => seek_distance,
        false => {
            let new_head = requests.remove(0);
            fcfs(
                new_head,
                seek_distance + new_head.abs_diff(head) as i32,
                requests,
            )
        }
    }
}

//shortest seek time first
fn sstf(head: i32, seek_distance: i32, mut requests: Vec<i32>) -> i32 {
    match requests.is_empty() {
        true => seek_distance,
        false => {
            requests.sort_by(|a, b| a.abs_diff(head).cmp(&b.abs_diff(head)));
            //dbg!(requests.clone());
            let new_head = requests.remove(0);

            sstf(
                new_head,
                seek_distance + head.abs_diff(new_head) as i32,
                requests,
            )
        }
    }
}
fn cscan(mut head: i32, disk_end: i32, mut requests: Vec<i32>, direction: &mut Direction) -> i32 {
    requests.sort();
    requests.insert(0, head);
    let split_index: Option<usize>;
    match direction {
        Direction::LEFT => {
            requests.sort_by(|a, b| (head - a).cmp(&(head - *b)));
            //split into two

            dbg!(&requests);
            *direction = Direction::RIGHT;
        }
        Direction::RIGHT => {
            requests.push(disk_end);
            requests.sort_by(|a, b| (head - b).cmp(&(head - *a))); //this in theory does literall
                                                                   //the opposite
            *direction = Direction::LEFT;
        }
    }
    split_index = split_vec(head, &mut requests);

    match split_index {
        Some(foo) => {
            let mut seek: i32 = 0;
            let mut right_side = requests.split_off(foo);
            match direction {
                Direction::LEFT => right_side.push(0),
                Direction::RIGHT => right_side.push(disk_end),
            }
            dbg!(&right_side);
            seek = fcfs(head, seek, right_side.clone());
            requests.sort();
            dbg!(&requests);
            seek = fcfs(0, seek, requests);
            seek
        }
        //We are splitting based on an element we just pushed in (impossible case)
        None => 0,
    }
}
//defining initial direction here
fn scan(mut head: i32, disk_end: i32, mut requests: Vec<i32>, direction: &mut Direction) -> i32 {
    requests.sort();
    requests.insert(0, head);
    let split_index: Option<usize>;
    match direction {
        Direction::LEFT => {
            requests.sort_by(|a, b| (head - a).cmp(&(head - *b)));
            //split into two

            dbg!(&requests);
            *direction = Direction::RIGHT;
        }
        Direction::RIGHT => {
            requests.push(disk_end);
            requests.sort_by(|a, b| (head - b).cmp(&(head - *a))); //this in theory does literall
                                                                   //the opposite
            *direction = Direction::LEFT;
        }
    }
    split_index = split_vec(head, &mut requests);
    dbg!(&split_index, &requests);
    match split_index {
        Some(foo) => {
            let mut seek: i32 = 0;
            let mut right_side = requests.split_off(foo);
            match direction {
                Direction::LEFT => right_side.push(0),
                Direction::RIGHT => right_side.push(disk_end),
            }
            dbg!(&right_side);
            seek = fcfs(head, seek, right_side.clone());
            requests.sort();
            requests.reverse();
            dbg!(&requests);
            seek = fcfs(0, seek, requests);
            seek
        }
        //We are splitting based on an element we just pushed in (impossible case)
        None => 0,
    }
}
fn fscan(
    mut head: i32,
    disk_end: i32,
    mut requests: Vec<i32>,
    mut direction: Direction,
    capacity: usize,
) -> i32 {
    let mut seek1: i32 = 0;
    let mut seek2: i32 = 0;
    let mut q1: Vec<i32> = Vec::with_capacity(capacity);
    let mut q2: Vec<i32> = Vec::with_capacity(capacity);
    while !requests.clone().is_empty() {
        loop {
            if &q1.len() < &q1.capacity() && !requests.is_empty() {
                &q1.push(requests.remove(0));
            } else {
                break;
            }
        }
        loop {
            if q2.len() < q2.capacity() && !requests.is_empty() {
                q2.push(requests.remove(0));
            } else {
                break;
            }
        }
        dbg!(&q1, &q2);
        dbg!(requests.len());
        seek1 += cscan(head, disk_end, q1.clone(), &mut direction);
        seek2 += cscan(head, disk_end, q2.clone(), &mut direction);
    }

    seek1 + seek2
}
fn nscan(
    mut head: i32,
    disk_end: i32,
    mut requests: Vec<i32>,
    mut direction: Direction,
    capacity: usize,
    q_num: usize,
) -> i32 {
    let queues: Vec<Vec<i32>> = Vec::with_capacity(q_num);
    for mut queue in queues.clone() {
        for value in requests.iter().take(4) {
            queue.push(*value);
        }
        queue.sort();
        for _i in 1..4 {
            if requests.is_empty() {
                break;
            }
            requests.remove(0);
        }
    }
    flip(&direction); //flip once before the first iteration so it cancels out
    queues
        .iter()
        .map(|queue| {
            direction = flip(&direction);
            match direction {
                Direction::LEFT => head = queue[0],
                Direction::RIGHT => head = queue[queue.len() - 1],
            }
            cscan(head, disk_end, queue.to_vec(), &mut direction)
        })
        .sum()
}

fn split_vec(item: i32, items: &mut Vec<i32>) -> Option<usize> {
    for (i, &pooper) in items.clone().iter().enumerate() {
        if pooper == item {
            return Some(i);
        }
    }
    None
}
enum Direction {
    LEFT,
    RIGHT,
}
fn cycle<T>(slice: &[T], start_pos: usize) -> impl Iterator<Item = &T> {
    slice.iter().cycle().skip(start_pos).take(slice.len())
}
fn flip(direction: &Direction) -> Direction {
    match direction {
        Direction::LEFT => Direction::RIGHT,
        Direction::RIGHT => Direction::LEFT,
    }
}
