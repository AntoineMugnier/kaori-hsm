use crate::proto_state_machine::ProtoStateMachine;
use crate::state::{CoreEvt, ParentState, InitResult, CoreHandleResult, StateFn};

/// Type which ensapsulates most of the Kaorust state machine logic. An instance of this structure must be built from an instance of a  user-defines structure on which has been implemented the `ProtoStateMachine` and `State` traits.
/// Hence, an instance of this structure is a completely autonomous state machine mixing both user and library code. 
pub struct StateMachine<UserStateMachine: ProtoStateMachine>{
    user_state_machine : UserStateMachine,
    curr_state :StateFn<UserStateMachine>
}

struct Link<'a, UserStateMachine: ProtoStateMachine + ?Sized>{
        state_fn : StateFn<UserStateMachine>,
        next_link : Option<&'a Link<'a, UserStateMachine>> 
}

/// Struct encapsulating the business logic of hierarchical state machine
impl <UserStateMachine : ProtoStateMachine>StateMachine<UserStateMachine>{
   fn default_state(_user_sm: &mut UserStateMachine, _evt: &CoreEvt<<UserStateMachine as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachine>{
        panic!("dispatch() function called on state_machine before init")
    } 
    
    /// Build the Kaorust state machine from you structure which implements the `ProtoStateMachine` trait and as many variants
    /// of the `State` trait as you have states.
    pub fn from(user_state_machine : UserStateMachine) -> StateMachine<UserStateMachine>{
    
        StateMachine{user_state_machine, curr_state : Self::default_state}
    }

    /// Will trigger the initial transition of the state machine by calling
    /// `ProtoStateMachine::top_init`. That call willl lead to the first state of the machine to be
    /// set.
    pub fn init(&mut self){

        let user_state_machine = &mut self.user_state_machine;
        // Call user top initial pseudostate implementation
        let init_result = user_state_machine.init();
        match init_result{
            InitResult::TargetState(topmost_init_target_state_fn) =>{
                // Reach leaf state
                Self::dispatch_entry_evt(user_state_machine, topmost_init_target_state_fn);

                self.reach_init_target(topmost_init_target_state_fn);
            }

            InitResult::NotImplemented =>  panic!("Topmost Init should return a state")
        }
    }
    
    /// Dispatch an event of the type you have attributed to `ProtoStateMachine::Evt`.
    /// The `dispatch()` method should only be called after `init()`, otherwise the framework will
    /// panic
    pub fn dispatch(&mut self, user_evt : &<UserStateMachine as ProtoStateMachine>::Evt){

        // Dispatch user evt to current state 
        let evt = CoreEvt::UserEvt {user_evt};
        let current_state_fn = self.curr_state;
        self.dispatch_core_event(current_state_fn, &evt);
   }


    fn dispatch_entry_evt(user_state_machine : &mut UserStateMachine, state_fn : StateFn<UserStateMachine>){
        let entry_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::EntryEvt;
        state_fn(user_state_machine, &entry_evt);
    }

    fn dispatch_get_super_state(user_state_machine : &mut UserStateMachine, state_fn : StateFn<UserStateMachine>) -> ParentState<UserStateMachine>{
        let get_parent_state_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::GetParentStateEvt;
        let core_handle_result = state_fn(user_state_machine, &get_parent_state_evt);
        if let CoreHandleResult::GetParentStateResult(parent_state_fn) = core_handle_result{
            parent_state_fn
        }
        else{
            panic!() //error should not be possible
        }
        
    }

    fn dispatch_init_evt(user_state_machine : &mut UserStateMachine, state_fn : StateFn<UserStateMachine>) ->InitResult<UserStateMachine>{
        let init_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::InitEvt;
        let init_result = state_fn(user_state_machine, &init_evt);
        match init_result{
            CoreHandleResult::InitResult(init_result) => return init_result,
            _ => panic!() //error, should not be possible
        } 
    }

    fn dispatch_exit_evt(user_state_machine : &mut UserStateMachine, state_fn : StateFn<UserStateMachine>){
        let exit_evt = CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>::ExitEvt;
        state_fn(user_state_machine, &exit_evt);
    }

    fn reach_init_target(&mut self, target_state_fn : StateFn<UserStateMachine>){
        
        let mut current_target_state_fn = target_state_fn;
        let user_state_machine = &mut self.user_state_machine;

        while let InitResult::TargetState(next_target_state) = Self::dispatch_init_evt(user_state_machine, current_target_state_fn){ 
            current_target_state_fn = next_target_state;
            Self::dispatch_entry_evt(user_state_machine, current_target_state_fn);
        }
        
        self.curr_state = current_target_state_fn;
    }
    
    fn handle_ignored_evt(&mut self, parent_state_variant : ParentState<UserStateMachine>,evt : &CoreEvt::<<UserStateMachine as ProtoStateMachine>::Evt>){
        match parent_state_variant{
            ParentState::Exists(super_state) => self.dispatch_core_event(super_state, evt),
            ParentState::TopReached => {}
        }
    }
    
    fn exit_substates(user_state_machine : &mut UserStateMachine, curr_state_fn : StateFn<UserStateMachine>, original_state_fn : StateFn<UserStateMachine>){
        let mut next_state_fn = curr_state_fn;

        while next_state_fn as *const fn() != original_state_fn as *const fn(){
            if let ParentState::Exists(parent_state_fn) = Self::dispatch_get_super_state(user_state_machine, next_state_fn){
                Self::dispatch_exit_evt(user_state_machine, next_state_fn);
                next_state_fn = parent_state_fn;
            }
            else{
                panic!()
           }
        }
    }
    
    fn find_dissociate_states<'a>(&mut self, original_state_link : &'a Link<'a, UserStateMachine>, target_state_link : &'a Link<'a, UserStateMachine>) -> (Option<&'a Link<'a, UserStateMachine>>,Option< &'a Link<'a, UserStateMachine>>){
       
        let mut original_state_link = original_state_link;
        let mut target_state_link = target_state_link;
        
        loop {

            if original_state_link.state_fn as *const fn() != target_state_link.state_fn as *const fn(){
                return (Some(original_state_link), Some(target_state_link));
            }
                
            if let Some(next_original_state_link) = original_state_link.next_link{
                
                if let Some(next_target_state_link) = target_state_link.next_link{
                    original_state_link = next_original_state_link;
                    target_state_link = next_target_state_link;
                }
                else{
                    return (original_state_link.next_link, None);
                }
            }

            else{
            
                if target_state_link.next_link.is_some(){
                    return (None, target_state_link.next_link);
                }
                
                else{
                    return (None, None);
                }
            
            
            }
        }
    }

