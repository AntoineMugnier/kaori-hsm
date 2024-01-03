# kaori-hsm

## kaori_hsm state machine library
kaori_hsm is a library for developing Hierarchical State Machines (HSMs) in Rust. Low memory
footprint and execution speed are primary focuses of this library as it is designed to
run on systems with low resources such as microcontrollers. As being hardware-independent,
the library can be run on any system for which there is a rust compiler available for it.
Some of the key advantages of this library are:
- No use of dynamic memory allocation
- Fast execution, low stack and program memory usage
- no use of rust standard library, nor any other external crate
### What are hierarchical state machines ?
States machines are software entities processing events differently depending on the state in
which they are. Different input events may lead to different actions being performed by the state
machine and can trigger transitions to other states.

Hierarchical State Machines are state machines which can have nested states. This means that if
an event cannot be handled in a state, its super state could eventually handle it.
HSMs are therefore particularly useful for designing state machines with complex behavior.

For understanding how state machines and especially HSMs work, I especially recommend the video series
made by Miro Samek that you can find [here](https://youtube.com/playlist?list=PLPW8O6W-1chxym7TgIPV9k5E8YJtSBToI&si=mfiiiq3EMLj1bJpH)

### How to use the library ?
To build your own state machine, you first have to define the structure that will hold its
data and then you will need to implement the following traits of the library on it: the [`ProtoStateMachine`]
trait and as many variants of the [`State<Tag>`] trait as you want to define states.

The following sequence has to be followed in order to build an operational state machine:
- Create an instance of the structure which will hold the data of your state machine.
- Encapsulate an instance of this structure into an InitStateMachine instance using the [`InitStateMachine::from()`] function.
- Initialize the state machine by calling the [`InitStateMachine::init()`] method on this instance. It will initialize the state machine and lead
it to its first state. A [`StateMachine`] instance will be returned from this method. This type represents a fully operational state machine
and only exposes the [`StateMachine::dispatch()`] method used for injecting event variants into it.

### Examples across the  project
This library features many examples that show you its potential and help you understand how to use it. Most of them can be
run without any specific hardware.
You will find small examples embedded in the library types and functions definitions composing this library. Those examples
focus primarily on featuring the use case of those types and functions.
Then there are more complex examples that you will find in the `kaori_hsm/examples` directory.
Those are easy to play with and a make a good base for making your own state machines.
Integrations tests in the `kaori_hsm/tests` directory can also serve the purpose of examples,
but are very rigid and contain a lot of test-specific code.
Finally you will find on [this repository](https://github.com/AntoineMugnier/kaori-hsm-perf-test)
a project designed to test the performance of this library on a stm32f103c8T6 microcontroller.
The performance test may not be easy to understand for a newcomer to the library, but it may be the most practical example.

### An introductory hierachical state machine example
The following example shows the transcription of the HSM below into code using the `kaori_hsm`
library. The test uses a queue onto which the HSM posts a specific string every time it
takes a specific action. After initializing the HSM or dispatching an event to it, the test
code checks that the series of strings on the queue matches the expectation.

![intro_hsm](https://github.com/AntoineMugnier/kaori-hsm/blob/assets/intro_fm.png?raw=true)
```rust
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use kaori_hsm::*;
enum BasicEvt{
    A,
    B,
    C
}

struct BasicStateMachine{
   sender: Sender<String>,
}

impl BasicStateMachine{
    pub fn new(sender: Sender<String>) -> BasicStateMachine {
       BasicStateMachine { sender }
   }

    // Post a string to the test queue
    fn post_string(&self, s : &str){
        self.sender.send(String::from(s)).unwrap();
    }
}

impl ProtoStateMachine for BasicStateMachine{
  type Evt = BasicEvt;

  fn init(&mut self) -> InitResult<Self> {
      self.post_string("TOP_INIT");
      init_transition!(S1)
  }
}

#[state(super_state= Top)]
impl State<S1> for BasicStateMachine{

    fn init(&mut self) -> InitResult<Self> {
        self.post_string("S1-INIT");
        init_transition!(S11)
    }

    fn entry(&mut self) {
       self.post_string("S1-ENTRY");
    }

    fn exit(&mut self) {
       self.post_string("S1-EXIT");
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
                self.post_string("S1-HANDLES-A");
                handled!()
            }
            _ => ignored!()
        }
    }
}

#[state(super_state= S1)]
impl State<S11> for BasicStateMachine{

    fn entry(&mut self) {
       self.post_string("S11-ENTRY");
    }

    fn exit(&mut self) {
       self.post_string("S11-EXIT");
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
        BasicEvt::B =>{
            self.post_string("S11-HANDLES-B");
            transition!(S12)
        }
            _ => ignored!()
        }
    }
}
#[state(super_state= S1)]
impl State<S12> for BasicStateMachine{

    fn entry(&mut self) {
       self.post_string("S12-ENTRY");
    }

    fn exit(&mut self) {
       self.post_string("S12-EXIT");
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            _ => ignored!()
        }
    }
}


   let (sender, mut receiver) = channel();

   let basic_state_machine = BasicStateMachine::new(sender);

   let ism = InitStateMachine::from(basic_state_machine);

   // Execute the topmost initial transition of the state machine, leading to S11 state
   let mut sm = ism.init();
   assert_eq_sm_output(&receiver, &["TOP_INIT", "S1-ENTRY", "S1-INIT", "S11-ENTRY"]);

    // Dispatch event A to the HSM in S11 state. Event is ignored in S11 and handled by the
    // parent state S1.
   sm.dispatch(&BasicEvt::A);
   assert_eq_sm_output(&receiver, &["S1-HANDLES-A"]);

   // Dispatch event B to the HSM in S11 state, provoking a transition to S12 state
   sm.dispatch(&BasicEvt::B);
   assert_eq_sm_output(&receiver, &["S11-HANDLES-B", "S11-EXIT", "S12-ENTRY"]);
```
### Cargo commands index
The present directory must be `kaori_hsm/kaori_hsm` to run every cargo command.
#### Building the lib in release mode
```shell
cargo build --release
````
### Running doc test
```shell
cargo test --doc
```
#### Running a specific integration test
```shell
cargo test --test [test_name]
```
#### Running a specific example from the `examples` directory
```shell
cargo run --example [example_name]
```

License: MIT OR Apache-2.0
