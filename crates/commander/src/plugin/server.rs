use core::panic;
use std::{any::TypeId, iter::once, net::SocketAddr, path::Path};

use axum::{
    http::{header, request::Parts, HeaderName, HeaderValue, StatusCode},
    Extension, Router,
};
use axum_extra::routing::{SecondElementIs, TypedPath};
use bevy::{
    app::App,
    utils::{tracing, HashMap},
};
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
    validate_request::ValidateRequestHeaderLayer,
};
use utoipa::{
    openapi::{
        path::{OperationBuilder, ParameterBuilder},
        request_body::RequestBodyBuilder,
        Content, PathItem, PathItemType, Ref, RefOr, Required, ResponsesBuilder, Schema,
    },
    ToSchema,
};

use crate::{
    event::SocketConnectionEvent,
    response::{ApiError, ApiResponse},
    router::{accept_headers::AcceptHeaders, CommandSchema},
    rpc::{NoInput, RpcChannel, RpcCommand, RpcDispatch, RpcDispatcher, RpcEndpoint},
};

use crate::router::{CommanderSchemaBuilder, CommanderState};

pub struct CommanderServer {
    events_tx: async_channel::Sender<SocketConnectionEvent>,

    commands: HashMap<&'static str, Box<dyn RpcDispatch>>,

    router: Option<Router<()>>,

    schema: Option<CommanderSchemaBuilder>,
}

impl CommanderServer {
    pub(crate) fn new(events_tx: async_channel::Sender<SocketConnectionEvent>) -> Self {
        Self {
            events_tx,
            commands: HashMap::new(),
            router: Some(Router::new()),

            schema: Some(CommanderSchemaBuilder::default()),
        }
    }
}

pub struct ThenRegisterEndpoint<'then, C> {
    _command: std::marker::PhantomData<C>,

    app: &'then mut App,
}

impl<'then, C> ThenRegisterEndpoint<'then, C>
where
    C: RpcCommand,
{
    fn add_operation_schema<T, P>(&mut self)
    where
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app.with_schema(|mut schema| {
            let params_schema = match P::schema().1 {
                RefOr::Ref(_) => panic!("params schema must be inline"),
                RefOr::T(Schema::Object(params_schema)) => params_schema,
                RefOr::T(_) => panic!("params schema must be an object"),
            };

            let mut operation_summary = None;
            let mut operation_description = Vec::new();

            let description = params_schema.description.unwrap_or_default();

            #[cfg(debug_assertions)]
            if description.is_empty() {
                tracing::warn!("missing description for rpc route `{}`", T::Command::NAME);
                tracing::warn!("    either no doc comment has been written, or the struct was");
                tracing::warn!("    created without a following `{{ }}`. For example, if the");
                tracing::warn!("    struct is `struct Foo;` instead of struct Foo {{ }}, doc");
                tracing::warn!("    comments will be ignored.");
            }

            for entry in description.split('\n').filter(|line| !line.is_empty()) {
                if operation_summary.is_none() {
                    operation_summary = Some(entry.trim().to_string());
                } else {
                    operation_description.push(entry.trim());
                }
            }

            let operation_description = operation_description.join("\n\n");

            let mut operation = OperationBuilder::new()
                .operation_id(Some(T::Command::NAME))
                .summary(operation_summary)
                .description(if !operation_description.is_empty() {
                    Some(operation_description)
                } else {
                    None
                });

            {
                let mut parameters = Vec::new();

                for (prop_name, prop_schema) in params_schema.properties {
                    // TODO: extract the rest of the props
                    parameters.push(
                        ParameterBuilder::new()
                            .required(if params_schema.required.contains(&prop_name) {
                                Required::True
                            } else {
                                Required::False
                            })
                            .name(prop_name)
                            .schema(Some(prop_schema))
                            .build(),
                    );
                }

                operation = operation.parameters(Some(parameters));
            }

            // `NoInput` is special and means there is no request body.
            if TypeId::of::<<T::Command as RpcCommand>::Input>() != TypeId::of::<NoInput>() {
                operation = operation.request_body(Some(
                    RequestBodyBuilder::new()
                        .content(
                            "application/json",
                            Content::new(Ref::from_schema_name(
                                <T::Command as RpcCommand>::Input::schema().0,
                            )),
                        )
                        .build(),
                ));
            }

            operation = operation.responses(
                <T::Command as RpcCommand>::Output::apply_responses(ResponsesBuilder::new())
                    .build(),
            );

            schema.paths = schema
                .paths
                .path(P::PATH, PathItem::new(PathItemType::Get, operation.build()));

            schema
        });
    }

    pub fn get<H, T, P>(mut self, handler: H) -> &'then mut App
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app
            .with_router(|router| router.route(P::PATH, axum::routing::get(handler)));

        self.add_operation_schema::<T, P>();

        self.app
    }

    pub fn post<H, T, P>(mut self, handler: H) -> &'then mut App
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app
            .with_router(|router| router.route(P::PATH, axum::routing::post(handler)));

        self.add_operation_schema::<T, P>();

        self.app
    }

    pub fn put<H, T, P>(mut self, handler: H) -> &'then mut App
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app
            .with_router(|router| router.route(P::PATH, axum::routing::put(handler)));

        self.add_operation_schema::<T, P>();

        self.app
    }

    pub fn patch<H, T, P>(mut self, handler: H) -> &'then mut App
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app
            .with_router(|router| router.route(P::PATH, axum::routing::patch(handler)));

        self.add_operation_schema::<T, P>();

        self.app
    }

    pub fn delete<H, T, P>(mut self, handler: H) -> &'then mut App
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + RpcEndpoint<Command = C> + 'static,
        P: TypedPath + ToSchema<'static>,
    {
        self.app
            .with_router(|router| router.route(P::PATH, axum::routing::delete(handler)));

        self.add_operation_schema::<T, P>();

        self.app
    }
}

