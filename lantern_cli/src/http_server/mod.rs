use crate::types::PoolClient;
use crate::{logger::LogLevel, types::AnyhowVoidResult};
use actix_web::{
    error::ErrorInternalServerError, middleware::Logger, web, App, HttpServer, Result,
};
use cli::HttpServerArgs;
use deadpool_postgres::{Config as PoolConfig, Pool};
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod cli;
mod collection;
mod index;
mod pq;
mod search;
mod setup;

pub const COLLECTION_TABLE_NAME: &str = "_lantern_internal.http_collections";

struct AppPool {
    inner: Pool,
}

impl AppPool {
    fn new(pool: Pool) -> AppPool {
        Self { inner: pool }
    }

    async fn get(&self) -> Result<PoolClient, actix_web::Error> {
        let client = self.inner.get().await;
        match client {
            Err(e) => Err(ErrorInternalServerError(e)),
            Ok(client) => Ok(client),
        }
    }
}

pub struct AppState {
    db_uri: String,
    pool: AppPool,
    is_remote_database: bool,
    #[allow(dead_code)]
    logger: crate::logger::Logger,
}

/*
* PATCH - /collection/:name/:id -
*   { row: Row }
* */

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lantern HTTP API",
        description = "This is an HTTP wrapper over Lantern database, which also includes pq and external indexing functionalities from Lantern CLI.

The API endpoints are not SQL injection safe, so it can provide maximum flexibility for data manipulation, so please sanitize user input before sending requests to this API."
    ),
    paths(
        collection::create,
        collection::list,
        collection::get,
        collection::delete,
        collection::insert_data,
        search::vector_search,
        index::create_index,
        index::delete_index,
        pq::quantize_table,
    ),
    components(schemas(
        collection::CollectionInfo,
        collection::CreateTableInput,
        collection::InserDataInput,
        search::SearchInput,
        search::SearchResponse,
        index::CreateIndexInput,
        pq::CreatePQInput
    ))
)]
pub struct ApiDoc;

#[actix_web::main]
pub async fn start(
    args: HttpServerArgs,
    logger: Option<crate::logger::Logger>,
) -> AnyhowVoidResult {
    let logger = logger.unwrap_or(crate::logger::Logger::new("Lantern HTTP", LogLevel::Debug));
    logger.info(&format!(
        "Starting web server on http://{host}:{port}",
        host = args.host,
        port = args.port,
    ));
    logger.info(&format!(
        "Documentation available at http://{host}:{port}/swagger-ui/",
        host = args.host,
        port = args.port,
    ));
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let mut config = PoolConfig::new();
    config.url = Some(args.db_uri.clone());
    let pool = config.create_pool(None, NoTls)?;

    setup::setup_tables(&pool).await?;
    let state = web::Data::new(AppState {
        db_uri: args.db_uri.clone(),
        is_remote_database: args.remote_database,
        pool: AppPool::new(pool),
        logger,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r - %s %Dms"))
            .app_data(state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(collection::list)
            .service(collection::get)
            .service(collection::create)
            .service(collection::delete)
            .service(collection::insert_data)
            .service(search::vector_search)
            .service(index::create_index)
            .service(index::delete_index)
            .service(pq::quantize_table)
    })
    .bind((args.host.clone(), args.port))?
    .run()
    .await?;
    Ok(())
}
