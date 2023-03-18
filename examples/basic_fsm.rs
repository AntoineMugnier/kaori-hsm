use kaorust::CoreEvt;
use kaorust::HandleResult;
use kaorust::StateFn;
use kaorust::State;
use kaorust::TopState;
use kaorust::Top;
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

    fn get_parent_state() -> StateFn<Self> {
        Self::state_handle::<S2>()
    }

    fn init(&mut self) {
    
    }

    fn exit(&mut self) {
    
    }

    fn entry(&mut self) {
    
    }

    fn handle(&mut self, evt: BasicEvt) -> HandleResult<Self> {
         Self::transition::<S2>()
    }    
}

struct S2{} impl State<S2> for BasicStateMachine{
    

    fn get_parent_state() -> StateFn<Self> {
        Self::state_handle::<S1>()
    }

    fn init(&mut self) {
    
    }

    fn exit(&mut self) {
    
    }

    fn entry(&mut self) {
    
   }
    fn handle(&mut self, evt: BasicEvt) -> HandleResult<Self> {
        //HandleResult::Handled
        Self::ignored()
    }    
}

fn main(){
    println!("Hello");

}
