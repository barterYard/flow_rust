// @generated
/// Generated client implementations.
pub mod execution_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ExecutionApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExecutionApiClient<tonic::transport::Channel> {
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
    impl<T> ExecutionApiClient<T>
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
        ) -> ExecutionApiClient<InterceptedService<T, F>>
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
            ExecutionApiClient::new(InterceptedService::new(inner, interceptor))
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
                "/flow.execution.ExecutionAPI/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("flow.execution.ExecutionAPI", "Ping"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_at_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountAtBlockIdResponse>,
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
                "/flow.execution.ExecutionAPI/GetAccountAtBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.execution.ExecutionAPI", "GetAccountAtBlockID"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_script_at_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteScriptAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptAtBlockIdResponse>,
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
                "/flow.execution.ExecutionAPI/ExecuteScriptAtBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "ExecuteScriptAtBlockID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_events_for_block_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventsForBlockIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventsForBlockIDsResponse>,
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
                "/flow.execution.ExecutionAPI/GetEventsForBlockIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetEventsForBlockIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultResponse>,
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
                "/flow.execution.ExecutionAPI/GetTransactionResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetTransactionResult",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_result_by_index(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultResponse>,
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
                "/flow.execution.ExecutionAPI/GetTransactionResultByIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetTransactionResultByIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_results_by_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultsResponse>,
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
                "/flow.execution.ExecutionAPI/GetTransactionResultsByBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetTransactionResultsByBlockID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_register_at_block_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRegisterAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRegisterAtBlockIdResponse>,
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
                "/flow.execution.ExecutionAPI/GetRegisterAtBlockID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetRegisterAtBlockID",
                    ),
                );
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
                "/flow.execution.ExecutionAPI/GetLatestBlockHeader",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "flow.execution.ExecutionAPI",
                        "GetLatestBlockHeader",
                    ),
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
                "/flow.execution.ExecutionAPI/GetBlockHeaderByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("flow.execution.ExecutionAPI", "GetBlockHeaderByID"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod execution_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExecutionApiServer.
    #[async_trait]
    pub trait ExecutionApi: Send + Sync + 'static {
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> std::result::Result<tonic::Response<super::PingResponse>, tonic::Status>;
        async fn get_account_at_block_id(
            &self,
            request: tonic::Request<super::GetAccountAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountAtBlockIdResponse>,
            tonic::Status,
        >;
        async fn execute_script_at_block_id(
            &self,
            request: tonic::Request<super::ExecuteScriptAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteScriptAtBlockIdResponse>,
            tonic::Status,
        >;
        async fn get_events_for_block_i_ds(
            &self,
            request: tonic::Request<super::GetEventsForBlockIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventsForBlockIDsResponse>,
            tonic::Status,
        >;
        async fn get_transaction_result(
            &self,
            request: tonic::Request<super::GetTransactionResultRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultResponse>,
            tonic::Status,
        >;
        async fn get_transaction_result_by_index(
            &self,
            request: tonic::Request<super::GetTransactionByIndexRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultResponse>,
            tonic::Status,
        >;
        async fn get_transaction_results_by_block_id(
            &self,
            request: tonic::Request<super::GetTransactionsByBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionResultsResponse>,
            tonic::Status,
        >;
        async fn get_register_at_block_id(
            &self,
            request: tonic::Request<super::GetRegisterAtBlockIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRegisterAtBlockIdResponse>,
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
    }
    #[derive(Debug)]
    pub struct ExecutionApiServer<T: ExecutionApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExecutionApi> ExecutionApiServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExecutionApiServer<T>
    where
        T: ExecutionApi,
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
                "/flow.execution.ExecutionAPI/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<T: ExecutionApi> tonic::server::UnaryService<super::PingRequest>
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
                "/flow.execution.ExecutionAPI/GetAccountAtBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountAtBlockIDSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetAccountAtBlockIdRequest>
                    for GetAccountAtBlockIDSvc<T> {
                        type Response = super::GetAccountAtBlockIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountAtBlockIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_at_block_id(request).await
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
                        let method = GetAccountAtBlockIDSvc(inner);
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
                "/flow.execution.ExecutionAPI/ExecuteScriptAtBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteScriptAtBlockIDSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::ExecuteScriptAtBlockIdRequest>
                    for ExecuteScriptAtBlockIDSvc<T> {
                        type Response = super::ExecuteScriptAtBlockIdResponse;
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
                "/flow.execution.ExecutionAPI/GetEventsForBlockIDs" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventsForBlockIDsSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetEventsForBlockIDsRequest>
                    for GetEventsForBlockIDsSvc<T> {
                        type Response = super::GetEventsForBlockIDsResponse;
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
                "/flow.execution.ExecutionAPI/GetTransactionResult" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetTransactionResultRequest>
                    for GetTransactionResultSvc<T> {
                        type Response = super::GetTransactionResultResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionResultRequest>,
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
                "/flow.execution.ExecutionAPI/GetTransactionResultByIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultByIndexSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetTransactionByIndexRequest>
                    for GetTransactionResultByIndexSvc<T> {
                        type Response = super::GetTransactionResultResponse;
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
                "/flow.execution.ExecutionAPI/GetTransactionResultsByBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionResultsByBlockIDSvc<T: ExecutionApi>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetTransactionsByBlockIdRequest>
                    for GetTransactionResultsByBlockIDSvc<T> {
                        type Response = super::GetTransactionResultsResponse;
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
                "/flow.execution.ExecutionAPI/GetRegisterAtBlockID" => {
                    #[allow(non_camel_case_types)]
                    struct GetRegisterAtBlockIDSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
                    > tonic::server::UnaryService<super::GetRegisterAtBlockIdRequest>
                    for GetRegisterAtBlockIDSvc<T> {
                        type Response = super::GetRegisterAtBlockIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRegisterAtBlockIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_register_at_block_id(request).await
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
                        let method = GetRegisterAtBlockIDSvc(inner);
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
                "/flow.execution.ExecutionAPI/GetLatestBlockHeader" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestBlockHeaderSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
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
                "/flow.execution.ExecutionAPI/GetBlockHeaderByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockHeaderByIDSvc<T: ExecutionApi>(pub Arc<T>);
                    impl<
                        T: ExecutionApi,
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
    impl<T: ExecutionApi> Clone for ExecutionApiServer<T> {
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
    impl<T: ExecutionApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExecutionApi> tonic::server::NamedService for ExecutionApiServer<T> {
        const NAME: &'static str = "flow.execution.ExecutionAPI";
    }
}
