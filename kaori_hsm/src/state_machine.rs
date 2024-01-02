use crate::proto_state_machine::ProtoStateMachine;
use crate::sm_business_logic::SMBusinessLogic;
use crate::state::{CoreEvt, StateFn};

//For doc
#[allow(unused_imports)]
use crate::{InitStateMachine, State};

/// Represent a fully functional state machine, which already transitioned to its
/// first state. It may be only created from a call to [`InitStateMachine::init()`].
pub struct StateMachine<UserStateMachine: ProtoStateMachine> {
    pub(crate) user_state_machine: UserStateMachine,
    pub(crate) curr_state: StateFn<UserStateMachine>,
}

impl<UserStateMachine: ProtoStateMachine> StateMachine<UserStateMachine> {
    /// Dispatch an event to the state machine. The event is of the type you have set
    /// in [`ProtoStateMachine::Evt`].
    pub fn dispatch(&mut self, user_evt: &<UserStateMachine as ProtoStateMachine>::Evt) {
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

impl <UserStateMachine: ProtoStateMachine>SMBusinessLogic for StateMachine<UserStateMachine>{
}
