use crate::proto_state_machine::ProtoStateMachine;
use crate::state::{StateFn, HandleResult};

pub struct Top{}


pub enum ParentState<UserStateMachine : ProtoStateMachine + ?Sized>{
    TopReached,
    Exists(StateFn<UserStateMachine>)
}

pub enum InitResult<UserStateMachine : ProtoStateMachine + ?Sized>{
    NotImplemented,
    TargetState(StateFn<UserStateMachine>)
}



pub enum CoreEvt<'a, UserEvtT>{
    InitEvt,
    EntryEvt,
    ExitEvt,
    GetParentStateEvt,
    UserEvt{user_evt : &'a UserEvtT}
}

