mod std_async_tcp;
use embassy_executor::Executor;
use static_cell::StaticCell;
pub use std_async_tcp::AsyncTcp;

pub static EXECUTOR: StaticCell<Executor> = StaticCell::new();

pub async fn run(device: &mut embassy_ha::Device<'_>, stream: &mut AsyncTcp) {
    let events = embassy_sync::channel::Channel::<
        embassy_sync::blocking_mutex::raw::NoopRawMutex,
        embassy_ha::MqttState,
        1,
    >::new();
    let run = embassy_ha::run(device, stream, events.sender().into(), Default::default());
    let drain_events = async {
        loop {
            tracing::debug!("MQTT state: {:?}", events.receive().await);
        }
    };
    let embassy_futures::select::Either::First(result) = embassy_futures::select::select(run, drain_events).await;
    result.unwrap();
}

#[macro_export]
macro_rules! example_main {
    () => {
        fn main() {
            // Initialize tracing if tracing feature is enabled
            #[cfg(feature = "tracing")]
            {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }

            let executor = common::EXECUTOR.init(Executor::new());
            executor.run(|spawner| {
                spawner.spawn(main_task(spawner).unwrap());
            });
        }
    };
}
