use crate::proto_state_machine::ProtoStateMachine;
use crate::state::{denatured, CoreEvt, CoreHandleResult, InitResult, ParentState, StateFn};

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

trait BusinessLogic{

    fn default_state(
        _user_sm: denatured::OpaquePtr,
        _evt: &denatured::CoreEvt,
    ) -> denatured::CoreHandleResult {
        panic!("dispatch() function called on state_machine before init")
    }


    fn find_dissociate_states<'a>(
        original_state_link: &'a Link<'a>,
        target_state_link: &'a Link<'a>,
    ) -> (
        Option<&'a Link<'a>>,
        Option<&'a Link<'a>>
    ) {
        let mut original_state_link = original_state_link;
        let mut target_state_link = target_state_link;

        loop {
            if original_state_link.state_fn as *const fn()
                != target_state_link.state_fn as *const fn()
            {
                return (Some(original_state_link), Some(target_state_link));
            }

            if let Some(next_original_state_link) = original_state_link.next_link {
                if let Some(next_target_state_link) = target_state_link.next_link {
                    original_state_link = next_original_state_link;
                    target_state_link = next_target_state_link;
                } else {
                    return (original_state_link.next_link, None);
                }
            } else {
                if target_state_link.next_link.is_some() {
                    return (None, target_state_link.next_link);
                } else {
                    return (None, None);
                }
            }
        }
    }

    fn enter_substates(
        user_state_machine: denatured::OpaquePtr,
        target_state_link: &Link,
    )  {
        let mut target_state_link = target_state_link;

        while let Some(next_state) = target_state_link.next_link {
            Self::dispatch_entry_evt(user_state_machine, next_state.state_fn);
            target_state_link = next_state;
        }
    }

    fn reach_target_state(
        user_state_machine: denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        original_state_link: Link,
        target_state_link: Link,
    ) {
        if let denatured::ParentState::Exists(original_state_parent_fn) = Self::dispatch_get_super_state(
            user_state_machine,
            original_state_link.state_fn,
        ) {
            let new_original_state_link = Link {
                state_fn: original_state_parent_fn,
                next_link: Some(&original_state_link),
            };

            if let denatured::ParentState::Exists(target_state_parent_fn) = Self::dispatch_get_super_state(
                user_state_machine,
                target_state_link.state_fn,
            ) {
                let new_target_state_link = Link {
                    state_fn: target_state_parent_fn,
                    next_link: Some(&target_state_link),
                };
                Self::reach_target_state( user_state_machine, current_state_fn, new_original_state_link, new_target_state_link)
            } else {
                Self::reach_target_state( user_state_machine, current_state_fn, new_original_state_link, target_state_link)
            }
        } else {
            if let denatured::ParentState::Exists(parent_state_fn) = Self::dispatch_get_super_state(
                user_state_machine,
                target_state_link.state_fn,
            ) {
                let new_target_state_link = Link {
                    state_fn: parent_state_fn,
                    next_link: Some(&target_state_link),
                };
                Self::reach_target_state(user_state_machine,current_state_fn, original_state_link, new_target_state_link)
            } else {
                let (dissociated_original_state, dissociated_target_state) =
                    Self::find_dissociate_states(&original_state_link, &target_state_link);

                if let Some(dissociated_original_state_link) = dissociated_original_state {
                    Self::exit_substates(
                        user_state_machine,
                        current_state_fn,
                        dissociated_original_state_link.state_fn,
                    );
                    Self::dispatch_exit_evt(
                        user_state_machine,
                        dissociated_original_state_link.state_fn,
                    );
                }

                if let Some(dissociated_target_state_link) = dissociated_target_state {
                    Self::dispatch_entry_evt(
                        user_state_machine,
                        dissociated_target_state_link.state_fn,
                    );
                    Self::enter_substates(
                        user_state_machine,
                        dissociated_target_state_link,
                    );
                }
            }
        }
    }

    fn dispatch_get_super_state(
        user_state_machine: denatured::OpaquePtr,
        state_fn: denatured::StateFn,
    ) -> denatured::ParentState {
        let get_parent_state_evt =
            denatured::CoreEvt::GetParentStateEvt;

        let core_handle_result = state_fn(user_state_machine, &get_parent_state_evt);

        if let denatured::CoreHandleResult::GetParentStateResult(parent_state_fn) = core_handle_result {
            parent_state_fn
        } else {
            panic!("Variant returned by state fn is not ParentState")
        }
    }


    fn dispatch_exit_evt(
        user_state_machine: denatured::OpaquePtr,
        state_fn: denatured::StateFn,
    ) {
        let exit_evt = denatured::CoreEvt::ExitEvt;
        state_fn(user_state_machine, &exit_evt);
    }
   
    fn dispatch_init_evt(
        user_state_machine: denatured::OpaquePtr,
        state_fn: denatured::StateFn,
    ) -> denatured::InitResult {
        let init_evt = denatured::CoreEvt::InitEvt;
        let init_result = state_fn(user_state_machine, &init_evt);
        match init_result {
            denatured::CoreHandleResult::InitResult(init_result) => return init_result,
            _ => panic!("Variant returned by state fn is not InitResult")

        }
    }

    fn reach_init_target(
        user_state_machine: denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        target_state_fn: denatured::StateFn
    )  {
        let mut current_target_state_fn = target_state_fn;

        while let denatured::InitResult::TargetState(next_target_state) =
            Self::dispatch_init_evt(user_state_machine, current_target_state_fn)
        {
            current_target_state_fn = next_target_state;
            Self::dispatch_entry_evt(user_state_machine, current_target_state_fn);
        }

        *current_state_fn = current_target_state_fn;
    }

    fn exit_substates(
        user_state_machine: denatured::OpaquePtr,
        curr_state_fn: &mut denatured::StateFn,
        target_state_fn: denatured::StateFn,
    )  {
        let mut next_state_fn = *curr_state_fn;

        while next_state_fn != target_state_fn {
            if let denatured::ParentState::Exists(parent_state_fn) =
                Self::dispatch_get_super_state(user_state_machine, next_state_fn)
            {
                Self::dispatch_exit_evt(user_state_machine, next_state_fn);
                next_state_fn = parent_state_fn;
            } else {
                panic!("Target state not found when ascending state hierarchy")
            }
        }
        *curr_state_fn = next_state_fn
    }

    fn dispatch_entry_evt(
       user_state_machine: denatured::OpaquePtr,
       state_fn: denatured::StateFn
   ) {
       let entry_evt = denatured::CoreEvt::EntryEvt;
       state_fn(user_state_machine, &entry_evt);
   }

    /// Will trigger the execution of the initial pseudostate of the state machine by calling
    /// `ProtoStateMachine::init`. That call will lead to the first state of the machine to be
    /// set.   
    /// This method should only be called once
    fn init(user_state_machine : denatured::OpaquePtr, current_state_fn: &mut denatured::StateFn, init_result : &denatured::InitResult) {
        match init_result {
            denatured::InitResult::TargetState(topmost_init_target_state_fn) => {
                // Reach leaf state
                Self::dispatch_entry_evt(user_state_machine, *topmost_init_target_state_fn);

                Self::reach_init_target(user_state_machine, current_state_fn, *topmost_init_target_state_fn);
            }

            denatured::InitResult::NotImplemented => panic!("Topmost Init should return a state"),
        }
    }

    fn handle_transition(
        user_state_machine : denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        handling_state_fn: denatured::StateFn,
        target_state_fn: denatured::StateFn,
    ) {
        Self::exit_substates(
            user_state_machine,
            current_state_fn,
            handling_state_fn,
        );
        
        *current_state_fn = handling_state_fn;

        // Special handling in case of targetting the current state
        if handling_state_fn  == target_state_fn {
            Self::dispatch_exit_evt(user_state_machine, handling_state_fn);
            Self::dispatch_entry_evt(user_state_machine, handling_state_fn);
        }

        let curr_state_link = Link {
            state_fn: handling_state_fn,
            next_link: None,
        };
        let target_state_link = Link {
            state_fn: target_state_fn,
            next_link: None,
        };

        Self::reach_target_state(user_state_machine, current_state_fn, curr_state_link, target_state_link);

        let curr_state_after_target_reached = target_state_fn;
        Self::reach_init_target(user_state_machine, current_state_fn, curr_state_after_target_reached);
    }

    fn handle_ignored_evt(
        user_state_machine : denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        parent_state_variant: denatured::ParentState,
        evt: &denatured::CoreEvt,
    ) {
        match parent_state_variant {
            denatured::ParentState::Exists(super_state) => Self::dispatch_core_event(user_state_machine, current_state_fn, super_state, evt),
            denatured::ParentState::TopReached => {}
        }
    }

    fn dispatch_core_event(
        user_state_machine : denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        handling_state_fn: denatured::StateFn,
        evt: &denatured::CoreEvt,
    ){
        let core_handle_result = handling_state_fn(user_state_machine, evt);

        // Treat result of the event dispatch
        match core_handle_result {
            denatured::CoreHandleResult::Handled => {}
            denatured::CoreHandleResult::Ignored(parent_state_fn) => {
                Self::handle_ignored_evt(user_state_machine, current_state_fn, parent_state_fn, evt)
            }
            denatured::CoreHandleResult::Transition(target_state_fn) => {
                Self::handle_transition(user_state_machine, current_state_fn, *current_state_fn, target_state_fn)
            }
            _ => {}
        }
    }
    
    fn dispatch_event(
        user_state_machine : denatured::OpaquePtr,
        current_state_fn: &mut denatured::StateFn,
        evt: &denatured::CoreEvt
    ){
        
        Self::dispatch_core_event(user_state_machine, current_state_fn, *current_state_fn, evt)
    }

}
impl <UserStateMachine: ProtoStateMachine> BusinessLogic for StateMachine<UserStateMachine>{

} 
struct Link<'a> {
    state_fn: denatured::StateFn,
    next_link: Option<&'a Link<'a>>,
}

/// Struct encapsulating the business logic of hierarchical state machine
impl<UserStateMachine: ProtoStateMachine> StateMachine<UserStateMachine> {
    fn default_state(
        _user_sm: &mut UserStateMachine,
        _evt: &CoreEvt<<UserStateMachine as ProtoStateMachine>::Evt>,
    ) -> CoreHandleResult<UserStateMachine> {
        panic!("dispatch() function called on state_machine before init")
    }

    /// Build the kaori_hsm state machine from you structure which implements the
    /// `ProtoStateMachine` trait and as many variants of the [`State<tag>`] trait as
    /// you have states.
    pub fn from(user_state_machine: UserStateMachine) -> StateMachine<UserStateMachine> {
        StateMachine {
            user_state_machine,
            curr_state: Self::default_state,
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
            <StateMachine<UserStateMachine> as BusinessLogic>::init(
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
            <StateMachine<UserStateMachine> as BusinessLogic>::dispatch_event(
                core::mem::transmute(&mut self.user_state_machine), 
                core::mem::transmute(&mut self.curr_state),
                core::mem::transmute(&evt)) 
        }
    }
}
