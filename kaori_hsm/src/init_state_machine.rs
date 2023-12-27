use crate::proto_state_machine::ProtoStateMachine;
use crate::{sm_business_logic::SMBusinessLogic, StateMachine};

//For doc
#[allow(unused_imports)]
use crate::state::State;

/// Represents a precursor to a state machine, waiting to be initialized.
/// Built using [`InitStateMachine::from()`] from an instance of a  user-defined structure
/// on which has been implemented the `ProtoStateMachine` and `State` traits.
pub struct InitStateMachine<UserStateMachine: ProtoStateMachine> {
    user_state_machine: UserStateMachine,
}

impl<UserStateMachine: ProtoStateMachine> InitStateMachine<UserStateMachine> {
    /// Build the precursor state machine from you structure which implements the
    /// `ProtoStateMachine` trait and as many variants of the [`State<tag>`] trait as
    /// you have states.
    pub fn from(user_state_machine: UserStateMachine) -> InitStateMachine<UserStateMachine> {
        InitStateMachine { user_state_machine }
    }

    /// Consume the structure instance, triggerring the call to [`ProtoStateMachine::init()`] and
    /// performing transition to the first state. A fully operational state machine
    /// is returned.   
    pub fn init(mut self) -> StateMachine<UserStateMachine> {
        // Call user top initial pseudostate implementation
        let init_result = self.user_state_machine.init();

        unsafe {
            let curr_state_fn = <Self as SMBusinessLogic>::init(
                core::mem::transmute(&mut self.user_state_machine),
                core::mem::transmute(&init_result),
            );

            StateMachine {
                user_state_machine: self.user_state_machine,
                curr_state: core::mem::transmute(curr_state_fn),
            }
        }
    }
}

impl <UserStateMachine: ProtoStateMachine>SMBusinessLogic for InitStateMachine<UserStateMachine>{

}
