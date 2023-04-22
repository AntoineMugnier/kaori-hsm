use kaorust::HandleResult;
use kaorust::InitResult;
use kaorust::State;
use kaorust::ProtoStateMachine;
use kaorust::StateMachine;
use kaorust::ParentState;
use kaorust_derive::state;
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
      Self::init_transition::<S1>()  
    }
}
#[state(state_name= S1, super_state_name= Top)]
impl State<state_name> for BasicStateMachine{

    fn init(&mut self) -> InitResult<Self> {
        println!("S1-INIT");
        Self::init_transition::<S11>()
    }

    fn exit(&mut self) {
        println!("S1-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S1-ENTRY"); 
    }

    fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
                println!("S1-HANDLE-A");
                Self::handled()
            },
            BasicEvt::B => {
                println!("S1-HANDLE-B");
                Self::transition::<S12>()
            }
            _ => Self::ignored()
        }
    }    
}

#[state(state_name= S11, super_state_name= S1)]
impl State<state_name> for BasicStateMachine{
    
    fn exit(&mut self) {
        println!("S11-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S11-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
                println!("S11-HANDLE-A");
                Self::transition::<S121>()
            },
            _ => Self::ignored()
        }
    }    
}
struct S12{} impl State<S12> for BasicStateMachine{
    
    fn init(&mut self) -> InitResult<Self> {
        println!("S12-INIT"); 
        Self::init_transition::<S121>()
    }
    fn get_parent_state() -> ParentState<Self> {
        Self::return_parent_state::<S1>()
    }

    fn exit(&mut self) {
        println!("S12-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S12-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::B => {
                println!("S12-HANDLE-B");
                Self::handled()
            },
            _ => Self::ignored()
        }
    }    
}

struct S121{} impl State<S121> for BasicStateMachine{
    

    fn get_parent_state() -> ParentState<Self> {
        Self::return_parent_state::<S12>()
    }

    fn exit(&mut self) {
        println!("S121-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S121-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::B => {
                println!("S121-HANDLE-B");
                Self::handled()
            },
            _ => Self::ignored()
        }
    }    
}

struct S122{} impl State<S122> for BasicStateMachine{

    

    fn get_parent_state() -> ParentState<Self> {
        Self::return_parent_state::<S12>()
    }

    fn exit(&mut self) {
        println!("S122-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S122-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::B => {
                println!("S122-HANDLE-B");
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

    sm.dispatch(&evt_b);
    //sm.dispatch(&evt_b);
    //sm.dispatch(&evt_a);
}
