//! Kaorust crate allows its users to develop efficiently Hierarchical State Machine (HSM) in code
//! events to it
mod proto_state_machine;
mod state;
mod state_machine;
mod misc;

pub use misc::{InitResult, ParentState};
pub use state::{State, HandleResult};
pub use proto_state_machine::ProtoStateMachine;
pub use state_machine::StateMachine;

