use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::env;
use std::rc::Rc;

use crate::dto::TokenClaims;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = futures_util::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(JwtMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract and validate JWT
            let auth_header = req.headers().get("Authorization");

            let token = match auth_header {
                Some(header) => {
                    let header_str = header.to_str().map_err(|_| ErrorUnauthorized("Invalid header"))?;
                    if !header_str.starts_with("Bearer ") {
                        return Err(ErrorUnauthorized("Invalid token format"));
                    }
                    &header_str[7..]
                }
                None => {
                    return Err(ErrorUnauthorized("Missing authorization header"));
                }
            };

            let jwt_secret = env::var("JWT_SECRET")
                .map_err(|_| ErrorUnauthorized("JWT secret not configured"))?;

            let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
            let validation = Validation::new(Algorithm::HS256);

            match decode::<TokenClaims>(token, &decoding_key, &validation) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    service.call(req).await
                }
                Err(_) => Err(ErrorUnauthorized("Invalid token")),
            }
        })
    }
}

