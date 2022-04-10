#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayTuneRequest {
    /// The tune to be played
    #[prost(message, optional, tag = "1")]
    pub tune_description: ::core::option::Option<TuneDescription>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayTuneResponse {
    #[prost(message, optional, tag = "1")]
    pub tune_result: ::core::option::Option<TuneResult>,
}
/// Tune description, containing song elements and tempo.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TuneDescription {
    /// The list of song elements (notes, pauses, ...) to be played
    #[prost(enumeration = "SongElement", repeated, tag = "1")]
    pub song_elements: ::prost::alloc::vec::Vec<i32>,
    /// The tempo of the song (range: 32 - 255)
    #[prost(int32, tag = "2")]
    pub tempo: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TuneResult {
    /// Result enum value
    #[prost(enumeration = "tune_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TuneResult`.
pub mod tune_result {
    /// Possible results returned for tune requests.
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Invalid tempo (range: 32 - 255)
        InvalidTempo = 2,
        /// Invalid tune: encoded string must be at most 247 chars
        TuneTooLong = 3,
        /// Failed to send the request
        Error = 4,
        /// No system connected
        NoSystem = 5,
    }
}
/// An element of the tune
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SongElement {
    /// After this element, start playing legato
    StyleLegato = 0,
    /// After this element, start playing normal
    StyleNormal = 1,
    /// After this element, start playing staccato
    StyleStaccato = 2,
    /// After this element, set the note duration to 1
    Duration1 = 3,
    /// After this element, set the note duration to 2
    Duration2 = 4,
    /// After this element, set the note duration to 4
    Duration4 = 5,
    /// After this element, set the note duration to 8
    Duration8 = 6,
    /// After this element, set the note duration to 16
    Duration16 = 7,
    /// After this element, set the note duration to 32
    Duration32 = 8,
    /// Play note A
    NoteA = 9,
    /// Play note B
    NoteB = 10,
    /// Play note C
    NoteC = 11,
    /// Play note D
    NoteD = 12,
    /// Play note E
    NoteE = 13,
    /// Play note F
    NoteF = 14,
    /// Play note G
    NoteG = 15,
    /// Play a rest
    NotePause = 16,
    /// After this element, sharp the note (half a step up)
    Sharp = 17,
    /// After this element, flat the note (half a step down)
    Flat = 18,
    /// After this element, shift the note 1 octave up
    OctaveUp = 19,
    /// After this element, shift the note 1 octave down
    OctaveDown = 20,
}
#[doc = r" Generated client implementations."]
pub mod tune_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable creating and sending a tune to be played on the system."]
    #[derive(Debug, Clone)]
    pub struct TuneServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TuneServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TuneServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TuneServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TuneServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Send a tune to be played by the system."]
        pub async fn play_tune(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayTuneRequest>,
        ) -> Result<tonic::Response<super::PlayTuneResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.tune.TuneService/PlayTune");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod tune_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TuneServiceServer."]
    #[async_trait]
    pub trait TuneService: Send + Sync + 'static {
        #[doc = " Send a tune to be played by the system."]
        async fn play_tune(
            &self,
            request: tonic::Request<super::PlayTuneRequest>,
        ) -> Result<tonic::Response<super::PlayTuneResponse>, tonic::Status>;
    }
    #[doc = " Enable creating and sending a tune to be played on the system."]
    #[derive(Debug)]
    pub struct TuneServiceServer<T: TuneService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TuneService> TuneServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TuneServiceServer<T>
    where
        T: TuneService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mavsdk.rpc.tune.TuneService/PlayTune" => {
                    #[allow(non_camel_case_types)]
                    struct PlayTuneSvc<T: TuneService>(pub Arc<T>);
                    impl<T: TuneService> tonic::server::UnaryService<super::PlayTuneRequest> for PlayTuneSvc<T> {
                        type Response = super::PlayTuneResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayTuneRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).play_tune(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayTuneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TuneService> Clone for TuneServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TuneService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TuneService> tonic::transport::NamedService for TuneServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.tune.TuneService";
    }
}
