use crate::proto_state_machine::ProtoStateMachine;
use crate::sm_business_logic;
use crate::state::{CoreEvt, StateFn};

//For doc
#[allow(unused_imports)]
use crate::state::State;

/// Type representing a completely functional state machine.
/// Built using [`StateMachine::from()`] from an instance of a  user-defined structure
/// on which has been implemented the `ProtoStateMachine` and `State` traits.
pub struct StateMachine<UserStateMachine: ProtoStateMachine> {
    pub(crate) user_state_machine: UserStateMachine,
    pub(crate) curr_state: StateFn<UserStateMachine>,
}

/// Struct encapsulating the business logic of hierarchical state machine
impl<UserStateMachine: ProtoStateMachine> StateMachine<UserStateMachine> {

    /// Dispatch an event of the type you have attributed to `ProtoStateMachine::Evt`.
    /// The `dispatch()` method should only be called after `init()`, otherwise the framework will
    /// panic
    pub fn dispatch(&mut self, user_evt: &<UserStateMachine as ProtoStateMachine>::Evt) {
        // Dispatch user evt to current state
        let evt = CoreEvt::UserEvt { user_evt };
        unsafe{
            sm_business_logic::dispatch_event(
                core::mem::transmute(&mut self.user_state_machine), 
                core::mem::transmute(&mut self.curr_state),
                core::mem::transmute(&evt)) 
        }
    }
}
