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
    use std::collections::HashMap;
    use twitter2_api::infra::jwt_handler;
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

        api.and(v1.and(posts(app.clone())))
            .with(security::cors())
            .boxed()
    }

    fn posts(app: App) -> BoxedFilter<(impl Reply,)> {
        let posts = warp::path("posts");
        let own_posts =
            warp::path("own")
                .and(security::authorization())
                .map(move |verification_result| {
                    claims_handle_helper(verification_result, |claims| {
                        app.services
                            .post_service
                            .pagenate_posts_of_user_by_sub_id(&claims.sub, 20, 0)
                            .map_or_else(
                                |err| {
                                    let mut data = HashMap::new();
                                    data.insert("error".to_string(), format!("{}", err));
                                    warp::reply::with_status(
                                        warp::reply::json(&data),
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                    )
                                },
                                |own_posts| {
                                    warp::reply::with_status(
                                        warp::reply::json(&own_posts),
                                        StatusCode::OK,
                                    )
                                },
                            )
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
        let internal_server_error_message = "Internal Server Error".to_string();

        let claims = match verification_result {
            Ok(claims) => claims,
            Err(jwt_error) => match jwt_error {
                jwt_handler::JwtError::FetchJwks(_) => {
                    return reply::with_status(
                        reply::json(&internal_server_error_message),
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
