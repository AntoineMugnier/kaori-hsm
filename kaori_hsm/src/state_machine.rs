use crate::proto_state_machine::TopState;
use crate::sm_business_logic::SMBusinessLogic;
use crate::state::{CoreEvt, StateFn};

//For doc
#[allow(unused_imports)]
use crate::{InitStateMachine, State};

/// Represent a fully functional state machine, which already transitioned to its
/// first state. It may be only created from a call to [`InitStateMachine::init()`].
pub struct StateMachine<UserStateMachine: TopState> {
    pub(crate) user_state_machine: UserStateMachine,
    pub(crate) curr_state: StateFn<UserStateMachine>,
}

impl<UserStateMachine: TopState> StateMachine<UserStateMachine> {
    /// Dispatch an event to the state machine. The event is of the type you have set
    /// in [`TopState::Evt`].
    pub fn dispatch(&mut self, user_evt: &<UserStateMachine as TopState>::Evt) {
        let evt = CoreEvt::UserEvt { user_evt };
        unsafe {
            <Self as SMBusinessLogic>::dispatch_evt_to_current_state(
                core::mem::transmute(&mut self.user_state_machine),
                core::mem::transmute(&mut self.curr_state),
                core::mem::transmute(&evt),
            )
        }
    }
}

impl<UserStateMachine: TopState> SMBusinessLogic for StateMachine<UserStateMachine> {}
