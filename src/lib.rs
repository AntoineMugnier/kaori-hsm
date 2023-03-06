pub struct TopState;
mod Sealed{
  
}

pub trait State<T: CoreStateMachine>{
    type ParentState;

    fn init(data : &mut T::Data);

    fn entry(data: &mut T::Data);

    fn exit(data : &mut T::Data);

    fn handle(data : &mut T::Data, evt: T::Evt);
    
    fn core_handle(data : &mut T::Data, evt: CoreEvt<T::Evt>){

    }
}
//Todo delete pub
pub enum CoreEvt<UserEvtT>{
    Init,
    Entry,
    Exit,
    User{user_evt : UserEvtT}
}

pub trait CoreStateMachine{
    type Evt;
    type Data;
}

impl <DataT, UserEvtT> CoreStateMachine for StateMachine<DataT, UserEvtT>{
   type Evt = UserEvtT;
   type Data = DataT;
}

pub struct StateMachine<DataT, UserEvtT>{
    current_state : fn(&mut DataT, evt: CoreEvt<UserEvtT>),
    data : DataT
}

impl <DataT, UserEvt>StateMachine<DataT, UserEvt>{
    
    pub fn new<EntryState: State<Self>>(data : DataT) -> StateMachine<DataT, UserEvt>{
        StateMachine{current_state: EntryState::core_handle, data}
    }

    pub fn dispatch(&mut self, evt : CoreEvt<UserEvt>){

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
