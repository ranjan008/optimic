//! Core Types for Optimic Protocol
//! 
//! This module defines the fundamental data structures used throughout
//! the Optimic blockchain, including accounts, transactions, orders, and options.

use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::{DateTime, Utc};

/// Account address type
pub type AccAddress = String;

/// Validator address type  
pub type ValAddress = String;

/// Asset identifier
pub type AssetId = String;

/// Market identifier
pub type MarketId = String;

/// Order identifier
pub type OrderId = u64;

/// Option identifier
pub type OptionId = String;

/// Trade identifier
pub type TradeId = u64;

/// Price type (using string to avoid floating point precision issues)
pub type Price = String;

/// Timestamp type
pub type Timestamp = DateTime<Utc>;

/// Big integer type for token amounts
pub type Uint128 = String;

/// Signed integer type for P&L calculations
pub type Int128 = String;

/// Account structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: AccAddress,
    pub public_key: Option<String>,
    pub account_number: u64,
    pub sequence: u64,
    pub balances: std::collections::HashMap<AssetId, Uint128>,
}

/// Validator structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub operator_address: ValAddress,
    pub consensus_pubkey: String,
    pub jailed: bool,
    pub status: BondStatus,
    pub tokens: Uint128,
    pub delegator_shares: Uint128,
    pub commission: Commission,
}

/// Bond status for validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BondStatus {
    Unbonded,
    Unbonding,
    Bonded,
}

/// Commission structure for validators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commission {
    pub rate: String,           // Commission rate (e.g., "0.10" for 10%)
    pub max_rate: String,       // Maximum commission rate
    pub max_change_rate: String, // Maximum daily change rate
}

/// Order structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub trader: AccAddress,
    pub market: MarketId,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Uint128,
    pub price: Option<Price>,
    pub filled_quantity: Uint128,
    pub status: OrderStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub time_in_force: TimeInForce,
}

/// Order side (Buy or Sell)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Submitted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

/// Time in force for orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC, // Good Till Cancelled
    IOC, // Immediate Or Cancel
    FOK, // Fill Or Kill
    GTD(Timestamp), // Good Till Date
}

/// Market structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: MarketId,
    pub base_asset: AssetId,
    pub quote_asset: AssetId,
    pub min_order_size: Uint128,
    pub tick_size: Price,
    pub market_type: MarketType,
    pub status: MarketStatus,
}

/// Market type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketType {
    Spot,
    Options,
}

/// Market status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketStatus {
    Active,
    Suspended,
    Closed,
}

/// Option contract structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    pub id: OptionId,
    pub underlying_asset: AssetId,
    pub strike_price: Price,
    pub expiry_date: Timestamp,
    pub option_type: OptionType,
    pub style: OptionStyle,
    pub settlement_type: SettlementType,
    pub status: OptionStatus,
}

/// Option type (Call or Put)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

/// Option style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionStyle {
    European,
    American,
}

/// Settlement type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementType {
    Cash,
    Physical,
}

/// Option status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionStatus {
    Active,
    Expired,
    Exercised,
    Assigned,
}

/// Trade structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: TradeId,
    pub market_id: MarketId,
    pub buyer: AccAddress,
    pub seller: AccAddress,
    pub quantity: Uint128,
    pub price: Price,
    pub timestamp: Timestamp,
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
}

/// Portfolio structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub owner: AccAddress,
    pub balances: std::collections::HashMap<AssetId, Balance>,
    pub positions: std::collections::HashMap<MarketId, Position>,
    pub orders: Vec<OrderId>,
    pub unrealized_pnl: Int128,
    pub realized_pnl: Int128,
}

/// Balance structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub total: Uint128,
    pub available: Uint128,
    pub locked: Uint128,
}

/// Position structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub market_id: MarketId,
    pub quantity: Int128, // Positive for long, negative for short
    pub average_price: Price,
    pub unrealized_pnl: Int128,
    pub last_update: Timestamp,
}

/// Greeks for options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}

/// Display implementations
impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "Market"),
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Stop => write!(f, "Stop"),
            OrderType::StopLimit => write!(f, "Stop Limit"),
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Submitted => write!(f, "Submitted"),
            OrderStatus::PartiallyFilled => write!(f, "Partially Filled"),
            OrderStatus::Filled => write!(f, "Filled"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
            OrderStatus::Rejected => write!(f, "Rejected"),
            OrderStatus::Expired => write!(f, "Expired"),
        }
    }
}
