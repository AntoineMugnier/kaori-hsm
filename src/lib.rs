
mod sealed{
  
}

pub enum ParentState<UserStateMachine>
    where UserStateMachine : ProtoStateMachine + ?Sized{
    ProtoStateMachine,
    SubState(StateFn<UserStateMachine>)
}

pub struct Top{}

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
      InitResult(State::<StateT>::core_handle) 
   }
   
    fn return_top_state() -> ParentState<Self>{
        ParentState::ProtoStateMachine
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
        ParentState::SubState(State::<StateTag>::core_handle)
    }
}

pub type StateFn<UserStateMachineT> = fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachineT>;


pub struct InitResult<UserStateMachine : ProtoStateMachine + ?Sized>(
    StateFn<UserStateMachine>
);

pub enum HandleResult<UserStateMachineT: ProtoStateMachine + ?Sized>{
    Ignored,
    Handled,
    Transition(StateFn<UserStateMachineT>),
}

pub enum CoreHandleResult<UserStateMachineT: ProtoStateMachine + ?Sized>{
    Ignored(ParentState<UserStateMachineT>),
    Handled,
    Transition(StateFn<UserStateMachineT>),
    ReturnParentState(ParentState<UserStateMachineT>)
}

pub trait State<T>
where Self : ProtoStateMachine{
    
    fn get_parent_state() -> ParentState<Self>;

    fn init(&mut self){
        // No implementation
    }

    fn entry(&mut self){
        // No implementation
    }

    fn exit(&mut self){
        // No implementation
    }

    fn handle(&mut self, evt:&<Self as ProtoStateMachine>::Evt) -> HandleResult<Self>;
    
    fn core_handle(&mut self, evt: &CoreEvt::<<Self as ProtoStateMachine>::Evt>) -> CoreHandleResult<Self>{
        match evt{
            CoreEvt::Init => {
                <Self as State<T>>::init(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::Entry => {
                <Self as State<T>>::entry(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::Exit => {
                <Self as State<T>>::exit(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::GetParentState =>{
                return CoreHandleResult::ReturnParentState(Self::get_parent_state());
            }
            CoreEvt::User { user_evt } => {
                match <Self as State<T>>::handle(self, user_evt){
                    HandleResult::Ignored => return CoreHandleResult::Ignored(Self::get_parent_state()),
                    HandleResult::Handled => return CoreHandleResult::Handled,
                    HandleResult::Transition(state_fn) => CoreHandleResult::Transition(state_fn)
                }
            }
        }
    }
}

pub enum CoreEvt<'a, UserEvtT>{
    Init,
    Entry,
    Exit,
    GetParentState,
    User{user_evt : &'a UserEvtT}
}


pub struct StateMachine<UserStateMachine: ProtoStateMachine>{
    user_state_machine : UserStateMachine,
    curr_state : Option<StateFn<UserStateMachine>>
}

impl <UserStateMachine : ProtoStateMachine>StateMachine<UserStateMachine>{
    

    pub fn new(user_state_machine : UserStateMachine) -> StateMachine<UserStateMachine>{
    
        StateMachine{user_state_machine, curr_state : None}
    }

    pub fn dispatch_entry_evt(&mut self, state_fn : StateFn<UserStateMachine>){
        let entry_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::Entry;
        state_fn(&mut self.user_state_machine, &entry_evt);
    }

    pub fn dispatch_exit_evt(&mut self, state_fn : StateFn<UserStateMachine>){
        let exit_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::Exit;
        state_fn(&mut self.user_state_machine, &exit_evt);
    }

    pub fn init(&mut self){
        let init_result = self.user_state_machine.init();

        self.curr_state = Some(init_result.0);

        self.dispatch_entry_evt(self.curr_state.unwrap());
    }   

    fn handle_ignored_evt(&mut self, parent_state_variant : ParentState<UserStateMachine>,evt : &CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>){
        match parent_state_variant{
            ParentState::SubState(super_state) => self.dispatch_core_event(super_state, evt),
            ParentState::ProtoStateMachine => {}
        }
    }

    fn handle_transition(&mut self, target_state_fn : StateFn<UserStateMachine>){
        

        //Exit current state
        self.dispatch_exit_evt(self.curr_state.unwrap());
        
        self.curr_state = Some(target_state_fn); 
        
        self.dispatch_entry_evt(self.curr_state.unwrap());
    }
    
    fn dispatch_core_event(&mut self, state_fn : StateFn<UserStateMachine>, evt : & CoreEvt<<UserStateMachine as ProtoStateMachine>::Evt>){
        let core_handle_result = state_fn(&mut self.user_state_machine, evt);
        
        // Treat result of the event dispatch
        match core_handle_result{
            CoreHandleResult::Handled => {},
            CoreHandleResult::Ignored(parent_state_fn) => self.handle_ignored_evt(parent_state_fn, evt),
            CoreHandleResult::Transition(target_state_fn) => self.handle_transition(target_state_fn),
            _ => {}
        }
    }

    pub fn dispatch(&mut self, user_evt : &<UserStateMachine as ProtoStateMachine>::Evt){

        // Dispatch user evt to current state 
        let evt = CoreEvt::User {user_evt};
        let current_state_fn = self.curr_state.unwrap(); 
        self.dispatch_core_event(current_state_fn, &evt);
   }

}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
