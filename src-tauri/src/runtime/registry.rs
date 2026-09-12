use super::process::refresh_snapshot;
use super::types::{
    CompiledCommand, OutputArtifact, QueuedRun, RunAccepted, RunSnapshot, RunStatus,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct RunRegistry {
    runs: HashMap<Uuid, RunSnapshot>,
    queue: VecDeque<QueuedRun>,
    active: Option<Uuid>,
    accepting: bool,
}
impl RunRegistry {
    pub fn new() -> Self {
        Self {
            accepting: true,
            ..Self::default()
        }
    }
    pub fn enqueue(&mut self, command: CompiledCommand) -> Result<RunAccepted, String> {
        if !self.accepting {
            return Err("application is shutting down".into());
        }
        let id = Uuid::new_v4();
        let position = self.queue.len();
        self.queue.push_back(QueuedRun { id, command });
        self.runs.insert(
            id,
            RunSnapshot {
                run_id: id,
                status: RunStatus::Queued,
                queue_position: Some(position),
                error: None,
                artifacts: vec![],
            },
        );
        Ok(RunAccepted {
            run_id: id,
            queue_position: position,
        })
    }
    pub fn start_next(&mut self) -> Option<(Uuid, CompiledCommand)> {
        let job = self.queue.pop_front()?;
        self.active = Some(job.id);
        if let Some(s) = self.runs.get_mut(&job.id) {
            s.status = RunStatus::Running;
            s.queue_position = None;
        }
        for (i, q) in self.queue.iter().enumerate() {
            if let Some(s) = self.runs.get_mut(&q.id) {
                s.queue_position = Some(i);
            }
        }
        Some((job.id, job.command))
    }
    pub fn cancel(&mut self, id: Uuid) -> Result<(), String> {
        if self.active == Some(id) {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
                s.queue_position = None;
            }
            return Ok(());
        }
        let Some(index) = self.queue.iter().position(|q| q.id == id) else {
            return Err("run not found or already finished".into());
        };
        self.queue.remove(index);
        if let Some(s) = self.runs.get_mut(&id) {
            s.status = RunStatus::Cancelled;
            s.queue_position = None;
        }
        for (i, q) in self.queue.iter().enumerate() {
            if let Some(s) = self.runs.get_mut(&q.id) {
                s.queue_position = Some(i);
            }
        }
        Ok(())
    }
    pub fn finish(&mut self, id: Uuid, result: Result<Vec<OutputArtifact>, String>) {
        if let Some(s) = self.runs.get_mut(&id) {
            if s.status == RunStatus::Cancelled {
                if self.active == Some(id) {
                    self.active = None;
                }
                return;
            }
            match result {
                Ok(a) => {
                    s.status = RunStatus::Completed;
                    s.artifacts = a;
                }
                Err(e) => {
                    s.status = RunStatus::Failed;
                    s.error = Some(e);
                }
            }
        }
        if self.active == Some(id) {
            self.active = None;
        }
    }
    pub fn get(&self, id: Uuid) -> Option<RunSnapshot> {
        self.runs.get(&id).cloned().map(refresh_snapshot)
    }
    pub fn list(&self) -> Vec<RunSnapshot> {
        let mut runs = self
            .runs
            .values()
            .cloned()
            .map(refresh_snapshot)
            .collect::<Vec<_>>();
        runs.sort_by_key(|run| run.run_id);
        runs
    }
    pub fn shutdown(&mut self) {
        self.accepting = false;
        if let Some(id) = self.active {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
            }
        }
        let ids = self.queue.drain(..).map(|q| q.id).collect::<Vec<_>>();
        for id in ids {
            if let Some(s) = self.runs.get_mut(&id) {
                s.status = RunStatus::Cancelled;
                s.queue_position = None;
            }
        }
    }

    pub(super) fn should_stop(&self) -> bool {
        !self.accepting && self.active.is_none() && self.queue.is_empty()
    }
}
pub type SharedRunRegistry = Arc<Mutex<RunRegistry>>;
