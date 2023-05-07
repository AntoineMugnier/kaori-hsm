pub use crate::state::{HandleResult, InitResult, ParentState, CoreHandleResult, State};



pub trait ProtoStateMachine
{
    type Evt;
    fn init(&mut self)-> InitResult<Self>;

  
    fn return_top_state() -> ParentState<Self>{
        ParentState::TopReached
    }

    fn return_parent_state <StateTag>() -> ParentState<Self>
    where Self : State<StateTag>{
        ParentState::Exists(State::<StateTag>::core_handle)        
    }
}
