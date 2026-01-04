use core::sync::atomic::{AtomicBool, Ordering};

static TRANSITION_STATE: AtomicBool = AtomicBool::new(true);

pub fn get_transition_state() -> bool {
    TRANSITION_STATE.load(Ordering::SeqCst)
}

pub fn external_set_transition_state(state: bool) {
    TRANSITION_STATE.store(state, Ordering::SeqCst);
}
