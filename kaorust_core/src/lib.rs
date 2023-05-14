#![doc = include_str!("../../README.md")]

mod proto_state_machine;
mod state;
mod state_machine;

pub use state::{InitResult, ParentState, State, HandleResult};
pub use proto_state_machine::ProtoStateMachine;
pub use state_machine::StateMachine;
extern crate kaorust_derive;
pub use kaorust_derive::state;

/// Sugar for constructing a `InitResult::TargetState` enum variant containing the target of the
/// initial transition. Can be either used in [`ProtoStateMachine::init`] or [`State<Tag>::init`]
/// # Example
/// ```
///# use kaorust_core::*; 
///# enum BasicEvt{A}
///#
///# struct BasicStateMachine{
///# }
///#
///#
/// impl ProtoStateMachine for BasicStateMachine{
///   type Evt = BasicEvt;
///
///   fn init(&mut self) -> InitResult<Self> {
///     init_transition!(S0)  
///   }
///
/// }
///
/// #[state(super_state= Top)]
/// impl State<S0> for BasicStateMachine{
///    fn init(&mut self) -> InitResult<Self> {
///        println!("S0-INIT");
///        init_transition!(S1)
///    }
///
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///         match evt{
///             _ => ignored!()
///             }
///         }
///     }    
///#  
///# #[state(super_state= Top)]
///# impl State<S1> for BasicStateMachine{
///#  
///#      fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///#          match evt{
///#              _ => ignored!()
///#              }
///#          }
///#  }
///```
#[macro_export]
macro_rules! init_transition {
    ($target_state_tag:ident) => {
        kaorust_core::InitResult::TargetState(kaorust_core::State::<$target_state_tag>::core_handle) 
    }
}

/// Sugar for constructing a `HandleResult::Transition` enum variant containing the target of the
/// transition
/// # Example
/// ```
///# use kaorust_core::*; 
///# enum BasicEvt{A}
///#
///# struct BasicStateMachine{
///# }
///#
///#
///# impl ProtoStateMachine for BasicStateMachine{
///#   type Evt = BasicEvt;
///#
///#   fn init(&mut self) -> InitResult<Self> {
///#    init_transition!(S0)  
///#   }
///#
///# }
///#
///# #[state(super_state= Top)]
///# impl State<S1> for BasicStateMachine{
///#
///#     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///#         match evt{
///#             _ => ignored!()
///#             }
///#         }
///#     }    
///# 
/// #[state(super_state= Top)]
/// impl State<S0> for BasicStateMachine{
/// 
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///         match evt{
///             BasicEvt::A => {
///               println!("S0-HANDLES-A");
///               transition!(S0)
///             }
///         }
///     }    
/// }
///```
#[macro_export]
macro_rules! transition {
    ($target_state_tag:ident) => {
        kaorust_core::HandleResult::Transition(kaorust_core::State::<$target_state_tag>::core_handle) 
    }
}
/// Sugar for constructing a `HandleResult::Ignored` enum variant meaning no event has been handled
/// ```
///# use kaorust_core::*; 
///# enum BasicEvt{A}
///#
///# struct BasicStateMachine{
///# }
///#
///#
///# impl ProtoStateMachine for BasicStateMachine{
///#   type Evt = BasicEvt;
///#
///#   fn init(&mut self) -> InitResult<Self> {
///#    init_transition!(S0)  
///#   }
///#
///# }
///#
/// #[state(super_state= Top)]
/// impl State<S0> for BasicStateMachine{
/// 
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///         match evt{
///             _ => ignored!()
///         }
///     }    
/// }
///```
#[macro_export]
macro_rules! ignored {
    () => {kaorust_core::HandleResult::Ignored}
}
/// Sugar for constructing a `HandleResult::Handle` enum variant meaning the event has been caught
/// without transition occuring.
/// ```
///# use kaorust_core::*; 
///# enum BasicEvt{A}
///#
///# struct BasicStateMachine{
///# }
///#
///#
///# impl ProtoStateMachine for BasicStateMachine{
///#   type Evt = BasicEvt;
///#
///#   fn init(&mut self) -> InitResult<Self> {
///#    init_transition!(S0)  
///#   }
///#
///# }
///#
/// #[state(super_state= Top)]
/// impl State<S0> for BasicStateMachine{
/// 
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///         match evt{
///             A => {
///                 println!("S0-HANDLES-A");
///                 handled!()
///             }
///         }
///     }    
/// }
///```
#[macro_export]
macro_rules! handled {
    () => {kaorust_core::HandleResult::Handled}
}
