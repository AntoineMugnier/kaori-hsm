use crate::state::denatured;

pub fn top_state_fn(_user_state_machine : denatured::OpaquePtr, _evt: &denatured::CoreEvt) -> denatured::CoreHandleResult{
    panic!("Top state function should never be called !");
}
pub(crate) fn enter_substates(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link: &Link,
    cutout_state_fn : denatured::StateFn
) {
    let mut target_state_link = target_state_link;
    let mut ignore_link = true;
    while{
        if target_state_link.state_fn == cutout_state_fn{
            ignore_link = false;
        }
        target_state_link.next_link.is_some()
   }{
        target_state_link = target_state_link.next_link.unwrap();
        if ignore_link == false{
            dispatch_entry_evt(user_state_machine, target_state_link.state_fn);
        }
    }
}
pub(crate) fn search_matching_state(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link : &Link,
    original_state_fn : denatured::StateFn
)-> denatured::StateFn {
    let mut state_link = target_state_link;
    while{
        if state_link.state_fn == original_state_fn{
            return original_state_fn 
        }
        state_link.next_link.is_some()
    }{
        state_link = state_link.next_link.unwrap();
    }
    
    dispatch_exit_evt(user_state_machine, original_state_fn);
    
    if let denatured::ParentState::Exists(parent_state_fn) =
            dispatch_get_super_state(user_state_machine, original_state_fn)
        {
        return search_matching_state(user_state_machine, target_state_link, parent_state_fn)
    }else {
        return search_matching_state(user_state_machine, target_state_link, top_state_fn);
        }
}

pub(crate) fn reach_target_state(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link: Link,
    original_state_fn: denatured::StateFn,
) {
    
        if let denatured::ParentState::Exists(parent_state_fn) =
            dispatch_get_super_state(user_state_machine, target_state_link.state_fn)
        {
            let parent_state_link = Link {
                state_fn: parent_state_fn,
                next_link: Some(&target_state_link),
            };
            reach_target_state(user_state_machine, parent_state_link, original_state_fn)
        } else {

        let parent_state_link = Link {
                state_fn: top_state_fn,
                next_link: Some(&target_state_link),
            };
            let common_state = search_matching_state(user_state_machine, &parent_state_link, original_state_fn);
            enter_substates(user_state_machine, &target_state_link, common_state)
        }
    
}

pub(crate) fn dispatch_get_super_state(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) -> denatured::ParentState {
    let get_parent_state_evt = denatured::CoreEvt::GetParentStateEvt;

    let core_handle_result = state_fn(user_state_machine, &get_parent_state_evt);

    if let denatured::CoreHandleResult::GetParentStateResult(parent_state_fn) = core_handle_result {
        parent_state_fn
    } else {
        panic!("Variant returned by state fn is not ParentState")
    }
}

pub(crate) fn dispatch_exit_evt(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) {
    let exit_evt = denatured::CoreEvt::ExitEvt;
    state_fn(user_state_machine, &exit_evt);
}

fn dispatch_init_evt(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) -> denatured::InitResult {
    let init_evt = denatured::CoreEvt::InitEvt;
    let init_result = state_fn(user_state_machine, &init_evt);
    match init_result {
        denatured::CoreHandleResult::InitResult(init_result) => return init_result,
        _ => panic!("Variant returned by state fn is not InitResult"),
    }
}

pub(crate) fn reach_init_target(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_fn: denatured::StateFn,
) -> denatured::StateFn {
    let mut current_target_state_fn = target_state_fn;

    while let denatured::InitResult::TargetState(next_target_state) =
        dispatch_init_evt(user_state_machine, current_target_state_fn)
    {
        current_target_state_fn = next_target_state;
        dispatch_entry_evt(user_state_machine, current_target_state_fn);
    }

    current_target_state_fn
}

pub(crate) fn exit_substates(
    user_state_machine: &mut denatured::OpaqueType,
    curr_state_fn: denatured::StateFn,
    target_state_fn: denatured::StateFn,
) {
    let mut next_state_fn = curr_state_fn;

    while next_state_fn != target_state_fn {
        if let denatured::ParentState::Exists(parent_state_fn) =
            dispatch_get_super_state(user_state_machine, next_state_fn)
        {
            dispatch_exit_evt(user_state_machine, next_state_fn);
            next_state_fn = parent_state_fn;
        } else {
            panic!("Target state not found when ascending state hierarchy")
        }
    }
}

pub(crate) fn dispatch_entry_evt(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) {
    let entry_evt = denatured::CoreEvt::EntryEvt;
    state_fn(user_state_machine, &entry_evt);
}
/// Will trigger the execution of the initial pseudostate of the state machine by calling
/// `ProtoStateMachine::init`. That call will lead to the first state of the machine to be
/// set.   
/// This method should only be called once
pub(crate) fn init(
    user_state_machine: &mut denatured::OpaqueType,
    init_result: &denatured::InitResult,
) -> denatured::StateFn {
    match init_result {
        denatured::InitResult::TargetState(topmost_init_target_state_fn) => {
            // Reach leaf state
            dispatch_entry_evt(user_state_machine, *topmost_init_target_state_fn);

            reach_init_target(user_state_machine, *topmost_init_target_state_fn)
        }

        denatured::InitResult::NotImplemented => panic!("Topmost Init should return a state"),
    }
}

pub(crate) fn handle_transition(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    handling_state_fn: denatured::StateFn,
    target_state_fn: denatured::StateFn,
) {
    exit_substates(user_state_machine, *current_state_fn, handling_state_fn);

    // Special handling in case of targetting the current state
    if handling_state_fn == target_state_fn {
        dispatch_exit_evt(user_state_machine, handling_state_fn);
        dispatch_entry_evt(user_state_machine, handling_state_fn);
    }
    else{
        let target_state_link = Link {
            state_fn: target_state_fn,
            next_link: None,
        };
        
        reach_target_state(
            user_state_machine,
            target_state_link,
            handling_state_fn,
        );
    }
    *current_state_fn = reach_init_target(user_state_machine, target_state_fn);
}

pub(crate) fn handle_ignored_evt(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    parent_state_variant: denatured::ParentState,
    evt: &denatured::CoreEvt,
) {
    match parent_state_variant {
        denatured::ParentState::Exists(super_state) => {
            dispatch_core_event(user_state_machine, current_state_fn, super_state, evt)
        }
        denatured::ParentState::TopReached => {}
    }
}

pub(crate) fn dispatch_core_event(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    handling_state_fn: denatured::StateFn,
    evt: &denatured::CoreEvt,
) {
    let core_handle_result = handling_state_fn(user_state_machine, evt);

    // Treat result of the event dispatch
    match core_handle_result {
        denatured::CoreHandleResult::Handled => {}
        denatured::CoreHandleResult::Ignored(parent_state_fn) => {
            handle_ignored_evt(user_state_machine, current_state_fn, parent_state_fn, evt)
        }
        denatured::CoreHandleResult::Transition(target_state_fn) => handle_transition(
            user_state_machine,
            current_state_fn,
            handling_state_fn,
            target_state_fn,
        ),
        _ => {}
    }
}

pub(crate) fn dispatch_event(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    evt: &denatured::CoreEvt,
) {
    dispatch_core_event(user_state_machine, current_state_fn, *current_state_fn, evt)
}

pub(crate) struct Link<'a> {
    state_fn: denatured::StateFn,
    next_link: Option<&'a Link<'a>>,
}
