use std::{
    iter::once,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    http::{header, request::Parts, HeaderName, HeaderValue, StatusCode},
    Extension, Router,
};
use axum_extra::routing::{SecondElementIs, TypedPath};
use bevy::{app::App, ecs::system::Resource, utils::HashMap};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{mpsc, oneshot},
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    trace::TraceLayer,
};

use crate::{
    event::SocketConnectionEvent,
    response::ApiError,
    rpc::{RpcChannel, RpcCommand, RpcDispatch, RpcDispatcher},
};

use crate::router::{CommanderSchemaBuilder, CommanderState};

#[derive(Resource)]
pub struct CommanderServer {
    events_tx: async_channel::Sender<SocketConnectionEvent>,

    commands: HashMap<&'static str, Box<dyn RpcDispatch>>,

    router: Arc<Mutex<Option<Router<()>>>>,

    schema: Option<CommanderSchemaBuilder>,
}

impl CommanderServer {
    pub(crate) fn new(events_tx: async_channel::Sender<SocketConnectionEvent>) -> Self {
        Self {
            events_tx,
            commands: HashMap::new(),
            router: Arc::new(Mutex::new(Some(Router::new()))),

            schema: Some(CommanderSchemaBuilder::default()),
        }
    }
}

pub trait CommanderServerExt {
    fn with_schema<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(CommanderSchemaBuilder) -> CommanderSchemaBuilder;

    fn register_command<C>(&mut self) -> &mut Self
    where
        C: RpcCommand + 'static;

    fn with_router<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(Router<()>) -> Router<()>;

    fn get_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath;

    fn post_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath;

    fn put_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath;

    fn patch_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath;

    fn delete_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath;
}

impl CommanderServerExt for App {
    fn with_schema<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(CommanderSchemaBuilder) -> CommanderSchemaBuilder,
    {
        let mut setup = self
            .world
            .get_resource_mut::<CommanderServer>()
            .expect("commander plugin not initialized");

        let new_schema = func(
            setup
                .schema
                .take()
                .expect("commander plugin already started"),
        );

        setup.schema.replace(new_schema);

        self
    }

    fn register_command<C>(&mut self) -> &mut Self
    where
        C: RpcCommand + 'static,
    {
        let mut setup = self
            .world
            .get_resource_mut::<CommanderServer>()
            .expect("commander plugin not initialized");

        let (rpc_tx, rpc_rx) = mpsc::unbounded_channel();

        setup
            .commands
            .insert(C::NAME, Box::new(RpcDispatcher::<C>::new(rpc_tx)));

        self.insert_resource(RpcChannel::new(rpc_rx));

        self.with_schema(|mut schema| {
            schema.components = schema.components.schema_from::<C>();
            schema.components = schema.components.schema_from::<C::Output>();
            schema
        });

        self
    }

    fn with_router<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(Router<()>) -> Router<()>,
    {
        let setup = self
            .world
            .get_resource_mut::<CommanderServer>()
            .expect("commander plugin not initialized");

        let mut router = setup.router.lock().expect("router lock poisoned");

        let new_router = func(router.take().expect("commander plugin already started"));

        router.replace(new_router);

        drop(router);

        self
    }

    fn get_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath,
    {
        self.with_router(|router| router.route(P::PATH, axum::routing::get(handler)));

        self
    }

    fn post_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath,
    {
        self.with_router(|router| router.route(P::PATH, axum::routing::post(handler)));

        self
    }

    fn put_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath,
    {
        self.with_router(|router| router.route(P::PATH, axum::routing::put(handler)));

        self
    }

    fn patch_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath,
    {
        self.with_router(|router| router.route(P::PATH, axum::routing::patch(handler)));

        self
    }

    fn delete_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath,
    {
        self.with_router(|router| router.route(P::PATH, axum::routing::delete(handler)));

        self
    }
}

impl CommanderServer {
    pub fn start_listening(
        &mut self,
        addr: impl ToSocketAddrs + Send + 'static,
    ) -> oneshot::Receiver<SocketAddr> {
        let router = self
            .router
            .lock()
            .expect("commander router lock poisoned")
            .take()
            .expect("commander server has already been started");

        let api_router = router
            .route("/schema", axum::routing::get(crate::router::get_schema))
            .fallback(handler_404)
            .layer(axum::middleware::from_fn(
                crate::body::layer::transform_response,
            ))
            // Add compression here since it isn't supported by the streaming layer
            .layer(CompressionLayer::new());

        // let sse_router = Router::new()
        //     // SSE layers only accept a single mime type
        //     .route_layer(ValidateRequestHeaderLayer::custom(AcceptHeaders::new([
        //         mime::TEXT_EVENT_STREAM,
        //     ])));

        let socket_router =
            Router::new().route("/ws", axum::routing::get(crate::router::ws_handler));

        let router = Router::new()
            // We construct the router in this way since the socket routes have different
            // layer requirements compared to the other routes.
            .merge(api_router)
            // .merge(sse_router)
            .merge(socket_router)
            // Ubiquitous layers
            .layer(
                ServiceBuilder::new()
                    // Pre-processing before instrumenting tracing
                    .layer(SetSensitiveRequestHeadersLayer::new(once(
                        header::AUTHORIZATION,
                    )))
                    .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                        "x-request-id",
                    )))
                    // Log using `tracing`
                    .layer(TraceLayer::new_for_http())
                    // We want to record metrics for all requests as soon as possible
                    // .layer(prometheus_layer)
                    // Rate limit before doing significant work
                    // .layer(
                    //     ServiceBuilder::new()
                    //         .layer(HandleErrorLayer::new(handle_governor_error))
                    //         .layer(GovernorLayer {
                    //             config: Box::leak(governor_conf),
                    //         }),
                    // )
                    .layer(
                        CorsLayer::new()
                            .allow_headers([
                                header::ACCEPT,
                                header::CONTENT_TYPE,
                                header::AUTHORIZATION,
                            ])
                            .allow_methods(tower_http::cors::Any)
                            .allow_origin(AllowOrigin::predicate(
                                |origin: &HeaderValue, _request_parts: &Parts| {
                                    if origin == HeaderValue::from_static("https://spacetraders.io")
                                    {
                                        return true;
                                    }

                                    if origin == HeaderValue::from_static("http://localhost:3000") {
                                        return true;
                                    }

                                    false
                                },
                            )),
                    ),
            );

        let router = router.layer(Extension(CommanderState::new(
            self.events_tx.clone(),
            {
                let mut commands = HashMap::default();
                std::mem::swap(&mut self.commands, &mut commands);
                commands
            },
            self.schema
                .take()
                .expect("commander schema missing")
                .build(),
        )));

        let (ready_tx, ready_rx) = oneshot::channel::<SocketAddr>();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to create tokio runtime");

            rt.block_on(async move {
                let listener = TcpListener::bind(addr).await.expect("failed to bind");

                ready_tx
                    .send(listener.local_addr().expect("failed to get bound addr"))
                    .ok();

                axum::serve(
                    listener,
                    router.into_make_service_with_connect_info::<SocketAddr>(),
                )
                .await
                .expect("commander server failed");
            })
        });

        ready_rx
    }
}

async fn handler_404() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND)
        .with_name("not_found")
        .with_message("That endpoint does not exist.")
}
