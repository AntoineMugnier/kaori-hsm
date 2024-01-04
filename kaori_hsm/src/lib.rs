//! # kaori_hsm state machine library
//! kaori_hsm is a library for developing Hierarchical State Machines (HSMs) in Rust. Low memory
//! footprint and execution speed are primary focuses of this library as it is designed to 
//! run on systems with low resources such as microcontrollers. As being hardware-independent,
//! the library can be run on any system for which there is a rust compiler available for it.
//! Some of the key advantages of this library are:
//! - No use of dynamic memory allocation
//! - Fast execution, low stack and program memory usage
//! - no use of rust standard library, nor any other external crate
//!## What are hierarchical state machines ?
//! States machines are software entities processing events differently depending on the state in
//! which they are. Different input events may lead to different actions being performed by the state
//! machine and can trigger transitions to other states.
//!
//! Hierarchical State Machines are state machines which can have nested states. This means that if
//! an event cannot be handled in a state, its super state could eventually handle it.
//! HSMs are therefore particularly useful for designing state machines with complex behavior.
//!
//! For understanding how state machines and especially HSMs work, I especially recommend the video series
//! made by Miro Samek that you can find [here](https://youtube.com/playlist?list=PLPW8O6W-1chxym7TgIPV9k5E8YJtSBToI&si=mfiiiq3EMLj1bJpH)
//!
//! ## How to use the library ?
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
//! ## Examples across the  project
//! This library features many examples that show you its potential and help you understand how to use it. Most of them can be
//! run without any specific hardware.  
//! You will find small examples embedded in the library types and functions definitions composing this library. Those examples
//! focus primarily on featuring the use case of those types and functions.  
//! Then there are more complex examples that you will find in the `kaori_hsm/examples` directory.
//! Those are easy to play with and a make a good base for making your own state machines.
//! Integrations tests in the `kaori_hsm/tests` directory can also serve the purpose of examples,
//! but are very rigid and contain a lot of test-specific code.
//! Finally you will find on [this repository](https://github.com/AntoineMugnier/kaori-hsm-perf-test)
//! a project designed to test the performance of this library on a stm32f103c8T6 microcontroller.
//! The performance test may not be easy to understand for a newcomer to the library, but it may be the most practical example.
//!
//! ## An introductory hierarchical state machine example
//! The following example features an hypothetical state machine written using the `kaori_hsm` library. This HSM simulates the blinking
//! of a led depending on the change of the state of a button. When the state machine boots up, the led is
//! off. At the time the button is pressed, the led starts blinking. When the button is released, the led
//! stop blinking.
//! This example is associated with a testing code. The test uses a queue onto which the HSM posts a
//! specific string every time it takes a specific action. After initializing the HSM or dispatching
//! an event to it, the test code checks that the series of strings on the queue matches the expectation.
//!
//! ![intro_hsm](https://github.com/AntoineMugnier/kaori-hsm/blob/assets/intro_sm.png?raw=true)
//! ```rust
//! use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
//! use kaori_hsm::*;
//! enum BlinkingEvent{
//!     ButtonPressed,
//!     ButtonReleased,
//!     TimerTick,
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
//!     // Post a string to the test queue
//!     fn post_string(&self, s : &str){
//!         self.sender.send(String::from(s)).unwrap();
//!     }
//! }
//!
//! impl ProtoStateMachine for BasicStateMachine{
//!   type Evt = BlinkingEvent;
//!
//!   fn init(&mut self) -> InitResult<Self> {
//!       self.post_string("Starting HSM");
//!       init_transition!(BlinkingDisabled)
//!   }
//!}
//!
//! #[state(super_state= Top)]
//! impl State<BlinkingDisabled> for BasicStateMachine{
//!
//!     fn handle(&mut self, evt: & BlinkingEvent) -> HandleResult<Self> {
//!         match evt{
//!             BlinkingEvent::ButtonPressed => {
//!                 self.post_string("Button pressed");
//!                 transition!(BlinkingEnabled)
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! } 
//! #[state(super_state= Top)]
//! impl State<BlinkingEnabled> for BasicStateMachine{
//!
//!     fn entry(&mut self) {
//!        self.post_string("Arm timer");
//!     }
//!
//!     fn exit(&mut self) {
//!        self.post_string("Disarm timer");
//!     }
//!
//!     fn init(&mut self) -> InitResult<Self>{
//!         init_transition!(LedOn)
//!     }
//!
//!     fn handle(&mut self, evt: & BlinkingEvent) -> HandleResult<Self> {
//!         match evt{
//!             BlinkingEvent::ButtonReleased => {
//!                 self.post_string("Button released");
//!                 transition!(BlinkingDisabled)
//!             }
//!             _ => ignored!()
//!         }
//!     }
//! }    
//!  
//! #[state(super_state= BlinkingEnabled)]
//! impl State<LedOn> for BasicStateMachine{
//!
//!     fn entry(&mut self) {
//!        self.post_string("Led turned on");
//!     }
//!
//!     fn exit(&mut self) {
//!        self.post_string("Led turned off");
//!     }
//!
//!     fn handle(&mut self, evt: & BlinkingEvent) -> HandleResult<Self> {
//!         match evt{
//!         BlinkingEvent::TimerTick =>{
//!             self.post_string("Timer tick");
//!             transition!(LedOff)
//!         }
//!             _ => ignored!()
//!         }
//!     }
//! }
//!
//! #[state(super_state= BlinkingEnabled)]
//! impl State<LedOff> for BasicStateMachine{
//!
//!     fn handle(&mut self, evt: & BlinkingEvent) -> HandleResult<Self> {
//!         match evt{
//!         BlinkingEvent::TimerTick =>{
//!             self.post_string("Timer tick");
//!             transition!(LedOn)
//!         }
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
//!# // Panics if the seies of events comming out of the state machine does not match to expectations
//!# fn assert_eq_sm_output(receiver:  &Receiver<String>, expectations: &[&str]) {
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
//!    // Execute the topmost initial transition of the state machine, leading to BlinkingDisabled
//!    // state
//!    let mut sm = ism.init();
//!    assert_eq_sm_output(&receiver, &["Starting HSM"]);
//!     
//!    // Event ButtonReleased is ignored in this state
//!    sm.dispatch(&BlinkingEvent::ButtonReleased);
//!    assert_eq_sm_output(&receiver, &[]);
//!    
//!    sm.dispatch(&BlinkingEvent::ButtonPressed);
//!    assert_eq_sm_output(&receiver, &["Button pressed", "Arm timer","Led turned on"]);
//! 
//!    sm.dispatch(&BlinkingEvent::TimerTick);
//!    assert_eq_sm_output(&receiver, &["Timer tick", "Led turned off"]);
//!
//!    sm.dispatch(&BlinkingEvent::TimerTick);
//!    assert_eq_sm_output(&receiver, &["Timer tick", "Led turned on"]);
//!
//!    sm.dispatch(&BlinkingEvent::ButtonReleased);
//!    assert_eq_sm_output(&receiver, &["Button released","Led turned off", "Disarm timer"]);
//!```
//! ## Cargo commands index
//! The present directory must be `kaori_hsm/kaori_hsm` to run every cargo command.
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
//! ### Running a specific example from the `examples` directory
//! ```shell
//! cargo run --example [example_name]
//! ```
//! 
//! ## License
//! 
//! Licensed under either of
//! 
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//! 
//! at your option.

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
