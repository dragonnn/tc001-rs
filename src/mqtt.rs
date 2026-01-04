use alloc::{boxed::Box, string::String};
use core::fmt::Write as _;

use embassy_net::IpEndpoint;
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
    channel::Channel,
    signal::Signal,
};
use embassy_time::{Duration, Instant, Timer, with_timeout};
use esp_hal::gpio::Level;
use mountain_mqtt::{
    client::{Client, ClientError, ConnectionSettings, EventHandlerError},
    data::quality_of_service::QualityOfService,
    mqtt_manager::{ConnectionId, MqttOperations},
    packets::publish::ApplicationMessage,
};
use mountain_mqtt_embassy::mqtt_manager::{self, FromApplicationMessage, MqttEvent, Settings};
use static_cell::StaticCell;

static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, MqttAction, 32> = Channel::new();
static ACTION_CHANNEL: Channel<CriticalSectionRawMutex, MqttEvent<Event>, 32> = Channel::new();

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Button,
}

#[derive(Debug, Clone)]
pub enum MqttAction {
    AnnounceAndSubscribe { connection_id: ConnectionId },
    Action(Action),
}

impl MqttOperations for Action {
    async fn perform<'a, 'b, C>(
        &'b mut self,
        client: &mut C,
        _client_id: &'a str,
        _connection_id: ConnectionId,
        _is_retry: bool,
    ) -> Result<(), ClientError>
    where
        C: Client<'a>,
    {
        info!("Performing MQTT action: {:?}", self);
        match self {
            Action::Button => {
                //let payload = "42";
                //client.publish(PUBLISH_TOPIC, payload.as_bytes(), QualityOfService::Qos1, false).await?;
            }
        }
        Ok(())
    }
}

impl MqttOperations for MqttAction {
    async fn perform<'a, 'b, C>(
        &'b mut self,
        client: &mut C,
        client_id: &'a str,
        current_connection_id: ConnectionId,
        is_retry: bool,
    ) -> Result<(), ClientError>
    where
        C: Client<'a>,
    {
        info!("Performing MQTT action: {:?}", self);
        match self {
            // Specific to one connection, not retried
            Self::AnnounceAndSubscribe { connection_id } => {
                info!("Current connection ID: {:?}, action connection ID: {:?}", current_connection_id, connection_id);
                if connection_id == &current_connection_id && !is_retry {
                    client.publish("ignore", "true".as_bytes(), QualityOfService::Qos1, false).await?;
                    client.subscribe("ignore", QualityOfService::Qos1).await?;
                }
            }
            // Actions are sent on any connection, and retried
            Self::Action(action) => {
                action.perform(client, client_id, current_connection_id, is_retry).await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Led(Level),
}

impl<const P: usize> FromApplicationMessage<P> for Event {
    fn from_application_message(message: &ApplicationMessage<P>) -> Result<Self, EventHandlerError> {
        info!("Received MQTT message on topic: {:?}", message);
        let received = match message.topic_name {
            RECEIVE_TOPIC => {
                let state = parse_led(message.payload)?;
                Ok(Self::Led(state))
            }
            _ => Err(EventHandlerError::UnexpectedApplicationMessageTopic),
        }?;

        Ok(received)
    }
}

fn parse_led(payload: &[u8]) -> Result<Level, EventHandlerError> {
    match payload.iter().next().copied().map(char::from) {
        Some('1') => Ok(Level::Low),
        Some('0') => Ok(Level::High),
        _ => Err(EventHandlerError::InvalidApplicationMessage),
    }
}

const MQTT_BROKER_ADDRESS: &str = dotenvy_macro::dotenv!("MQTT_BROKER_ADDRESS");
const MQTT_BROKER_PORT: &str = dotenvy_macro::dotenv!("MQTT_BROKER_PORT");
const MQTT_USER: &str = dotenvy_macro::dotenv!("MQTT_USER");
const MQTT_PASSWORD: &str = dotenvy_macro::dotenv!("MQTT_PASSWORD");

#[embassy_executor::task]
pub async fn mqtt_task(stack: embassy_net::Stack<'static>) {
    crate::wifi::wait_for_connection(&stack).await;
    Timer::after_secs(5).await;

    info!("Starting MQTT client...");

    let mac = esp_radio::wifi::station_mac();
    info!("Device MAC address: {:02X?}", mac);

    let mut mqtt_prefix = String::with_capacity(13);

    write!(&mut mqtt_prefix, "awtrix_{:02x}{:02x}{:02x}", mac[3], mac[4], mac[5]).ok();

    let mqtt_prefix = mqtt_prefix.into_boxed_str();

    let mqtt_prefix = Box::leak(mqtt_prefix);

    let address: embassy_net::Ipv4Address = MQTT_BROKER_ADDRESS.parse().expect("invalid MQTT_BROKER_ADDRESS");
    let port: u16 = MQTT_BROKER_PORT.parse().expect("invalid MQTT_BROKER_PORT");

    let settings = Settings::new(address, port);
    let connection_settings = ConnectionSettings::authenticated(mqtt_prefix, MQTT_USER, MQTT_PASSWORD.as_bytes());

    EVENT_CHANNEL.send(MqttAction::AnnounceAndSubscribe { connection_id: ConnectionId::new(0) }).await;

    let event_sender = ACTION_CHANNEL.sender();
    let action_receiver = EVENT_CHANNEL.receiver();

    mqtt_manager::run::<MqttAction, Event, _, 16, 4096, 32>(
        stack,
        connection_settings,
        settings,
        event_sender,
        action_receiver,
    )
    .await;
}

#[embassy_executor::task]
pub async fn mqtt_action_task() {}
