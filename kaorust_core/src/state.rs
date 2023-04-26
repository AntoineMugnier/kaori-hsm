use crate::proto_state_machine::ProtoStateMachine;
use crate::misc::{CoreEvt, InitResult, HandleResult, ParentState, CoreHandleResult};

pub trait State<T>
where Self : ProtoStateMachine{
    
    fn get_parent_state() -> ParentState<Self>;

    fn init(&mut self) -> InitResult<Self>{
       InitResult::NotImplemented
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
            CoreEvt::InitEvt => {
               return CoreHandleResult::InitResult(<Self as State<T>>::init(self));
            }
            CoreEvt::EntryEvt => {
                <Self as State<T>>::entry(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::ExitEvt => {
                <Self as State<T>>::exit(self);
                return CoreHandleResult::Handled;
            }
            CoreEvt::GetParentStateEvt =>{
                return CoreHandleResult::GetParentStateResult(Self::get_parent_state());
            }
            CoreEvt::UserEvt { user_evt } => {
                match <Self as State<T>>::handle(self, user_evt){
                    HandleResult::Ignored => return CoreHandleResult::Ignored(Self::get_parent_state()),
                    HandleResult::Handled => return CoreHandleResult::Handled,
                    HandleResult::Transition(state_fn) => CoreHandleResult::Transition(state_fn)
                }
            }
        }
    }
}
