
mod sealed{
  
}
pub trait TopState
{
    type Evt;
    fn init(&mut self);

    fn transition<StateT>() -> HandleResult<Self>
    where Self: State<StateT> {
      HandleResult::Transition(State::<StateT>::core_handle) 
   }

}

type StateFn<UserStateMachineT> = fn(&mut UserStateMachineT, CoreEvt<<UserStateMachineT as TopState>::Evt>) -> HandleResult<UserStateMachineT>;

pub struct Top{}

pub enum HandleResult<UserStateMachineT: TopState + ?Sized>{
    Ignored(StateFn<UserStateMachineT>),
    Handled,
    Transition(StateFn<UserStateMachineT>),
}

pub trait State<T>
where Self : TopState{
    type ParentState;
    
    
    fn transition<StateT>() -> HandleResult<Self>
    where Self: State<StateT> {
      HandleResult::Transition(State::<StateT>::core_handle) 
   }


    fn ignored() -> HandleResult<Self>
    where Self: State<<Self as State<T>>::ParentState> {
        return HandleResult::Ignored(State::<<Self as State<T>>::ParentState>::core_handle);
    }

    fn init(&mut self);

    fn entry(&mut self);

    fn exit(&mut self);

    fn handle(&mut self, evt: <Self as TopState>::Evt) -> HandleResult<Self>;
    
    fn core_handle(&mut self, evt: CoreEvt::<<Self as TopState>::Evt>) -> HandleResult<Self>{
        match evt{
            CoreEvt::Init => {
                <Self as State<T>>::init(self);
                return HandleResult::Handled;
            }
            CoreEvt::Entry => {
                <Self as State<T>>::entry(self);
                return HandleResult::Handled;
            }
            CoreEvt::Exit => {
                <Self as State<T>>::exit(self);
                return HandleResult::Handled;
            }
            CoreEvt::GetParentState =>{
                return HandleResult::Handled;
                //rturn HandleResult::Ignored(State::<State<T>::ParentState>::core_handle);
            }
            CoreEvt::User { user_evt } => {
                return HandleResult::Handled;
                //return <Self as State<T>>::handle(self, user_evt);
            }
        }
    }
}

pub enum CoreEvt<UserEvtT>{
    Init,
    Entry,
    Exit,
    GetParentState,
    User{user_evt : UserEvtT}
}


pub struct StateMachine<UserStateMachine: TopState>{
    user_state_machine : UserStateMachine,
    curr_state : Option<fn(&mut UserStateMachine, <UserStateMachine as TopState>::Evt) -> HandleResult<UserStateMachine>>
}

impl <UserStateMachine : TopState>StateMachine<UserStateMachine>{
    
    pub fn new(user_state_machine : UserStateMachine) -> StateMachine<UserStateMachine>{
        
        StateMachine{user_state_machine, curr_state : None}
    }

    pub fn dispatch(&mut self, evt : CoreEvt<<UserStateMachine as TopState>::Evt>){

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
