use kaorust::HandleResult;
use kaorust::InitResult;
use kaorust::State;
use kaorust::ProtoStateMachine;
use kaorust::StateMachine;
use kaorust::ParentState;
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
      println!("TOP_INIT");  
      Self::init_transition::<S0>()  
    }
}

struct S0{} impl State<S0> for BasicStateMachine{

    fn get_parent_state() -> ParentState<Self> {
        Self::return_top_state()
    }

    fn init(&mut self) -> InitResult<Self> {
        println!("S0-INIT");
        Self::init_transition::<S11>()
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
                println!("S0-HANDLE-A");
                Self::handled()
            },
            _ => Self::ignored()
        }
    }    
}

struct S11{} impl State<S11> for BasicStateMachine{
    
    fn get_parent_state() -> ParentState<Self> {
        Self::return_parent_state::<S0>()
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
                println!("S1-HANDLE-B");
                Self::handled()
            },
            _ => Self::ignored()
        }
    }    
}
struct S12{} impl State<S12> for BasicStateMachine{
    

    fn get_parent_state() -> ParentState<Self> {
        Self::return_parent_state::<S0>()
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
                println!("S1-HANDLE-B");
                Self::handled()
            },
            _ => Self::ignored()
        }
    }    
}
fn main(){
    let mut sm = StateMachine::new(BasicStateMachine{});
    sm.init();

    let evt_a = BasicEvt::A;
    let evt_b = BasicEvt::B;

    sm.dispatch(&evt_a);
    sm.dispatch(&evt_b);
    sm.dispatch(&evt_a);
}
