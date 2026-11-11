use migration::Migrator;
use reqwest::header::USER_AGENT;
use std::env;
use std::path::Path;

use async_trait::async_trait;
use axum::{
    Router as AxumRouter,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::{self},
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
use tower_http::cors::{AllowCredentials, AllowOrigin, CorsLayer};
use url::Url;

use crate::{
    caches::hostname_cache::HostnameCache,
    middlewares::{self, rate_limiting::GraphqlRateLimitState},
    schema::{self, graphiql, graphql_handler},
    seeds,
    session::create_session_layer,
    typesense::client::{create_typesense_client, init_typesense},
    workers,
};
#[allow(unused_imports)]
use crate::{controllers, tasks};
use crate::{settings, storage::create_storage};

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

        let schemas = schema::create_schemas(ctx.clone(), typesense.clone());
        let settings = settings::load_settings(ctx.clone())?;

        ctx.shared_store.insert(schemas);
        ctx.shared_store.insert(typesense);
        ctx.shared_store.insert(settings);

        let storage = create_storage(ctx.clone())?;

        Ok(ctx.into_builder().storage(storage.into()).build())
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        let rate_limit_state = GraphqlRateLimitState::from_env();

        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::verify_email::routes())
            .add_route(controllers::streams::audio_stream_controller::routes())
            .add_route(controllers::streams::video_stream_controller::routes())
            .add_route(
                Routes::new().add("/", get(graphiql)).add(
                    "/graphql",
                    post(graphql_handler)
                        .route_layer(middleware::from_fn(
                            middlewares::require_json_content::require_json_content_middleware,
                        ))
                        .route_layer(middleware::from_fn_with_state(
                            rate_limit_state,
                            middlewares::rate_limiting::rate_limiting_middleware,
                        )),
                ),
            )
    }

    async fn after_routes(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let client_url = env::var("CLIENT_URL").expect("CLIENT_URL is required in .env");

        let client_origin = Url::parse(&client_url)
            .expect("Invalid CLIENT_URL")
            .origin();

        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::mirror_request())
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION, USER_AGENT])
            .allow_credentials(AllowCredentials::predicate(
                move |origin: &HeaderValue, _request_parts| {
                    origin
                        .to_str()
                        .ok()
                        .and_then(|s| Url::parse(s).ok())
                        .is_some_and(|url| url.origin() == client_origin)
                },
            ));

        let session_layer = create_session_layer(&ctx).await;

        let hostname_cache = HostnameCache::new().await;

        Ok(router
            .layer(middleware::from_fn_with_state(
                ctx.clone(),
                middlewares::features::features_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                ctx.clone(),
                middlewares::current_user::current_user_middleware,
            ))
            .layer(middleware::from_fn(
                middlewares::require_user_agent::require_user_agent_middleware,
            ))
            .layer(session_layer)
            .layer(axum::Extension(hostname_cache))
            .layer(cors))
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue
            .register(workers::discord_notification::DiscordNotificationWorker::build(ctx))
            .await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::assign_role::AssignRole);
        tasks.register(tasks::clear_reset_passwords::ClearResetPasswords);
        tasks.register(tasks::create_user::CreateUser);
        tasks.register(tasks::print_schema::PrintSchema);
        tasks.register(tasks::prune_watch_history::PruneWatchHistory);
        tasks.register(tasks::search_index_anime::SearchIndexAnime);
        tasks.register(tasks::search_index_theme::SearchIndexTheme);
        tasks.register(tasks::search_index_entry::SearchIndexEntry);
        tasks.register(tasks::search_index_artist::SearchIndexArtist);
        tasks.register(tasks::search_index_playlist::SearchIndexPlaylist);
        tasks.register(tasks::search_index_series::SearchIndexSeries);
        tasks.register(tasks::search_index_song::SearchIndexSong);
        tasks.register(tasks::search_index_studio::SearchIndexStudio);
        tasks.register(tasks::search_index_video::SearchIndexVideo);
        tasks.register(tasks::sync_entry_favorites_count::SyncEntryFavoritesCount);
        // tasks-inject (do not remove)
    }

    async fn truncate(_ctx: &AppContext) -> Result<()> {
        Ok(())
    }

    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        seeds::role_seeder::seed_roles(ctx, base).await?;
        seeds::sanction_seeder::seed_sanctions(ctx, base).await?;
        seeds::feature_seeder::seed_features(ctx, base).await?;

        Ok(())
    }
}
