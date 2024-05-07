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

> [!IMPORTANT]
> During the development stage the ENDPOINT variable can be modified in the `wrangler.toml` file.
> However the sensitive variables must be in the `.dev.vars` file.

- `GOOGLE_APIKEY`: Google API token for the calendar.
- `GOOGLE_CALENDAR_ID`: Google calendar ID to follow.
- `ENDPOINT`: URL of the API to which the processed data of the calendars will be sent.
- `CHANNEL_ID`: Corresponds to the main channel of announcements.
- `BOT_CHANNEL_ID`: Corresponds to the second channel where the announcements will be made half an hour before.
- `ROLES`: The roles to be mentioned in discord, must be separated by `;`, e.g.: `1923764917246;12987469283746;1923746927346`.

### Local Tests

> [!NOTE]
> You can also run the tests as if it were a normal Rust application.
> With the `load test` command.

To test the worker locally, you only need to set the `ENDPOINT` environment variable in the `wrangler.toml` file. This is necessary only if you want to use a different endpoint than the one already configured.

### Automatic Deployment

This project is configured to deploy automatically using GitHub Actions workflows. For it to work correctly, you must configure the following secrets on GitHub:

- `CLOUDFLARE_ACCOUNT_ID`: ID of your Cloudflare account.
- `CLOUDFLARE_API_TOKEN`: Cloudflare API token.
- `GOOGLE_APIKEY`: Google API token for the calendar.
- `GOOGLE_CALENDAR_ID`: Google calendar ID to follow.
- `ENDPOINT`: URL of the API to which the processed data of the calendars will be sent.
- `CHANNEL_ID`: Corresponds to the main channel of announcements.
- `BOT_CHANNEL_ID`: Corresponds to the second channel where the announcements will be made half an hour before.
- `ROLES`: The roles to be mentioned in discord, must be separated by `;`, e.g.: `1923764917246;12987469283746;1923746927346`.
