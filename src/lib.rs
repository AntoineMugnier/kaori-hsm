use std::marker::PhantomData;


mod sealed{
  
}
pub trait TopState
{
    type Evt;
    fn init(&mut self);
}

pub trait State<T>
where Self : TopState{
    type ParentState ;
    
    fn init(&mut self);

    fn entry(&mut self);

    fn exit(&mut self);

    fn handle(&mut self, evt: <Self as TopState>::Evt);
    
    fn core_handle(&mut self, evt: CoreEvt::<<Self as TopState>::Evt>){
        match evt{
            CoreEvt::Init => <Self as State<T>>::init(self),
            CoreEvt::Entry => <Self as State<T>>::entry(self),
            CoreEvt::Exit => <Self as State<T>>::exit(self),
            CoreEvt::User { user_evt } => <Self as State<T>>::handle(self, user_evt),
        }
    }
}

//Todo delete pub
pub enum CoreEvt<UserEvtT>{
    Init,
    Entry,
    Exit,
    User{user_evt : UserEvtT}
}

pub struct StateMachine<UserStateMachine>{
    user_state_machine : UserStateMachine
}

impl <UserStateMachine : TopState>StateMachine<UserStateMachine>{
    
    pub fn new(user_state_machine : UserStateMachine) -> StateMachine<UserStateMachine>{
        StateMachine{user_state_machine}
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
