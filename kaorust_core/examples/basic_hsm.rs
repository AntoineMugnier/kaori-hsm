use kaorust_core::HandleResult;
use kaorust_core::InitResult;
use kaorust_core::State;
use kaorust_core::ProtoStateMachine;
use kaorust_core::StateMachine;
use kaorust_core::ParentState;
use kaorust_derive::state;
// Evt definition

use rand::Rng;
#[derive(Debug)] 
enum BasicEvt{
A,
B,
C,
D
}

struct BasicStateMachine{
    a: u8
}

impl BasicStateMachine{
    pub fn new() -> BasicStateMachine{
        BasicStateMachine { a: 0 }
    }
}
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
                println!("S1-HANDLES-A");
                Self::handled()
            },
            BasicEvt::C => {
                println!("S1-HANDLES-C");
                Self::transition::<S122>()
            },
            BasicEvt::D => {
                println!("S1-HANDLES-D");
                Self::transition::<S1>()
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
                println!("S11-HANDLES-A");
                Self::transition::<S121>()
            },
            BasicEvt::B => {
                println!("S11-HANDLES-B");
                
                self.a+=1;
                
                if self.a == 2{
                    self.a = 0;
                    Self::transition::<S12>()
                }
                else{
                    Self::ignored()
                }
            },
            _ => Self::ignored()
        }
    }    
}

#[state(state_name= S12, super_state_name= S1)]
impl State<state_name> for BasicStateMachine{
    
    fn init(&mut self) -> InitResult<Self> {
        println!("S12-INIT"); 
        Self::init_transition::<S121>()
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
                println!("S12-HANDLES-B");
                Self::handled()
            },
            BasicEvt::D => {
                println!("S12-HANDLES-D");
                Self::transition::<S121>()
            },
            _ => Self::ignored()
        }
    }    
}

#[state(state_name= S121, super_state_name= S12)]
impl State<state_name> for BasicStateMachine{

    fn exit(&mut self) {
        println!("S121-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S121-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{
            BasicEvt::A => {
                println!("S121-HANDLES-A");
                Self::transition::<S122>()
            },
            BasicEvt::B => {
                println!("S121-HANDLES-B");
                Self::transition::<S12>()
            },
            BasicEvt::C => {
                println!("S121-HANDLES-C");
                Self::transition::<S11>()
            },
            _ => Self::ignored()
        }
    }    
}

#[state(state_name= S122, super_state_name= S12)]
impl State<state_name> for BasicStateMachine{

    fn exit(&mut self) {
        println!("S122-EXIT"); 
    }

    fn entry(&mut self) {
        println!("S122-ENTRY"); 
   }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt{

            BasicEvt::B => {
                println!("S122-HANDLES-B");
                Self::handled()
            },
            BasicEvt::C => {
                println!("S122-HANDLES-C");
                Self::transition::<S122>()
            }, 
            BasicEvt::D => {
                println!("S122-HANDLES-D");
                Self::transition::<S1>()
            },
            _ => Self::ignored()
        }
    }    
}


fn make_evt_list(list_size: usize) -> Vec<BasicEvt>{
    let rng = rand::thread_rng;
    let mut evt_list = Vec::new();

    for _ in 0..list_size{

        match rng().gen_range(0..4){
            0 => evt_list.push(BasicEvt::A),
            1 => evt_list.push(BasicEvt::B),
            2 => evt_list.push(BasicEvt::C),
            3 => evt_list.push(BasicEvt::D),
            _ => {}
        }
    } 
    evt_list
}

fn main(){
    let basic_state_machine = BasicStateMachine::new();

    let mut sm = StateMachine::new(basic_state_machine);
    
    println!("Init state machine");
    sm.init();

    let list_size = 4;
    
    let evt_list = make_evt_list(list_size);
    
    for evt in evt_list{
        println!("\r\nDispatching evt {:?}", evt);
        sm.dispatch(&evt);
    }
}
