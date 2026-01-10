use embassy_futures::{
    join::join3,
    select::{select, select3, Either, Either3},
};
use embassy_time::Instant;
use esp_hal::gpio::{Input, Output};

#[embassy_executor::task]
pub async fn buzzer_task(mut output: Output<'static>) {}
