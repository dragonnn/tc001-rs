use alloc::string::{String, ToString};
use core::sync::atomic::Ordering;

use atomic_enum::atomic_enum;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{scan::ScanConfig, sta::StationConfig, ModeConfig, WifiController, WifiDevice, WifiEvent};

static WIFI_STATE: AtomicWiFiState = AtomicWiFiState::new(WiFiState::Disconnected);

#[atomic_enum]
#[derive(PartialEq, Eq)]
pub enum WiFiState {
    Disconnected = 0,
    Scanning,
    Connecting,
    Connected,
    Ip,
}

const SSID0: &str = dotenvy_macro::dotenv!("WIFI_SSID0");
const PASSWORD0: &str = dotenvy_macro::dotenv!("WIFI_PASSWORD0");
const SSID1: &str = dotenvy_macro::dotenv!("WIFI_SSID1");
const PASSWORD1: &str = dotenvy_macro::dotenv!("WIFI_PASSWORD1");

#[embassy_executor::task]
pub async fn wifi_task(mut controller: WifiController<'static>, storage: crate::storage::Storage) {
    info!("start connection task");
    storage.save(&crate::storage::Key::Wifi(SSID0), &PASSWORD0.to_string()).await.expect("failed saving ssid0");
    storage.save(&crate::storage::Key::Wifi(SSID1), &PASSWORD1.to_string()).await.expect("failed saving ssid1");

    let mut wifi_connect_errors: u8 = 0;
    loop {
        if controller.is_connected().unwrap_or_default() {
            // wait until we're no longer connected
            WIFI_STATE.store(WiFiState::Connected, Ordering::Relaxed);
            controller.wait_for_event(WifiEvent::StationDisconnected).await;
            error!("WiFi disconnected!");
            Timer::after(Duration::from_millis(5000)).await
        }

        let mut found_network = false;
        if !matches!(controller.is_started(), Ok(true)) {
            let mut client_config = ModeConfig::Station(StationConfig::default());
            controller.set_config(&client_config).unwrap();
            info!("Starting wifi");
            controller.start_async().await.unwrap();
            info!("Wifi started!");

            info!("Scan");
            WIFI_STATE.store(WiFiState::Scanning, Ordering::Relaxed);
            let scan_config = ScanConfig::default().with_max(5);
            let result = controller.scan_with_config_async(scan_config).await.unwrap();
            info!("Scan complete, found {} networks", result.len());
            for ap in result {
                info!("{:?}", ap);
                if let Ok(password) = storage.read::<String>(&crate::storage::Key::Wifi(&ap.ssid)).await {
                    info!("Found saved network: {}, trying to connect...", ap.ssid);
                    WIFI_STATE.store(WiFiState::Connecting, Ordering::Relaxed);
                    client_config =
                        ModeConfig::Station(StationConfig::default().with_ssid(ap.ssid).with_password(password));
                    controller.set_config(&client_config).unwrap();
                    found_network = true;
                    break;
                }
            }
        }
        info!("About to connect...");
        if found_network {
            match controller.connect_async().await {
                Ok(_) => {
                    WIFI_STATE.store(WiFiState::Connected, Ordering::Relaxed);
                    info!("Wifi connected!")
                }
                Err(e) => {
                    info!("Failed to connect to wifi: {e:?}");
                    wifi_connect_errors += 1;
                    Timer::after(Duration::from_millis(5000)).await
                }
            }
        } else {
            info!("No known networks found during scan.");
            wifi_connect_errors += 1;
            Timer::after(Duration::from_millis(5000)).await
        }

        if wifi_connect_errors >= 10 {
            error!("Too many wifi connection errors, restarting");
            esp_hal::system::software_reset();
        }
    }
}

#[embassy_executor::task]
pub async fn net_task(mut runner: embassy_net::Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

pub async fn wait_for_connection(stack: &embassy_net::Stack<'static>) {
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            WIFI_STATE.store(WiFiState::Ip, Ordering::Relaxed);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

pub fn get_wifi_state() -> WiFiState {
    WIFI_STATE.load(Ordering::Relaxed)
}
