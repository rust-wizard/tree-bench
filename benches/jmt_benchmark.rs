use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use jmt::{JellyfishMerkleTree, storage::{TreeReader, TreeWriter, NodeBatch}, KeyHash, Version};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow;
use bincode;
use blake2::Blake2s256;
use sha2::{Digest, Sha256};

struct InMemoryTreeStore {
    store: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl InMemoryTreeStore {
    fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl TreeReader for InMemoryTreeStore {
    fn get_node_option(
        &self,
        node_key: &jmt::storage::NodeKey,
    ) -> Result<Option<jmt::storage::Node>, anyhow::Error> {
        let store = self.store.read().map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?;
        let key_bytes = bincode::serialize(node_key)?;
        match store.get(&key_bytes) {
            Some(bytes) => Ok(Some(bincode::deserialize(bytes)?)),
            None => Ok(None),
        }
    }

    fn get_value_option(
        &self,
        _version: Version,
        key_hash: KeyHash,
    ) -> Result<Option<Vec<u8>>, anyhow::Error> {
        let store = self.store.read().map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?;
        match store.get(&key_hash.0.to_vec()) {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    fn get_rightmost_leaf(
        &self,
    ) -> Result<Option<(jmt::storage::NodeKey, jmt::storage::LeafNode)>, anyhow::Error> {
        // Simplified implementation
        Ok(None)
    }
}

impl TreeWriter for InMemoryTreeStore {
    fn write_node_batch(&self, node_batch: &NodeBatch) -> Result<(), anyhow::Error> {
        let mut store = self.store.write().map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?;
        
        for (node_key, node) in node_batch.nodes() {
            let key_bytes = bincode::serialize(node_key)?;
            let node_bytes = bincode::serialize(node)?;
            store.insert(key_bytes, node_bytes);
        }
        
        for ((_, key_hash), value_option) in node_batch.values() {
            if let Some(value) = value_option {
                store.insert(key_hash.0.to_vec(), value.clone());
            } else {
                store.remove(&key_hash.0.to_vec());
            }
        }
        
        Ok(())
    }
}

fn jmt_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("jmt_insert");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("insert", *size),
            size,
            |b, &size| {
                b.iter(|| {
                    // Create a fresh tree store for each benchmark iteration
                    let store = InMemoryTreeStore::new();
                    let jmt: JellyfishMerkleTree<'_, InMemoryTreeStore, Blake2s256> = JellyfishMerkleTree::new(&store);
                    let pairs: Vec<(Vec<u8>, Vec<u8>)> = (0..size)
                        .map(|i| (format!("key{}", i).into_bytes(), format!("value{}", i).into_bytes()))
                        .collect();
                    
                    for (key, value) in pairs {
                        let mut hasher = Sha256::new();
                        hasher.update(&key);
                        let hash_bytes: [u8; 32] = hasher.finalize().into();
                        let key_hash = KeyHash(hash_bytes);
                        let (_new_root, _proof) = jmt.put_value_set(
                            vec![(key_hash, Some(value))],
                            0
                        ).unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

fn jmt_get_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("jmt_get");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("get", *size),
            size,
            |b, &size| {
                b.iter(|| {
                    // Create a JMT with pre-populated data
                    let store = InMemoryTreeStore::new();
                    let jmt: JellyfishMerkleTree<'_, InMemoryTreeStore, Blake2s256> = JellyfishMerkleTree::new(&store);
                    
                    // Pre-populate the tree
                    let keys: Vec<Vec<u8>> = (0..size).map(|i| format!("key{}", i).into_bytes()).collect();
                    let values: Vec<Vec<u8>> = (0..size).map(|i| format!("value{}", i).into_bytes()).collect();
                    
                    let key_val_pairs: Vec<_> = keys.iter().cloned()
                        .zip(values.iter().cloned().map(Some))
                        .map(|(k, v)| {
                            let mut hasher = Sha256::new();
                            hasher.update(&k);
                            let hash_bytes: [u8; 32] = hasher.finalize().into();
                            (KeyHash(hash_bytes), v)
                        })
                        .collect();
                    
                    let (_root, _batch) = jmt.put_value_set(
                        key_val_pairs,
                        0
                    ).unwrap();

                    for key in &keys {
                        let mut hasher = Sha256::new();
                        hasher.update(key);
                        let hash_bytes: [u8; 32] = hasher.finalize().into();
                        let key_hash = KeyHash(hash_bytes);
                        let _result = jmt.get_with_proof(key_hash, 0).unwrap();
                    }
                });
            },
        );
    }
    group.finish();
}

fn jmt_update_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("jmt_update");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("update", *size),
            size,
            |b, &size| {
                b.iter(|| {
                    // Create a JMT with pre-populated data
                    let store = InMemoryTreeStore::new();
                    let jmt: JellyfishMerkleTree<'_, InMemoryTreeStore, Blake2s256> = JellyfishMerkleTree::new(&store);
                    
                    // Pre-populate the tree
                    let keys: Vec<Vec<u8>> = (0..size).map(|i| format!("key{}", i).into_bytes()).collect();
                    let values: Vec<Vec<u8>> = (0..size).map(|i| format!("value{}", i).into_bytes()).collect();
                    
                    let key_val_pairs: Vec<_> = keys.iter().cloned()
                        .zip(values.iter().cloned().map(Some))
                        .map(|(k, v)| {
                            let mut hasher = Sha256::new();
                            hasher.update(&k);
                            let hash_bytes: [u8; 32] = hasher.finalize().into();
                            (KeyHash(hash_bytes), v)
                        })
                        .collect();
                    
                    let (_root, _batch) = jmt.put_value_set(
                        key_val_pairs,
                        0
                    ).unwrap();

                    let update_pairs: Vec<_> = keys.iter().cloned()
                        .zip((0..size).map(|i| Some(format!("updated_value{}", i).into_bytes())))
                        .map(|(k, v)| {
                            let mut hasher = Sha256::new();
                            hasher.update(&k);
                            let hash_bytes: [u8; 32] = hasher.finalize().into();
                            (KeyHash(hash_bytes), v)
                        })
                        .collect();
                        
                    let (_new_root, _batch) = jmt.put_value_set(
                        update_pairs,
                        1
                    ).unwrap();
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, jmt_insert_benchmark, jmt_get_benchmark, jmt_update_benchmark);
criterion_main!(benches);
