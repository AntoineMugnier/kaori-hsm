use kaorust::StateMachine;
use kaorust::State;
use kaorust::TopState;

// Evt definition
enum BasicEvt{

}

struct BasicData{}

type BasicStateMachine = StateMachine<BasicData, BasicEvt>;

struct S1{} impl State<BasicStateMachine> for S1{
    type ParentState = TopState;
    fn init(data : &mut BasicData) {
    
    }

    fn exit(data : &mut BasicData) {
    
    }

    fn entry(data: &mut BasicData) {
    
    }

    fn handle(data : &mut BasicData, evt: BasicEvt) {
    
    }    
}

struct S2{} impl State<BasicStateMachine> for S2{
    
    type ParentState = S1;

    fn init(data : &mut BasicData) {
    
    }

    fn exit(data : &mut BasicData) {
    
    }

    fn entry(data: &mut BasicData) {
    
   }
    fn handle(data : &mut BasicData, evt: BasicEvt) {
    
    }    
}

fn main(){
    println!("Hello");
    let data = BasicData{};
    let bsm = BasicStateMachine::new::<S1>(data);


}
