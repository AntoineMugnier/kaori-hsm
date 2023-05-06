//! Kaorust is a Hierarchical State Machine (HSM) framework developed in Rust at destination of
//! embedded system. It can also be used safely on and desktop computers.  
//!
//!# Performance
//! The framework is designed to be lightweight, holding just a few kilobytes in memory.
//! It also has the advantages of being free of dynamic allocation and of virtual tables.
//! 
//! To build your own state machine, you first have to define the structure that will hold its
//! data and then you will need to implement The following traits of the framework on it: the `ProtoStateMachine`
//! trait and as many variant of the `State` trait as you want to define states. After that, you
//! will assemble a complete `StateMachine` by sending your state machine as argument to the
//! `StateMachine::from()` function.  
//!
//! The ProtoStateMachine trait requires you to implement the `ProtoStateMachine::Init()` method
//! which will be called at the initialization of the machine and set the first state.
//! The trait also requires you to define the `ProtoStateMachine::Evt` type, which will be the type of the enum
//! variants that you state machine will be able to accept in the `StateMachine::dispatch()`
//! method.  
//! The State trait is generic and must be implemented for each of the states you want to define for
//! your HSM. Each implementation of the `State` trait will require you to implement the following
//! methods:  
//! - `State::init()` : the initial transition. Only for leaf states. Triggered when
//! this state is the target of the transition. Returns a target substate to which we will enter
//! next.
//! - `State::entry()`: the entry statement. Called each time we enter this state
//! - `State::exit()`: the exit statement. Called each time we exit this state
//! - `State::handle()`: the exit statement. Called every time we dispatch an event, only if this
//! state is the current state of the HSM.
//!
mod proto_state_machine;
mod state;
mod state_machine;

pub use state::{InitResult, ParentState, State, HandleResult};
pub use proto_state_machine::ProtoStateMachine;
pub use state_machine::StateMachine;

