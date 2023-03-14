use tracing::{field::Empty, instrument};
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_interface_keyvalue::{
    GetResponse, IncrementRequest, KeyValue, KeyValueReceiver, ListAddRequest, ListDelRequest,
    ListRangeRequest, SetAddRequest, SetDelRequest, SetRequest, StringList,
};

const PROVIDER_NAME: &str = "My Provider";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(MyKeyvalue, Some(PROVIDER_NAME.to_string()))?;

    eprintln!("{PROVIDER_NAME} exiting");
    Ok(())
}

/// Custom keyvalue provider implementation.
#[derive(Clone, Provider)]
#[services(KeyValue)]
struct MyKeyvalue;
/// use default implementations of provider message handlers
impl ProviderDispatch for MyKeyvalue {}

/// Handle provider control commands
#[async_trait]
impl ProviderHandler for MyKeyvalue {
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        todo!()
    }

    #[instrument(level = "info", skip(self))]
    async fn delete_link(&self, actor_id: &str) {
        todo!()
    }
}
#[async_trait]
impl KeyValue for MyKeyvalue {
    /// Increments a numeric value, returning the new value
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn increment(&self, ctx: &Context, arg: &IncrementRequest) -> RpcResult<i32> {
        todo!()
    }

    /// Returns true if the store contains the key
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn contains<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        todo!()
    }

    /// Deletes a key, returning true if the key was deleted
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn del<TS: ToString + ?Sized + Sync>(&self, ctx: &Context, arg: &TS) -> RpcResult<bool> {
        todo!()
    }

    /// Gets a value for a specified key. If the key exists, the return structure contains exists:
    /// true and the value, otherwise the return structure contains exists == false.
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn get<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetResponse> {
        todo!()
    }

    /// Append a value onto the end of a list. Returns the new list size
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn list_add(&self, ctx: &Context, arg: &ListAddRequest) -> RpcResult<u32> {
        todo!()
    }

    /// Deletes a list and its contents
    /// input: list name
    /// returns: true if the list existed and was deleted
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor))]
    async fn list_clear<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        todo!()
    }

    /// Deletes an item from a list. Returns true if the item was removed.
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn list_del(&self, ctx: &Context, arg: &ListDelRequest) -> RpcResult<bool> {
        todo!()
    }

    /// Retrieves a range of values from a list using 0-based indices. Start and end values are
    /// inclusive, for example, (0,10) returns 11 items if the list contains at least 11 items. If
    /// the stop value is beyond the end of the list, it is treated as the end of the list.
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn list_range(&self, ctx: &Context, arg: &ListRangeRequest) -> RpcResult<StringList> {
        todo!()
    }

    /// Sets the value of a key. expires is an optional number of seconds before the value should be
    /// automatically deleted, or 0 for no expiration.
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn set(&self, ctx: &Context, arg: &SetRequest) -> RpcResult<()> {
        todo!()
    }

    /// Add an item into a set. Returns number of items added
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn set_add(&self, ctx: &Context, arg: &SetAddRequest) -> RpcResult<u32> {
        todo!()
    }

    /// Remove a item from the set. Returns number of items deleted
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn set_del(&self, ctx: &Context, arg: &SetDelRequest) -> RpcResult<u32> {
        todo!()
    }

    /// Deletes a set and its contents
    /// input: set name
    /// returns: true if the set existed and was deleted
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor))]
    async fn set_clear<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<bool> {
        todo!()
    }

    /// Return the intersection of all specified sets
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, keys = ?arg))]
    async fn set_intersection(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList> {
        todo!()
    }

    /// Returns the set at the given key
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, key = Empty))]
    async fn set_query<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<StringList> {
        todo!()
    }

    /// Return the union of all specified sets
    #[instrument(level = "debug", skip_all, fields(actor_id = ?ctx.actor, keys = ?arg))]
    async fn set_union(&self, ctx: &Context, arg: &StringList) -> RpcResult<StringList> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_increment() {
        todo!()
    }
}
