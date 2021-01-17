use crate::app::App;
use std::future::Future;
use std::net::SocketAddr;

pub struct Server {
    app: App,
}

impl Server {
    pub fn new(app: App) -> Self {
        Server { app }
    }

    pub fn run<T: Into<SocketAddr> + 'static>(&self, addr: T) -> impl Future<Output = ()> {
        warp::serve(router::routes(&self.app)).run(addr)
    }
}

mod router {
    use crate::app::App;
    use twitter2_api::{error::ServiceError, infra::jwt_handler};
    use warp::{
        filters::BoxedFilter,
        http::status::StatusCode,
        reply::{self, Json, Reply, WithStatus},
        Filter,
    };

    pub fn routes(app: &App) -> BoxedFilter<(impl Reply,)> {
        healthcheck().or(api(app)).boxed()
    }

    fn healthcheck() -> BoxedFilter<(String,)> {
        warp::path("healthcheck").map(|| "ok".into()).boxed()
    }

    fn api(app: &App) -> BoxedFilter<(impl Reply,)> {
        let api = warp::path("api");
        let v1 = warp::path("v1");

        api.and(v1.and(users(app).or(posts(app))))
            .with(security::cors())
            .boxed()
    }

    fn users(app: &App) -> BoxedFilter<(impl Reply,)> {
        let users = warp::path("users");

        let app_c = app.clone();
        let upsert = warp::put()
            .and(security::authorization())
            .map(move |verification_result| {
                claims_handle_helper(verification_result, |claims| {
                    let res = app_c.services.user_service.upsert(&claims.sub);

                    match res {
                        Ok(num) if num == 0 => {
                            reply::with_status(reply::json(&"".to_string()), StatusCode::NO_CONTENT)
                        }
                        Ok(_) => reply::with_status(
                            reply::json(&StatusCode::CREATED.to_string()),
                            StatusCode::CREATED,
                        ),
                        Err(_) => reply::with_status(
                            reply::json(&StatusCode::INTERNAL_SERVER_ERROR.to_string()),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ),
                    }
                })
            });

        users.and(upsert).boxed()
    }

    fn posts(app: &App) -> BoxedFilter<(impl Reply,)> {
        let posts = warp::path("posts");

        let app_c = app.clone();
        let own_posts =
            warp::path("own")
                .and(security::authorization())
                .map(move |verification_result| {
                    claims_handle_helper(verification_result, |claims| {
                        let res = app_c
                            .services
                            .post_service
                            .pagenate_posts_of_user_by_sub_id(&claims.sub, 20, 0);

                        match res {
                            Ok(own_posts) => {
                                reply::with_status(reply::json(&own_posts), StatusCode::OK)
                            }
                            Err(ServiceError::NotFound) => reply::with_status(
                                reply::json(&StatusCode::NOT_FOUND.to_string()),
                                StatusCode::NOT_FOUND,
                            ),
                            Err(ServiceError::DbQueryFailed(_)) => reply::with_status(
                                reply::json(&StatusCode::INTERNAL_SERVER_ERROR.to_string()),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ),
                        }
                    })
                });

        posts.and(own_posts).boxed()
    }

    /// Inspects `verification_result` to early reply when it's JwtError, or else delegates handling the claims to `handler`.
    fn claims_handle_helper<F>(
        verification_result: Result<jwt_handler::Claims, jwt_handler::JwtError>,
        handler: F,
    ) -> WithStatus<Json>
    where
        F: Fn(jwt_handler::Claims) -> WithStatus<Json>,
    {
        let invalid_token_message = "Invalid token".to_string();

        let claims = match verification_result {
            Ok(claims) => claims,
            Err(jwt_error) => match jwt_error {
                jwt_handler::JwtError::FetchJwks(_) => {
                    return reply::with_status(
                        reply::json(&StatusCode::INTERNAL_SERVER_ERROR.to_string()),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    );
                }
                jwt_handler::JwtError::DecodingFailed(_)
                | jwt_handler::JwtError::JwkNotFound
                | jwt_handler::JwtError::KidMissing => {
                    return reply::with_status(
                        reply::json(&invalid_token_message),
                        StatusCode::UNAUTHORIZED,
                    );
                }
            },
        };

        handler(claims)
    }

    mod security {
        use std::convert::Infallible;
        use std::env;
        use twitter2_api::infra::jwt_handler;
        use warp::{filters::BoxedFilter, Filter};

        pub fn authorization() -> BoxedFilter<(Result<jwt_handler::Claims, jwt_handler::JwtError>,)>
        {
            warp::header::<String>("authorization")
                .and_then(verify_token)
                .boxed()
        }

        async fn verify_token(
            autorization_token: String,
        ) -> Result<Result<jwt_handler::Claims, jwt_handler::JwtError>, Infallible> {
            let token = autorization_token
                .trim()
                .strip_prefix("Bearer ")
                .unwrap_or("");

            let verification_result = jwt_handler::verify(token).await;

            Ok(verification_result)
        }

        pub fn cors() -> warp::cors::Builder {
            let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set");
            warp::cors()
                .allow_origin(allowed_origin.as_str())
                .allow_headers(vec!["authorization"])
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        }
    }
}
