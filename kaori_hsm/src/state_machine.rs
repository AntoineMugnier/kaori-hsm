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
    user_state_machine: UserStateMachine,
    curr_state: StateFn<UserStateMachine>,
}

/// Struct encapsulating the business logic of hierarchical state machine
impl<UserStateMachine: ProtoStateMachine> StateMachine<UserStateMachine> {

    /// Build the kaori_hsm state machine from you structure which implements the
    /// `ProtoStateMachine` trait and as many variants of the [`State<tag>`] trait as
    /// you have states.
    pub fn from(user_state_machine: UserStateMachine) -> StateMachine<UserStateMachine> {
        
        unsafe{
            let default_state =  sm_business_logic::default_state;
            let default_state = default_state as *mut StateFn<UserStateMachine>;
        StateMachine {
            user_state_machine,
            curr_state: core::mem::transmute(default_state),
        }
        }
    }

    /// Will trigger the execution of the initial pseudostate of the state machine by calling
    /// `ProtoStateMachine::init`. That call willl lead to the first state of the machine to be
    /// set.   
    /// This method should only be called once
    pub fn init(&mut self) {
        // Call user top initial pseudostate implementation
        let init_result = self.user_state_machine.init();
        unsafe{
            sm_business_logic::init(
                core::mem::transmute(&mut self.user_state_machine), 
                core::mem::transmute(&mut self.curr_state),
                core::mem::transmute(&init_result)) 
        }
    }

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
