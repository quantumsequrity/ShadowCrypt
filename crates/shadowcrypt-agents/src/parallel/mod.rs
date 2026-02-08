//! Parallel execution system for agents
//!
//! Enables running multiple agents concurrently using threads.

use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::time::Duration;
use crossbeam::channel::{unbounded, Sender, Receiver};

use crate::agents::{Agent, AgentId, AgentAction};

/// Scheduler for parallel agent execution
#[derive(Clone)]
pub struct AgentScheduler {
    /// Number of worker threads
    pub num_workers: usize,
    /// Execution mode
    pub mode: ExecutionMode,
    /// Priority queue for agents
    priority_queue: Arc<RwLock<VecDeque<AgentId>>>,
}

impl Default for AgentScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentScheduler {
    /// Creates a new scheduler
    pub fn new() -> Self {
        Self {
            num_workers: num_cpus(),
            mode: ExecutionMode::Parallel,
            priority_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// Creates a scheduler with specific worker count
    pub fn with_workers(num_workers: usize) -> Self {
        Self {
            num_workers,
            mode: ExecutionMode::Parallel,
            priority_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// Sets the execution mode
    pub fn set_mode(&mut self, mode: ExecutionMode) {
        self.mode = mode;
    }

    /// Schedules an agent for execution
    pub fn schedule(&self, agent_id: AgentId) {
        self.priority_queue.write().push_back(agent_id);
    }

    /// Schedules multiple agents
    pub fn schedule_all(&self, agents: impl Iterator<Item = AgentId>) {
        let mut queue = self.priority_queue.write();
        for agent in agents {
            queue.push_back(agent);
        }
    }

    /// Gets the next agent to process
    pub fn next(&self) -> Option<AgentId> {
        self.priority_queue.write().pop_front()
    }

    /// Clears the queue
    pub fn clear(&self) {
        self.priority_queue.write().clear();
    }

    /// Returns queue size
    pub fn queue_size(&self) -> usize {
        self.priority_queue.read().len()
    }
}

/// Execution modes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Execute agents sequentially
    Sequential,
    /// Execute agents in parallel
    Parallel,
    /// Execute in batches
    Batched(usize),
}

/// Parallel executor for agents
pub struct ParallelExecutor {
    /// Worker threads
    workers: Vec<thread::JoinHandle<()>>,
    /// Channel to send work to workers
    work_sender: Sender<WorkItem>,
    /// Channel to receive results
    result_receiver: Receiver<WorkResult>,
    /// Result sender for workers
    result_sender: Sender<WorkResult>,
    /// Work receiver for workers
    work_receiver: Receiver<WorkItem>,
    /// Shutdown flag
    shutdown: Arc<RwLock<bool>>,
}

/// A work item for parallel execution
#[derive(Clone)]
pub struct WorkItem {
    pub agent_id: AgentId,
    pub turn: u32,
    pub context: WorkContext,
}

/// Context needed for agent decision making
#[derive(Clone, Default)]
pub struct WorkContext {
    /// Nearby agent positions
    pub nearby_agents: Vec<(AgentId, usize, usize)>,
    /// Visible enemies
    pub visible_enemies: Vec<AgentId>,
    /// Current position
    pub position: (usize, usize),
    /// Additional data
    pub data: HashMap<String, String>,
}

/// Result from processing an agent
#[derive(Clone, Debug)]
pub struct WorkResult {
    pub agent_id: AgentId,
    pub action: Option<AgentAction>,
    pub messages: Vec<AgentMessage>,
    pub duration_us: u64,
}

/// Message from an agent
#[derive(Clone, Debug)]
pub struct AgentMessage {
    pub from: AgentId,
    pub to: Option<AgentId>,
    pub content: String,
}

impl ParallelExecutor {
    /// Creates a new parallel executor
    pub fn new(num_workers: usize) -> Self {
        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();
        let shutdown = Arc::new(RwLock::new(false));

        let mut executor = Self {
            workers: Vec::new(),
            work_sender,
            result_receiver,
            result_sender,
            work_receiver,
            shutdown,
        };

        executor.spawn_workers(num_workers);
        executor
    }

    /// Spawns worker threads
    fn spawn_workers(&mut self, count: usize) {
        for _ in 0..count {
            let receiver = self.work_receiver.clone();
            let sender = self.result_sender.clone();
            let shutdown = self.shutdown.clone();

            let handle = thread::spawn(move || {
                while !*shutdown.read() {
                    match receiver.recv_timeout(Duration::from_millis(100)) {
                        Ok(work) => {
                            let start = std::time::Instant::now();

                            // Process the work item
                            let action = process_agent(&work);
                            let duration = start.elapsed().as_micros() as u64;

                            let result = WorkResult {
                                agent_id: work.agent_id,
                                action,
                                messages: Vec::new(),
                                duration_us: duration,
                            };

                            let _ = sender.send(result);
                        }
                        Err(_) => continue,
                    }
                }
            });

            self.workers.push(handle);
        }
    }

    /// Submits work for execution
    pub fn submit(&self, work: WorkItem) -> Result<(), String> {
        self.work_sender
            .send(work)
            .map_err(|e| e.to_string())
    }

    /// Submits multiple work items
    pub fn submit_batch(&self, items: Vec<WorkItem>) -> Result<(), String> {
        for item in items {
            self.submit(item)?;
        }
        Ok(())
    }

    /// Collects all available results
    pub fn collect_results(&self) -> Vec<WorkResult> {
        let mut results = Vec::new();
        while let Ok(result) = self.result_receiver.try_recv() {
            results.push(result);
        }
        results
    }

    /// Waits for all submitted work to complete
    pub fn wait_all(&self, timeout: Duration) -> Vec<WorkResult> {
        let start = std::time::Instant::now();
        let mut results = Vec::new();

        while start.elapsed() < timeout {
            while let Ok(result) = self.result_receiver.try_recv() {
                results.push(result);
            }
            thread::sleep(Duration::from_millis(1));
        }

        results
    }

    /// Shuts down the executor
    pub fn shutdown(self) {
        *self.shutdown.write() = true;
        for handle in self.workers {
            let _ = handle.join();
        }
    }

    /// Returns the number of workers
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}

/// Process an agent's turn
fn process_agent(work: &WorkItem) -> Option<AgentAction> {
    // This is a simplified version - in real implementation,
    // this would use the behavior tree to decide an action

    // Check if there are visible enemies
    if !work.context.visible_enemies.is_empty() {
        // Move toward nearest enemy
        if let Some(&enemy_id) = work.context.visible_enemies.first() {
            // Find enemy position
            if let Some((_, ex, ey)) = work.context.nearby_agents.iter().find(|(id, _, _)| *id == enemy_id) {
                let (px, py) = work.context.position;
                let dx = (*ex as i32 - px as i32).signum();
                let dy = (*ey as i32 - py as i32).signum();

                // If adjacent, attack
                if dx.abs() <= 1 && dy.abs() <= 1 {
                    return Some(AgentAction::Attack { target_id: enemy_id });
                }

                // Otherwise move toward
                return Some(AgentAction::Move { dx, dy });
            }
        }
    }

    // Default: random movement
    let mut rng = rand::thread_rng();
    use rand::Rng;
    let dx = rng.r#gen_range(-1..=1);
    let dy = rng.r#gen_range(-1..=1);
    Some(AgentAction::Move { dx, dy })
}

/// Synchronization primitive for agent data
pub struct SyncPrimitive<T> {
    data: Arc<RwLock<T>>,
}

impl<T> SyncPrimitive<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
        }
    }

    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, T> {
        self.data.read()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, T> {
        self.data.write()
    }
}

impl<T: Clone> Clone for SyncPrimitive<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::new(RwLock::new(self.data.read().clone())),
        }
    }
}

