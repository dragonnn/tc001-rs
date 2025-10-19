use embassy_futures::select::{select3, Either3};
use esp_hal::gpio::Input;

#[embassy_executor::task]
pub async fn button_task(mut left: Input<'static>, mut mid: Input<'static>, mut right: Input<'static>) {
    loop {
        match select3(left.wait_for_falling_edge(), mid.wait_for_falling_edge(), right.wait_for_falling_edge()).await {
            Either3::First(_) => info!("Left button pressed"),
            Either3::Second(_) => info!("Middle button pressed"),
            Either3::Third(_) => info!("Right button pressed"),
        }
    }
}
