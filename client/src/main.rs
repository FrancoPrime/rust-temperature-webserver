use crate::mqtt_client::MqttClient;

mod mqtt_client;

fn main() {
    let client = MqttClient::new("src/config.txt");
    client.run();
}
