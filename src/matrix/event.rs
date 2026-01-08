use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicSender},
};

static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, MatrixEvent, 16> = Channel::new();

#[derive(Debug, Clone, Copy)]
pub enum MatrixEvent {
    Left,
    Right,
    Select,
}

pub fn get_event_channel_sender() -> DynamicSender<'static, MatrixEvent> {
    EVENT_CHANNEL.dyn_sender()
}

pub fn get_event_channel_receiver() -> embassy_sync::channel::Receiver<'static, CriticalSectionRawMutex, MatrixEvent, 16>
{
    EVENT_CHANNEL.receiver()
}
