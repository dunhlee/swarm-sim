use common::message_layer::*;
use common::message_types::*;
use tokio::sync::mpsc;
use tokio::time;
use std::time::{Duration, Instant};
use anyhow::Result;
use common::commands::sim_command::SimCommand;
use crate::sim_state::SimulationState;

// simulation tick rate
pub const TICK_RATE_HZ: u32 = 60;

// period of a tick
pub const TICK_PERIOD: Duration = Duration::from_micros(16_666);

// the simulation loop struct owns these fields
// eventually add outputs
pub struct SimulationLoop
{
    // focuses on receiving commands from remote agents
    command_receiver: mpsc::Receiver<SimCommand>,
    state:SimulationState,
}

// The sim loop has these responsibilities
// - drain commands
// - apply commands
// - progress the world state
// - publish updates
// - track time
impl SimulationLoop
{
    pub fn new(state:SimulationState, command_receiver: mpsc::Receiver<SimCommand>) -> Self {
        Self
        {
            state,
            command_receiver,
        }
    }
}


