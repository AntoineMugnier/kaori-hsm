use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;

use kaorust_core::*;

// Evt definition
#[derive(Debug)]
enum BasicEvt {
    A,
    B,
    C,
    D,
    E,
}

struct BasicStateMachine {
    sender: Sender<String>,
}

impl BasicStateMachine {
    pub fn new(sender: Sender<String>) -> BasicStateMachine {
        BasicStateMachine { sender }
    }
}

impl BasicStateMachine{
    fn post_string(&self, s : &str){
        self.sender.send(String::from(s)).unwrap();
    }
}

impl ProtoStateMachine for BasicStateMachine {
    type Evt = BasicEvt;

    fn init(&mut self) -> InitResult<Self> {
        self.post_string("TOP_INIT");
        init_transition!(S1)
    }
}

#[state(super_state = Top)]
impl State<S1> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        self.post_string("S1-INIT");
        init_transition!(S11)
    }

    fn exit(&mut self) {
        self.post_string("S1-EXIT");
    }

    fn entry(&mut self) {
        self.post_string("S1-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                self.post_string("S1-HANDLES-A");
                handled!()
            }
            BasicEvt::C => {
                self.post_string("S1-HANDLES-C");
                transition!(S122)
            }
            BasicEvt::E => {
                self.post_string("S1-HANDLES-E");
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state = S1)]
impl State<S11> for BasicStateMachine {
    fn exit(&mut self) {
        self.post_string("S11-EXIT");
    }

    fn entry(&mut self) {
        self.post_string("S11-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                self.post_string("S11-HANDLES-A");
                transition!(S121)
            }
            BasicEvt::B => {
                self.post_string("S11-HANDLES-B");
                transition!(S12)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state = S1)]
impl State<S12> for BasicStateMachine {
    fn init(&mut self) -> InitResult<Self> {
        self.post_string("S12-INIT");
        init_transition!(S121)
    }

    fn exit(&mut self) {
        self.post_string("S12-EXIT");
    }

    fn entry(&mut self) {
        self.post_string("S12-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::B => {
                self.post_string("S12-HANDLES-B");
                handled!()
            }
            BasicEvt::D => {
                self.post_string("S12-HANDLES-D");
                transition!(S121)
            }
            _ => ignored!(),
        }
    }
}

#[state(super_state = S12)]
impl State<S121> for BasicStateMachine {
    fn exit(&mut self) {
        self.post_string("S121-EXIT");
    }

    fn entry(&mut self) {
        self.post_string("S121-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::A => {
                self.post_string("S121-HANDLES-A");
                transition!(S122)
            }
            BasicEvt::B => {
                self.post_string("S121-HANDLES-B");
                transition!(S12)
            }
            BasicEvt::C => {
                self.post_string("S121-HANDLES-C");
                transition!(S11)
            }
            _ => ignored!(),
        }
    }
}
#[state(super_state = S12)]
impl State<S122> for BasicStateMachine {
    fn exit(&mut self) {
        self.post_string("S122-EXIT");
    }

    fn entry(&mut self) {
        self.post_string("S122-ENTRY");
    }

    fn handle(&mut self, evt: &BasicEvt) -> HandleResult<Self> {
        match evt {
            BasicEvt::B => {
                self.post_string("S122-HANDLES-B");
                handled!()
            }
            BasicEvt::C => {
                self.post_string("S122-HANDLES-C");
                transition!(S122)
            }
            BasicEvt::D => {
                self.post_string("S122-HANDLES-D");
                transition!(S1)
            }
            _ => ignored!(),
        }
    }
}

fn collect_sm_output(receiver: &mut Receiver<String>) -> String {
    receiver.try_recv().unwrap_or_else(|err| match err {
        TryRecvError::Empty => panic!("Too many expectations for the SM output"),
        TryRecvError::Disconnected => panic!("Disconnected"),
    })
}

fn expect_output_series(receiver: &mut Receiver<String>, expectations: Vec<&str>) {
    for (index, expectation) in expectations.into_iter().enumerate() {
        let sm_output = collect_sm_output(receiver);
        if expectation != sm_output {
            panic!(
                "Expectation index {},  expected : {},  got: {}",
                index, expectation, sm_output
            )
        }
    }

    // Check that we have expected all the output of the SM
    match receiver.try_recv().err() {
        Some(TryRecvError::Empty) => { /* OK */ }
        Some(TryRecvError::Disconnected) => {
            panic!(" Sender is dead")
        }
        None => {
            panic!("Too few expectations for the SM output")
        }
    }
}

fn test_evt_injection(
    sm: &mut StateMachine<BasicStateMachine>,
    receiver: &mut Receiver<String>,
    evt: BasicEvt,
    expectations: Vec<&str>,
) {
    sm.dispatch(&evt);
    expect_output_series(receiver, expectations);
}

fn test_sm_init(
    sm: &mut StateMachine<BasicStateMachine>,
    receiver: &mut Receiver<String>,
    expectations: Vec<&str>,
) {
    sm.init();
    expect_output_series(receiver, expectations);
}

#[test]
fn hsm_test() {
    let (sender, mut receiver) = channel();

    let basic_state_machine = BasicStateMachine::new(sender);

    let mut sm = StateMachine::from(basic_state_machine);

    test_sm_init(
        &mut sm,
        &mut receiver,
        vec!["TOP_INIT", "S1-ENTRY", "S1-INIT", "S11-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::A,
        vec!["S11-HANDLES-A", "S11-EXIT", "S12-ENTRY", "S121-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::B,
        vec!["S121-HANDLES-B", "S121-EXIT", "S12-INIT", "S121-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::D,
        vec!["S12-HANDLES-D", "S121-EXIT", "S121-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::C,
        vec!["S121-HANDLES-C", "S121-EXIT", "S12-EXIT", "S11-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::C,
        vec!["S1-HANDLES-C", "S11-EXIT", "S12-ENTRY", "S122-ENTRY"],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::C,
        vec!["S122-HANDLES-C", "S122-EXIT", "S122-ENTRY"],
    );
    test_evt_injection(&mut sm, &mut receiver, BasicEvt::A, vec!["S1-HANDLES-A"]);
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::D,
        vec![
            "S122-HANDLES-D",
            "S122-EXIT",
            "S12-EXIT",
            "S1-INIT",
            "S11-ENTRY",
        ],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::B,
        vec![
            "S11-HANDLES-B",
            "S11-EXIT",
            "S12-ENTRY",
            "S12-INIT",
            "S121-ENTRY",
        ],
    );
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::A,
        vec!["S121-HANDLES-A", "S121-EXIT", "S122-ENTRY"],
    );
    test_evt_injection(&mut sm, &mut receiver, BasicEvt::B, vec!["S122-HANDLES-B"]);
    test_evt_injection(
        &mut sm,
        &mut receiver,
        BasicEvt::E,
        vec![
            "S1-HANDLES-E",
            "S122-EXIT",
            "S12-EXIT",
            "S1-EXIT",
            "S1-ENTRY",
            "S1-INIT",
            "S11-ENTRY",
        ],
    );
}
