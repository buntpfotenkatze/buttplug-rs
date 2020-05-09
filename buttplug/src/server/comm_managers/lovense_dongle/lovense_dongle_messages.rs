use serde::{Deserialize, Serialize};
use serde_repr::*;

#[repr(u16)]
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug)]
pub enum LovenseDongleResultCode {
  CommandSuccess = 200,
  DeviceConnectFailed = 201,
  DeviceConnectSuccess = 202,
  SearchStarted = 205,
  SearchStopped = 206,
  DeviceDisconnected = 403,
  DongleScanningInterruption = 501,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum LovenseDongleMessageType {
  #[serde(rename = "usb")]
  USB,
  #[serde(rename = "toy")]
  Toy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum LovenseDongleMessageFunc {
  #[serde(rename = "search")]
  Search,
  #[serde(rename = "stopSearch")]
  StopSearch,
  #[serde(rename = "status")]
  IncomingStatus,
  #[serde(rename = "command")]
  Command,
  #[serde(rename = "toyData")]
  ToyData,
  #[serde(rename = "connect")]
  Connect,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LovenseDongleOutgoingMessage {
  #[serde(rename = "type")]
  pub message_type: LovenseDongleMessageType,
  pub func: LovenseDongleMessageFunc,
  #[serde(skip_serializing_if="Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "cmd", skip_serializing_if="Option::is_none")]
  pub command: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub eager: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LovenseDongleIncomingData {
  #[serde(skip_serializing_if="Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub data: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub status: Option<LovenseDongleResultCode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LovenseDongleIncomingMessage {
  #[serde(rename = "type")]
  pub message_type: LovenseDongleMessageType,
  pub func: LovenseDongleMessageFunc,
  #[serde(skip_serializing_if="Option::is_none")]
  pub id: Option<String>,
  #[serde(rename = "cmd", skip_serializing_if="Option::is_none")]
  pub command: Option<String>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub eager: Option<u32>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub result: Option<LovenseDongleResultCode>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub data: Option<LovenseDongleIncomingData>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub message: Option<String>,
}
