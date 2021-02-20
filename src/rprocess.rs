#[derive(Debug)]
pub struct Process {
    pid: i32,
    ppid: i32,
}

pub fn find_all() -> Vec<Process> {
    vec![Process { pid: 1, ppid: 2 }]
}
