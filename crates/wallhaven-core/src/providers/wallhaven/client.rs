use crate::domain::{SearchPage, SearchRequest, WallpaperDetail};
use crate::error::CoreError;
use crate::providers::wallhaven::{dto, mapper, normalize};
use reqwest::redirect::Policy;
use reqwest::Url;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct RateLimiterState {
    timestamps: Vec<Instant>,
}

#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
    capacity: usize,
    period: Duration,
}

impl RateLimiter {
    pub fn new(capacity: usize, period: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                timestamps: Vec::with_capacity(capacity),
            })),
            capacity,
            period,
        }
    }

    pub async fn acquire(&self) -> Result<(), CoreError> {
        loop {
            let wait = {
                let mut state = self
                    .state
                    .lock()
                    .map_err(|_| CoreError::Storage("rate limiter lock poisoned".to_string()))?;
                let now = Instant::now();
                state
                    .timestamps
                    .retain(|timestamp| now.duration_since(*timestamp) < self.period);
                if state.timestamps.len() < self.capacity {
                    state.timestamps.push(now);
                    return Ok(());
                }
                let oldest = state.timestamps.first().copied().unwrap_or(now);
                self.period.saturating_sub(now.duration_since(oldest))
            };

            if wait.is_zero() {
                continue;
            }
            tokio::time::sleep(wait).await;
        }
    }
}

#[derive(Clone)]
pub struct WallhavenClient {
    http: reqwest::Client,
    base_url: Url,
    api_key: Option<String>,
    limiter: RateLimiter,
}

impl WallhavenClient {
    pub fn new(api_key: Option<String>) -> Result<Self, CoreError> {
        Self::with_base_url(api_key, "https://wallhaven.cc/api/v1/")
    }

    pub fn with_base_url(api_key: Option<String>, base_url: &str) -> Result<Self, CoreError> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("wallhaven-desktop/0.1.0")
            .redirect(redirect_policy())
            .build()
            .map_err(|error| CoreError::Network(error.to_string()))?;
        let base_url =
            Url::parse(base_url).map_err(|error| CoreError::Provider(error.to_string()))?;
        Ok(Self {
            http,
            base_url,
            api_key,
            limiter: RateLimiter::new(45, Duration::from_secs(60)),
        })
    }

    pub fn from_env() -> Result<Self, CoreError> {
        Self::new(std::env::var("WALLHAVEN_API_KEY").ok())
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn http(&self) -> reqwest::Client {
        self.http.clone()
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<SearchPage, CoreError> {
        if let Some(color) = request.filters.colors.as_ref() {
            if !color.trim().is_empty() && !normalize::is_valid_color(color) {
                return Err(CoreError::InvalidInput(format!(
                    "unsupported color: {color}"
                )));
            }
        }
        self.limiter.acquire().await?;
        let params = normalize::normalize_search(request, self.api_key.is_some());
        let url = self
            .base_url
            .join("search")
            .map_err(|error| CoreError::Provider(error.to_string()))?;
        let mut builder = self.http.get(url).query(&params);
        if let Some(key) = self.api_key.as_ref() {
            builder = builder.header("X-API-Key", key.as_str());
        }
        let response = builder.send().await.map_err(|error| {
            if error.is_timeout() {
                CoreError::Network(format!("request timed out: {error}"))
            } else {
                CoreError::Network(error.to_string())
            }
        })?;
        self.map_response(response).await
    }

    pub async fn wallpaper(&self, id: &str) -> Result<WallpaperDetail, CoreError> {
        validate_id(id)?;
        self.limiter.acquire().await?;
        let url = self
            .base_url
            .join(&format!("w/{id}"))
            .map_err(|error| CoreError::Provider(error.to_string()))?;
        let mut builder = self.http.get(url);
        if let Some(key) = self.api_key.as_ref() {
            builder = builder.header("X-API-Key", key.as_str());
        }
        let response = builder.send().await.map_err(|error| {
            if error.is_timeout() {
                CoreError::Network(format!("request timed out: {error}"))
            } else {
                CoreError::Network(error.to_string())
            }
        })?;
        self.map_detail_response(response).await
    }

    pub async fn detail(&self, id: &str) -> Result<WallpaperDetail, CoreError> {
        self.wallpaper(id).await
    }

    async fn map_response(&self, response: reqwest::Response) -> Result<SearchPage, CoreError> {
        let status = response.status();
        if status.as_u16() == 401 {
            return Err(CoreError::Unauthorized);
        }
        if status.as_u16() == 429 {
            return Err(CoreError::RateLimited);
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CoreError::Provider(format!(
                "search request failed with HTTP {}: {}",
                status.as_u16(),
                body.chars().take(500).collect::<String>()
            )));
        }
        let dto: dto::SearchResponseDto = response
            .json()
            .await
            .map_err(|error| CoreError::Provider(format!("invalid search JSON: {error}")))?;
        mapper::map_search_response(dto)
    }

    async fn map_detail_response(
        &self,
        response: reqwest::Response,
    ) -> Result<WallpaperDetail, CoreError> {
        let status = response.status();
        if status.as_u16() == 401 {
            return Err(CoreError::Unauthorized);
        }
        if status.as_u16() == 429 {
            return Err(CoreError::RateLimited);
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CoreError::Provider(format!(
                "detail request failed with HTTP {}: {}",
                status.as_u16(),
                body.chars().take(500).collect::<String>()
            )));
        }
        let dto: dto::DetailResponseDto = response
            .json()
            .await
            .map_err(|error| CoreError::Provider(format!("invalid detail JSON: {error}")))?;
        mapper::map_detail_response(dto)
    }
}

fn validate_id(id: &str) -> Result<(), CoreError> {
    if id.is_empty() || !id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        return Err(CoreError::InvalidInput(format!(
            "invalid wallpaper id: {id}"
        )));
    }
    Ok(())
}

fn redirect_policy() -> Policy {
    Policy::custom(|attempt| {
        if attempt.previous().len() >= 5 {
            return attempt.error("too many redirects");
        }
        let url = attempt.url();
        let allowed_host = matches!(
            url.host_str(),
            Some("wallhaven.cc") | Some("w.wallhaven.cc") | Some("th.wallhaven.cc")
        );
        if url.scheme() == "https" && allowed_host {
            attempt.follow()
        } else {
            attempt.stop()
        }
    })
}
