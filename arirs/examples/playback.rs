use arirs::{Asterisk, Event, PlayMediaBaseParams, PlayMediaParams};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).with(LevelFilter::TRACE).init();

    let (request_client, mut event_listener) = Asterisk::connect("http://localhost:8088/", "asterisk", "asterisk", "ari").await?;

    while let Some(event) = event_listener.recv().await {
        if let Event::StasisStart(event) = event {
            request_client
                .channel_play_media(
                    event.channel().id(),
                    PlayMediaParams {
                        playback_id: None,
                        base_params: PlayMediaBaseParams {
                            media: &["sound:hello"],
                            lang: Some("en"),
                            offset_ms: None,
                            skip_ms: None,
                        },
                    },
                )
                .await?;
        }
    }

    Ok(())
}
