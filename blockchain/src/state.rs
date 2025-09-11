//! State Management Module
//! 
//! This module handles all blockchain state operations including
//! storage, retrieval, and state transitions.

use crate::types::*;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};

/// State manager for blockchain data
pub struct StateManager {
    /// In-memory state cache
    cache: StateCache,
    
    /// Persistent storage backend
    storage: Box<dyn StateStorage>,
}

/// State cache for frequently accessed data
#[derive(Debug, Clone)]
struct StateCache {
    accounts: HashMap<AccAddress, Account>,
    orders: HashMap<OrderId, Order>,
    markets: HashMap<MarketId, Market>,
    portfolios: HashMap<AccAddress, Portfolio>,
}

/// Trait for state storage backends
pub trait StateStorage: Send + Sync {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()>;
    fn delete(&mut self, key: &[u8]) -> Result<()>;
    fn commit(&mut self) -> Result<Vec<u8>>; // Returns state root hash
}

/// In-memory storage implementation (for testing)
pub struct MemoryStorage {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new(storage: Box<dyn StateStorage>) -> Self {
        Self {
            cache: StateCache {
                accounts: HashMap::new(),
                orders: HashMap::new(),
                markets: HashMap::new(),
                portfolios: HashMap::new(),
            },
            storage,
        }
    }

    /// Get account by address
    pub fn get_account(&self, address: &AccAddress) -> Result<Option<Account>> {
        // Check cache first
        if let Some(account) = self.cache.accounts.get(address) {
            return Ok(Some(account.clone()));
        }

        // Check storage
        let key = format!("account:{}", address);
        if let Some(data) = self.storage.get(key.as_bytes())? {
            let account: Account = serde_json::from_slice(&data)?;
            return Ok(Some(account));
        }

        Ok(None)
    }

    /// Set account
    pub fn set_account(&mut self, account: Account) -> Result<()> {
        let address = account.address.clone();
        
        // Update cache
        self.cache.accounts.insert(address.clone(), account.clone());
        
        // Update storage
        let key = format!("account:{}", address);
        let value = serde_json::to_vec(&account)?;
        self.storage.set(key.as_bytes(), value)?;
        
        info!("Account updated: {}", address);
        Ok(())
    }

    /// Get order by ID
    pub fn get_order(&self, order_id: OrderId) -> Result<Option<Order>> {
        // Check cache first
        if let Some(order) = self.cache.orders.get(&order_id) {
            return Ok(Some(order.clone()));
        }

        // Check storage
        let key = format!("order:{}", order_id);
        if let Some(data) = self.storage.get(key.as_bytes())? {
            let order: Order = serde_json::from_slice(&data)?;
            return Ok(Some(order));
        }

        Ok(None)
    }

    /// Set order
    pub fn set_order(&mut self, order: Order) -> Result<()> {
        let order_id = order.id;
        
        // Update cache
        self.cache.orders.insert(order_id, order.clone());
        
        // Update storage
        let key = format!("order:{}", order_id);
        let value = serde_json::to_vec(&order)?;
        self.storage.set(key.as_bytes(), value)?;
        
        info!("Order updated: {}", order_id);
        Ok(())
    }

    /// Get market by ID
    pub fn get_market(&self, market_id: &MarketId) -> Result<Option<Market>> {
        // Check cache first
        if let Some(market) = self.cache.markets.get(market_id) {
            return Ok(Some(market.clone()));
        }

        // Check storage
        let key = format!("market:{}", market_id);
        if let Some(data) = self.storage.get(key.as_bytes())? {
            let market: Market = serde_json::from_slice(&data)?;
            return Ok(Some(market));
        }

        Ok(None)
    }

    /// Set market
    pub fn set_market(&mut self, market: Market) -> Result<()> {
        let market_id = market.id.clone();
        
        // Update cache
        self.cache.markets.insert(market_id.clone(), market.clone());
        
        // Update storage
        let key = format!("market:{}", market_id);
        let value = serde_json::to_vec(&market)?;
        self.storage.set(key.as_bytes(), value)?;
        
        info!("Market updated: {}", market_id);
        Ok(())
    }

    /// Get portfolio by address
    pub fn get_portfolio(&self, address: &AccAddress) -> Result<Option<Portfolio>> {
        // Check cache first
        if let Some(portfolio) = self.cache.portfolios.get(address) {
            return Ok(Some(portfolio.clone()));
        }

        // Check storage
        let key = format!("portfolio:{}", address);
        if let Some(data) = self.storage.get(key.as_bytes())? {
            let portfolio: Portfolio = serde_json::from_slice(&data)?;
            return Ok(Some(portfolio));
        }

        Ok(None)
    }

    /// Set portfolio
    pub fn set_portfolio(&mut self, portfolio: Portfolio) -> Result<()> {
        let address = portfolio.owner.clone();
        
        // Update cache
        self.cache.portfolios.insert(address.clone(), portfolio.clone());
        
        // Update storage
        let key = format!("portfolio:{}", address);
        let value = serde_json::to_vec(&portfolio)?;
        self.storage.set(key.as_bytes(), value)?;
        
        info!("Portfolio updated: {}", address);
        Ok(())
    }

    /// Commit all changes to storage
    pub fn commit(&mut self) -> Result<Vec<u8>> {
        info!("Committing state changes");
        self.storage.commit()
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.accounts.clear();
        self.cache.orders.clear();
        self.cache.markets.clear();
        self.cache.portfolios.clear();
        info!("State cache cleared");
    }
}

impl MemoryStorage {
    /// Create a new in-memory storage
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl StateStorage for MemoryStorage {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.data.get(key).cloned())
    }

    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.data.insert(key.to_vec(), value);
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.data.remove(key);
        Ok(())
    }

    fn commit(&mut self) -> Result<Vec<u8>> {
        // Calculate a simple hash of all data
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        let mut keys: Vec<_> = self.data.keys().collect();
        keys.sort();
        
        for key in keys {
            if let Some(value) = self.data.get(key) {
                hasher.update(key);
                hasher.update(value);
            }
        }
        
        Ok(hasher.finalize().to_vec())
    }
}
