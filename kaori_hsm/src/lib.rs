//! # kaori_hsm State machine framework 
//! kaori_hsm is a framework for developing Hierarchical State Machine(HSM) in Rust. Lightweight
//! performant, and free of any dynamic memory allocation, it is ideal for firmware development,
//! but it can also be used for any other kind of application.
//!
//!# What are Hierarchical state machines ?
//! States machines are software enties processing events differently depending on the state
//! they are. A state machine starts in an intial state. Different input events may lead to 
//! different actions being performed by the state machine and can trigger transition to other
//! states. In a conventional (flat) state machine, states are parallel and do not share behavior.
//! 
//! Hierarchical State Machines are state machines which can have nested states. By
//! having the capability of inheriting the behavior of upper states, code duplication is avoided
//! and the state machine become more comprehensible. HSMs are particularly useful for designing 
//! state machines with complex behavior and a lot of states.
//!
//! ## How to
//! To build your own state machine, you first have to define the structure that will hold its
//! data and then you will need to implement the following traits of the framework on it: the [`ProtoStateMachine`]
//! trait and as many variant of the [`State<Tag>`] trait as you want to define states.
//!
//! After that, you will assemble a complete `StateMachine` by sending an instance of your structure which
//! implements the mentioned traits as argument to the [`StateMachine::from()`] function.
//! 
//! A single call to the [`StateMachine::init()`] method will initialize the state machine and lead
//! it to its first state.It will after be ready to process events through the [`StateMachine::dispatch()`] method
//!
//!```rust
//!# use std::sync::mpsc::channel;
//!# use std::sync::mpsc::Receiver;
//!# use std::sync::mpsc::Sender;
//!# use std::sync::mpsc::TryRecvError;
//! use kaori_hsm::*; 
//! enum BasicEvt{
//!     A,
//!     B,
//!     C
//! }
//!
//! struct BasicStateMachine{
//!    sender: Sender<String>,
//! }
//!
//! impl BasicStateMachine{
//!     pub fn new(sender: Sender<String>) -> BasicStateMachine {
//!        BasicStateMachine { sender }
//!    }
//!
//!     fn post_string(&self, s : &str){
//!         self.sender.send(String::from(s)).unwrap();
//!     }
//! }
//!
//! impl ProtoStateMachine for BasicStateMachine{
//!   type Evt = BasicEvt;
//!
//!   fn init(&mut self) -> InitResult<Self> {
//!       self.post_string("TOP_INIT"); 
//!       init_transition!(S1)
//!   }
//! }
//!
//! #[state(super_state= Top)]
//! impl State<S1> for BasicStateMachine{
//!
//!     fn init(&mut self) -> InitResult<Self> {
//!         self.post_string("S1-INIT");
//!         init_transition!(S11)
//!     }
//!
//!     fn entry(&mut self) {
//!        self.post_string("S1-ENTRY");
//!     }
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
//!             BasicEvt::A => {
//!                 self.post_string("S1-HANDLES-A");
//!                 handled!()
//!             }
//!             BasicEvt::C =>{
//!                 self.post_string("S1-HANDLES-C");
//!                 transition!(S2)
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! }    
//!  
//! #[state(super_state= S1)]
//! impl State<S11> for BasicStateMachine{
//!
//!     fn exit(&mut self) {
//!        self.post_string("S11-EXIT");
//!     }
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
//!             BasicEvt::B => {
//!                  self.post_string("S11-HANDLES-B");
//!                  transition!(S2)
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! }
//!
//!#[state(super_state= Top)]
//! impl State<S2> for BasicStateMachine{
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
//!            BasicEvt::A => {
//!                 self.post_string("S2-HANDLES-A");
//!                 transition!(S1)
//!             }
//!            BasicEvt::B => {
//!                 self.post_string("S2-HANDLES-B");
//!                 handled!()
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! }
//!
//!# fn collect_sm_output(receiver: &Receiver<String>) -> String {
//!#     receiver.try_recv().unwrap_or_else(|err| match err {
//!#         TryRecvError::Empty => panic!("Too many expectations for the SM output"),
//!#         TryRecvError::Disconnected => panic!("Disconnected"),
//!#     })
//!# }
//!# 
//!# fn expect_output_series(receiver:  &Receiver<String>, expectations: &[&str]) {
//!#     for (index, &expectation) in expectations.iter().enumerate() {
//!#         let sm_output = collect_sm_output(receiver);
//!#         if expectation != sm_output {
//!#             panic!(
//!#                 "Expectation index {},  expected : {},  got: {}",
//!#                 index, expectation, sm_output
//!#             )
//!#         }
//!#     }
//!# 
//!#     // Check that we have expected all the output of the SM
//!#     match receiver.try_recv().err() {
//!#         Some(TryRecvError::Empty) => { /* OK */ }
//!#         Some(TryRecvError::Disconnected) => {
//!#             panic!(" Sender is dead")
//!#         }
//!#         None => {
//!#             panic!("Too few expectations for the SM output")
//!#         }
//!#     }
//!# }
//!
//!    let (sender, mut receiver) = channel();
//! 
//!    let basic_state_machine = BasicStateMachine::new(sender);
//!
//!    let mut sm = StateMachine::from(basic_state_machine);
//!    
//!    sm.init();
//!    expect_output_series(&receiver, &["TOP_INIT", "S1-ENTRY", "S1-INIT"]);
//!    
//!    sm.dispatch(&BasicEvt::A);
//!    expect_output_series(&receiver, &["S1-HANDLES-A"]);
//!    
//!    sm.dispatch(&BasicEvt::B);
//!    expect_output_series(&receiver, &["S11-HANDLES-B", "S11-EXIT"]);
//!
//!    sm.dispatch(&BasicEvt::B);
//!    expect_output_series(&receiver, &["S2-HANDLES-B"]);
//!
//!    sm.dispatch(&BasicEvt::A);
//!    expect_output_series(&receiver, &["S2-HANDLES-A", "S1-ENTRY", "S1-INIT"]);
//!    
//!    sm.dispatch(&BasicEvt::C);
//!    expect_output_series(&receiver, &["S1-HANDLES-C", "S11-EXIT"]);
//!
//!```

