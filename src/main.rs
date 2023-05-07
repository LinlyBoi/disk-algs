fn main() {
    let requests: Vec<i32> = vec![176, 79, 34, 60, 92, 11, 41, 114];
    let init_head = 50;
    println!(
        "FIFO seek distance is {}",
        fcfs(init_head, 0, requests.clone())
    );
    println!("SSTF seek distance is {}", sstf(init_head, 0, requests));
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
//defining initial direction here
fn scan(mut head: i32, disk_end: i32, mut requests: Vec<i32>, direction: Direction) -> i32 {
    requests.push(disk_end);
    match direction {
        Direction::LEFT => {
            requests.sort_by(|a, b| a.checked_sub(head).cmp(&b.checked_sub(head)));
        }
        Direction::RIGHT => {
            requests.sort_by(|a, b| b.checked_sub(head).cmp(&a.checked_sub(head)));
        }
    }
    requests
        .iter()
        .map(|request| {
            let seek = request.abs_diff(head) as i32; //TODO turn to function params
            head = *request;
            seek
        })
        .sum()
}
enum Direction {
    LEFT,
    RIGHT,
}