impl<T: Default> Default for SyncPrimitive<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Thread-safe agent store
#[derive(Default)]
pub struct AgentStore {
    agents: Arc<RwLock<HashMap<AgentId, Agent>>>,
    pending_actions: Arc<Mutex<Vec<(AgentId, AgentAction)>>>,
}

impl AgentStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an agent
    pub fn add(&self, agent: Agent) {
        self.agents.write().insert(agent.id, agent);
    }

    /// Gets an agent (read-only)
    pub fn get(&self, id: AgentId) -> Option<Agent> {
        self.agents.read().get(&id).cloned()
    }

    /// Updates an agent
    pub fn update<F>(&self, id: AgentId, f: F)
    where
        F: FnOnce(&mut Agent),
    {
        if let Some(agent) = self.agents.write().get_mut(&id) {
            f(agent);
        }
    }

    /// Queues an action for an agent
    pub fn queue_action(&self, agent_id: AgentId, action: AgentAction) {
        self.pending_actions.lock().push((agent_id, action));
    }

    /// Processes all pending actions
    pub fn process_actions(&self) {
        let actions: Vec<_> = self.pending_actions.lock().drain(..).collect();
        for (agent_id, action) in actions {
            self.apply_action(agent_id, action);
        }
    }

    /// Applies an action to an agent
    fn apply_action(&self, agent_id: AgentId, action: AgentAction) {
        self.update(agent_id, |agent| {
            match action {
                AgentAction::Move { dx, dy } => {
                    agent.x = (agent.x as i32 + dx).max(0) as usize;
                    agent.y = (agent.y as i32 + dy).max(0) as usize;
                }
                AgentAction::Rest => {
                    agent.heal(5);
                }
                _ => {}
            }
        });
    }

    /// Gets all agent IDs
    pub fn all_ids(&self) -> Vec<AgentId> {
        self.agents.read().keys().copied().collect()
    }

    /// Gets count
    pub fn count(&self) -> usize {
        self.agents.read().len()
    }
}

/// Gets the number of CPUs
fn num_cpus() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Batch processor for running agents in groups
pub struct BatchProcessor {
    pub batch_size: usize,
    pub current_batch: usize,
}

impl BatchProcessor {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            current_batch: 0,
        }
    }

    /// Gets the next batch of agents to process
    pub fn next_batch(&mut self, total: usize) -> std::ops::Range<usize> {
        let start = self.current_batch * self.batch_size;
        let end = (start + self.batch_size).min(total);
        self.current_batch += 1;

        if start >= total {
            self.current_batch = 0;
            return 0..0;
        }

        start..end
    }

    /// Resets the batch counter
    pub fn reset(&mut self) {
        self.current_batch = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler() {
        let scheduler = AgentScheduler::new();
        let id = AgentId::new();

        scheduler.schedule(id);
        assert_eq!(scheduler.queue_size(), 1);

        let next = scheduler.next();
        assert_eq!(next, Some(id));
        assert_eq!(scheduler.queue_size(), 0);
    }

    #[test]
    fn test_sync_primitive() {
        let prim = SyncPrimitive::new(42);
        assert_eq!(*prim.read(), 42);

        *prim.write() = 100;
        assert_eq!(*prim.read(), 100);
    }

    #[test]
    fn test_batch_processor() {
        let mut processor = BatchProcessor::new(10);

        let batch1 = processor.next_batch(25);
        assert_eq!(batch1, 0..10);

        let batch2 = processor.next_batch(25);
        assert_eq!(batch2, 10..20);

        let batch3 = processor.next_batch(25);
        assert_eq!(batch3, 20..25);
    }
}
