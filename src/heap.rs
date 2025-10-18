use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn heap_task() {
    loop {
        let heap_stats = esp_alloc::HEAP.stats();
        info!("Heap stats - total: {:?}", heap_stats);
        Timer::after(Duration::from_secs(5)).await;
    }
}
