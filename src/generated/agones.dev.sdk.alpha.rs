/// I am Empty
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// Store a count variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Count {
    #[prost(int64, tag = "1")]
    pub count: i64,
}
/// Store a boolean result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bool {
    #[prost(bool, tag = "1")]
    pub bool: bool,
}
/// The unique identifier for a given player.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerId {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
}
/// List of Player IDs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerIdList {
    #[prost(string, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod sdk_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " SDK service to be used in the GameServer SDK to the Pod Sidecar."]
    #[derive(Debug, Clone)]
    pub struct SdkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SdkClient<tonic::transport::Channel> {
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
    impl<T> SdkClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> SdkClient<InterceptedService<T, F>>
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
            SdkClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " PlayerConnect increases the SDK’s stored player count by one, and appends this playerID to GameServer.Status.Players.IDs."]
        #[doc = ""]
        #[doc = " GameServer.Status.Players.Count and GameServer.Status.Players.IDs are then set to update the player count and id list a second from now,"]
        #[doc = " unless there is already an update pending, in which case the update joins that batch operation."]
        #[doc = ""]
        #[doc = " PlayerConnect returns true and adds the playerID to the list of playerIDs if this playerID was not already in the"]
        #[doc = " list of connected playerIDs."]
        #[doc = ""]
        #[doc = " If the playerID exists within the list of connected playerIDs, PlayerConnect will return false, and the list of"]
        #[doc = " connected playerIDs will be left unchanged."]
        #[doc = ""]
        #[doc = " An error will be returned if the playerID was not already in the list of connected playerIDs but the player capacity for"]
        #[doc = " the server has been reached. The playerID will not be added to the list of playerIDs."]
        #[doc = ""]
        #[doc = " Warning: Do not use this method if you are manually managing GameServer.Status.Players.IDs and GameServer.Status.Players.Count"]
        #[doc = " through the Kubernetes API, as indeterminate results will occur."]
        pub async fn player_connect(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/PlayerConnect");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Decreases the SDK’s stored player count by one, and removes the playerID from GameServer.Status.Players.IDs."]
        #[doc = ""]
        #[doc = " GameServer.Status.Players.Count and GameServer.Status.Players.IDs are then set to update the player count and id list a second from now,"]
        #[doc = " unless there is already an update pending, in which case the update joins that batch operation."]
        #[doc = ""]
        #[doc = " PlayerDisconnect will return true and remove the supplied playerID from the list of connected playerIDs if the"]
        #[doc = " playerID value exists within the list."]
        #[doc = ""]
        #[doc = " If the playerID was not in the list of connected playerIDs, the call will return false, and the connected playerID list"]
        #[doc = " will be left unchanged."]
        #[doc = ""]
        #[doc = " Warning: Do not use this method if you are manually managing GameServer.status.players.IDs and GameServer.status.players.Count"]
        #[doc = " through the Kubernetes API, as indeterminate results will occur."]
        pub async fn player_disconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/PlayerDisconnect");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update the GameServer.Status.Players.Capacity value with a new capacity."]
        pub async fn set_player_capacity(
            &mut self,
            request: impl tonic::IntoRequest<super::Count>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/SetPlayerCapacity");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the current player capacity. This is always accurate from what has been set through this SDK,"]
        #[doc = " even if the value has yet to be updated on the GameServer status resource."]
        #[doc = ""]
        #[doc = " If GameServer.Status.Players.Capacity is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value."]
        pub async fn get_player_capacity(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Count>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/GetPlayerCapacity");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the current player count. This is always accurate from what has been set through this SDK,"]
        #[doc = " even if the value has yet to be updated on the GameServer status resource."]
        #[doc = ""]
        #[doc = " If GameServer.Status.Players.Count is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value."]
        pub async fn get_player_count(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::Count>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/GetPlayerCount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns if the playerID is currently connected to the GameServer. This is always accurate from what has been set through this SDK,"]
        #[doc = " even if the value has yet to be updated on the GameServer status resource."]
        #[doc = ""]
        #[doc = " If GameServer.Status.Players.IDs is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to determine connected status."]
        pub async fn is_player_connected(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerId>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/agones.dev.sdk.alpha.SDK/IsPlayerConnected");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of the currently connected player ids. This is always accurate from what has been set through this SDK,"]
        #[doc = " even if the value has yet to be updated on the GameServer status resource."]
        #[doc = ""]
        #[doc = " If GameServer.Status.Players.IDs is set manually through the Kubernetes API, use SDK.GameServer() or SDK.WatchGameServer() instead to view this value."]
        pub async fn get_connected_players(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::PlayerIdList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agones.dev.sdk.alpha.SDK/GetConnectedPlayers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
