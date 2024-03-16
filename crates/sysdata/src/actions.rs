use sysinfo::{Pid, Signal, System};

use crate::types;

pub fn process_signal(sys: &mut System, pid: usize, signal: types::ProcessSignal) {
    let Some(process) = sys.process(Pid::from(pid)) else {
        return;
    };

    let signal = match signal {
        types::ProcessSignal::Kill => Signal::Kill,
        types::ProcessSignal::Term => Signal::Term,
        types::ProcessSignal::Resume => Signal::Continue,
        types::ProcessSignal::Stop => Signal::Stop,
    };

    process.kill_with(signal);
}
