// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::time::Duration;

static CONNECTED_TO_EXALISE: AtomicBool = AtomicBool::new(false);
pub struct MqttClient(Mutex<AsyncClient>);

#[derive(Deserialize, Serialize, Debug)]
pub struct ExaliseSettings {
    pub mqtt_settings: ExaliseMqttSettings,
    pub http_settings: ExaliseHttpSettings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExaliseHttpSettings {
    pub http_key: String,
    pub http_secret: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExaliseMqttSettings {
    pub mqtt_key: String,
    pub mqtt_secret: String,
    pub device_key: String,
}

#[tokio::main]
async fn main() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let default_settings_string = r#"
  {
      "mqtt_settings": {
          "mqtt_key": "mqtt_key",
          "mqtt_secret": "mqtt_secret",
          "device_key": "device_key"
      },
      "http_settings": {
          "http_key": "http_key",
          "http_secret": "http_secret"
      }
  }"#;

    // Parse the string of data into serde_json::Value.
    let default_settings_json: ExaliseSettings =
        serde_json::from_str(default_settings_string).unwrap();

    let exalise_settings: ExaliseSettings;

    let file_open = std::fs::read_to_string(
        &"C:/Users/Gebruiker/Documents/entrance-desktop-settings/settings.exalise.json",
    );

    match file_open {
        Ok(v) => {
            let res = serde_json::from_str::<ExaliseSettings>(&v);

            match res {
                Ok(r) => {
                    exalise_settings = r;
                }
                Err(_err) => {
                    exalise_settings = default_settings_json;
                    // save file with new settings
                    // Save the JSON structure into the other file.
                    std::fs::write(
                    "C:/Users/Gebruiker/Documents/entrance-desktop-settings/settings.exalise.json",
                    serde_json::to_string_pretty(&exalise_settings).unwrap(),
                )
                .unwrap();
                }
            }
        }
        Err(_e) => {
            exalise_settings = default_settings_json;
            // save file with new settings
            // Save the JSON structure into the other file.
            std::fs::write(
                "C:/Users/Gebruiker/Documents/entrance-desktop-settings/settings.exalise.json",
                serde_json::to_string_pretty(&exalise_settings).unwrap(),
            )
            .unwrap();
        }
    }

    let device_key = exalise_settings.mqtt_settings.device_key.clone();
    let device_key_lastwill = device_key.clone();
    let device_key_clone = device_key_lastwill.clone();

    let mqtt_key = exalise_settings.mqtt_settings.mqtt_key.clone();
    let mqtt_secret = exalise_settings.mqtt_settings.mqtt_secret.clone();

    //connect to mqtt broker
    let broker_url = "mqtt.exalise.com";
    let broker_port = 1883;

    let mut mqttoptions = MqttOptions::new(device_key, broker_url, broker_port);

    let will = LastWill::new(
        format!("exalise/lastwill/{}", device_key_lastwill),
        "disconnected",
        QoS::AtLeastOnce,
        false,
    );

    mqttoptions.set_last_will(will);
    mqttoptions.set_credentials(mqtt_key, mqtt_secret);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_clean_session(false);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let client_clone = client.clone();

    client
        .subscribe(format!("exalise/messages/{}/#", device_key_clone), QoS::AtMostOnce)
        .await
        .unwrap();

    tauri::Builder::default()
        .manage(MqttClient(Mutex::new(client_clone)))
        .manage(exalise_settings)
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                // receive incoming notifications
                let mut connected = false;
                loop {
                    let notification = eventloop.poll().await;

                    match notification {
                        Ok(s) => {
                            println!("{:?}", s);
                            if !connected {
                                CONNECTED_TO_EXALISE.store(true, Ordering::Relaxed);

                                main_window.emit("exalise-connection", "connected").unwrap();

                                client
                                    .publish(
                                        format!("exalise/lastwill/{}", device_key_clone),
                                        QoS::AtLeastOnce,
                                        false,
                                        "connected".as_bytes().to_vec(),
                                    )
                                    .await
                                    .unwrap();
                                connected = true;
                            }

                            if let Event::Incoming(Packet::Publish(p)) = s {
                                let payload = std::str::from_utf8(p.payload.as_ref())
                                    .expect("Error converting payload to string");

                                let topic = p.topic;
                                let topic_split = topic.split("/");

                                let vec_topic: Vec<&str> = topic_split.collect();

                                if vec_topic[1] == "messages" {
                                    if vec_topic.len() == 3 {
                                        let s: String =
                                            format!("notification---{}", vec_topic[2]).to_owned();
                                        let s_slice: &str = &s[..];

                                        main_window.emit(s_slice, format!("{}", payload)).unwrap();
                                    } else {
                                        // remove key from topic
                                        let datapoint_key_split = vec_topic[3].split("_");

                                        let datapoint_key: Vec<&str> =
                                            datapoint_key_split.collect();

                                        let s: String = format!(
                                            "{}",
                                            datapoint_key[0]
                                        )
                                        .to_owned();
                                        let s_slice: &str = &s[..];

                                        main_window.emit(s_slice, format!("{}", payload)).unwrap();
                                    }
                                } else if vec_topic[1] == "lastwill" {
                                    let s: String =
                                        format!("notification---{}", vec_topic[2]).to_owned();
                                    let s_slice: &str = &s[..];

                                    main_window.emit(s_slice, format!("{}", payload)).unwrap();
                                }
                            }
                        }
                        Err(_e) => {
                            if connected {
                                CONNECTED_TO_EXALISE.store(false, Ordering::Relaxed);
                                main_window
                                    .emit("exalise-connection", "disconnected")
                                    .unwrap();
                                connected = false;
                            }
                        }
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![send_message, send_message_to_own_topic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
async fn send_message_to_own_topic(
    datapoint: String,
    value: String,
    mqtt_client: State<'_, MqttClient>,
    exalise_settings: State<'_, ExaliseSettings>
) -> Result<bool, bool> {
    let exalise_settings = ExaliseSettings {
        mqtt_settings: ExaliseMqttSettings {
            mqtt_key: exalise_settings.mqtt_settings.mqtt_key.clone(),
            mqtt_secret: exalise_settings.mqtt_settings.mqtt_secret.clone(),
            device_key: exalise_settings.mqtt_settings.device_key.clone(),
        },
        http_settings: ExaliseHttpSettings {
            http_key: exalise_settings.http_settings.http_key.clone(),
            http_secret: exalise_settings.http_settings.http_secret.clone(),
        }
    };

    let client = mqtt_client.0.lock().await;

    match client
        .publish(
            format!("exalise/messages/{}/{}", exalise_settings.mqtt_settings.device_key, datapoint),
            QoS::AtLeastOnce,
            false,
            value.as_bytes().to_vec(),
        )
        .await
    {
        Ok(_ok) => {
            return Ok(true);
        }
        Err(_err) => {
            return Err(false);
        }
    }
}

#[tauri::command(async)]
async fn send_message(
    device_key: String,
    datapoint: String,
    value: String,
    mqtt_client: State<'_, MqttClient>,
) -> Result<bool, bool> {
    let client = mqtt_client.0.lock().await;

    match client
        .publish(
            format!("exalise/messages/{}/{}", device_key, datapoint),
            QoS::AtLeastOnce,
            false,
            value.as_bytes().to_vec(),
        )
        .await
    {
        Ok(_ok) => {
            return Ok(true);
        }
        Err(_err) => {
            return Err(false);
        }
    }
}
