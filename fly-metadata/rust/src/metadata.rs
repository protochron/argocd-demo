// This file is @generated by wasmcloud/weld-codegen 0.7.0.
// It is not intended for manual editing.
// namespace: protochron.metadata

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetResponse {
    #[serde(rename = "appName")]
    #[serde(default)]
    pub app_name: String,
    #[serde(rename = "machineID")]
    #[serde(default)]
    pub machine_id: String,
    #[serde(rename = "privateIP")]
    #[serde(default)]
    pub private_ip: String,
    pub region: Region,
}

// Encode GetResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_get_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(4)?;
    e.str("appName")?;
    e.str(&val.app_name)?;
    e.str("machineID")?;
    e.str(&val.machine_id)?;
    e.str("privateIP")?;
    e.str(&val.private_ip)?;
    e.str("region")?;
    encode_region(e, &val.region)?;
    Ok(())
}

// Decode GetResponse from cbor input stream
#[doc(hidden)]
pub fn decode_get_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetResponse, RpcError> {
    let __result =
        {
            let mut app_name: Option<String> = None;
            let mut machine_id: Option<String> = None;
            let mut private_ip: Option<String> = None;
            let mut region: Option<Region> = None;

            let is_array = match d.datatype()? {
                wasmbus_rpc::cbor::Type::Array => true,
                wasmbus_rpc::cbor::Type::Map => false,
                _ => {
                    return Err(RpcError::Deser(
                        "decoding struct GetResponse, expected array or map".to_string(),
                    ))
                }
            };
            if is_array {
                let len = d.fixed_array()?;
                for __i in 0..(len as usize) {
                    match __i {
                        0 => app_name = Some(d.str()?.to_string()),
                        1 => machine_id = Some(d.str()?.to_string()),
                        2 => private_ip = Some(d.str()?.to_string()),
                        3 => {
                            region = Some(decode_region(d).map_err(|e| {
                                format!("decoding 'protochron.metadata#Region': {}", e)
                            })?)
                        }
                        _ => d.skip()?,
                    }
                }
            } else {
                let len = d.fixed_map()?;
                for __i in 0..(len as usize) {
                    match d.str()? {
                        "appName" => app_name = Some(d.str()?.to_string()),
                        "machineID" => machine_id = Some(d.str()?.to_string()),
                        "privateIP" => private_ip = Some(d.str()?.to_string()),
                        "region" => {
                            region = Some(decode_region(d).map_err(|e| {
                                format!("decoding 'protochron.metadata#Region': {}", e)
                            })?)
                        }
                        _ => d.skip()?,
                    }
                }
            }
            GetResponse {
                app_name: if let Some(__x) = app_name {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field GetResponse.app_name (#0)".to_string(),
                    ));
                },

                machine_id: if let Some(__x) = machine_id {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field GetResponse.machine_id (#1)".to_string(),
                    ));
                },

                private_ip: if let Some(__x) = private_ip {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field GetResponse.private_ip (#2)".to_string(),
                    ));
                },

                region: if let Some(__x) = region {
                    __x
                } else {
                    return Err(RpcError::Deser(
                        "missing field GetResponse.region (#3)".to_string(),
                    ));
                },
            }
        };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Region {
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

// Encode Region as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_region<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Region,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("city")?;
    e.str(&val.city)?;
    e.str("code")?;
    e.str(&val.code)?;
    e.str("name")?;
    e.str(&val.name)?;
    Ok(())
}

// Decode Region from cbor input stream
#[doc(hidden)]
pub fn decode_region(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Region, RpcError> {
    let __result = {
        let mut city: Option<String> = None;
        let mut code: Option<String> = None;
        let mut name: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Region, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => city = Some(d.str()?.to_string()),
                    1 => code = Some(d.str()?.to_string()),
                    2 => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "city" => city = Some(d.str()?.to_string()),
                    "code" => code = Some(d.str()?.to_string()),
                    "name" => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Region {
            city: if let Some(__x) = city {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Region.city (#0)".to_string(),
                ));
            },

            code: if let Some(__x) = code {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Region.code (#1)".to_string(),
                ));
            },

            name: if let Some(__x) = name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Region.name (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// The Metadata service has a single method, Get, which
/// retrieves a Fly Machine's metadata
/// wasmbus.contractId: protochron:fly_metadata
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait Metadata {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "protochron:fly_metadata"
    }
    /// Get a Fly Machine's metadata
    async fn get(&self, ctx: &Context) -> RpcResult<GetResponse>;
}

/// MetadataReceiver receives messages defined in the Metadata service trait
/// The Metadata service has a single method, Get, which
/// retrieves a Fly Machine's metadata
#[doc(hidden)]
#[async_trait]
pub trait MetadataReceiver: MessageDispatch + Metadata {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Get" => {
                let resp = Metadata::get(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Metadata::{}",
                message.method
            ))),
        }
    }
}

/// MetadataSender sends messages to a Metadata service
/// The Metadata service has a single method, Get, which
/// retrieves a Fly Machine's metadata
/// client for sending Metadata messages
#[derive(Clone, Debug)]
pub struct MetadataSender<T: Transport> {
    transport: T,
}

impl<T: Transport> MetadataSender<T> {
    /// Constructs a MetadataSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> MetadataSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl MetadataSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl MetadataSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Metadata provider
    /// implementing the 'protochron:fly_metadata' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "protochron:fly_metadata",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Metadata provider
    /// implementing the 'protochron:fly_metadata' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "protochron:fly_metadata",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Metadata for MetadataSender<T> {
    #[allow(unused)]
    /// Get a Fly Machine's metadata
    async fn get(&self, ctx: &Context) -> RpcResult<GetResponse> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Metadata.Get",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: GetResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': GetResponse", e)))?;
        Ok(value)
    }
}
