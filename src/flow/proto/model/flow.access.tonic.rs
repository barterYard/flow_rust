// @generated
/// Generated client implementations.
pub mod access_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AccessApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccessApiClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccessApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccessApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AccessApiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> std::result::Result<tonic::Response<super::PingResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "Ping"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_node_version_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeVersionInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeVersionInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetNodeVersionInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetNodeVersionInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_latest_block_header(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestBlockHeaderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetLatestBlockHeader",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetLatestBlockHeader"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_header_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeaderByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetBlockHeaderByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetBlockHeaderByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_header_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockHeaderByHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetBlockHeaderByHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetBlockHeaderByHeight"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_latest_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestBlockRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetLatestBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetLatestBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockByIdRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetBlockByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetBlockByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockByHeightRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetBlockByHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetBlockByHeight"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_collection_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CollectionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetCollectionByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetCollectionByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SendTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/SendTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "SendTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetTransactionResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetTransactionResult"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_result_by_index(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetTransactionResultByIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "GetTransactionResultByIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_results_by_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetTransactionResultsByBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "GetTransactionResultsByBlockID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transactions_by_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetTransactionsByBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetTransactionsByBlockID"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.access.AccessAPI", "GetAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_at_latest_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountAtLatestBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetAccountAtLatestBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetAccountAtLatestBlock"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_at_block_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountAtBlockHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetAccountAtBlockHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetAccountAtBlockHeight"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_script_at_latest_block(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteScriptAtLatestBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/ExecuteScriptAtLatestBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "ExecuteScriptAtLatestBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_script_at_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteScriptAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/ExecuteScriptAtBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "ExecuteScriptAtBlockID"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_script_at_block_height(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteScriptAtBlockHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/ExecuteScriptAtBlockHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "ExecuteScriptAtBlockHeight",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_events_for_height_range(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventsForHeightRangeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetEventsForHeightRange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetEventsForHeightRange"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_events_for_block_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventsForBlockIDsRequest>,
        ) -> std::result::Result<tonic::Response<super::EventsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetEventsForBlockIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetEventsForBlockIDs"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_network_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNetworkParametersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetNetworkParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetNetworkParameters"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_latest_protocol_state_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetLatestProtocolStateSnapshotRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ProtocolStateSnapshotResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetLatestProtocolStateSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "GetLatestProtocolStateSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_execution_result_for_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionResultForBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutionResultForBlockIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetExecutionResultForBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.AccessAPI",
                        "GetExecutionResultForBlockID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_execution_result_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionResultByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutionResultByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.AccessAPI/GetExecutionResultByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.AccessAPI", "GetExecutionResultByID"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod access_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccessApiServer.
    #[async_trait]
    pub trait AccessApi: Send + Sync + 'static {
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> std::result::Result<tonic::Response<super::PingResponse>, tonic::Status>;
        async fn get_node_version_info(
            &self,
            request: tonic::Request<super::GetNodeVersionInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNodeVersionInfoResponse>,
            tonic::Status,
        >;
        async fn get_latest_block_header(
            &self,
            request: tonic::Request<super::GetLatestBlockHeaderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        >;
        async fn get_block_header_by_id(
            &self,
            request: tonic::Request<super::GetBlockHeaderByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        >;
        async fn get_block_header_by_height(
            &self,
            request: tonic::Request<super::GetBlockHeaderByHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlockHeaderResponse>,
            tonic::Status,
        >;
        async fn get_latest_block(
            &self,
            request: tonic::Request<super::GetLatestBlockRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status>;
        async fn get_block_by_id(
            &self,
            request: tonic::Request<super::GetBlockByIdRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status>;
        async fn get_block_by_height(
            &self,
            request: tonic::Request<super::GetBlockByHeightRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockResponse>, tonic::Status>;
        async fn get_collection_by_id(
            &self,
            request: tonic::Request<super::GetCollectionByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CollectionResponse>,
            tonic::Status,
        >;
        async fn send_transaction(
            &self,
            request: tonic::Request<super::SendTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendTransactionResponse>,
            tonic::Status,
        >;
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResponse>,
            tonic::Status,
        >;
        async fn get_transaction_result(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultResponse>,
            tonic::Status,
        >;
        async fn get_transaction_result_by_index(
            &self,
            request: tonic::Request<super::GetTransactionByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultResponse>,
            tonic::Status,
        >;
        async fn get_transaction_results_by_block_id(
            &self,
            request: tonic::Request<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionResultsResponse>,
            tonic::Status,
        >;
        async fn get_transactions_by_block_id(
            &self,
            request: tonic::Request<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionsResponse>,
            tonic::Status,
        >;
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountResponse>,
            tonic::Status,
        >;
        async fn get_account_at_latest_block(
            &self,
            request: tonic::Request<super::GetAccountAtLatestBlockRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountResponse>, tonic::Status>;
        async fn get_account_at_block_height(
            &self,
            request: tonic::Request<super::GetAccountAtBlockHeightRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountResponse>, tonic::Status>;
        async fn execute_script_at_latest_block(
            &self,
            request: tonic::Request<super::ExecuteScriptAtLatestBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        >;
        async fn execute_script_at_block_id(
            &self,
            request: tonic::Request<super::ExecuteScriptAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        >;
        async fn execute_script_at_block_height(
            &self,
            request: tonic::Request<super::ExecuteScriptAtBlockHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptResponse>,
            tonic::Status,
        >;
        async fn get_events_for_height_range(
            &self,
            request: tonic::Request<super::GetEventsForHeightRangeRequest>,
        ) -> std::result::Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn get_events_for_block_i_ds(
            &self,
            request: tonic::Request<super::GetEventsForBlockIDsRequest>,
        ) -> std::result::Result<tonic::Response<super::EventsResponse>, tonic::Status>;
        async fn get_network_parameters(
            &self,
            request: tonic::Request<super::GetNetworkParametersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNetworkParametersResponse>,
            tonic::Status,
        >;
        async fn get_latest_protocol_state_snapshot(
            &self,
            request: tonic::Request<super::GetLatestProtocolStateSnapshotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProtocolStateSnapshotResponse>,
            tonic::Status,
        >;
        async fn get_execution_result_for_block_id(
            &self,
            request: tonic::Request<super::GetExecutionResultForBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutionResultForBlockIdResponse>,
            tonic::Status,
        >;
        async fn get_execution_result_by_id(
            &self,
            request: tonic::Request<super::GetExecutionResultByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutionResultByIdResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AccessApiServer<T: AccessApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccessApi> AccessApiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccessApiServer<T>
    where
        T: AccessApi,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/flow.access.AccessAPI/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: AccessApi>(pub Arc<T>);
                    impl<T: AccessApi> tonic::server::UnaryService<super::PingRequest>
                    for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetNodeVersionInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeVersionInfoSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetNodeVersionInfoRequest>
                    for GetNodeVersionInfoSvc<T> {
                        type Response = super::GetNodeVersionInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNodeVersionInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_node_version_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeVersionInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetLatestBlockHeader" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestBlockHeaderSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetLatestBlockHeaderRequest>
                    for GetLatestBlockHeaderSvc<T> {
                        type Response = super::BlockHeaderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLatestBlockHeaderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_latest_block_header(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLatestBlockHeaderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetBlockHeaderByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeaderByIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetBlockHeaderByIdRequest>
                    for GetBlockHeaderByIDSvc<T> {
                        type Response = super::BlockHeaderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockHeaderByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_header_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHeaderByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetBlockHeaderByHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeaderByHeightSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetBlockHeaderByHeightRequest>
                    for GetBlockHeaderByHeightSvc<T> {
                        type Response = super::BlockHeaderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockHeaderByHeightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_header_by_height(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockHeaderByHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetLatestBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestBlockSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetLatestBlockRequest>
                    for GetLatestBlockSvc<T> {
                        type Response = super::BlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLatestBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_latest_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLatestBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetBlockByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetBlockByIdRequest>
                    for GetBlockByIDSvc<T> {
                        type Response = super::BlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetBlockByHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHeightSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetBlockByHeightRequest>
                    for GetBlockByHeightSvc<T> {
                        type Response = super::BlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByHeightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_height(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetCollectionByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionByIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetCollectionByIdRequest>
                    for GetCollectionByIDSvc<T> {
                        type Response = super::CollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_collection_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/SendTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendTransactionSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::SendTransactionRequest>
                    for SendTransactionSvc<T> {
                        type Response = super::SendTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetTransactionRequest>
                    for GetTransactionSvc<T> {
                        type Response = super::TransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetTransactionResult" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetTransactionRequest>
                    for GetTransactionResultSvc<T> {
                        type Response = super::TransactionResultResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_result(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionResultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetTransactionResultByIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultByIndexSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetTransactionByIndexRequest>
                    for GetTransactionResultByIndexSvc<T> {
                        type Response = super::TransactionResultResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionByIndexRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_result_by_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionResultByIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetTransactionResultsByBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultsByBlockIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetTransactionsByBlockIdRequest>
                    for GetTransactionResultsByBlockIDSvc<T> {
                        type Response = super::TransactionResultsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsByBlockIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_results_by_block_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionResultsByBlockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetTransactionsByBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsByBlockIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetTransactionsByBlockIdRequest>
                    for GetTransactionsByBlockIDSvc<T> {
                        type Response = super::TransactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsByBlockIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_by_block_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsByBlockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetAccountRequest>
                    for GetAccountSvc<T> {
                        type Response = super::GetAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetAccountAtLatestBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountAtLatestBlockSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetAccountAtLatestBlockRequest>
                    for GetAccountAtLatestBlockSvc<T> {
                        type Response = super::AccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAccountAtLatestBlockRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_at_latest_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountAtLatestBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetAccountAtBlockHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountAtBlockHeightSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetAccountAtBlockHeightRequest>
                    for GetAccountAtBlockHeightSvc<T> {
                        type Response = super::AccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAccountAtBlockHeightRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_at_block_height(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountAtBlockHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/ExecuteScriptAtLatestBlock" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteScriptAtLatestBlockSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<
                        super::ExecuteScriptAtLatestBlockRequest,
                    > for ExecuteScriptAtLatestBlockSvc<T> {
                        type Response = super::ExecuteScriptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecuteScriptAtLatestBlockRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).execute_script_at_latest_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteScriptAtLatestBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/ExecuteScriptAtBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteScriptAtBlockIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::ExecuteScriptAtBlockIdRequest>
                    for ExecuteScriptAtBlockIDSvc<T> {
                        type Response = super::ExecuteScriptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteScriptAtBlockIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).execute_script_at_block_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteScriptAtBlockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/ExecuteScriptAtBlockHeight" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteScriptAtBlockHeightSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<
                        super::ExecuteScriptAtBlockHeightRequest,
                    > for ExecuteScriptAtBlockHeightSvc<T> {
                        type Response = super::ExecuteScriptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecuteScriptAtBlockHeightRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).execute_script_at_block_height(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteScriptAtBlockHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetEventsForHeightRange" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventsForHeightRangeSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetEventsForHeightRangeRequest>
                    for GetEventsForHeightRangeSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetEventsForHeightRangeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_events_for_height_range(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEventsForHeightRangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetEventsForBlockIDs" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventsForBlockIDsSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetEventsForBlockIDsRequest>
                    for GetEventsForBlockIDsSvc<T> {
                        type Response = super::EventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEventsForBlockIDsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_events_for_block_i_ds(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEventsForBlockIDsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetNetworkParameters" => {
                    #[allow(non_camel_case_types)]
                    struct GetNetworkParametersSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetNetworkParametersRequest>
                    for GetNetworkParametersSvc<T> {
                        type Response = super::GetNetworkParametersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNetworkParametersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_network_parameters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNetworkParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetLatestProtocolStateSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestProtocolStateSnapshotSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<
                        super::GetLatestProtocolStateSnapshotRequest,
                    > for GetLatestProtocolStateSnapshotSvc<T> {
                        type Response = super::ProtocolStateSnapshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetLatestProtocolStateSnapshotRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_latest_protocol_state_snapshot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLatestProtocolStateSnapshotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetExecutionResultForBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetExecutionResultForBlockIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<
                        super::GetExecutionResultForBlockIdRequest,
                    > for GetExecutionResultForBlockIDSvc<T> {
                        type Response = super::ExecutionResultForBlockIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetExecutionResultForBlockIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_execution_result_for_block_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExecutionResultForBlockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.AccessAPI/GetExecutionResultByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetExecutionResultByIDSvc<T: AccessApi>(pub Arc<T>);
                    impl<
                        T: AccessApi,
                    > tonic::server::UnaryService<super::GetExecutionResultByIdRequest>
                    for GetExecutionResultByIDSvc<T> {
                        type Response = super::ExecutionResultByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExecutionResultByIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_execution_result_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExecutionResultByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AccessApi> Clone for AccessApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AccessApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccessApi> tonic::server::NamedService for AccessApiServer<T> {
        const NAME: &'static str = "flow.access.AccessAPI";
    }
}
/// Generated client implementations.
pub mod execution_data_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ExecutionDataApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExecutionDataApiClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ExecutionDataApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ExecutionDataApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ExecutionDataApiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_execution_data_by_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionDataByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExecutionDataByBlockIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.ExecutionDataAPI/GetExecutionDataByBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.ExecutionDataAPI",
                        "GetExecutionDataByBlockID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_execution_data(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeExecutionDataRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::SubscribeExecutionDataResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.ExecutionDataAPI/SubscribeExecutionData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.access.ExecutionDataAPI",
                        "SubscribeExecutionData",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeEventsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/flow.access.ExecutionDataAPI/SubscribeEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.access.ExecutionDataAPI", "SubscribeEvents"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod execution_data_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExecutionDataApiServer.
    #[async_trait]
    pub trait ExecutionDataApi: Send + Sync + 'static {
        async fn get_execution_data_by_block_id(
            &self,
            request: tonic::Request<super::GetExecutionDataByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetExecutionDataByBlockIdResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeExecutionData method.
        type SubscribeExecutionDataStream: futures_core::Stream<
                Item = std::result::Result<
                    super::SubscribeExecutionDataResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn subscribe_execution_data(
            &self,
            request: tonic::Request<super::SubscribeExecutionDataRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeExecutionDataStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeEvents method.
        type SubscribeEventsStream: futures_core::Stream<
                Item = std::result::Result<super::SubscribeEventsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_events(
            &self,
            request: tonic::Request<super::SubscribeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeEventsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ExecutionDataApiServer<T: ExecutionDataApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExecutionDataApi> ExecutionDataApiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExecutionDataApiServer<T>
    where
        T: ExecutionDataApi,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/flow.access.ExecutionDataAPI/GetExecutionDataByBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetExecutionDataByBlockIDSvc<T: ExecutionDataApi>(pub Arc<T>);
                    impl<
                        T: ExecutionDataApi,
                    > tonic::server::UnaryService<
                        super::GetExecutionDataByBlockIdRequest,
                    > for GetExecutionDataByBlockIDSvc<T> {
                        type Response = super::GetExecutionDataByBlockIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetExecutionDataByBlockIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_execution_data_by_block_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExecutionDataByBlockIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.ExecutionDataAPI/SubscribeExecutionData" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeExecutionDataSvc<T: ExecutionDataApi>(pub Arc<T>);
                    impl<
                        T: ExecutionDataApi,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeExecutionDataRequest,
                    > for SubscribeExecutionDataSvc<T> {
                        type Response = super::SubscribeExecutionDataResponse;
                        type ResponseStream = T::SubscribeExecutionDataStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeExecutionDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_execution_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeExecutionDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/flow.access.ExecutionDataAPI/SubscribeEvents" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeEventsSvc<T: ExecutionDataApi>(pub Arc<T>);
                    impl<
                        T: ExecutionDataApi,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeEventsRequest,
                    > for SubscribeEventsSvc<T> {
                        type Response = super::SubscribeEventsResponse;
                        type ResponseStream = T::SubscribeEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ExecutionDataApi> Clone for ExecutionDataApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ExecutionDataApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExecutionDataApi> tonic::server::NamedService for ExecutionDataApiServer<T> {
        const NAME: &'static str = "flow.access.ExecutionDataAPI";
    }
}
