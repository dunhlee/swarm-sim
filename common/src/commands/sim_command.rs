use glam::Vec3;

// simple commands for simulation, for now
pub enum SimCommand
{
    JoinSession {player: String},
    SpawnAgent {id: u64},
    MoveAgent {id: u64, dir: Vec3},
    Pause
}