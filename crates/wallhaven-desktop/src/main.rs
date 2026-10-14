mod app;
mod components;
mod features;
mod strings;
mod styles;

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let content_security_policy = concat!(
        "default-src 'self'; ",
        "script-src 'self' 'unsafe-inline' 'unsafe-eval'; ",
        "style-src 'self' 'unsafe-inline'; ",
        "img-src 'self' data: https://th.wallhaven.cc https://w.wallhaven.cc https://wallhaven.cc; ",
        "connect-src 'self' https://wallhaven.cc ws: wss: http://localhost:*; ",
        "frame-ancestors 'none'; ",
        "base-uri 'self';"
    );
    let config = dioxus::desktop::Config::new()
        .with_custom_head(format!(
            r#"<meta http-equiv="Content-Security-Policy" content="{content_security_policy}">
<style>{}</style>"#,
            styles::CSS
        ))
        .with_window(dioxus::desktop::WindowBuilder::new().with_title(strings::APP_TITLE));

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app::App);
}
