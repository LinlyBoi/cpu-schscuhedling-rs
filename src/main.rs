use std::io;
fn main() {
    // Take user input
    let proc_num: i32;
    print!("Enter process number!");
    let mut processes: Vec<Process> = vec![];

    let mut input: String = String::from("");
    _ = io::stdin().read_line(&mut input);
    proc_num = input.trim().parse().expect("NOT NUMBER");

    let mut num: i32 = 0;
    while num < proc_num {
        print!("Enter arrival for process");
        let mut temp_arrival = String::new();
        _ = io::stdin().read_line(&mut temp_arrival);
        let temp_arrival: i32 = temp_arrival.trim().parse().expect("AAAAAA");

        print!("Enter burst for process");
        let mut temp_burst = String::new();
        _ = io::stdin().read_line(&mut temp_burst);
        let temp_burst: i32 = temp_burst.trim().parse().expect("AAAAAA");

        let mut temp_proc = Process {
            arrival: temp_arrival,
            burst: temp_burst,
            turnaround: 0,
            waiting: 0,
            remaining: temp_burst,
            completion: 0,
            completed: false,
        };
        processes.push(temp_proc);
        num += 1;
    }
    println!("You've entered: ");
    srt(&processes, 1);
    for proc in processes {
        println!("{:#?}", proc)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Process {
    arrival: i32,
    burst: i32,
    turnaround: i32,
    waiting: i32,
    remaining: i32,
    completion: i32,
    completed: bool,
}
impl Process {
    pub fn proc(mut self, q: i32, current: i32) -> i32 {
        self.remaining -= q;
        if self.remaining == 0 {
            self.completed = true;
            self.completion = current + q;
        }
        current + q
    }
    pub fn calc_turn(mut self) {
        self.turnaround = self.completion - self.arrival
    }
    pub fn calc_wait(mut self) {
        self.waiting = self.turnaround - self.burst
    }
}

pub fn srt(procs: &Vec<Process>, q: i32) {
    let mut smallest = &procs[0];
    let mut current = 0;
    for value in procs {
        if value.arrival <= current && value.remaining < smallest.remaining && !value.completed {
            smallest = value;
            current = value.proc(q, current);
        }
        if value.completed {
            value.calc_turn();
            value.calc_wait();
        }
    }
}
