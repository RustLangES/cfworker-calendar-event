<div align="right">
<a href="./README.md">ES</a>
</div>
# Calendar Events Worker

This is a Cloudflare worker that runs daily at 22:00 UTC-0. Its function is to get the upcoming events from our google calendar and then sends them in Markdown format to an endpoint defined in the `ENDPOINT` environment variable. In our case, this endpoint corresponds to a Discord bot that notifies interested people with the role of `@announcements` within our discord server.

## Operation

1. The worker is automatically activated at 22:00 UTC-0 thanks to a schedule trigger in Cloudflare.
2. A call is made to the Google Calendar API to get the event details.
3. A POST request is sent to the API defined in the `ENDPOINT` environment variable with the challenge data in Markdown format.
4. Roles and persons are notified about the upcoming event.

## Configuration

### Requirements

To build and deploy this project, you will need the following:

- [Rust](https://rust-lang.org)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Environment Variables

- `ENDPOINT`: URL of the endpoint to which the daily challenge data will be sent.

### Local Tests

> [!NOTE]
> You can also run the tests as if it were a normal Rust application.
> With the `load test` command.

To test the worker locally, you only need to set the `ENDPOINT` environment variable in the `wrangler.toml` file. This is necessary only if you want to use a different endpoint than the one already configured.

### Automatic Deployment

This project is configured to deploy automatically using GitHub Actions workflows. For it to work correctly, you must configure the following secrets on GitHub:

- `CLOUDFLARE_ACCOUNT_ID`: ID of your Cloudflare account.
- CLOUDFLARE_API_TOKEN`: Cloudflare API token.
- GOOGLE_APIKEY`: Google API token for the calendar.
- GOOGLE_CALENDAR_ID`: Google calendar ID to follow.
- `ENDPOINT`: API URL to which the daily challenge data will be sent.
