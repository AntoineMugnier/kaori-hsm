# kaori_hsm

## kaori_hsm State machine library
kaori_hsm is a library for developing Hierarchical State Machines (HSMs) in Rust. Lightweigness
and execution speed are primary focuses of this library as it is designed to run on systems with
low ressources such as microcontrollers, but it can freely be used
on any other support.

## What are Hierarchical state machines ?
States machines are software enties processing events differently depending on the state in
which they are they are. Different input events may lead to different actions being performed by the state
machine and can trigger transition to other states.

Hierarchical State Machines are state machines which can have nested states. This means that if
an event cannot be handled in a state, its super state could eventually handle it.
HSMs are therefore particularly useful for designing state machines with complex behavior.

For understanding how state machines and especially HSMs work, I especially recomment the video series
made by Miro Samek that you can find [here](https://youtube.com/playlist?list=PLPW8O6W-1chxym7TgIPV9k5E8YJtSBToI&si=mfiiiq3EMLj1bJpH)

### How to use the framework
To build your own state machine, you first have to define the structure that will hold its
data and then you will need to implement the following traits of the library on it: the [`ProtoStateMachine`]
trait and as many variant of the [`State<Tag>`] trait as you want to define states.

The following sequence has to be followed in order to build an operational state machine.
The builder pattern in used in order to enforce statically the steps order:
- Create an instance of the structure you previously defined.
- Call the [`InitStateMachine::from()`] function with the instance as argument. A ['InitStateMachine'] instance will be returned.
- Call the [`InitStateMachine::init()`] method on this instance. It will initialize the state machine and lead
it to transition to its first state. A [`StateMachine`] instance will be returned from this method, constituing the operational state machine.
This structure only exposes the [`StateMachine::dispatch()`] method used for injecting events into it.

## Ressources
This library features many examples to help you understand how to use it. Some examples can be run
directly on your computer. This includes examples provided with the different user-exposed
functions but also the one below. Some other examples are designed to run on hardware. In that
case the material you need and its setup are described in the example file. This incudes
examples in the `kaori_hsm/examples` directory.

```rust
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

    fn post_string(&self, s : &str){
        self.sender.send(String::from(s)).unwrap();
    }
} impl ProtoStateMachine for BasicStateMachine{
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

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
                self.post_string("S1-HANDLES-A");
                handled!()
            }
            BasicEvt::C =>{
                self.post_string("S1-HANDLES-C");
                transition!(S2)
            }
            _ => ignored!()
        }
    }
}

#[state(super_state= S1)]
impl State<S11> for BasicStateMachine{

    fn exit(&mut self) {
       self.post_string("S11-EXIT");
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::B => {
                 self.post_string("S11-HANDLES-B");
                 transition!(S2)
            }
            _ => ignored!()
        }
    }
}

#[state(super_state= Top)]
impl State<S2> for BasicStateMachine{

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
           BasicEvt::A => {
                self.post_string("S2-HANDLES-A");
                transition!(S1)
            }
           BasicEvt::B => {
                self.post_string("S2-HANDLES-B");
                handled!()
            }
            _ => ignored!()
        }
    }
}

#
#

   let (sender, mut receiver) = channel();

   let basic_state_machine = BasicStateMachine::new(sender);

   let ism = InitStateMachine::from(basic_state_machine);

   let mut sm = ism.init();
   expect_output_series(&receiver, &["TOP_INIT", "S1-ENTRY", "S1-INIT"]);

   sm.dispatch(&BasicEvt::A);
   expect_output_series(&receiver, &["S1-HANDLES-A"]);

   sm.dispatch(&BasicEvt::B);
   expect_output_series(&receiver, &["S11-HANDLES-B", "S11-EXIT"]);
return;
   sm.dispatch(&BasicEvt::B);
   expect_output_series(&receiver, &["S2-HANDLES-B"]);

   sm.dispatch(&BasicEvt::A);
   expect_output_series(&receiver, &["S2-HANDLES-A", "S1-ENTRY", "S1-INIT"]);

   sm.dispatch(&BasicEvt::C);
   expect_output_series(&receiver, &["S1-HANDLES-C", "S11-EXIT"]);

```
