//For doc
#[allow(unused_imports)]
use crate::{StateMachine, InitStateMachine,state::{CoreHandleResult, HandleResult, InitResult, ParentState, State}};

/// Define the initial pseudostate and the type of event variant the state machine can receive.
/// # Example
///```rust
///# use kaori_hsm::*;
/// enum BasicEvt{
/// A,
/// B,
/// C
/// }
///#
///# struct BasicStateMachine{
///# }
///#
///# #[state(super_state= Top)]
///# impl State<S0> for BasicStateMachine{
///#
///#     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///#         match evt{
///#             BasicEvt::A => {
///#               println!("S0-HANDLES-A");
///#               handled!()
///#             },
///#             _ => ignored!()
///#         }
///#     }    
///# }
///
/// impl ProtoStateMachine for BasicStateMachine{
///   type Evt = BasicEvt;
///
///   fn init(&mut self) -> InitResult<Self> {
///     println!("TOP_INIT");
///     init_transition!(S0)  
///   }
/// }
///```
pub trait ProtoStateMachine {
    /// Type that must be defined as an enum by the user in order to define the events which can be handled
    /// by the state machine. The [`State<tag>::handle()`] and the [`StateMachine::dispatch()`] methods of
    /// the state machine accept the type `Evt` as argument.
    type Evt;

    /// First user code to execute during the lifetime of the state machine.
    /// Executing only once, this method allows the user to execute some custom code before
    /// returning the first state to which the state machine will transition.
    /// This method execution is triggered by the call to [`InitStateMachine::init()`].
    /// # Implementation policy
    /// The user must implement this method and return a state which has the top state as its
    /// parent.  
    fn init(&mut self) -> InitResult<Self>;
}
