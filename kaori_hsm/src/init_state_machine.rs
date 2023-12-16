use crate::proto_state_machine::ProtoStateMachine;
use crate::state::StateFn;
use crate::{sm_business_logic, StateMachine};

//For doc
#[allow(unused_imports)]
use crate::state::State;

/// Type representing a completely functional state machine.
/// Built using [`InitStateMachine::from()`] from an instance of a  user-defined structure
/// on which has been implemented the `ProtoStateMachine` and `State` traits.
pub struct InitStateMachine<UserStateMachine: ProtoStateMachine> {
    user_state_machine: UserStateMachine,
    curr_state: StateFn<UserStateMachine>,
}

impl<UserStateMachine: ProtoStateMachine> InitStateMachine<UserStateMachine> {
    /// Build the kaori_hsm state machine from you structure which implements the
    /// `ProtoStateMachine` trait and as many variants of the [`State<tag>`] trait as
    /// you have states.
    pub fn from(user_state_machine: UserStateMachine) -> InitStateMachine<UserStateMachine> {
        unsafe {
            let default_state = sm_business_logic::default_state;
            let default_state = default_state as *mut StateFn<UserStateMachine>;
            InitStateMachine {
                user_state_machine,
                curr_state: core::mem::transmute(default_state),
            }
        }
    }

    /// Consume the structure instance, triggerring the call to `ProtoStateMachine::init() and
    /// performing transition to the first state. A fully operational state machine
    /// is returned.   
    pub fn init(mut self) -> StateMachine<UserStateMachine> {
        // Call user top initial pseudostate implementation
        let init_result = self.user_state_machine.init();
        unsafe {
            sm_business_logic::init(
                core::mem::transmute(&mut self.user_state_machine),
                core::mem::transmute(&mut self.curr_state),
                core::mem::transmute(&init_result),
            )
        }
        StateMachine {
            user_state_machine: self.user_state_machine,
            curr_state: self.curr_state,
        }
    }
}
