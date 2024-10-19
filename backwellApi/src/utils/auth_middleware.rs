// src/utils/auth_middleware.rs

use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::error::ErrorUnauthorized;
use futures::future::{ok, Ready, LocalBoxFuture};
use futures::FutureExt;
use crate::utils::auth::verify_jwt;
use serde::{Deserialize, Serialize};

pub struct AuthorizedUser {
    pub user_id: String,
}

pub struct AuthMiddleware;

pub fn auth_middleware() -> AuthMiddleware {
    AuthMiddleware
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: actix_service::Service<
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_service::Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: actix_service::Service<
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();

        let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = auth_header.trim_start_matches("Bearer ").trim();

                match verify_jwt(token) {
                    Ok(claims) => {
                        // Adjuntar el usuario autorizado al request
                        req.extensions_mut().insert(AuthorizedUser {
                            user_id: claims.sub,
                        });
                        let fut = self.service.call(req);
                        return async move { fut.await }.boxed_local();
                    }
                    Err(_) => {
                        let res = HttpResponse::Unauthorized().body("Token inválido o expirado");
                        return async move { Err(ErrorUnauthorized(res)) }.boxed_local();
                    }
                }
            }
        }

        let res = HttpResponse::Unauthorized().body("No autorizado");
        async move { Err(ErrorUnauthorized(res)) }.boxed_local()
    }
}
