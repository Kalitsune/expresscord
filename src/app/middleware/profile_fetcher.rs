use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// see https://actix.rs/docs/middleware

pub struct ProfileFetcher;

impl <S, B> Transform<S, ServiceRequest> for ProfileFetcher
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ProfileFetcherMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProfileFetcherMiddleware { service }))
    }
}

pub struct ProfileFetcherMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProfileFetcherMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let discord_id = req.match_info().get("discord_id");

        if discord_id.is_some() {
            // request user profile
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            Ok(fut.await?)
        })
    }
}

