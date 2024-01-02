//! # kaori_hsm State machine library
//! kaori_hsm is a library for developing Hierarchical State Machines (HSMs) in Rust. Lightweigness
//! and execution speed are primary focuses of this library as it is designed to run on systems with
//! low resources such as microcontrollers. As being hardware-independent, the library can
//! be run on any system for which there is a rust compiler available for it.
//! Some of the key advantages of this library are:
//! - No use of dynamic memory allocation
//! - Fast execution, low stack and program memory usage
//! - no use of rust standard library, neither of other external crate
//!
//!## What are Hierarchical state machines ?
//! States machines are software entities processing events differently depending on the state in
//! which they are. Different input events may lead to different actions being performed by the state
//! machine and can trigger transition to other states. 
//!
//! Hierarchical State Machines are state machines which can have nested states. This means that if
//! an event cannot be handled in a state, its super state could eventually handle it.
//! HSMs are therefore particularly useful for designing state machines with complex behavior.
//! 
//! For understanding how state machines and especially HSMs work, I especially recommend the video series
//! made by Miro Samek that you can find [here](https://youtube.com/playlist?list=PLPW8O6W-1chxym7TgIPV9k5E8YJtSBToI&si=mfiiiq3EMLj1bJpH)
//!
//! ## How to use the library (see the code below)?
//! To build your own state machine, you first have to define the structure that will hold its
//! data and then you will need to implement the following traits of the library on it: the [`ProtoStateMachine`]
//! trait and as many variants of the [`State<Tag>`] trait as you want to define states.
//! 
//! The following sequence has to be followed in order to build an operational state machine:
//! - Create an instance of the structure which will hold the data of your state machine.
//! - Encapsulate an instance of this structure into an InitStateMachine instance using the [`InitStateMachine::from()`] function. 
//! - Initialize the state machine by calling the [`InitStateMachine::init()`] method on this instance. It will initialize the state machine and lead
//! it to its first state. A [`StateMachine`] instance will be returned from this method. This type represents a fully operational state machine
//! and only exposes the [`StateMachine::dispatch()`] method used for injecting event variants into it.
//!
//! ## Examples across project
//! This library features many examples that show you its potential and help you understand how to use it. Most of them can be
//! run without any specific hardware.  
//! You will find small examples embedded in the library types and functions definitions composing this library. Those examples
//! focus primarily on featuring the use case of those types and functions.  
//! Then there are more complex examples that you will find in the `kaori_hsm/examples` directory.
//! Those ones are eay to play with and a make a good base for making your own state machines.
//! Integrations tests in the `kaori_hsm/tests` directory can also serve the purpose of examples,
//! but are very rigid and contains a lot of test-specific code.
//! Finally you will find on [this repository](https://github.com/AntoineMugnier/kaori-hsm-perf-test)
//! a project designed to test the performance of this library on a stm32f103c8T6 microcontroller.
//! The performance test may not be easy to understand for a newcomer to the library, but it may be the most practical example.
//!
//! ### A simple hierachical state machine
//! ![intro_hsm](https://github.com/AntoineMugnier/kaori-hsm/blob/assets/intro_fm.png?raw=true)
//! ```rust
//! use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
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
//!     fn exit(&mut self) {
//!        self.post_string("S1-EXIT");
//!     }
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
//!             BasicEvt::A => {
//!                 self.post_string("S1-HANDLES-A");
//!                 handled!()
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! }    
//!  
//! #[state(super_state= S1)]
//! impl State<S11> for BasicStateMachine{
//! 
//!     fn entry(&mut self) {
//!        self.post_string("S11-ENTRY");
//!     }
//!
//!     fn exit(&mut self) {
//!        self.post_string("S11-EXIT");
//!     }
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
//!         BasicEvt::B =>{
//!             self.post_string("S11-HANDLES-B");
//!             transition!(S12)
//!         }
//!             _ => ignored!()
//!         }
//!     }
//! }
//! #[state(super_state= S1)]
//! impl State<S12> for BasicStateMachine{
//!
//!     fn entry(&mut self) {
//!        self.post_string("S12-ENTRY");
//!     }
//!
//!     fn exit(&mut self) {
//!        self.post_string("S12-EXIT");
//!     }
//!
//!     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
//!         match evt{
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
//!# // Panics if the seies of events comming out of the state machine does not match to expectations
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
//!    let ism = InitStateMachine::from(basic_state_machine);
//!
//!    let mut sm = ism.init();
//!    expect_output_series(&receiver, &["TOP_INIT", "S1-ENTRY", "S1-INIT", "S11-ENTRY"]);
//!    
//!    sm.dispatch(&BasicEvt::A);
//!    expect_output_series(&receiver, &["S1-HANDLES-A"]);
//!    
//!    sm.dispatch(&BasicEvt::B);
//!    expect_output_series(&receiver, &["S11-HANDLES-B", "S11-EXIT", "S12-ENTRY"]);
//!```
//! ## Cargo commands index
//! The present directory must be `kaori_hsm` to run every cargo command.
//! ### Building the lib in release mode
//! ```shell 
//! cargo build --release
//! ````
//! ### Running doc test
//! ```shell
//! cargo test --doc
//! ```
//! ### Running a specific integration test
//! ```shell
//! cargo test --test [test_name]
//! ```
//! ### Running a specific example
//! ```shell
//! cargo run --example [example_name]
//! ```

#![no_std]
mod init_state_machine;
mod proto_state_machine;
mod sm_business_logic;
mod state;
mod state_machine;
pub use init_state_machine::InitStateMachine;
pub use proto_state_machine::ProtoStateMachine;
pub use state::{HandleResult, InitResult, ParentState, State};
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
    };
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
    };
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
    () => {
        kaori_hsm::HandleResult::Ignored
    };
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
    () => {
        kaori_hsm::HandleResult::Handled
    };
}
