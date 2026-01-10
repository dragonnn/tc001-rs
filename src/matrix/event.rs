use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicSender},
};
use embassy_time::Duration;

static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, MatrixEventDetails, 16> = Channel::new();

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MatrixEvent {
    Left,
    Right,
    Select,
}

#[derive(Debug, Clone)]
pub struct MatrixEventDetails {
    pub duration: Duration,
    pub main: MatrixEvent,
    pub events: heapless::index_set::FnvIndexSet<MatrixEvent, 4>,
}

impl MatrixEventDetails {
    pub fn new() -> Self {
        Self {
            duration: Duration::from_millis(0),
            events: heapless::index_set::FnvIndexSet::new(),
            main: MatrixEvent::Left,
        }
    }

    pub fn push_event(&mut self, event: MatrixEvent) {
        self.events.insert(event).ok();
    }

    pub fn push_left(&mut self) {
        self.push_event(MatrixEvent::Left);
    }

    pub fn push_select(&mut self) {
        self.push_event(MatrixEvent::Select);
    }

    pub fn push_right(&mut self) {
        self.push_event(MatrixEvent::Right);
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn set_main(&mut self, event: MatrixEvent) {
        self.main = event;
    }

    pub fn get_main(&self) -> MatrixEvent {
        self.main
    }

    pub fn has_event(&self, event: MatrixEvent) -> bool {
        self.events.contains(&event)
    }

    pub fn has_left(&self) -> bool {
        self.has_event(MatrixEvent::Left)
    }

    pub fn has_select(&self) -> bool {
        self.has_event(MatrixEvent::Select)
    }

    pub fn has_right(&self) -> bool {
        self.has_event(MatrixEvent::Right)
    }

    pub fn is_long_press(&self) -> bool {
        self.duration >= Duration::from_millis(500)
    }

    pub fn is_single_press(&self) -> bool {
        self.events.len() == 1
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub fn get_event_channel_sender() -> DynamicSender<'static, MatrixEventDetails> {
    EVENT_CHANNEL.dyn_sender()
}

pub fn get_event_channel_receiver(
) -> embassy_sync::channel::Receiver<'static, CriticalSectionRawMutex, MatrixEventDetails, 16> {
    EVENT_CHANNEL.receiver()
}
