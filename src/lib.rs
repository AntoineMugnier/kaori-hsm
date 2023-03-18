
mod sealed{
  
}
pub trait TopState
{
    type Evt;
    fn init(&mut self)-> InitResult<Self>;

    fn transition<StateT>() -> HandleResult<Self>
    where Self: State<StateT> {
      HandleResult::Transition(State::<StateT>::core_handle) 
   }

    fn init_transition<StateT>() -> InitResult<Self>
    where Self: State<StateT> {
      InitResult(State::<StateT>::core_handle) 
   }
    
    fn ignored() -> HandleResult<Self>
    {
        return HandleResult::Ignored;
    }

    fn state_handle <StateTag>() -> StateFn<Self>
    where Self : State<StateTag>{
        State::<StateTag>::core_handle
    }
}

pub type StateFn<UserStateMachineT> = fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as TopState>::Evt>) -> HandleResult<UserStateMachineT>;

pub struct Top{}

pub struct InitResult<UserStateMachine : TopState + ?Sized>(
    StateFn<UserStateMachine>
);

pub enum HandleResult<UserStateMachineT: TopState + ?Sized>{
    Ignored,
    Handled,
    Transition(StateFn<UserStateMachineT>)
}

pub trait State<T>
where Self : TopState{
    
    fn get_parent_state() -> StateFn<Self>;

    fn init(&mut self);

    fn entry(&mut self);

    fn exit(&mut self);

    fn handle(&mut self, evt:&<Self as TopState>::Evt) -> HandleResult<Self>;
    
    fn core_handle(&mut self, evt: &CoreEvt::<<Self as TopState>::Evt>) -> HandleResult<Self>{
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
                return HandleResult::Transition(Self::get_parent_state());
            }
            CoreEvt::User { user_evt } => {
                return <Self as State<T>>::handle(self, user_evt);
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
    curr_state : Option<StateFn<UserStateMachine>>
}

impl <UserStateMachine : TopState>StateMachine<UserStateMachine>{
    

    pub fn new(user_state_machine : UserStateMachine) -> StateMachine<UserStateMachine>{
    
        StateMachine{user_state_machine, curr_state : None}
    }
    
    pub fn init(&mut self){
        let init_result = self.user_state_machine.init();
        self.curr_state = Some(init_result.0);
    }   

    fn handle_ignored_evt(&mut self, state_fn : StateFn<UserStateMachine>,evt : &CoreEvt::<<UserStateMachine as TopState>::Evt>){
        
        // Prepare event to dispatch to the current state function for retrieving the parent state
        let get_parent_state_evt = CoreEvt::<<UserStateMachine as TopState>::Evt>::GetParentState;
        
        // Dispatch event and retrieve parent state function pointer in an `HandleResult::Transition` variant
        if let HandleResult::Transition(super_state) = state_fn(&mut self.user_state_machine, &get_parent_state_evt){
                self.dispatch_core_event(super_state, evt);
        }
        else{
            //Error
        }
        
    }

    fn handle_transition(&mut self, state_fn : StateFn<UserStateMachine>){
        
    }
    
    fn dispatch_core_event(&mut self, state_fn : StateFn<UserStateMachine>, evt : & CoreEvt<<UserStateMachine as TopState>::Evt>){
        let handle_result = state_fn(&mut self.user_state_machine, evt);
        
        // Treat result of the event dispatch
        match handle_result{
            HandleResult::Handled => {},
            HandleResult::Ignored => self.handle_ignored_evt(state_fn, evt),
            HandleResult::Transition(target_state_fn) =>self.handle_transition(target_state_fn),
        }
 
    }

    pub fn dispatch(&mut self, user_evt : <UserStateMachine as TopState>::Evt){

        // Dispatch user evt to current state 
        let evt = CoreEvt::User {user_evt};
        let current_state_fn = self.curr_state.unwrap(); 
        self.dispatch_core_event(current_state_fn, &evt);
   }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
