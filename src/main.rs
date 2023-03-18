use std::io;
fn main() {
    // Take user input
    let proc_num: i32;
    let mut processes: Vec<Process> = vec![];

    let mut input: String = String::from("");
    println!("Enter process number!");
    _ = io::stdin().read_line(&mut input);
    proc_num = input.trim().parse().expect("NOT NUMBER");

    let mut num: i32 = 0;
    while num < proc_num {
        println!("Enter arrival for process");
        let mut temp_arrival = String::new();
        _ = io::stdin().read_line(&mut temp_arrival);
        let temp_arrival: i32 = temp_arrival.trim().parse().expect("AAAAAA");

        println!("Enter burst for process");
        let mut temp_burst = String::new();
        _ = io::stdin().read_line(&mut temp_burst);
        let temp_burst: i32 = temp_burst.trim().parse().expect("AAAAAA");

        let temp_proc = Process {
            arrival: temp_arrival,
            burst: temp_burst,
            turnaround: 0,
            waiting: 0,
            remaining: temp_burst,
            completion_time: 0,
            completed: false,
        };
        processes.push(temp_proc);
        num += 1;
    }
    println!("sorting!");
    let sorted = sjf(processes, vec![], 0);
    println!("You've entered: ");
    for proc in sorted {
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
    completion_time: i32,
    completed: bool,
}
impl Process {
    pub fn proc(mut self, q: i32, current: i32) -> (Process, i32, i32) {
        self.remaining -= q;
        if self.remaining > 0 {
            (self, self.remaining - q, current + q)
        } else {
            (self, 0, current)
        }
    }
    pub fn calc_turn(self) -> i32 {
        self.completion_time - self.arrival
    }
    pub fn calc_wait(self) -> i32 {
        self.turnaround - self.burst
    }
    pub fn one_shot(mut self, current: i32) -> (Process, i32) {
        self.completion_time = current + self.remaining;
        self.completed = true;
        self.remaining = 0;
        (self, self.completion_time)
    }
}

// pub fn srt(procs: Vec<Process>, q: i32) -> Vec<Process> {
// procs.sort_by(|a, b| a.arrival.cmp(&b.arrival));
//     let mut smallest = procs[0];
//     let mut current = 0;
//     let mut completed: Vec<Process> = vec![];
//     todo!()
// }

pub fn fifo(mut procs: Vec<Process>) -> Vec<Process> {
    procs.sort_by(|a, b| a.arrival.cmp(&b.arrival));
    let mut clock = procs[0].arrival;
    let mut completed: Vec<Process> = vec![];
    for proc in procs {
        if proc.arrival <= clock {
            let mut done_proc: Process;
            (done_proc, clock) = proc.one_shot(clock);
            done_proc.turnaround = done_proc.calc_turn();
            done_proc.waiting = done_proc.calc_wait();
            completed.push(done_proc);
        }
    }
    completed
}

pub fn sjf(mut procs: Vec<Process>, mut completed: Vec<Process>, mut clock: i32) -> Vec<Process> {
    procs.sort_unstable_by_key(|proc| (proc.burst, proc.arrival));

    if procs.is_empty() {
        completed
    } else {
        let mut i = 0;
        while i < procs.len() {
            if procs[i].arrival <= clock {
                let mut done_proc: Process;
                (done_proc, clock) = procs[i].one_shot(clock);
                done_proc.turnaround = done_proc.calc_turn();
                done_proc.waiting = done_proc.calc_wait();
                completed.push(done_proc);
                procs.remove(i);
                return sjf(procs, completed, clock);
            } else {
                i += 1
            }
        }
        sjf(procs, completed, clock + 1)
    }
}
