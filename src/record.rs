use crate::snapshot::Snapshot;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

fn both<A, B>(first: Option<A>, second: Option<B>) -> Option<(A, B)> {
    if let (Some(first), Some(second)) = (first, second) {
        Some((first, second))
    } else {
        None
    }
}

pub struct Record<'before, 'after> {
    before: &'before Snapshot,
    after: &'after Snapshot,
}

impl<'before, 'after> From<(&'before Snapshot, &'after Snapshot)> for Record<'before, 'after> {
    fn from((before, after): (&'before Snapshot, &'after Snapshot)) -> Self {
        Self { before, after }
    }
}

impl<'before, 'after> Record<'before, 'after> {
    pub fn duration(&self) -> Duration {
        self.after
            .time
            .duration_since(self.before.time)
            .expect("went back in time")
    }

    pub fn total_cpu_time(&self) -> u64 {
        self.after.total_cpu_time() - self.before.total_cpu_time()
    }

    pub fn total_energy(&self) -> u64 {
        self.after.total_energy() - self.before.total_energy()
    }

    pub fn processes_time(&self) -> HashMap<i32, u64> {
        let before_times = self.before.processes_time();
        let after_times = self.after.processes_time();
        let before_pids = before_times.keys().copied().collect::<HashSet<_>>();
        let after_pids = after_times.keys().copied().collect::<HashSet<_>>();
        before_pids
            .intersection(&after_pids)
            .filter_map(|pid| {
                both(before_times.get(pid), after_times.get(pid))
                    .map(|item| (*pid, item.1 - item.0))
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn print(&self) {
        let duration = self.duration().as_secs_f64();
        let energy = self.total_energy();
        let power = energy as f64 / duration;
        let processes_time: u64 = self.processes_time().values().sum();
        println!(
            "duration={}s total_cpu_time={} processes_time={} energy={}µJ power={}µW",
            duration,
            self.total_cpu_time(),
            processes_time,
            energy,
            power
        );
    }
}
