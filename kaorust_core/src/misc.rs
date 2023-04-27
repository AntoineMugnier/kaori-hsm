use crate::proto_state_machine::ProtoStateMachine;
use crate::state::{CoreHandleResult, HandleResult};

pub struct Top{}

pub type StateFn<UserStateMachineT> = fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachineT>;
pub type RawStateFn<UserStateMachineT> = *const fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachineT>;


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

