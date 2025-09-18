#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_cache_basic_operations() {
        let env = Env::default();
        let cache = Cache::new(&env, "test");
        let key = String::from_str(&env, "test_key");
        let value = String::from_str(&env, "test_value");

        // Test set and get
        cache.set(&key, &value);
        let retrieved = cache.get::<String, String>(&key);
        assert_eq!(retrieved, Some(value.clone()));

        // Test remove
        cache.remove(&key);
        let retrieved = cache.get::<String, String>(&key);
        assert_eq!(retrieved, None);
    }

    #[test]
    fn test_cache_lru_eviction() {
        let env = Env::default();
        let cache = Cache::new(&env, "test");

        // Fill cache beyond capacity
        for i in 0..MAX_CACHE_ITEMS + 1 {
            let key = String::from_str(&env, &format!("key_{}", i));
            let value = String::from_str(&env, &format!("value_{}", i));
            cache.set(&key, &value);
        }

        // First item should be evicted
        let first_key = String::from_str(&env, "key_0");
        let retrieved = cache.get::<String, String>(&first_key);
        assert_eq!(retrieved, None);

        // Last item should still be present
        let last_key = String::from_str(&env, &format!("key_{}", MAX_CACHE_ITEMS));
        let retrieved = cache.get::<String, String>(&last_key);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_cache_ttl() {
        let env = Env::default();
        let cache = Cache::with_ttl(&env, "test", 1); // 1 second TTL
        let key = String::from_str(&env, "test_key");
        let value = String::from_str(&env, "test_value");

        cache.set(&key, &value);
        
        // Advance time
        env.ledger().set_timestamp(env.ledger().timestamp() + 2);

        let retrieved = cache.get::<String, String>(&key);
        assert_eq!(retrieved, None);
    }

    #[test]
    fn test_cache_clear() {
        let env = Env::default();
        let cache = Cache::new(&env, "test");

        // Add multiple items
        for i in 0..5 {
            let key = String::from_str(&env, &format!("key_{}", i));
            let value = String::from_str(&env, &format!("value_{}", i));
            cache.set(&key, &value);
        }

        // Clear cache
        cache.clear();

        // Verify all items are removed
        for i in 0..5 {
            let key = String::from_str(&env, &format!("key_{}", i));
            let retrieved = cache.get::<String, String>(&key);
            assert_eq!(retrieved, None);
        }
    }
}