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
        };
        processes.push(temp_proc);
        num += 1;
    }
    println!("sorting!");
    let sorted = round_robin(processes, 2);
    // let sorted = round_robin(processes, vec![], 0, 2);
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
}
impl Process {
    pub fn quan_zap(self, q: i32, current: i32) -> (i32, i32) {
        if q < 0 {
            println!("Why would you ever do this?");
            return (0, 0);
        }
        if self.remaining >= q {
            (self.remaining - q, current + q)
        } else {
            (0, current + self.remaining)
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
        self.remaining = 0;
        (self, self.completion_time)
    }
    pub fn robin_zap(self, q: i32) -> i32 {
        if self.remaining >= q {
            self.remaining - q
        } else {
            0
        }
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
pub fn round_robin(procs: Vec<Process>, q: i32) -> Vec<Process> {
    let mut buffer = procs.to_owned();
    buffer.sort_by(|a, b| a.arrival.cmp(&b.arrival));
    let mut current_time = 0;
    let mut in_cpu: Vec<Process> = vec![];
    let mut done: Vec<Process> = vec![];
    let mut last: Process = Process {
        arrival: 0,
        burst: 0,
        completion_time: 0,
        remaining: 0,
        turnaround: 0,
        waiting: 0,
    };
    loop {
        let (ready, not_ready) = check_arrival(buffer, current_time);
        buffer = not_ready;
        in_cpu = into_cpu(ready.to_owned(), in_cpu);
        if last.remaining > 0 {
            in_cpu.push(last)
        }
        if !in_cpu.is_empty() {
            last = in_cpu.remove(0);
            (last.remaining, current_time) = last.quan_zap(q, current_time);
            done = check_done(last, current_time, done);
        }
        println!("{}", done.len());
        println!("{}, {}", buffer.len(), ready.len());
        if done.len() == procs.len() {
            break;
        }
    }
    done
}
pub fn check_arrival(buffer: Vec<Process>, current_time: i32) -> (Vec<Process>, Vec<Process>) {
    let mut ready: Vec<Process> = vec![];
    let mut not_ready: Vec<Process> = vec![];
    for proc in buffer {
        if proc.remaining <= current_time {
            ready.push(proc);
        } else {
            not_ready.push(proc);
        }
    }

    (ready, not_ready)
}
pub fn into_cpu(ready: Vec<Process>, mut cpu: Vec<Process>) -> Vec<Process> {
    for proc in ready {
        cpu.push(proc);
    }
    cpu
}
pub fn check_done(mut proc: Process, current_time: i32, mut done: Vec<Process>) -> Vec<Process> {
    if proc.remaining == 0 {
        proc.completion_time = current_time;
        proc.turnaround = proc.calc_turn();
        proc.waiting = proc.calc_wait();
        done.push(proc);
    }
    done
}

// This code sucks!
