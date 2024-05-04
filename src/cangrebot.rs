use reqwest::Client;
use serde_json::json;
use worker::{console_debug, console_warn};

pub async fn send(client: &Client, endpoint: &str) {
    let req = json!({ });

    let res = client
        .post(endpoint)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&req).unwrap())
        .send()
        .await
        .inspect_err(|e| console_warn!("Reqwest Error: {e:?}"))
        .unwrap()
        .text()
        .await
        .inspect_err(|e| console_warn!("Json Error: {e:?}"))
        .unwrap();

    console_debug!("Result: {res:?}");
}
