use crate::proto_state_machine::ProtoStateMachine;

pub enum ParentState<UserStateMachine>
    where UserStateMachine : ProtoStateMachine + ?Sized{
    ProtoStateMachine,
    SubState(StateFn<UserStateMachine>)
}

pub struct Top{}

pub type StateFn<UserStateMachineT> = fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachineT>;
pub type RawStateFn<UserStateMachineT> = *const fn(&mut UserStateMachineT, &CoreEvt<<UserStateMachineT as ProtoStateMachine>::Evt>) -> CoreHandleResult<UserStateMachineT>;

pub struct InitResult<UserStateMachine : ProtoStateMachine + ?Sized>(
    pub Option<StateFn<UserStateMachine>>
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
    ReturnParentState(ParentState<UserStateMachineT>),
    InitResult(InitResult<UserStateMachineT>)
}

pub enum CoreEvt<'a, UserEvtT>{
    Init,
    Entry,
    Exit,
    GetParentState,
    User{user_evt : &'a UserEvtT}
}
