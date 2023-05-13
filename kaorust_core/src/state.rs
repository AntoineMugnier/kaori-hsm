use crate::proto_state_machine::ProtoStateMachine;

pub type StateFn<UserStateMachineT> = fn(
    &mut UserStateMachineT,
    &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>,
) -> CoreHandleResult<UserStateMachineT>;

pub enum HandleResult<UserStateMachineT: ProtoStateMachine + ?Sized> {
    Ignored,
    Handled,
    Transition(StateFn<UserStateMachineT>),
}

pub enum CoreHandleResult<UserStateMachineT: ProtoStateMachine + ?Sized> {
    Ignored(ParentState<UserStateMachineT>),
    Handled,
    Transition(StateFn<UserStateMachineT>),
    GetParentStateResult(ParentState<UserStateMachineT>),
    InitResult(InitResult<UserStateMachineT>),
}

pub enum ParentState<UserStateMachine: ProtoStateMachine + ?Sized> {
    TopReached,
    Exists(StateFn<UserStateMachine>),
}

pub enum InitResult<UserStateMachine: ProtoStateMachine + ?Sized> {
    NotImplemented,
    TargetState(StateFn<UserStateMachine>),
}

pub enum CoreEvt<'a, UserEvtT> {
    InitEvt,
    EntryEvt,
    ExitEvt,
    GetParentStateEvt,
    UserEvt { user_evt: &'a UserEvtT },
}

/// Generic trait which must be implemented on the state machine structure for defining each of its states.
///
/// The `tag` argument, which corresponds to the state name, has no other purpose than to create a
/// unique variant of this trait for the specific state to implement. If the `#[state()]`
/// procedural macro is used, the tag is automatically defined by parsing the state implementation.
/// # Example
/// ```
///# use kaorust_core::*;
///#
///# enum BasicEvt{
///# A,
///# B,
///# C,
///# D
///# }
///#
///# struct BasicStateMachine{
///# }
///#
///#
///# impl ProtoStateMachine for BasicStateMachine{
///#   type Evt = BasicEvt;
///#
///#   fn init(&mut self) -> InitResult<Self> {
///#     println!("TOP_INIT");
///#    init_transition!(S1)  
///#   }
///# }
///#
///#
///#[state(super_state= Top)]
///impl State<S0> for BasicStateMachine{
///
///    fn init(&mut self) -> InitResult<Self> {
///        println!("S0-INIT");
///        init_transition!(S1)
///    }
///
///    fn exit(&mut self) {
///        println!("S0-EXIT");
///    }
///
///    fn entry(&mut self) {
///        println!("S0-ENTRY");
///    }
///
///    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///        match evt{
///            BasicEvt::A => {
///                println!("S0-HANDLES-A");
///                handled!()
///            },
///            BasicEvt::B => {
///                println!("S0-HANDLES-D");
///                transition!(S1)
///           },
///           _ => ignored!()
///        }
///    }    
///}
///# #[state(super_state= S0)]
///# impl State<S1> for BasicStateMachine{
///#
///#     fn exit(&mut self) {
///#         println!("S1-EXIT");
///#     }
///#
///#     fn entry(&mut self) {
///#         println!("S1-ENTRY");
///#     }
///#
///#     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///#         match evt{
///#             BasicEvt::A => {
///#                 println!("S1-HANDLES-A");
///#                 handled!()
///#             },
///#              _ => ignored!()
///#         }
///#     }    
///# }
/// ```
/// *Note: It is recommended to use the `#[state()]` procedural macro before the state implementation
/// in order to limit code verbosity.*
pub trait State<Tag>
where
    Self: ProtoStateMachine,
{
    /// Return the parent state of this state, which can be either
    /// the `top` state or another user-defined state.
    ///
    ///# Implementation policy
    /// Must be implemented for every state
    ///
    /// *Note: This method is automatically implemented if you use the `#[state()]` procedural macro*
    fn get_parent_state() -> ParentState<Self>;

    /// Initial transition of the state. Is called during a transition targeting the present state, after its entry statement has been executed.
    ///
    ///# Implementation policy
    /// The default implementation is mandatory for leaf states.
    /// All non-leaf state must implement this method, otherwise a panic will occur when
    /// a transition for which this state is the target executes.
    /// The implemented method must return only the `TargetState` variant of the enum `InitResult` containing the target substate.
    /// *Note: It is recommended to use the `ìnit_transition!()` macro for returning the target
    /// substate.*
    fn init(&mut self) -> InitResult<Self> {
        InitResult::NotImplemented
    }

    /// Executed when the state machine enters the present state during a transition.
    ///# Implementation policy
    /// The implementation of this method is optional
    ///
    fn entry(&mut self) {
        // No implementation
    }

    /// Executed when the state machine exits the present state during a transition.
    ///
    ///# Implementation policy
    /// The implementation of this method is optional
    fn exit(&mut self) {
        // No implementation
    }

    /// Events injected into the state machine through the `StateMachine::dispatch()` method are
    /// received by this method if the present state is the current state of the state machine.
    ///  
    /// # Implementation policy
    /// No default implementation, must be implemented for every state.
    /// This method implementation is typically a `match` statement on the event variant. The handling of each event may return either:
    /// - `HanbleResult::Transition`: immediately trigger a transition to the target state, which may
    /// become the next current state of the state machine.
    /// - `HandleResult::Handled`: The event is handled without transition
    ///
    /// In the case the event does not match any expected event, `HandleResult::Ignored` must be
    /// returned. In this case, `handle()` methods of upper states will be called with the ignored
    /// event.
    // *Note: Its is recommended to use the provided `transition!()`, `handled!()` and `ignored!()` macros instead of assembling manually the enum variant of `HandleResult`*
    fn handle(&mut self, evt: &<Self as ProtoStateMachine>::Evt) -> HandleResult<Self>;

    #[doc(hidden)]
    fn core_handle(
        &mut self,
        evt: &CoreEvt<<Self as ProtoStateMachine>::Evt>,
    ) -> CoreHandleResult<Self> {
        match evt {
            CoreEvt::InitEvt => {
                return CoreHandleResult::InitResult(<Self as State<Tag>>::init(self));
            }
            CoreEvt::EntryEvt => {
                <Self as State<Tag>>::entry(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::ExitEvt => {
                <Self as State<Tag>>::exit(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::GetParentStateEvt => {
                return CoreHandleResult::GetParentStateResult(Self::get_parent_state());
            }
            CoreEvt::UserEvt { user_evt } => match <Self as State<Tag>>::handle(self, user_evt) {
                HandleResult::Ignored => return CoreHandleResult::Ignored(Self::get_parent_state()),
                HandleResult::Handled => return CoreHandleResult::Handled,
                HandleResult::Transition(state_fn) => CoreHandleResult::Transition(state_fn),
            },
        }
    }
}
