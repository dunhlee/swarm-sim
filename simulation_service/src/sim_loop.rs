use common::message_layer::*;
use common::message_types::*;
use tokio::sync::mpsc;
use tokio::time;
use std::time::{Duration, Instant};
use anyhow::Result;
use common::commands::command::Command;
use crate::sim_state::SimulationState;

// simulation tick rate
pub const TICK_RATE_HZ: u32 = 60;

// period of a tick
pub const TICK_PERIOD: Duration = Duration::from_micros(16_666);

pub struct SimulationLoop
{
    // focuses on receiving commands from remote agents
    command_receiver: mpsc::Receiver<Box<dyn Command>>,
    state:SimulationState,
}

impl SimulationLoop
{
    pub fn new(command_receiver: mpsc::Receiver<Box<dyn Command>>, state:SimulationState) -> Self {
        Self
        {
            command_receiver,
            state
        }
    }
}


