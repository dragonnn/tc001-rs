use embassy_futures::{
    join::join3,
    select::{select, select3, Either, Either3},
};
use embassy_time::Instant;
use esp_hal::gpio::Input;

use crate::matrix::event::{get_event_channel_sender, MatrixEvent, MatrixEventDetails};

#[embassy_executor::task]
pub async fn button_task(mut left: Input<'static>, mut mid: Input<'static>, mut right: Input<'static>) {
    let sender = get_event_channel_sender();
    let mut buttons = Buttons::new(left, mid, right);

    loop {
        buttons.wait_for_press().await;
        let matrix_event = buttons.wait_for_release().await;
        info!("Sending button event: {:?}", matrix_event);
        sender.send(matrix_event).await;
    }
}

struct Buttons {
    left: Input<'static>,
    mid: Input<'static>,
    right: Input<'static>,

    //button: MatrixEvent,
    duration: Instant,
    event: MatrixEventDetails,
}

impl Buttons {
    pub fn new(left: Input<'static>, mid: Input<'static>, right: Input<'static>) -> Self {
        Self { left, mid, right, event: MatrixEventDetails::new(), duration: Instant::now() }
    }

    async fn wait_for_any_press(&mut self) -> MatrixEvent {
        match select3(
            self.left.wait_for_falling_edge(),
            self.mid.wait_for_falling_edge(),
            self.right.wait_for_falling_edge(),
        )
        .await
        {
            Either3::First(_) => MatrixEvent::Left,
            Either3::Second(_) => MatrixEvent::Select,
            Either3::Third(_) => MatrixEvent::Right,
        }
    }

    async fn wait_for_all_release(&mut self) {
        join3(self.left.wait_for_high(), self.mid.wait_for_high(), self.right.wait_for_high()).await;
    }

    async fn wait_for_release_or_more_press(
        main: &mut Input<'static>,
        ((extra0, extra0_event), (extra1, extra1_event)): (
            (&mut Input<'static>, MatrixEvent),
            (&mut Input<'static>, MatrixEvent),
        ),
    ) -> Option<MatrixEvent> {
        match select(
            main.wait_for_rising_edge(),
            select(extra0.wait_for_falling_edge(), extra1.wait_for_falling_edge()),
        )
        .await
        {
            Either::First(_) => None,
            Either::Second(either) => match either {
                Either::First(_) => Some(extra0_event),
                Either::Second(_) => Some(extra1_event),
            },
        }
    }

    pub async fn wait_for_press(&mut self) {
        self.event.clear();
        let button = self.wait_for_any_press().await;
        info!("Button {:?} pressed", button);
        self.event.set_main(button);
        self.event.push_event(button);
        self.duration = Instant::now();
    }

    pub async fn wait_for_release(&mut self) -> MatrixEventDetails {
        loop {
            let (main, (extra0, extra1)) = match self.event.get_main() {
                MatrixEvent::Left => {
                    (&mut self.left, ((&mut self.mid, MatrixEvent::Select), (&mut self.right, MatrixEvent::Right)))
                }
                MatrixEvent::Select => {
                    (&mut self.mid, ((&mut self.left, MatrixEvent::Left), (&mut self.right, MatrixEvent::Right)))
                }
                MatrixEvent::Right => {
                    (&mut self.right, ((&mut self.left, MatrixEvent::Left), (&mut self.mid, MatrixEvent::Select)))
                }
            };

            if let Some(new_button) = Self::wait_for_release_or_more_press(main, (extra0, extra1)).await {
                info!("Button {:?} pressed while waiting for release of {:?}", new_button, self.event.get_main());
                self.event.push_event(new_button);
            } else {
                break;
            }
        }
        self.event.set_duration(Instant::now().checked_duration_since(self.duration).unwrap_or_default());
        info!("Button {:?} released after {:?}", self.event.get_main(), self.event.duration);
        self.wait_for_all_release().await;
        self.event.clone()
    }
}
