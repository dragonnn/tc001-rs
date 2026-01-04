use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use embassy_executor::Spawner;
use embassy_time::Timer;
use static_cell::StaticCell;

//light.awtrix_d12e5c_indicator_1

const MQTT_BROKER_ADDRESS: &str = dotenvy_macro::dotenv!("MQTT_BROKER_ADDRESS");
const MQTT_BROKER_PORT: &str = dotenvy_macro::dotenv!("MQTT_BROKER_PORT");
const MQTT_USER: &str = dotenvy_macro::dotenv!("MQTT_USER");
const MQTT_PASSWORD: &str = dotenvy_macro::dotenv!("MQTT_PASSWORD");

static RESOURCES: StaticCell<embassy_ha::DeviceResources> = StaticCell::new();

#[embassy_executor::task]
pub async fn ha_task(spawner: Spawner, stack: embassy_net::Stack<'static>) {
    crate::wifi::wait_for_connection(&stack).await;
    Timer::after(embassy_time::Duration::from_secs(1)).await;

    let mac = esp_radio::wifi::station_mac();
    info!("Device MAC address: {:02X?}", mac);

    let mut device_id = String::with_capacity(13);

    write!(&mut device_id, "rwtrix_{:02x}{:02x}{:02x}", mac[3], mac[4], mac[5]).ok();

    let device_id = Box::leak(device_id.into_boxed_str());

    let device = embassy_ha::new(
        RESOURCES.init(Default::default()),
        embassy_ha::DeviceConfig { device_id, device_name: device_id, manufacturer: "Dragonn", model: "AWTRIX 3" },
    );

    let switch_indicator1 = embassy_ha::create_switch(
        &device,
        "ind1",
        embassy_ha::SwitchConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Indicator 1"),
                icon: Some("mdi:arrow-top-right-thick"),
                ..Default::default()
            },
            class: embassy_ha::SwitchClass::Generic,
            command_policy: embassy_ha::CommandPolicy::PublishState,
        },
    );

    let switch_indicator2 = embassy_ha::create_switch(
        &device,
        "ind2",
        embassy_ha::SwitchConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Indicator 2"),
                icon: Some("mdi:arrow-right-thick"),
                ..Default::default()
            },
            class: embassy_ha::SwitchClass::Generic,
            command_policy: embassy_ha::CommandPolicy::PublishState,
        },
    );

    let switch_indicator3 = embassy_ha::create_switch(
        &device,
        "ind3",
        embassy_ha::SwitchConfig {
            common: embassy_ha::EntityCommonConfig {
                name: Some("Indicator 3"),
                icon: Some("mdi:arrow-bottom-right-thick"),
                ..Default::default()
            },
            class: embassy_ha::SwitchClass::Generic,
            command_policy: embassy_ha::CommandPolicy::PublishState,
        },
    );

    spawner.must_spawn(switch_class(switch_indicator1));
    spawner.must_spawn(switch_class(switch_indicator2));
    spawner.must_spawn(switch_class(switch_indicator3));

    let mqtt_params =
        embassy_ha::MqttConnectParams { username: Some(MQTT_USER), password: Some(MQTT_PASSWORD.as_bytes()) };

    embassy_ha::connect_and_run(stack, device, MQTT_BROKER_ADDRESS, mqtt_params).await;
}

#[embassy_executor::task(pool_size = 3)]
async fn switch_class(mut switch: embassy_ha::Switch<'static>) {
    loop {
        let state = switch.toggle();
        //info!("state = {}", state);
        Timer::after_secs(2).await;
    }
}
