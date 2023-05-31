pub use crate::state::{HandleResult, InitResult, ParentState, CoreHandleResult, State};
pub use crate::StateMachine;

/// Define the initial pseudostate and the type of event variant the state machine can receive
/// # Example
///```rust
///# use kaori_hsm::*; 
/// enum BasicEvt{
/// A{counter: u8},
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
///#             BasicEvt::A{counter} => {
///#               println!("counter: {}", counter);
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
pub trait ProtoStateMachine
{
    /// Type that must be implemented by the user in order to define the events that can be handled
    /// by the state machine. Should be defined as an enum with each variants defining a unique
    /// event type. The [`State<tag>::handle()`] and the [`StateMachine::dispatch()`] methods of the state machine accept the type `Evt` as argument.
  type Evt;

    /// Initial pseudostate whose role is to initialize the state machine and lead to its
    /// default state. This method call is triggered by the call to [`StateMachine::init`]
    /// Usually, in the implementation of this method, the user sets up internal objects used by the state machine.
    fn init(&mut self)-> InitResult<Self>;
}
