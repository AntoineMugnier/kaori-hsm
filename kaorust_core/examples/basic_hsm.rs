use kaorust_core::*;
// Evt definition

use rand::Rng;
#[derive(Debug)]
enum BasicEvt {
    A,
    B,
    C,
    D,
}

struct BasicStateMachine {
    a: u8,
}

impl BasicStateMachine {
    pub fn new() -> BasicStateMachine {
        BasicStateMachine { a: 0 }
    }
}
//type BasicStateMachine = StateMachine<BasicData, BasicEvt>;

impl ProtoStateMachine for BasicStateMachine {
    type Evt = BasicEvt;

    fn init(&mut self) -> InitResult<Self> {
        println!("TOP_INIT");
        init_transition!(S11)
    }
}

#[state(super_state= Top)]
impl State<S1> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        println!("S1-INIT");
        init_transition!(S11)
    }

    fn exit(&mut self) {
        println!("S1-EXIT");
    }

    fn entry(&mut self) {
        println!("S1-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                println!("S1-HANDLES-A");
                handled!()
            }
            BasicEvt::C => {
                println!("S1-HANDLES-C");
                transition!(S122)
            }
            BasicEvt::D => {
                println!("S1-HANDLES-D");
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S1)]
impl State<S11> for BasicStateMachine {
    fn exit(&mut self) {
        println!("S11-EXIT");
    }

    fn entry(&mut self) {
        println!("S11-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                println!("S11-HANDLES-A");
                transition!(S121)
            }
            BasicEvt::B => {
                println!("S11-HANDLES-B");

                self.a += 1;

                if self.a == 2 {
                    self.a = 0;
                    transition!(S12)
                } else {
                    ignored!()
                }
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S1)]
impl State<S12> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        println!("S12-INIT");
        init_transition!(S121)
    }

    fn exit(&mut self) {
        println!("S12-EXIT");
    }

    fn entry(&mut self) {
        println!("S12-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::B => {
                println!("S12-HANDLES-B");
                handled!()
            }
            BasicEvt::D => {
                println!("S12-HANDLES-D");
                transition!(S121)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S12)]
impl State<S121> for BasicStateMachine {
    fn exit(&mut self) {
        println!("S121-EXIT");
    }

    fn entry(&mut self) {
        println!("S121-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                println!("S121-HANDLES-A");
                transition!(S122)
            }
            BasicEvt::B => {
                println!("S121-HANDLES-B");
                transition!(S12)
            }
            BasicEvt::C => {
                println!("S121-HANDLES-C");
                transition!(S11)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state= S12)]
impl State<S122> for BasicStateMachine {
    fn exit(&mut self) {
        println!("S122-EXIT");
    }

    fn entry(&mut self) {
        println!("S122-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::B => {
                println!("S122-HANDLES-B");
                handled!()
            }
            BasicEvt::C => {
                println!("S122-HANDLES-C");
                transition!(S122)
            }
            BasicEvt::D => {
                println!("S122-HANDLES-D");
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}

fn make_evt_list(list_size: usize) -> Vec<BasicEvt> {
    let rng = rand::thread_rng;
    let mut evt_list = Vec::new();

    for _ in 0..list_size {
        match rng().gen_range(0..4) {
            0 => evt_list.push(BasicEvt::A),
            1 => evt_list.push(BasicEvt::B),
            2 => evt_list.push(BasicEvt::C),
            3 => evt_list.push(BasicEvt::D),
            _ => {}
        }
    }
    evt_list
}

fn main() {
    let basic_state_machine = BasicStateMachine::new();

    let mut sm = StateMachine::from(basic_state_machine);

    println!("Init state machine");
    sm.init();

    let list_size = 4;

    let evt_list = make_evt_list(list_size);

    for evt in evt_list {
        println!("\r\nDispatching evt {:?}", evt);
        sm.dispatch(&evt);
    }
}
