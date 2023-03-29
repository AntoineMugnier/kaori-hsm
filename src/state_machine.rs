use crate::proto_state_machine::ProtoStateMachine;
use crate::misc::{CoreEvt, StateFn, ParentState, CoreHandleResult};

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

    pub fn dispatch_get_super_state(&mut self, state_fn : StateFn<UserStateMachine>) -> ParentState<UserStateMachine>{
        let get_parent_state_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::GetParentState;
        let core_handle_result = state_fn(&mut self.user_state_machine, &get_parent_state_evt);
        if let CoreHandleResult::ReturnParentState(parent_state_fn) = core_handle_result{
            parent_state_fn
        }
        else{
            panic!() //error should not be possible
        }
        
    }

    pub fn dispatch_init_evt(&mut self, state_fn : StateFn<UserStateMachine>) -> Option<StateFn<UserStateMachine>>{
        let init_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::Init;
        let init_result = state_fn(&mut self.user_state_machine, &init_evt);
        match init_result{
            CoreHandleResult::InitResult(init_result) => init_result.0,
            _ => panic!() //error, should not be possible
        } 
    }

    pub fn dispatch_exit_evt(&mut self, state_fn : StateFn<UserStateMachine>){
        let exit_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::Exit;
        state_fn(&mut self.user_state_machine, &exit_evt);
    }

    pub fn reach_init_target(&mut self, target_state_fn : StateFn<UserStateMachine>) -> StateFn<UserStateMachine>{
        
        let mut current_target_state_fn = target_state_fn;
        
        while let Some(next_target_state) = self.dispatch_init_evt(current_target_state_fn){ 
            current_target_state_fn = next_target_state;
            self.dispatch_entry_evt(current_target_state_fn);
        }
        
        current_target_state_fn
    }

    pub fn init(&mut self){

        // Call user top initial pseudostate implementation
        let init_result = self.user_state_machine.init();
        let topmost_init_target_state_fn = init_result.0.unwrap_or_else(|| panic!("Topmost Init should return a state"));

        // Reach leaf state
        self.dispatch_entry_evt(topmost_init_target_state_fn);
        let last_init_state_fn = self.reach_init_target(topmost_init_target_state_fn);

        self.curr_state = Some(last_init_state_fn);
    }   

    fn handle_ignored_evt(&mut self, parent_state_variant : ParentState<UserStateMachine>,evt : &CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>){
        match parent_state_variant{
            ParentState(Some(super_state)) => self.dispatch_core_event(super_state, evt),
            ParentState(None) => {}
        }
    }
    
    fn exit_substates(&mut self, original_state_fn : StateFn<UserStateMachine>){
        let curr_state_fn = self.curr_state.unwrap();
        let mut next_state_fn = curr_state_fn;

        while next_state_fn as *const fn() != original_state_fn as *const fn(){
            if let ParentState(Some(parent_state_fn)) = self.dispatch_get_super_state(next_state_fn){
                self.dispatch_exit_evt(next_state_fn);
                next_state_fn = parent_state_fn;
            }
            else{
                panic!()
            }
        }     
    }

    fn handle_transition(&mut self, original_state_fn : StateFn<UserStateMachine>, target_state_fn : StateFn<UserStateMachine>){
        
       self.exit_substates(original_state_fn);

        self.curr_state = Some(target_state_fn); 
        
        self.dispatch_entry_evt(self.curr_state.unwrap());
    }
    
    fn dispatch_core_event(&mut self, state_fn : StateFn<UserStateMachine>, evt : & CoreEvt<<UserStateMachine as ProtoStateMachine>::Evt>){
        let core_handle_result = state_fn(&mut self.user_state_machine, evt);
        
        // Treat result of the event dispatch
        match core_handle_result{
            CoreHandleResult::Handled => {},
            CoreHandleResult::Ignored(parent_state_fn) => self.handle_ignored_evt(parent_state_fn, evt),
            CoreHandleResult::Transition(target_state_fn) => self.handle_transition(state_fn, target_state_fn),
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
