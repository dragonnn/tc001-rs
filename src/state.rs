use core::sync::atomic::{AtomicBool, Ordering};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

static TRANSITION_STATE: AtomicBool = AtomicBool::new(true);
static TRANSITION_INTERNAL_CHANGED: Signal<CriticalSectionRawMutex, bool> = Signal::new();

static INDICATORS_STATE: [AtomicBool; 3] = [AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false)];

static STATE_CHANGED: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub fn get_transition_state() -> bool {
    TRANSITION_STATE.load(Ordering::Relaxed)
}

pub fn external_set_transition_state(state: bool) {
    TRANSITION_STATE.store(state, Ordering::Relaxed);
    STATE_CHANGED.signal(());
}

pub fn internal_set_transition_state(state: bool) {
    TRANSITION_STATE.store(state, Ordering::Relaxed);
    TRANSITION_INTERNAL_CHANGED.signal(state);
    STATE_CHANGED.signal(());
}

pub async fn wait_for_internal_transition_state_change() -> bool {
    TRANSITION_INTERNAL_CHANGED.wait().await
}

pub fn set_indicator_state(index: usize, state: bool) {
    if index < INDICATORS_STATE.len() {
        INDICATORS_STATE[index].store(state, Ordering::Relaxed);
        STATE_CHANGED.signal(());
    }
}

pub fn get_indicators_state() -> [bool; 3] {
    [
        INDICATORS_STATE[0].load(Ordering::Relaxed),
        INDICATORS_STATE[1].load(Ordering::Relaxed),
        INDICATORS_STATE[2].load(Ordering::Relaxed),
    ]
}

#[embassy_executor::task]
pub async fn state_task(storage: crate::storage::Storage) {
    let transition = storage.read::<bool>(&crate::storage::Key::TransitionState).await.unwrap_or(true);
    TRANSITION_STATE.store(transition, Ordering::Relaxed);
    let indicators = storage.read::<[bool; 3]>(&crate::storage::Key::IndicatorsState).await.unwrap_or([false; 3]);
    for (i, state) in indicators.iter().enumerate() {
        INDICATORS_STATE[i].store(*state, Ordering::Relaxed);
    }

    loop {
        STATE_CHANGED.wait().await;
        let transition = TRANSITION_STATE.load(Ordering::Relaxed);
        let indicators = get_indicators_state();
        storage.save(&crate::storage::Key::TransitionState, &transition).await.expect("failed saving transition state");
        storage.save(&crate::storage::Key::IndicatorsState, &indicators).await.expect("failed saving indicators state");
        info!("State saved: transition={}, indicators={:?}", transition, indicators);
    }
}
