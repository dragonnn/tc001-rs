use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn heap_task() {
    loop {
        let heap_stats = esp_alloc::HEAP.stats();
        let current_percentage = heap_stats.current_usage as f32 / heap_stats.size as f32 * 100.0;
        let max_percentage = heap_stats.max_usage as f32 / heap_stats.size as f32 * 100.0;
        //info!("Heap stats - total: {:?}", heap_stats);
        info!(
            "Heap usage - current: {} bytes ({:.2}%), max: {} bytes ({:.2}%)",
            heap_stats.current_usage, current_percentage, heap_stats.max_usage, max_percentage
        );
        Timer::after(Duration::from_secs(5)).await;
    }
}
