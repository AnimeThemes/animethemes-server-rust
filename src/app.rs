use migration::Migrator;
use std::env;
use std::path::Path;

use async_trait::async_trait;
use axum::{
    Router as AxumRouter,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::from_fn_with_state,
    routing::{get, post},
};
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    prelude::Routes,
    task::Tasks,
};
use tower_http::cors::{AllowOrigin, CorsLayer};
use url::Url;

#[allow(unused_imports)]
use crate::{controllers, tasks, workers::downloader::DownloadWorker};
use crate::{
    middlewares::{current_user::current_user_middleware, features::features_middleware},
    schema::{self, graphiql, graphql_handler},
    session::create_session_layer,
    typesense::client::{create_typesense_client, init_typesense},
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        option_env!("APP_NAME").unwrap_or("AnimeThemes")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        let typesense = create_typesense_client();

        init_typesense(typesense.clone());

        let schema = schema::create_schema(ctx.clone(), typesense.clone());

        ctx.shared_store.insert(schema);
        ctx.shared_store.insert(typesense);

        Ok(ctx)
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::verify_email::routes())
            .add_route(
                Routes::new()
                    .add("/", get(graphiql))
                    .add("/graphql", post(graphql_handler)),
            )
    }

    async fn after_routes(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let parsed_url = Url::parse(&ctx.config.server.host).expect("invalid server.host");

        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(
                move |origin: &HeaderValue, _request_parts| {
                    let Ok(origin) = origin.to_str() else {
                        return false;
                    };

                    let Ok(origin) = Url::parse(origin) else {
                        return false;
                    };

                    origin.scheme() == parsed_url.scheme() && origin.host() == parsed_url.host()
                },
            ))
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
            .allow_credentials(true);

        let session_layer = create_session_layer(&ctx).await;

        Ok(router
            .layer(from_fn_with_state(ctx.clone(), features_middleware))
            .layer(from_fn_with_state(ctx.clone(), current_user_middleware))
            .layer(session_layer)
            .layer(cors))
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::print_schema::PrintSchema);
        tasks.register(tasks::search_index_anime::SearchIndexAnime);
        tasks.register(tasks::search_index_animetheme::SearchIndexAnimeTheme);
        tasks.register(tasks::search_index_animethemeentry::SearchIndexAnimeThemeEntry);
        tasks.register(tasks::search_index_artist::SearchIndexArtist);
        tasks.register(tasks::search_index_playlist::SearchIndexPlaylist);
        tasks.register(tasks::search_index_series::SearchIndexSeries);
        tasks.register(tasks::search_index_song::SearchIndexSong);
        tasks.register(tasks::search_index_studio::SearchIndexStudio);
        tasks.register(tasks::search_index_video::SearchIndexVideo);
        tasks.register(tasks::clear_reset_passwords::ClearResetPasswords);
        // tasks-inject (do not remove)
    }

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        Ok(())
    }
}
