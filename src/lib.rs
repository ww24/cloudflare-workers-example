use serde_json::json;
use worker::*;

mod jwt;
mod polyfill;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();
    polyfill::set_performance();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/region", |req, _| {
            // TODO: なぜか JP ではなく 13 が返ってくるため調査する
            let cf_region = req.cf().region_code().unwrap_or("ZZ".into());
            Response::from_json(&json!({ "region": cf_region }))
        })
        .get("/generate-token", |_, ctx| {
            let pk = match ctx.var("JWT_PRIVATE_KEY") {
                Ok(v) => v.to_string(),
                Err(e) => {
                    console_error!("Error: {}", e);
                    return Response::error("internal error", 500);
                }
            };
            let issuer = match ctx.var("JWT_ISSUER") {
                Ok(v) => v.to_string(),
                Err(e) => {
                    console_error!("Error: {}", e);
                    return Response::error("internal error", 500);
                }
            };
            let res = jwt::generate_token(pk, issuer);
            match res {
                Ok(token) => Response::from_json(&json!({ "token": token })),
                Err(e) => {
                    console_error!("internal error: {}", e);
                    Response::error("internal error", 500)
                }
            }
        })
        .get("/verify-token", |req, ctx| {
            match req.headers().get("authorization").unwrap_or(None) {
                Some(v) => {
                    let token = match jwt::strip_bearer_token(v) {
                        Some(t) => t,
                        None => {
                            return Response::error("unauthorized", 401);
                        }
                    };
                    let pk = match ctx.var("JWT_PUBLIC_KEY") {
                        Ok(v) => v.to_string(),
                        Err(e) => {
                            console_error!("Error: {}", e);
                            return Response::error("internal error", 500);
                        }
                    };
                    let issuer = match ctx.var("JWT_ISSUER") {
                        Ok(v) => v.to_string(),
                        Err(e) => {
                            console_error!("Error: {}", e);
                            return Response::error("internal error", 500);
                        }
                    };
                    match jwt::verify_token(pk, issuer, token) {
                        Ok(claims) => {
                            return Response::from_json(&json!({ "value": claims }));
                        }
                        Err(e) => {
                            console_error!("forbidden: {}", e);
                            return Response::error("forbidden", 403);
                        }
                    }
                }
                None => {
                    return Response::error("unauthorized", 401);
                }
            }
        })
        .run(req, env)
        .await
}
