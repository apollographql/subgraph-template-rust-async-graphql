use async_graphql::{EmptySubscription, Object, ID};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::middleware::from_extractor;
use axum::{extract::Extension, routing::post, Router};
use http::{HeaderValue, Method};
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::thing::{get_thing, CreateThing, Thing};

mod router_auth;
mod thing;

struct Query;

#[Object]
impl Query {
    // TODO: Fill in query AND entity resolvers
    /// This will show up in the supergraph schema as part of Query.
    async fn thing(&self, id: ID) -> Option<Thing> {
        get_thing(id)
    }

    /// This will be available to other subgraphs as an entity.
    #[graphql(entity)]
    async fn thing_entity_by_id(&self, id: ID) -> Option<Thing> {
        get_thing(id)
    }
}

struct Mutation;

#[Object]
impl Mutation {
    // TODO: Fill in mutation resolvers
    async fn create_thing(&self, thing: CreateThing) -> Thing {
        let CreateThing { id, name } = thing;
        Thing { id, name }
    }
}

type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[must_use]
pub fn app() -> Router {
    let schema: Schema = Schema::build(Query, Mutation, EmptySubscription)
        .enable_federation()
        .limit_complexity(100)
        .finish();

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers(Any)
        .allow_origin(
            "https://studio.apollographql.com"
                .parse::<HeaderValue>()
                .expect("Can enable sandbox CORS"),
        );

    Router::new().route("/", post(graphql_handler)).layer(
        ServiceBuilder::new()
            .layer(Extension(schema))
            .layer(CompressionLayer::new())
            .layer(cors)
            .layer(from_extractor::<router_auth::RequireRouterAuth>()),
    )
}
