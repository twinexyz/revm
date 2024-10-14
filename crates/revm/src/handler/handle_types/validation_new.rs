use crate::{handler::mainnet, Context, EvmWiring};
use auto_impl::auto_impl;
use database_interface::{Database, EmptyDB};
use specification::hardfork::{LatestSpec, Spec};
use std::sync::Arc;
use transaction::Transaction;
use wiring::{
    default::{CfgEnv, Env, EnvWiring},
    evm_wiring::HardforkTrait,
    result::{EVMError, EVMResultGeneric, InvalidTransaction},
    Block, Cfg, EthereumWiring,
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

pub struct Handler<CTX, ERROR> {
    validation: Box<dyn ValidationWire<Context = CTX, Error = ERROR>>,
}

// impl<V: ValidationWire> Handler<V> {
//     pub fn v(&self) -> &V {
//         &self.validation
//     }
// }

// /// Example of context impl
// pub fn temp() {
//     let h = Handler {
//         validation: Validation::<Context<EthereumWiring<EmptyDB, ()>>, LatestSpec> {
//             _phantom: std::marker::PhantomData,
//         },
//     };

//     let _ = h.v().validate_env(&Context::default());
// }

pub trait ValidationWire {
    type Context: TransactionGetter + BlockGetter + DatabaseGetter + CfgGetter;
    type Error;

    /// Validate env.
    fn validate_env(&self, env: &Self::Context) -> Result<(), Self::Error>;

    /// Validate transactions against state.
    fn validate_tx_against_state(&self, context: &Self::Context) -> Result<(), Self::Error>;

    /// Validate initial gas.
    fn validate_initial_tx_gas(&self, context: &Self::Context) -> Result<u64, Self::Error>;
}

pub struct EthValidation<CTX, ERROR, Fork: Spec> {
    pub _phantom: std::marker::PhantomData<(CTX, ERROR, Fork)>,
}

impl<CTX, ERROR, Fork: Spec> EthValidation<CTX, ERROR, Fork> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl<CTX, ERROR, FORK: Spec> ValidationWire for EthValidation<CTX, ERROR, FORK>
where
    CTX: TransactionGetter + BlockGetter + DatabaseGetter + CfgGetter,
    ERROR: From<InvalidTransaction>,
{
    type Context = CTX;
    type Error = ERROR;

    fn validate_env(&self, ctx: &Self::Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_tx_against_state(&self, _context: &Self::Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_initial_tx_gas(&self, ctx: &Self::Context) -> Result<u64, Self::Error> {
        mainnet::new_validate_initial_tx_gas::<&Self::Context, FORK, InvalidTransaction>(&ctx)
            .map_err(Into::into)
    }
}
