use crate::proto_state_machine::TopState;
use crate::{sm_business_logic::SMBusinessLogic, StateMachine};

//For doc
#[allow(unused_imports)]
use crate::state::State;

/// Represent a precursor to a state machine, waiting to be initialized.
/// Built using [`InitStateMachine::from()`] from an instance of a  user-defined structure
/// on which has been implemented the [`TopState`] and [`State`] traits.
pub struct InitStateMachine<UserStateMachine: TopState> {
    user_state_machine: UserStateMachine,
}

impl<UserStateMachine: TopState> InitStateMachine<UserStateMachine> {
    /// Build the precursor state machine from you structure which implements the
    /// `TopState` trait and as many variants of the [`State<tag>`] trait as
    /// you have states.
    pub fn from(user_state_machine: UserStateMachine) -> InitStateMachine<UserStateMachine> {
        InitStateMachine { user_state_machine }
    }

    /// Consume the structure instance, triggerring the call to [`TopState::init()`] and
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

impl<UserStateMachine: TopState> SMBusinessLogic for InitStateMachine<UserStateMachine> {}
