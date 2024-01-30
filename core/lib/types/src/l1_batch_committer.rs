use zksync_basic_types::{ethabi::Token, U256};

use crate::{commitment::L1BatchWithMetadata, utils};

pub trait L1BatchCommitter
where
    Self: std::fmt::Debug + Send + Sync,
{
    fn l1_commit_data(&self, l1_batch_with_metadata: &L1BatchWithMetadata) -> Token;
    fn l1_commit_data_size(&self, l1_batch_with_metadata: &L1BatchWithMetadata) -> usize {
        crate::ethabi::encode(&[Token::Array(vec![
            self.l1_commit_data(l1_batch_with_metadata)
        ])])
        .len()
    }
}

#[derive(Debug, Clone)]
pub struct RollupModeL1BatchCommitter {}

#[derive(Debug, Clone)]
pub struct ValidiumModeL1BatchCommitter {}

impl L1BatchCommitter for RollupModeL1BatchCommitter {
    fn l1_commit_data(&self, l1_batch_with_metadata: &L1BatchWithMetadata) -> Token {
        println!("RollupModeL1BatchCommitter");
        let header = &l1_batch_with_metadata.header;
        let metadata = &l1_batch_with_metadata.metadata;
        if l1_batch_with_metadata
            .header
            .protocol_version
            .unwrap()
            .is_pre_boojum()
        {
            Token::Tuple(vec![
                Token::Uint(U256::from(header.number.0)),
                Token::Uint(U256::from(header.timestamp)),
                Token::Uint(U256::from(metadata.rollup_last_leaf_index)),
                Token::FixedBytes(metadata.merkle_root_hash.as_bytes().to_vec()),
                Token::Uint(U256::from(header.l1_tx_count)),
                Token::FixedBytes(metadata.l2_l1_merkle_root.as_bytes().to_vec()),
                Token::FixedBytes(header.priority_ops_onchain_data_hash().as_bytes().to_vec()),
                Token::Bytes(metadata.initial_writes_compressed.clone()),
                Token::Bytes(metadata.repeated_writes_compressed.clone()),
                Token::Bytes(metadata.l2_l1_messages_compressed.clone()),
                Token::Array(
                    header
                        .l2_to_l1_messages
                        .iter()
                        .map(|message| Token::Bytes(message.to_vec()))
                        .collect(),
                ),
                Token::Array(
                    l1_batch_with_metadata
                        .factory_deps
                        .iter()
                        .map(|bytecode| Token::Bytes(bytecode.to_vec()))
                        .collect(),
                ),
            ])
        } else {
            Token::Tuple(vec![
                // `batchNumber`
                Token::Uint(U256::from(header.number.0)),
                // `timestamp`
                Token::Uint(U256::from(header.timestamp)),
                // `indexRepeatedStorageChanges`
                Token::Uint(U256::from(metadata.rollup_last_leaf_index)),
                // `newStateRoot`
                Token::FixedBytes(metadata.merkle_root_hash.as_bytes().to_vec()),
                // `numberOfLayer1Txs`
                Token::Uint(U256::from(header.l1_tx_count)),
                // `priorityOperationsHash`
                Token::FixedBytes(header.priority_ops_onchain_data_hash().as_bytes().to_vec()),
                // `bootloaderHeapInitialContentsHash`
                Token::FixedBytes(
                    metadata
                        .bootloader_initial_content_commitment
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                ),
                // `eventsQueueStateHash`
                Token::FixedBytes(
                    metadata
                        .events_queue_commitment
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                ),
                // `systemLogs`
                Token::Bytes(metadata.l2_l1_messages_compressed.clone()),
                // `totalL2ToL1Pubdata`
                Token::Bytes(
                    header
                        .pubdata_input
                        .clone()
                        .unwrap_or(utils::construct_pubdata(&l1_batch_with_metadata)),
                ),
            ])
        }
    }
}

impl L1BatchCommitter for ValidiumModeL1BatchCommitter {
    fn l1_commit_data(&self, l1_batch_with_metadata: &L1BatchWithMetadata) -> Token {
        println!("ValidiumModeL1BatchCommitter");
        let header = &l1_batch_with_metadata.header;
        let metadata = &l1_batch_with_metadata.metadata;
        if header.protocol_version.unwrap().is_pre_boojum() {
            Token::Tuple(vec![
                Token::Uint(U256::from(header.number.0)),
                Token::Uint(U256::from(header.timestamp)),
                Token::Uint(U256::from(metadata.rollup_last_leaf_index)),
                Token::FixedBytes(metadata.merkle_root_hash.as_bytes().to_vec()),
                Token::Uint(U256::from(header.l1_tx_count)),
                Token::FixedBytes(metadata.l2_l1_merkle_root.as_bytes().to_vec()),
                Token::FixedBytes(header.priority_ops_onchain_data_hash().as_bytes().to_vec()),
                Token::Bytes(metadata.initial_writes_compressed.clone()),
                Token::Bytes(metadata.repeated_writes_compressed.clone()),
                Token::Bytes(metadata.l2_l1_messages_compressed.clone()),
                Token::Array(
                    header
                        .l2_to_l1_messages
                        .iter()
                        .map(|message| Token::Bytes(message.to_vec()))
                        .collect(),
                ),
                Token::Array(
                    l1_batch_with_metadata
                        .factory_deps
                        .iter()
                        .map(|bytecode| Token::Bytes(bytecode.to_vec()))
                        .collect(),
                ),
            ])
        } else {
            Token::Tuple(vec![
                // `batchNumber`
                Token::Uint(U256::from(header.number.0)),
                // `timestamp`
                Token::Uint(U256::from(header.timestamp)),
                // `indexRepeatedStorageChanges`
                Token::Uint(U256::from(metadata.rollup_last_leaf_index)),
                // `newStateRoot`
                Token::FixedBytes(metadata.merkle_root_hash.as_bytes().to_vec()),
                // `numberOfLayer1Txs`
                Token::Uint(U256::from(header.l1_tx_count)),
                // `priorityOperationsHash`
                Token::FixedBytes(header.priority_ops_onchain_data_hash().as_bytes().to_vec()),
                // `bootloaderHeapInitialContentsHash`
                Token::FixedBytes(
                    metadata
                        .bootloader_initial_content_commitment
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                ),
                // `eventsQueueStateHash`
                Token::FixedBytes(
                    metadata
                        .events_queue_commitment
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                ),
                // `systemLogs`
                Token::Bytes(metadata.l2_l1_messages_compressed.clone()),
            ])
        }
    }
}