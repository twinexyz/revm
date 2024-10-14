use crate::Context;
use auto_impl::auto_impl;
use database_interface::{Database, EmptyDB};
use specification::hardfork::{LatestSpec, Spec};
use std::sync::Arc;
use transaction::Transaction;
use wiring::{
    default::{CfgEnv, Env, EnvWiring},
    evm_wiring::HardforkTrait,
    result::{EVMError, EVMResultGeneric, InvalidTransaction},
    Block, Cfg, EthereumWiring, EvmWiring,
};

pub trait CfgGetter {
    type Cfg: Cfg;

    fn cfg(&self) -> &Self::Cfg;
}

impl<EvmWiringT: EvmWiring> CfgGetter for Context<EvmWiringT> {
    type Cfg = CfgEnv;

    fn cfg(&self) -> &Self::Cfg {
        &self.evm.inner.env.cfg
    }
}

pub trait DatabaseGetter {
    type Database: Database;

    fn db(&mut self) -> &mut Self::Database;
}

impl<EvmWiringT: EvmWiring> DatabaseGetter for Context<EvmWiringT> {
    type Database = EvmWiringT::Database;

    fn db(&mut self) -> &mut Self::Database {
        &mut self.evm.db
    }
}

#[auto_impl(&, Box, Arc)]
pub trait TransactionGetter {
    type Transaction: Transaction;

    fn tx(&self) -> &Self::Transaction;
}

impl<BLOCK: Block, TX: Transaction> TransactionGetter for Env<BLOCK, TX> {
    type Transaction = TX;

    fn tx(&self) -> &Self::Transaction {
        &self.tx
    }
}

impl<EvmWiringT: EvmWiring> TransactionGetter for Context<EvmWiringT> {
    type Transaction = EvmWiringT::Transaction;

    fn tx(&self) -> &Self::Transaction {
        &self.evm.env.tx
    }
}

#[auto_impl(&, Box, Arc)]
pub trait BlockGetter {
    type Block: Block;

    fn block(&self) -> &Self::Block;
}

impl<BLOCK: Block, TX: Transaction> BlockGetter for Env<BLOCK, TX> {
    type Block = BLOCK;

    fn block(&self) -> &Self::Block {
        &self.block
    }
}

impl<EvmWiringT: EvmWiring> BlockGetter for Context<EvmWiringT> {
    type Block = EvmWiringT::Block;

    fn block(&self) -> &Self::Block {
        &self.evm.env.block
    }
}

pub type EvmError<DB, TX> = EVMError<DB, TX>;