#![no_std]
mod proto_state_machine;
mod state;
mod state_machine;
mod sm_business_logic;

pub use state::{InitResult, ParentState, State, HandleResult};
pub use proto_state_machine::ProtoStateMachine;
pub use state_machine::StateMachine;
extern crate kaori_hsm_derive;
pub use kaori_hsm_derive::state;

/// Sugar for constructing a `InitResult::TargetState` enum variant containing the target of the
/// initial transition. Can be either used in [`ProtoStateMachine::init`] or [`State<Tag>::init`]
/// # Example
/// ```
///# use kaori_hsm::*; 
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
        kaori_hsm::InitResult::TargetState(kaori_hsm::State::<$target_state_tag>::core_handle) 
    }
}

/// Sugar for constructing a `HandleResult::Transition` enum variant containing the target of the
/// transition
/// # Example
/// ```
///# use kaori_hsm::*; 
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
        kaori_hsm::HandleResult::Transition(kaori_hsm::State::<$target_state_tag>::core_handle) 
    }
}
/// Sugar for constructing a `HandleResult::Ignored` enum variant meaning no event has been handled
/// ```
///# use kaori_hsm::*; 
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
    () => {kaori_hsm::HandleResult::Ignored}
}
/// Sugar for constructing a `HandleResult::Handle` enum variant meaning the event has been caught
/// without transition occuring.
/// ```
///# use kaori_hsm::*; 
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
///             BasicEvt::A => {
///                 println!("S0-HANDLES-A");
///                 handled!()
///             }
///         }
///     }    
/// }
///```
#[macro_export]
macro_rules! handled {
    () => {kaori_hsm::HandleResult::Handled}
}
