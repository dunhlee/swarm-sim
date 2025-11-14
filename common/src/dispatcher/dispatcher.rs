use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use crate::commands::command::Command;

// The dispatcher stores commands indexed by their NATS subject.
pub type CommandMap = Arc<RwLock<HashMap<String, Arc<dyn Command + Send + Sync>>>>;

#[derive(Clone)]
pub struct Dispatcher {
    commands: CommandMap,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a command by the NATS subject it listens to.
    pub async fn register<C>(&self, command: C)
    where
        C: Command + Send + Sync + 'static,
    {
        let mut map = self.commands.write().await;
        map.insert(command.subject().to_string(), Arc::new(command));
    }

    /// Dispatch a message to the command responsible for the subject.
    pub async fn dispatch(&self, subject: &str, payload: &[u8]) {
        let map = self.commands.read().await;

        if let Some(cmd) = map.get(subject) {
            cmd.execute(payload).await;
        } else {
            eprintln!("[Dispatcher] No command registered for subject: {subject}");
        }
    }
}

