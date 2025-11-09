use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationUpdate {
    pub id: u32,
    pub tick: u64,
    pub position: [f32; 3],
}