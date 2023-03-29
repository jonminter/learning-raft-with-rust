use single_value_store_proto::single_value_store;
use single_value_store_proto::single_value_store::single_value_store_server::SingleValueStore;
use tracing::info;

pub(crate) struct SingleValueStoreImpl {}

#[tonic::async_trait]
impl SingleValueStore for SingleValueStoreImpl {
    async fn get(
        &self,
        _: tonic::Request<single_value_store::GetRequest>,
    ) -> Result<tonic::Response<single_value_store::GetResponse>, tonic::Status> {
        info!("Client requested value: {:?}", 0);
        Ok(tonic::Response::new(single_value_store::GetResponse {
            value: 0,
        }))
    }

    async fn set(
        &self,
        request: tonic::Request<single_value_store::SetRequest>,
    ) -> Result<tonic::Response<single_value_store::SetResponse>, tonic::Status> {
        info!("Client set value: {:?}", request.into_inner().value);
        Ok(tonic::Response::new(single_value_store::SetResponse {}))
    }
}
