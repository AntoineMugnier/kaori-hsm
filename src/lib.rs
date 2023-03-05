struct NoneState;

mod Sealed{
  
}

pub trait State<T: StateMachine>{
    fn init(sm : &mut T);

    fn entry(sm: &mut T);

    fn exit(sm : &mut T);

    fn handle(sm : &mut T, evt: T::Evt);
    
    fn core_handle(sm : &mut T, evt: CoreEvt<T::Evt>){

    }
}
//Todo delete pub
pub enum CoreEvt<USER_EVT_T>{
    Init,
    Entry,
    Exit,
    User{user_evt : USER_EVT_T}
}

pub trait StateMachine{
    type Evt;
    fn dispatch(&mut self, evt : CoreEvt<Self::Evt>){}
    //fn get_curr_state(&mut self) -> fn(&mut StateMachine, evt: Self::Evt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
