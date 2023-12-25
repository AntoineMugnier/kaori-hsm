use crate::state::denatured;

// Function existing only in order to attribute a unique adress to what is refered as the
// top state. Is not meant to be called.
pub fn top_state_fn(
    _user_state_machine: *mut denatured::OpaqueType,
    _evt: &denatured::CoreEvt,
) -> denatured::CoreHandleResult {
    panic!("Top state function should never be called !");
}

// Descending phase of a transition. Entry condition in every LCA descendant are successively
// executed until the target state is reach.
pub(crate) fn enter_substates(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link: &Link,
    lca_state_fn: denatured::StateFn,
) {
    let mut target_state_link = target_state_link;
    let mut ignore_link = true;
    while {
        if target_state_link.state_fn == lca_state_fn {
            ignore_link = false;
        }
        target_state_link.next_link.is_some()
    } {
        target_state_link = target_state_link.next_link.unwrap();
        if ignore_link == false {
            dispatch_entry_evt(user_state_machine, target_state_link.state_fn);
        }
    }
}

// Search for the LCA (Least Common Ancestor) state between the target and the source state. Also
// proceed to eventually exiting every state in the handling state lineage before the LCA is found.
pub(crate) fn search_lca_state(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link: &Link,
    source_state_fn: denatured::StateFn,
) -> denatured::StateFn {
    let mut source_state_fn = source_state_fn;
    loop {
        let mut state_link = target_state_link;

        while {
            if state_link.state_fn == source_state_fn {
                return source_state_fn;
            }
            state_link.next_link.is_some()
        } {
            state_link = state_link.next_link.unwrap();
        }

        dispatch_exit_evt(user_state_machine, source_state_fn);

        if let denatured::ParentState::Exists(parent_state_fn) =
            dispatch_get_super_state(user_state_machine, source_state_fn)
        {
            source_state_fn = parent_state_fn;
        } else {
            source_state_fn = top_state_fn;
        }
    }
}

// Recursive function whose role is to create a stack-allocated linked list of all the ancestors
// of the target state up to the top state. This linked list is then used by the function
// `search_lca_state()` for finding the lca and transitioning to the target state.
pub(crate) fn reach_target_state(
    user_state_machine: &mut denatured::OpaqueType,
    target_state_link: Link,
    source_state_fn: denatured::StateFn,
) {
    if let denatured::ParentState::Exists(parent_state_fn) =
        dispatch_get_super_state(user_state_machine, target_state_link.state_fn)
    {
        let parent_state_link = Link {
            state_fn: parent_state_fn,
            next_link: Some(&target_state_link),
        };
        reach_target_state(user_state_machine, parent_state_link, source_state_fn)
    } else {
        let parent_state_link = Link {
            state_fn: top_state_fn,
            next_link: Some(&target_state_link),
        };
        let lca_state_fn =
            search_lca_state(user_state_machine, &parent_state_link, source_state_fn);
        enter_substates(user_state_machine, &target_state_link, lca_state_fn)
    }
}

// Return the parent state of the `state_fn` state sent as argument
#[inline(always)]
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

// Trigger the exit condition of the state `state_fn`
pub(crate) fn dispatch_exit_evt(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) {
    let exit_evt = denatured::CoreEvt::ExitEvt;
    state_fn(user_state_machine, &exit_evt);
}

// Trigger the initial transition of the state `state_fn`
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

// Descend the state hierarchy by potentially executing the series of initial transitions and entry
// conditions until the leaf state is reached.
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

// Exit all ascendants of the `source_state_fn` until the `lca_state_fn` is reached
pub(crate) fn exit_substates(
    user_state_machine: &mut denatured::OpaqueType,
    source_state_fn: denatured::StateFn,
    lca_state_fn: denatured::StateFn,
) {
    let mut next_state_fn = source_state_fn;

    while next_state_fn != lca_state_fn {
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

// Trigger the entry condition of the state `state_fn`
pub(crate) fn dispatch_entry_evt(
    user_state_machine: &mut denatured::OpaqueType,
    state_fn: denatured::StateFn,
) {
    let entry_evt = denatured::CoreEvt::EntryEvt;
    state_fn(user_state_machine, &entry_evt);
}

// Reach the first state of the state machine by descending from init conditions into init
// conditions.
pub(crate) fn init(
    user_state_machine: &mut denatured::OpaqueType,
    init_result: &denatured::InitResult,
) -> denatured::StateFn {
    match init_result {
        denatured::InitResult::TargetState(topmost_init_target_state_fn) => {
            dispatch_entry_evt(user_state_machine, *topmost_init_target_state_fn);
            reach_init_target(user_state_machine, *topmost_init_target_state_fn)
        }
        denatured::InitResult::NotImplemented => panic!("Topmost Init should return a state"),
    }
}

// Take a transition from the `handling_state_fn` to the `target_state_fn`, thus setting
// `target_state_fn` as the new current state of the state machine at the end of the process.
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
    } else {
        let target_state_link = Link {
            state_fn: target_state_fn,
            next_link: None,
        };

        reach_target_state(user_state_machine, target_state_link, handling_state_fn);
    }
    *current_state_fn = reach_init_target(user_state_machine, target_state_fn);
}

// Dispatch the user event to the parent state at the condition it is not the top state.
pub(crate) fn dispatch_evt_to_parent(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    parent_state_variant: denatured::ParentState,
    evt: &denatured::CoreEvt,
) {
    match parent_state_variant {
        denatured::ParentState::Exists(super_state) => {
            dispatch_evt_to_handling_state(user_state_machine, current_state_fn, super_state, evt)
        }
        denatured::ParentState::TopReached => {}
    }
}

// Dispatch the user event to a state different from the current one.
pub(crate) fn dispatch_evt_to_handling_state(
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
            dispatch_evt_to_parent(user_state_machine, current_state_fn, parent_state_fn, evt)
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

// Dispatch the user event to the current state.
pub(crate) fn dispatch_evt_to_current_state(
    user_state_machine: &mut denatured::OpaqueType,
    current_state_fn: &mut denatured::StateFn,
    evt: &denatured::CoreEvt,
) {
    dispatch_evt_to_handling_state(user_state_machine, current_state_fn, *current_state_fn, evt)
}

pub(crate) struct Link<'a> {
    state_fn: denatured::StateFn,
    next_link: Option<&'a Link<'a>>,
}
