//! An either implementation for the [`SparseTrieInterface`]

use core::fmt::Debug;

use crate::{
    blinded::BlindedProvider, LeafLookup, LeafLookupError, SparseTrieInterface, SparseTrieUpdates,
    TrieMasks,
};
use alloc::vec::Vec;
use alloy_primitives::B256;
use reth_execution_errors::SparseTrieResult;
use reth_trie_common::{Nibbles, TrieNode};

/// An `Either` type for types that implement the sparse trie interface.
#[derive(Debug)]
pub enum Either<A, B> {
    /// The left sparse trie
    Left(A),
    /// The right sparse trie
    Right(B),
}

impl<A, B> SparseTrieInterface for Either<A, B>
where
    A: SparseTrieInterface,
    B: SparseTrieInterface,
{
    fn with_root(
        self,
        root: TrieNode,
        masks: TrieMasks,
        retain_updates: bool,
    ) -> SparseTrieResult<Self> {
        match self {
            Self::Left(trie) => trie.with_root(root, masks, retain_updates).map(Self::Left),
            Self::Right(trie) => trie.with_root(root, masks, retain_updates).map(Self::Right),
        }
    }

    fn with_updates(self, retain_updates: bool) -> Self {
        match self {
            Self::Left(trie) => Self::Left(trie.with_updates(retain_updates)),
            Self::Right(trie) => Self::Right(trie.with_updates(retain_updates)),
        }
    }

    fn reserve_nodes(&mut self, additional: usize) {
        match self {
            Self::Left(trie) => trie.reserve_nodes(additional),
            Self::Right(trie) => trie.reserve_nodes(additional),
        }
    }

    fn reveal_node(
        &mut self,
        path: Nibbles,
        node: TrieNode,
        masks: TrieMasks,
    ) -> SparseTrieResult<()> {
        match self {
            Self::Left(trie) => trie.reveal_node(path, node, masks),
            Self::Right(trie) => trie.reveal_node(path, node, masks),
        }
    }

    fn update_leaf<P: BlindedProvider>(
        &mut self,
        full_path: Nibbles,
        value: Vec<u8>,
        provider: P,
    ) -> SparseTrieResult<()> {
        match self {
            Self::Left(trie) => trie.update_leaf(full_path, value, provider),
            Self::Right(trie) => trie.update_leaf(full_path, value, provider),
        }
    }

    fn remove_leaf<P: BlindedProvider>(
        &mut self,
        full_path: &Nibbles,
        provider: P,
    ) -> SparseTrieResult<()> {
        match self {
            Self::Left(trie) => trie.remove_leaf(full_path, provider),
            Self::Right(trie) => trie.remove_leaf(full_path, provider),
        }
    }

    fn root(&mut self) -> B256 {
        match self {
            Self::Left(trie) => trie.root(),
            Self::Right(trie) => trie.root(),
        }
    }

    fn update_subtrie_hashes(&mut self) {
        match self {
            Self::Left(trie) => trie.update_subtrie_hashes(),
            Self::Right(trie) => trie.update_subtrie_hashes(),
        }
    }

    fn get_leaf_value(&self, full_path: &Nibbles) -> Option<&Vec<u8>> {
        match self {
            Self::Left(trie) => trie.get_leaf_value(full_path),
            Self::Right(trie) => trie.get_leaf_value(full_path),
        }
    }

    fn find_leaf(
        &self,
        full_path: &Nibbles,
        expected_value: Option<&Vec<u8>>,
    ) -> Result<LeafLookup, LeafLookupError> {
        match self {
            Self::Left(trie) => trie.find_leaf(full_path, expected_value),
            Self::Right(trie) => trie.find_leaf(full_path, expected_value),
        }
    }

    fn take_updates(&mut self) -> SparseTrieUpdates {
        match self {
            Self::Left(trie) => trie.take_updates(),
            Self::Right(trie) => trie.take_updates(),
        }
    }

    fn wipe(&mut self) {
        match self {
            Self::Left(trie) => trie.wipe(),
            Self::Right(trie) => trie.wipe(),
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Left(trie) => trie.clear(),
            Self::Right(trie) => trie.clear(),
        }
    }
}
