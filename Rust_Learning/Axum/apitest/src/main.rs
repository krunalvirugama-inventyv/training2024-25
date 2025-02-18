use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::interval;

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        handle_socket(socket).await;
    })
}

async fn handle_socket(mut socket: WebSocket) {
    let mut latitude = None;
    let mut longitude = None;

    // Step 1: Receive User's Latitude and Longitude
    if let Some(Ok(Message::Text(text))) = socket.recv().await {
        if let Ok(coords) = serde_json::from_str::<Value>(&text) {
            latitude = coords["latitude"].as_f64();
            longitude = coords["longitude"].as_f64();
        }
    }

    if latitude.is_none() || longitude.is_none() {
        let _ = socket
            .send(Message::Text("Invalid latitude/longitude".into()))
            .await;
        return;
    }

    let user_lat = latitude.unwrap();
    let user_lon = longitude.unwrap();

    let client = Client::new();
    let mut interval = interval(Duration::from_secs(10)); // Every 5 minutes

    // Step 2: Fetch Data Every 5 Minutes & Compare
    loop {
        interval.tick().await;

        match fetch_gdacs_data(&client).await {
            Ok(events) => {
                let json_data = serde_json::from_str(&events);

                if json_data.is_err() {
                    let _ = socket
                        .send(Message::Text("Failed to parse JSON data".into()))
                        .await;
                    continue;
                }

                // Find matching event based on lat/lon
                if let Some(matching_event) =
                    find_matching_event(user_lat, user_lon, &json_data.unwrap())
                {
                    let _ = socket
                        .send(Message::Text(
                            format!(
                                "Matched Event: {}",
                                serde_json::to_string(&matching_event).unwrap()
                            )
                            .into(),
                        ))
                        .await;
                } else {
                    let _ = socket
                        .send(Message::Text("No matching event found".into()))
                        .await;
                }
            }
            Err(e) => {
                let _ = socket
                    .send(Message::Text(format!("Failed to fetch data: {}", e).into()))
                    .await;
            }
        }
    }
}

async fn fetch_gdacs_data(client: &Client) -> Result<String, reqwest::Error> {
    let url = "https://www.gdacs.org/gdacsapi/api/events/geteventlist/EVENTS4APP";
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}

// Step 3: Compare Coordinates (User Lat/Lon vs API Event Lat/Lon)
fn find_matching_event(user_lat: f64, user_lon: f64, events: &Value) -> Option<Value> {
    if let Some(features) = events["features"].as_array() {
        for feature in features {
            if let Some(coords) = feature["geometry"]["coordinates"].as_array() {
                if let (Some(event_lat), Some(event_lon)) = (
                    coords.get(0).and_then(|v| v.as_f64()),
                    coords.get(1).and_then(|v| v.as_f64()),
                ) {
                    if (event_lat - user_lat).abs() < f64::EPSILON
                        && (event_lon - user_lon).abs() < f64::EPSILON
                    {
                        return Some(feature.clone());
                    }
                }
            }
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on ws://0.0.0.0:3000/ws");
    axum::serve(listener, app).await.unwrap();
}
