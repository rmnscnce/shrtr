/*
 * Copyright 2022 rmnscnce@ya.ru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

/// ### "Favicon" of the site
///
/// Modify `src/assets/favicon.ico` to change the favicon
const FAVICON: &'static [u8] = include_bytes!("assets/favicon.ico");

/// ### Homepage of the website
///
/// Modify `src/assets/html/default.html` to change how the homepage behaves
const DEFAULT_PAGE: &'static str = include_str!("assets/html/default.html");

/// ### "Error 404" page of the website
///
/// Modify `src/assets/html/e404.html` to change how the page behaves
const E404_PAGE: &'static str = include_str!("assets/html/e404.html");

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let path = req.path();

    Router::new()
        .get_async(&path, |req, ctx| async move {
            let kv = ctx.kv("SHRTR_MAP")?;

            match req.path().as_str() {
                "/" => Response::from_html(DEFAULT_PAGE),
                "/favicon.ico" => Response::from_bytes(FAVICON.to_vec()),
                "/404" => Response::from_html(E404_PAGE),
                _ => {
                    if let Ok(Some(target)) = kv.get(&req.path()).text().await {
                        Response::redirect(Url::parse(&target)?)
                    } else {
                        let ret = Response::from_html(E404_PAGE)?.with_status(404);
                        Ok(ret)
                    }
                }
            }
        })
        .run(req, env)
        .await
}
