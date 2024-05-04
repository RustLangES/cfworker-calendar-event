use worker::{console_error, event, Env, ScheduleContext, ScheduledEvent};

mod cangrebot;

#[event(scheduled)]
pub async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    let Ok(calendar_api) = env.secret("GOOGLE_APIKEY").map(|e| e.to_string()) else {
        console_error!("Calendar Google API Secret not found");
        return;
    };

    let Ok(calendar_id) = env.secret("GOOGLE_CALENDAR_ID").map(|e| e.to_string()) else {
        console_error!("Calendar Google API Secret not found");
        return;
    };

    let Ok(endpoint) = env.secret("ENDPOINT").map(|e| e.to_string()) else {
        console_error!("Calendar Google API Secret not found");
        return;
    };
}
