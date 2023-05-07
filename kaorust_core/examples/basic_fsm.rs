use kaorust_core::*;

// Evt definition
enum BasicEvt{
A,
B
}

struct BasicStateMachine{}

//type BasicStateMachine = StateMachine<BasicData, BasicEvt>;

impl ProtoStateMachine for BasicStateMachine{
    type Evt = BasicEvt;

    fn init(&mut self) -> InitResult<Self> {
      println!("TOP-INIT");  
      init_transition!(S0)  
    }
}
struct S0{} impl State<S0> for BasicStateMachine{

    fn get_parent_state() -> ParentState<Self> {
        Self::return_top_state()
    }

    fn exit(&mut self) {
        println!("S0-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S0-ENTRY"); 
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
              println!("S0-HANDLES-A");
              transition!(S1)
            },
            _ => ignored!()
        }
    }    
}

struct S1{} impl State<S1> for BasicStateMachine{
    

    fn get_parent_state() -> ParentState<Self> {
        Self::return_top_state()
    }

    fn exit(&mut self) {
        println!("S1-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S1-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::B => {
                println!("S1-HANDLES-B");
                handled!()
            },
            _ => ignored!()
        }
    }    
}

fn main(){
    let mut sm = StateMachine::from(BasicStateMachine{});
    sm.init();

    let evt_a = BasicEvt::A;
    sm.dispatch(&evt_a);
    sm.dispatch(&evt_a);
    
    let evt_b = BasicEvt::B;
    sm.dispatch(&evt_b)
}
