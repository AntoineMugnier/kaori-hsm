use kaorust::StateMachine;
use kaorust::State;
use kaorust::TopState;

// Evt definition
enum BasicEvt{

}

struct BasicStateMachine{}

//type BasicStateMachine = StateMachine<BasicData, BasicEvt>;

impl TopState for BasicStateMachine{
    type Evt = BasicEvt;

    fn init(&mut self) {
    
    }
}

struct S1{} impl State<S1> for BasicStateMachine{

    type ParentState = Self;
    fn init(&mut self) {
    
    }

    fn exit(&mut self) {
    
    }

    fn entry(&mut self) {
    
    }

    fn handle(&mut self, evt: BasicEvt) {
    
    }    
}

struct S2{} impl State<S2> for BasicStateMachine{
    
    type ParentState = S1;

    fn init(&mut self) {
    
    }

    fn exit(&mut self) {
    
    }

    fn entry(&mut self) {
    
   }
    fn handle(&mut self, evt: BasicEvt) {
    
    }    
}

fn main(){
    println!("Hello");

}
