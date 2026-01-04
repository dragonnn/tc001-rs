use alloc::string::String;
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
    Timer::after(embassy_time::Duration::from_secs(5)).await;

    let mac = esp_radio::wifi::station_mac();
    info!("Device MAC address: {:02X?}", mac);

    let mut mqtt_prefix = String::with_capacity(13);

    write!(&mut mqtt_prefix, "awtrix_{:02x}{:02x}{:02x}", mac[3], mac[4], mac[5]).ok();

    let mut device = embassy_ha::new(
        RESOURCES.init(Default::default()),
        embassy_ha::DeviceConfig {
            device_id: "example-device-id",
            device_name: "Example Device Name",
            manufacturer: "Example Device Manufacturer",
            model: "Example Device Model",
        },
    );

    let sensor = embassy_ha::create_binary_sensor(
        &device,
        "binary-sensor-id",
        embassy_ha::BinarySensorConfig {
            common: embassy_ha::EntityCommonConfig { name: Some("Binary Sensor"), ..Default::default() },
            class: embassy_ha::BinarySensorClass::Smoke,
        },
    );

    spawner.must_spawn(binary_sensor_class(sensor));

    let mqtt_params =
        embassy_ha::MqttConnectParams { username: Some(MQTT_USER), password: Some(MQTT_PASSWORD.as_bytes()) };

    embassy_ha::connect_and_run(stack, device, MQTT_BROKER_ADDRESS, mqtt_params).await;
}

#[embassy_executor::task]
async fn binary_sensor_class(mut switch: embassy_ha::BinarySensor<'static>) {
    loop {
        let state = switch.toggle();
        //info!("state = {}", state);
        Timer::after_secs(2).await;
    }
}