pub trait CommanderServerExt {
    fn with_schema<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(CommanderSchemaBuilder) -> CommanderSchemaBuilder;

    fn register_command<C>(&mut self) -> ThenRegisterEndpoint<C>
    where
        C: RpcCommand + 'static;

    fn with_router<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(Router<()>) -> Router<()>;

    fn get_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>;

    fn post_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>;

    fn put_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>;

    fn patch_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>;

    fn delete_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>;
}

impl CommanderServerExt for App {
    fn with_schema<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(CommanderSchemaBuilder) -> CommanderSchemaBuilder,
    {
        let mut setup = self
            .world
            .get_non_send_resource_mut::<CommanderServer>()
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

    fn register_command<C>(&mut self) -> ThenRegisterEndpoint<C>
    where
        C: RpcCommand + 'static,
    {
        let mut setup = self
            .world
            .get_non_send_resource_mut::<CommanderServer>()
            .expect("commander plugin not initialized");

        let (rpc_tx, rpc_rx) = mpsc::unbounded_channel();

        setup
            .commands
            .insert(C::NAME, Box::new(RpcDispatcher::<C>::new(rpc_tx)));

        self.insert_resource(RpcChannel::new(rpc_rx));

        self.with_schema(|mut schema| {
            schema.components = C::Output::apply_components(schema.components);

            let (command_name, command_schema) = C::schema();

            let (command_title, command_description) = match command_schema {
                RefOr::Ref(_) => panic!("command schema must be inline"),

                RefOr::T(Schema::Object(command_schema)) => {
                    (command_schema.title, command_schema.description)
                }

                RefOr::T(_) => panic!("command schema must be an object"),
            };

            // `NoInput` is special and means there is no request body.
            let input = if TypeId::of::<C::Input>() != TypeId::of::<NoInput>() {
                let (input_name, input_schema) = C::Input::schema();

                schema.components = schema.components.schema(input_name, input_schema);

                Some(Ref::from_schema_name(input_name))
            } else {
                None
            };

            // Extract the schemas from the responses so we can build the Command schema later
            let responses = C::Output::apply_responses(ResponsesBuilder::new())
                .build()
                .responses;

            schema.commands.push(CommandSchema {
                id: C::NAME,

                name: command_name,
                title: command_title,
                description: command_description,

                input,
                responses,
            });

            // let command_type_name = std::any::type_name::<C>();

            // let command_type_name = command_type_name
            //     .split('<')
            //     .next()
            //     .unwrap_or(command_type_name)
            //     .split("::")
            //     .last()
            //     .expect("failed to get type name");

            // schema
            //     .jsonschema
            //     .generator
            //     .definitions_mut()
            //     .insert(command_type_name.to_string(), C::Input::to_json_schema());

            // let output_type_name = std::any::type_name::<C::Output>();

            // let output_type_name = output_type_name
            //     .split('<')
            //     .next()
            //     .unwrap_or(output_type_name)
            //     .split("::")
            //     .last()
            //     .expect("failed to get type name");

            // schema
            //     .jsonschema
            //     .generator
            //     .definitions_mut()
            //     .insert(output_type_name.to_string(), C::Output::to_json_schema());

            schema
        });

        ThenRegisterEndpoint {
            _command: std::marker::PhantomData,

            app: self,
        }
    }

    fn with_router<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(Router<()>) -> Router<()>,
    {
        let mut setup = self
            .world
            .get_non_send_resource_mut::<CommanderServer>()
            .expect("commander plugin not initialized");

        let new_router = func(
            setup
                .router
                .take()
                .expect("commander plugin already started"),
        );

        setup.router.replace(new_router);

        self
    }

    fn get_endpoint<H, T, P>(&mut self, handler: H) -> &mut Self
    where
        H: axum::handler::Handler<T, ()>,
        T: SecondElementIs<P> + 'static,
        P: TypedPath + ToSchema<'static>,
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
        schema_path: Option<&Path>,
    ) -> oneshot::Receiver<SocketAddr> {
        let router = self
            .router
            .take()
            .expect("commander server has already been started");

        let schema_router = Router::new()
            .route(
                "/openapi.json",
                axum::routing::get(crate::router::get_openapi_schema)
                    // The schema route only accepts the `application/json` mime type
                    .layer(ValidateRequestHeaderLayer::custom(AcceptHeaders::new([
                        mime::APPLICATION_JSON,
                    ]))),
            )
            // .route(
            //     "/schema.json",
            //     axum::routing::get(crate::router::get_json_schema)
            //         // The schema route only accepts the `application/json` mime type
            //         .layer(ValidateRequestHeaderLayer::custom(AcceptHeaders::new([
            //             mime::APPLICATION_JSON,
            //         ]))),
            // )
            .route(
                "/redoc",
                axum::routing::get(crate::router::get_redoc)
                    // The schema route only accepts the `text/html` mime type
                    .layer(ValidateRequestHeaderLayer::custom(AcceptHeaders::new([
                        mime::TEXT_HTML,
                    ]))),
            );

        let api_router = router
            .fallback(handler_404)
            .layer(axum::middleware::from_fn(
                crate::body::layer::transform_response,
            ));

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
            .merge(schema_router)
            .merge(api_router)
            // Add compression here since it isn't supported by the streaming layer
            .layer(CompressionLayer::new())
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

        let schema = self
            .schema
            .take()
            .expect("commander schema missing")
            .build();

        if let Some(schema_path) = schema_path {
            let absolute_path = std::fs::canonicalize(schema_path).expect("failed to canonicalize");

            tracing::info!("writing schemas to {}", absolute_path.display());

            // Write the different schemas to disk
            if let Err(err) = std::fs::write(
                absolute_path.join("openapi.json"),
                serde_json::to_string_pretty(&schema.build())
                    .expect("failed to serialize openapi schema"),
            ) {
                tracing::error!("failed to write openapi.json: {}", err);
            }
        }

        let router = router.layer(Extension(CommanderState::new(
            self.events_tx.clone(),
            {
                let mut commands = HashMap::default();
                std::mem::swap(&mut self.commands, &mut commands);
                commands
            },
            schema,
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
