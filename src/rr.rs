use std::collections::VecDeque;

use crate::Process;
pub fn round_robin(procs: Vec<Process>, q: i32) -> VecDeque<Process> {
    let mut buffer = procs.to_owned();
    buffer.sort_by(|a, b| a.arrival.cmp(&b.arrival));
    let mut current_time = 0;
    let mut in_cpu: VecDeque<Process> = VecDeque::new();
    let mut done: VecDeque<Process> = VecDeque::new();
    let mut last: Process = Process {
        arrival: 0,
        burst: 0,
        completion_time: 0,
        remaining: 0,
        turnaround: 0,
        waiting: 0,
    };
    loop {
        if done.len() == procs.len() {
            break;
        }

        let (ready, not_ready) = check_arrival(buffer.to_owned(), current_time);
        in_cpu = into_cpu(ready.to_owned(), in_cpu);

        if last.remaining > 0 {
            in_cpu.push_back(last)
        }

        match in_cpu.pop_front() {
            Some(mut proc) => {
                (proc.remaining, current_time) = proc.quan_zap(q, current_time);
                done = check_done(proc, current_time, done);
                if proc.remaining != 0 {
                    last = proc;
                }
            }
            None => current_time += 1,
        };
        println!("current time: {}", current_time);
        println!("in_cpu: {}", in_cpu.len());
        // println!("last remaining time: {}", last.remaining);
        // println!("currently done: {}", done.len());
        println!("buffer: {}, ready: {}", buffer.len(), ready.len());
    }
    done
}
pub fn check_arrival(buffer: Vec<Process>, current_time: i32) -> (VecDeque<Process>, Vec<Process>) {
    let mut ready: VecDeque<Process> = VecDeque::new();
    let mut not_ready: Vec<Process> = vec![];
    for proc in buffer {
        if proc.arrival <= current_time {
            ready.push_back(proc)
        } else {
            not_ready.push(proc)
        }
    }
    (ready, not_ready)
}
pub fn into_cpu(ready: VecDeque<Process>, mut cpu: VecDeque<Process>) -> VecDeque<Process> {
    for proc in ready {
        cpu.push_back(proc);
    }
    cpu
}
pub fn check_done(
    mut proc: Process,
    current_time: i32,
    mut done: VecDeque<Process>,
) -> VecDeque<Process> {
    if proc.remaining == 0 {
        proc.completion_time = current_time;
        proc.turnaround = proc.calc_turn();
        proc.waiting = proc.calc_wait();
        done.push_back(proc);
    }
    done
}

// This code sucks!
