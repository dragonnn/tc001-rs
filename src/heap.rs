use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn heap_task() {
    loop {
        Timer::after(Duration::from_secs(60)).await;
        let heap_stats = esp_alloc::HEAP.stats();
        info!("Heap stats - total: {:?}", heap_stats);
    }
}