    fn enter_substates(user_state_machine : &mut UserStateMachine, target_state_link  : &Link<UserStateMachine>) {
        
        let mut target_state_link = target_state_link;
        
        while let Some(next_state) = target_state_link.next_link{
            Self::dispatch_entry_evt(user_state_machine, next_state.state_fn);
            target_state_link = next_state;
        }

    }

    fn reach_target_state(&mut self, original_state_link : Link<UserStateMachine>, target_state_link : Link<UserStateMachine>){
        
        if let ParentState::Exists(original_state_parent_fn) = Self::dispatch_get_super_state(&mut self.user_state_machine, original_state_link.state_fn){
            let new_original_state_link = Link{state_fn: original_state_parent_fn, next_link :Some(&original_state_link)};
            
            if let ParentState::Exists(target_state_parent_fn) = Self::dispatch_get_super_state(&mut self.user_state_machine, target_state_link.state_fn){
                let new_target_state_link = Link{state_fn: target_state_parent_fn, next_link :Some(&target_state_link)};
                self.reach_target_state(new_original_state_link, new_target_state_link)
            }
            else{
                self.reach_target_state(new_original_state_link, target_state_link)
            }
        }
        else{
            
            if let ParentState::Exists(parent_state_fn) = Self::dispatch_get_super_state(&mut self.user_state_machine, target_state_link.state_fn){
                let new_target_state_link = Link{state_fn: parent_state_fn, next_link :Some(&target_state_link)};
                self.reach_target_state(original_state_link, new_target_state_link)
            }
            else{
                let (dissociated_original_state, dissociated_target_state) = self.find_dissociate_states(&original_state_link, &target_state_link);
                if let Some(dissociated_original_state_link) = dissociated_original_state{
                    Self::exit_substates(&mut self.user_state_machine, self.curr_state, dissociated_original_state_link.state_fn);
                    Self::dispatch_exit_evt(&mut self.user_state_machine, dissociated_original_state_link.state_fn);
                
                }

                if let Some(dissociated_target_state_link) = dissociated_target_state{
                    Self::dispatch_entry_evt(&mut self.user_state_machine, dissociated_target_state_link.state_fn);
                    Self::enter_substates(&mut self.user_state_machine, dissociated_target_state_link);
                }
            }   
        }
    }
     
    fn handle_transition(&mut self, handling_state_fn : StateFn<UserStateMachine>, target_state_fn : StateFn<UserStateMachine>){
       
        Self::exit_substates(&mut self.user_state_machine, self.curr_state, handling_state_fn);

        // Special handling in case of targetting the current state
        if handling_state_fn as *const fn() == target_state_fn as *const fn(){
            Self::dispatch_exit_evt(&mut self.user_state_machine, handling_state_fn);
            Self::dispatch_entry_evt(&mut self.user_state_machine, handling_state_fn);
        }

        let curr_state_link = Link { state_fn: handling_state_fn, next_link: None };
        let target_state_link = Link { state_fn: target_state_fn, next_link: None };

        self.reach_target_state(curr_state_link, target_state_link);

        let curr_state_after_target_reached = target_state_fn;
        self.reach_init_target(curr_state_after_target_reached);
        
        
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

    
}
