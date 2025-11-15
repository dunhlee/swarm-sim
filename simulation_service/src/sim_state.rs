use tokio::sync::mpsc;
use common::commands::command::Command;

pub struct SimulationState
{
    pub tick: u64,
}

impl SimulationState
{
    pub fn new() -> Self
    {
        Self {
            tick: 0,
        }
    }
}