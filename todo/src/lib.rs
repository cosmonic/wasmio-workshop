use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetAddRequest, SetDelRequest, SetRequest,
};
use wasmcloud_interface_logging::{debug, info, warn};
mod ui;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
/// An actor that receives RESTful HTTP API queries and stores proper requests as
/// Todo objects in a Key-Value store
pub struct TodoActor {}

/// Implementation of HttpServer contract, receive HttpRequest return HttpResponse
///
/// # Arguments
/// * `ctx` - The wasmCloud context object. Largely this is used for metadata and can simply be passed around
/// * `req` - HTTP request sent from an HTTP server
///  
/// # Returns
/// * `RpcResult<HttpResponse>` - Standard result type wrapping an HTTP response struct
#[async_trait]
impl HttpServer for TodoActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        handle_todo_request(ctx, req).await
    }
}

/// Handles an HTTP request as a Todo RESTful API query
///
/// # Arguments
/// * `req` - HTTP request sent from an HTTP server
///  
/// # Returns
/// * `RpcResult<HttpResponse>` - Standard result type wrapping an HTTP response struct
pub async fn handle_todo_request(ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
    debug!("incoming req: {:?}", req);

    let trimmed_path = req.path.trim_end_matches('/');
    match (req.method.as_ref(), trimmed_path) {
        ("GET", "") => ui::get_asset(&req.path).await,
        ("POST", "/api") => match serde_json::from_slice(&req.body) {
            Ok(input) => match create_todo(ctx, input).await {
                Ok(todo) => HttpResponse::json(todo, 200),
                Err(e) => Err(RpcError::ActorHandler(format!("creating todo: {:?}", e))),
            },
            Err(e) => Ok(HttpResponse::bad_request(format!(
                "malformed body: {:?}",
                e
            ))),
        },

        ("GET", "/api") => match get_all_todos(ctx).await {
            Ok(todos) => HttpResponse::json(todos, 200),
            Err(e) => Err(RpcError::ActorHandler(format!("getting all todos: {}", e))),
        },

        ("GET", url) => match get_todo(ctx, url).await {
            Ok(todo) => HttpResponse::json(todo, 200),
            // Fallback, attempt to get assets for a GET request that isn't a Todo
            Err(_) => ui::get_asset(&req.path).await,
        },

        ("PATCH", url) => match serde_json::from_slice(&req.body) {
            Ok(update) => match update_todo(ctx, url, update).await {
                Ok(todo) => HttpResponse::json(todo, 200),
                Err(e) => Err(RpcError::ActorHandler(format!("updating todo: {}", e))),
            },
            Err(e) => Ok(HttpResponse::bad_request(format!(
                "malformed body: {:?}",
                e
            ))),
        },

        ("DELETE", "/api") => match delete_all_todos(ctx).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!("deleting all todos: {}", e))),
        },

        ("DELETE", url) => match delete_todo(ctx, url).await {
            Ok(_) => Ok(HttpResponse::default()),
            Err(e) => Err(RpcError::ActorHandler(format!("deleting todo: {}", e))),
        },

        (_, _) => {
            warn!("no route for this request: {:?}", req);
            Ok(HttpResponse::not_found())
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Incoming HTTP payload to create a Todo item
pub struct InputTodo {
    title: String,
    order: Option<i32>,
}

#[derive(Serialize, Deserialize)]
/// Incoming HTTP payload to update a Todo item
pub struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
    order: Option<i32>,
}

#[derive(Serialize, Deserialize)]
/// Todo structure as stored in a Key-Value store
pub struct Todo {
    url: String,
    title: String,
    completed: bool,
    order: i32,
}

impl Todo {
    fn new(url: String, title: String, order: i32) -> Self {
        Self {
            url,
            title,
            completed: false,
            order,
        }
    }

    /// Updates a Todo given optional parameters to change title, completed, or order
    fn update(self, update: UpdateTodo) -> Todo {
        Todo {
            url: self.url,
            title: update.title.unwrap_or(self.title),
            completed: update.completed.unwrap_or(self.completed),
            order: update.order.unwrap_or(self.order),
        }
    }
}

/// Creates a Todo entry in a Key-Value store, adding it both as an entry with a unique ID
/// and into a set of all Todos for easy management
///
/// # Arguments
/// * `input` - [InputTodo] struct to create
///  
/// # Returns
/// * `Result<Todo>` - Created [Todo] struct
pub async fn create_todo(ctx: &Context, input: InputTodo) -> Result<Todo> {
    info!("Creating a todo...");
    let id = KeyValueSender::new()
        .increment(
            ctx,
            &IncrementRequest {
                key: "next_id".to_string(),
                value: 1,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    let todo = Todo::new(
        format!("/api/{}", id),
        input.title,
        input.order.unwrap_or(0),
    );

    KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: todo.url.clone(),
                value: serde_json::to_string(&todo)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    KeyValueSender::new()
        .set_add(
            ctx,
            &SetAddRequest {
                set_name: "all_urls".to_string(),
                value: todo.url.clone(),
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(todo)
}

/// Updates a Todo entry in a Key-Value store
///
/// # Arguments
/// * `update` - [UpdateTodo] struct to override an existing Todo
///  
/// # Returns
/// * `Result<Todo>` - Updated [Todo] struct
pub async fn update_todo(ctx: &Context, url: &str, update: UpdateTodo) -> Result<Todo> {
    info!("Updating a todo...");

    let todo = get_todo(ctx, url).await?;
    let todo = todo.update(update);

    KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: todo.url.clone(),
                value: serde_json::to_string(&todo)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;
    Ok(todo)
}

/// Retrieves all Todos stored in the Key-Value store
///
/// # Returns
/// * `Result<Vec<Todo>>` - All Todos
pub async fn get_all_todos(ctx: &Context) -> Result<Vec<Todo>> {
    info!("Getting all todos...");

    let urls = KeyValueSender::new()
        .set_query(ctx, "all_urls")
        .await
        .map_err(|e| anyhow!(e))?;

    let mut result = Vec::new();
    for url in urls {
        result.push(get_todo(ctx, &url).await?)
    }
    Ok(result)
}

/// Retrieves a single Todo from the Key-Value store
///
/// # Arguments
/// * `url` - The RESTful API path to the Todo id
///  
/// # Returns
/// * `Result<Todo>` - [Todo] struct, if found
pub async fn get_todo(ctx: &Context, url: &str) -> Result<Todo> {
    info!("Getting a todo...");

    let todo_str = KeyValueSender::new()
        .get(ctx, url)
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    let todo = serde_json::from_str(&todo_str)?;

    Ok(todo)
}

/// Deletes all Todos from the Key-Value store
pub async fn delete_all_todos(ctx: &Context) -> Result<()> {
    info!("Deleting all todos...");

    let urls = KeyValueSender::new()
        .set_query(ctx, "all_urls")
        .await
        .map_err(|e| anyhow!(e))?;

    for url in urls {
        delete_todo(ctx, &url).await?
    }

    Ok(())
}

/// Deletes a single Todo from the Key-Value store
///
/// # Arguments
/// * `url` - The RESTful API path to the Todo id
pub async fn delete_todo(ctx: &Context, url: &str) -> Result<()> {
    info!("Deleting a todo...");

    KeyValueSender::new()
        .set_del(
            ctx,
            &SetDelRequest {
                set_name: "all_urls".to_string(),
                value: url.to_string(),
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    KeyValueSender::new()
        .del(ctx, url)
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
