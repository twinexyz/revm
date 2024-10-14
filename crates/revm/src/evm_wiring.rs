use crate::{
    handler::{
        EthValidation, ExecutionHandler, PostExecutionHandler, PreExecutionHandler,
        ValidationHandler, ValidationWire,
    },
    EvmHandler,
};
use context::Context;
use database_interface::Database;
use interpreter::table::InstructionTables;
use specification::spec_to_generic;
use std::fmt::Debug;
use std::vec::Vec;
use wiring::{
    result::{EVMError, EVMErrorWiring},
    EthereumWiring, EvmWiring as PrimitiveEvmWiring,
};

pub trait EvmWiring: PrimitiveEvmWiring {
    /// Creates a new handler with the given hardfork.
    fn handler<'evm>(hardfork: Self::Hardfork) -> EvmHandler<'evm, Self>;
}

impl<DB: Database, EXT: Debug> EvmWiring for EthereumWiring<DB, EXT> {
    fn handler<'evm>(hardfork: Self::Hardfork) -> EvmHandler<'evm, Self>
    where
        DB: Database,
        //EXT: 'ev,
    {
        spec_to_generic!(
            hardfork,
            EvmHandler {
                spec_id: hardfork,
                instruction_table: InstructionTables::new_plain::<SPEC>(),
                registers: Vec::new(),
                validation: ValidationHandler::new::<SPEC>(),
                new_v: EthValidation::<Context<Self>, EVMErrorWiring<Self>, SPEC>::new_boxed(),
                pre_execution: PreExecutionHandler::new::<SPEC>(),
                post_execution: PostExecutionHandler::mainnet::<SPEC>(),
                execution: ExecutionHandler::new::<SPEC>(),
            }
        )
    }
}
