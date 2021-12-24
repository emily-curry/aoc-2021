use crate::ari_logi_checkpoint::AriLogiCheckpoint;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InstructionState {
    New,
    Complete,
    InProgress(Box<AriLogiCheckpoint>),
}

impl Default for InstructionState {
    fn default() -> Self {
        InstructionState::New
    }
}
