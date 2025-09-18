// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Env, Map, Vec};

// Default cache TTL settings
const DEFAULT_CACHE_TTL: u32 = 900; // 15 minutes
const EXTENDED_CACHE_TTL: u32 = 3600; // 1 hour
const MAX_CACHE_ITEMS: u32 = 1000;

/// Generic caching utility for Soroban contracts
/// Implements a simple LRU (Least Recently Used) cache using temporary storage
pub struct Cache<'a> {
    env: &'a Env,
    namespace: &'static str,
    ttl: u32,
}

impl<'a> Cache<'a> {
    pub fn new(env: &'a Env, namespace: &'static str) -> Self {
        Self {
            env,
            namespace,
            ttl: DEFAULT_CACHE_TTL,
        }
    }

    pub fn with_ttl(env: &'a Env, namespace: &'static str, ttl: u32) -> Self {
        Self { env, namespace, ttl }
    }

    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: Clone,
        V: Clone,
    {
        let cache_key = self.make_key(key);
        self.env.storage().temporary().get(&cache_key)
    }

    pub fn set<K, V>(&self, key: &K, value: &V)
    where
        K: Clone,
        V: Clone,
    {
        let cache_key = self.make_key(key);
        self.env.storage().temporary().set(&cache_key, value);
        self.env.storage().temporary().extend_ttl(&cache_key, 0, self.ttl);
        
        // Update LRU tracking
        self.update_lru(key);
    }

    pub fn remove<K>(&self, key: &K)
    where
        K: Clone,
    {
        let cache_key = self.make_key(key);
        self.env.storage().temporary().remove(&cache_key);
        self.remove_from_lru(key);
    }

    pub fn clear(&self) {
        let lru_key = self.get_lru_key();
        if let Some(items) = self.env.storage().temporary().get::<_, Vec<Vec<u8>>>(&lru_key) {
            for key_bytes in items.iter() {
                self.env.storage().temporary().remove(&key_bytes);
            }
        }
        self.env.storage().temporary().remove(&lru_key);
    }

    fn make_key<K>(&self, key: &K) -> Vec<u8>
    where
        K: Clone,
    {
        let mut bytes = Vec::new(self.env);
        bytes.extend_from_slice(self.namespace.as_bytes());
        bytes.extend_from_slice(b":");
        bytes.extend_from_slice(&self.env.bytes_new().to_bytes(key));
        bytes
    }

    fn get_lru_key(&self) -> Vec<u8> {
        let mut bytes = Vec::new(self.env);
        bytes.extend_from_slice(self.namespace.as_bytes());
        bytes.extend_from_slice(b":lru");
        bytes
    }

    fn update_lru<K>(&self, key: &K)
    where
        K: Clone,
    {
        let lru_key = self.get_lru_key();
        let mut items = self.env
            .storage()
            .temporary()
            .get::<_, Vec<Vec<u8>>>(&lru_key)
            .unwrap_or_else(|| Vec::new(self.env));

        let key_bytes = self.make_key(key);
        
        // Remove if exists
        items.retain(|k| k != &key_bytes);
        
        // Add to front
        items.push_front(key_bytes);
        
        // Trim if too large
        while items.len() > MAX_CACHE_ITEMS as u32 {
            if let Some(old_key) = items.pop_back() {
                self.env.storage().temporary().remove(&old_key);
            }
        }

        self.env.storage().temporary().set(&lru_key, &items);
        self.env.storage().temporary().extend_ttl(&lru_key, 0, self.ttl);
    }

    fn remove_from_lru<K>(&self, key: &K)
    where
        K: Clone,
    {
        let lru_key = self.get_lru_key();
        if let Some(mut items) = self.env.storage().temporary().get::<_, Vec<Vec<u8>>>(&lru_key) {
            let key_bytes = self.make_key(key);
            items.retain(|k| k != &key_bytes);
            self.env.storage().temporary().set(&lru_key, &items);
        }
    }
}