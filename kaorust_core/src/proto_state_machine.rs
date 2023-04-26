pub use crate::misc::{ParentState, HandleResult, InitResult};
pub use crate::state::State;

pub trait ProtoStateMachine
{
    type Evt;
    fn init(&mut self)-> InitResult<Self>;

    fn transition<StateT>() -> HandleResult<Self>
    where Self: State<StateT> {
      HandleResult::Transition(State::<StateT>::core_handle) 
   }

    fn init_transition<StateT>() -> InitResult<Self>
    where Self: State<StateT> {
      InitResult::TargetState(State::<StateT>::core_handle) 
   }
   
    fn return_top_state() -> ParentState<Self>{
        ParentState::TopReached
    }
    
    fn ignored() -> HandleResult<Self>
    {
        HandleResult::Ignored
    }

    fn handled() -> HandleResult<Self>{
        HandleResult::Handled
    }

    fn return_parent_state <StateTag>() -> ParentState<Self>
    where Self : State<StateTag>{
        ParentState::Exists(State::<StateTag>::core_handle)        
    }
}
