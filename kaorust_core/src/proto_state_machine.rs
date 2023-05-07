pub use crate::state::{HandleResult, InitResult, ParentState, CoreHandleResult, State};



pub trait ProtoStateMachine
{
    type Evt;
   
    fn init(&mut self)-> InitResult<Self>;
}
