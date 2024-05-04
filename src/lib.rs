use reqwest::ClientBuilder;
use worker::{event, Env, ScheduleContext, ScheduledEvent};

mod calendar;
mod cangrebot;

#[cfg(target_arch = "wasm32")]
use worker::console_error;

#[event(scheduled)]
pub async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    let calendar_api = env
        .secret("GOOGLE_APIKEY")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let calendar_id = env
        .secret("GOOGLE_CALENDAR_ID")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let endpoint = env
        .secret("ENDPOINT")
        .map(|e| e.to_string())
        .expect("Calendar Google API Secret not found");

    let client = ClientBuilder::default()
        .build()
        .expect("Cannot build client reqwest");

    // Get events
    // TODO: filter and map events
    let _calendar = calendar::get(&client, &calendar_id, &calendar_api, "", "").await;

    cangrebot::send(&client, &endpoint).await;
}
