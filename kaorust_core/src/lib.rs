//! Kaorust crate allows its users to develop efficiently Hierarchical State Machine (HSM) in code
//! events to it
pub mod proto_state_machine;
pub mod state;
pub mod state_machine;
pub mod misc;

pub use misc::{InitResult, ParentState};
pub use state::{State, HandleResult};
pub use proto_state_machine::ProtoStateMachine;
pub use state_machine::StateMachine;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
