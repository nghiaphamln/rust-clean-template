use std::rc::Rc;
use std::time::Duration;

use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Clone)]
pub struct BruteForceConfig {
    pub max_login_attempts: i32,
    pub lockout_duration_minutes: i64,
    pub rate_limit_requests_per_minute: u32,
}

impl BruteForceConfig {
    pub fn from_env() -> Self {
        let max_login_attempts = std::env::var("BRUTE_FORCE_MAX_LOGIN_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(5);

        let lockout_duration_minutes = std::env::var("BRUTE_FORCE_LOCKOUT_DURATION_MINUTES")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(30);

        let rate_limit_requests_per_minute =
            std::env::var("BRUTE_FORCE_RATE_LIMIT_REQUESTS_PER_MINUTE")
                .ok()
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(10);

        Self {
            max_login_attempts,
            lockout_duration_minutes,
            rate_limit_requests_per_minute,
        }
    }
}

impl Default for BruteForceConfig {
    fn default() -> Self {
        Self {
            max_login_attempts: 5,
            lockout_duration_minutes: 30,
            rate_limit_requests_per_minute: 10,
        }
    }
}

pub struct BruteForceMiddleware<S> {
    service: Rc<S>,
    config: BruteForceConfig,
    rate_limits: Rc<Mutex<HashMap<String, (u32, Instant)>>>,
}

impl<S> Service<ServiceRequest> for BruteForceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let config = self.config.clone();
        let rate_limits = self.rate_limits.clone();

        Box::pin(async move {
            let path = req.path();

            if !path.contains("/auth/login") {
                return service.call(req).await;
            }

            let ip_address = req
                .connection_info()
                .realip_remote_addr()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            {
                let mut rate_map = rate_limits.lock().unwrap();
                let (count, start) = rate_map
                    .entry(ip_address.clone())
                    .or_insert((0, Instant::now()));

                if start.elapsed() > Duration::from_secs(60) {
                    *count = 0;
                    *start = Instant::now();
                }

                *count += 1;

                if *count > config.rate_limit_requests_per_minute {
                    let res = ServiceResponse::new(
                        req.request().clone(),
                        HttpResponse::TooManyRequests().json(serde_json::json!({
                            "error": "TooManyRequests",
                            "message": "Rate limit exceeded. Please try again later.",
                            "status_code": 429
                        })),
                    );
                    return Ok(res.map_into_boxed_body());
                }
            }

            let response = service.call(req).await?;

            Ok(response)
        })
    }
}

#[derive(Clone, Default)]
pub struct BruteForceMiddlewareFactory {
    config: BruteForceConfig,
}

impl BruteForceMiddlewareFactory {
    pub fn new(config: BruteForceConfig) -> Self {
        Self { config }
    }
}

impl<S> Transform<S, ServiceRequest> for BruteForceMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = BruteForceMiddleware<S>;
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let config = self.config.clone();
        Box::pin(async move {
            Ok(BruteForceMiddleware {
                service: Rc::new(service),
                config,
                rate_limits: Rc::new(Mutex::new(HashMap::new())),
            })
        })
    }
}
