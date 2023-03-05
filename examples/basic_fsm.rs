use kaorust::StateMachine;
use kaorust::State;
// Evt definition
enum BasicEvt{

}

impl StateMachine for BasicStateMachine{

    type Evt = BasicEvt;
}
struct S1{}

impl State<BasicStateMachine> for S1{
}
//State machine struct
struct BasicStateMachine{

}

impl BasicStateMachine{

}

fn main(){
    println!("Hello");
}
