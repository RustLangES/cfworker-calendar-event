<div align="right">
<a href="./README_EN.md">EN</a>
</div>

# Calendar Events Worker

Este es un worker de Cloudflare que se ejecuta diariamente a las 22:00 UTC-0. Su función es obtener los eventos proximos de nuestro calendario de google y luego los envía en formato Markdown a un endpoint definido en la variable de entorno `ENDPOINT`. En nuestro caso, este punto final corresponde a un bot de Discord que notifica a las personaes interesadas con el rol de `@anuncios` dentro de nuestro servidor de discord.

## Funcionamiento

1. El worker se activa automáticamente a las 22:00 UTC-0 gracias a un disparador de programación (schedule trigger) en Cloudflare.
2. Se realiza una llamada a la API de Google Calendar para obtener los detalles de los eventos.
3. Se envía una solicitud POST a la API definido en la variable de entorno `ENDPOINT` con los datos del reto en formato Markdown.
4. Se notifica a los roles y personas acerca del evento proximo.

## Configuración

### Requisitos

Para construir y desplegar este proyecto, necesitarás lo siguiente:

- [Rust](https://rust-lang.org)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Variables de Entorno

> [!IMPORTANT]
> Durante la etapa de desarrollo la variable de ENDPOINT puedes modificarla en el archivo `wrangler.toml`
> Sin embargo las variables sensibles deben ir en el archivo `.dev.vars`

- `GOOGLE_APIKEY`: Token de API de Google para el calendario.
- `GOOGLE_CALENDAR_ID`: ID del calendario de Google a seguir.
- `ENDPOINT`: URL de la API a la que se enviarán los datos procesados de los calendarios.
- `CHANNEL_ID`: Corresponde al canal principal de anuncios.
- `BOT_CHANNEL_ID`: Corresponde al segundo canal en donde se haran los anuncios de media hora antes.
- `ROLES`: Los roles a los que se hara mencion en discord, deben estar separados por `;`, ejemplo: `1923764917246;12987469283746;1923746927346`
- `BOT_APIKEY`: Solo es necesario para nuestro caso que tenemos limitado nuestro endpoint para usuarios permitidos

### Pruebas Locales

> [!NOTE]
> Tambien puedes correr los tests como si fuera una aplicacion normal de Rust
> Con el comando de `cargo test`

Para probar el worker localmente, solo necesitas configurar la variable de entorno `ENDPOINT` en el archivo `wrangler.toml`. Esto es necesario solo si quieres usar un punto final diferente al que ya está configurado.

### Despliegue Automático

Este proyecto está configurado para desplegar automáticamente utilizando los flujos de trabajo de GitHub Actions. Para que funcione correctamente, debes configurar los siguientes secretos en GitHub:

- `CLOUDFLARE_ACCOUNT_ID`: ID de tu cuenta de Cloudflare.
- `CLOUDFLARE_API_TOKEN`: Token de API de Cloudflare.
- `GOOGLE_APIKEY`: Token de API de Google para el calendario.
- `GOOGLE_CALENDAR_ID`: ID del calendario de Google a seguir.
- `ENDPOINT`: URL de la API a la que se enviarán los datos procesados de los calendarios.
- `CHANNEL_ID`: Corresponde al canal principal de anuncios.
- `BOT_CHANNEL_ID`: Corresponde al segundo canal en donde se haran los anuncios de media hora antes.
- `ROLES`: Los roles a los que se hara mencion en discord, deben estar separados por `;`, ejemplo: `1923764917246;12987469283746;1923746927346`
- `BOT_APIKEY`: Solo es necesario para nuestro caso que tenemos limitado nuestro endpoint para usuarios permitidos
