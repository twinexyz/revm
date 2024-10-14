use crate::{handler::mainnet, Context, EvmWiring};
use database_interface::Database;
use specification::hardfork::Spec;
use std::sync::Arc;
use transaction::Transaction;
use wiring::{
    default::EnvWiring,
    evm_wiring::HardforkTrait,
    result::{EVMError, EVMResultGeneric, InvalidTransaction},
    Block,
};

pub trait ContextWire {
    type Transaction: Transaction;
    type Block: Block;
    type Database: Database;
    type Cfg;

    fn tx(&self) -> &Self::Transaction;
    fn block(&self) -> &Self::Block;
    fn cfg(&self) -> &Self::Cfg;
    fn db(&mut self) -> &mut Self::Database;
}

// pub type EvmResultGeneric<T, ContextWireT> = Result<T, EvmErrorGeneric<ContextWireT>>;

// pub type EvmErrorGeneric<ContextWireT> = EVMError<
//     <<ContextWireT as ContextWire>::Database as Database>::Error,
//     <<ContextWireT as ContextWire>::Transaction as Transaction>::TransactionError,
// >;

// pub trait ValidationWire {
//     type Context: ContextWire;
//     type Hardfork: HardforkTrait;

//     /// Validate env.
//     fn validate_env(&self, env: &Self::Context) -> EvmResultGeneric<(), Self::Context>;

//     /// Validate transactions against state.
//     fn validate_tx_against_state(
//         &self,
//         context: &Self::Context,
//     ) -> EvmResultGeneric<(), Self::Context>;

//     /// Validate initial gas.
//     fn validate_initial_tx_gas(
//         &self,
//         context: &Self::Context,
//     ) -> EvmResultGeneric<u64, Self::Context>;
// }

// pub struct Validation<Ctx: ContextWire, Fork: HardforkTrait> {
//     pub _phantom: std::marker::PhantomData<(Ctx, Fork)>,
// }

// impl<CTX: ContextWire, FORK: HardforkTrait> ValidationWire for Validation<CTX, FORK> {
//     type Context = CTX;
//     type Hardfork = FORK;

//     fn validate_env(&self, _env: &Self::Context) -> EvmResultGeneric<(), Self::Context> {
//         Ok(())
//     }

//     fn validate_tx_against_state(
//         &self,
//         _context: &Self::Context,
//     ) -> EvmResultGeneric<(), Self::Context> {
//         Ok(())
//     }

//     fn validate_initial_tx_gas(
//         &self,
//         _context: &Self::Context,
//     ) -> EvmResultGeneric<u64, Self::Context> {
//         Ok(0)
//     }
// }

// pub trait ExecWire {
//     type Context: ContextWire;
//     type Frame: ContextWire;
//     type Precompile: ContextWire;

//     fn first_frame() -> Self::Frame;
//     fn exec() -> Self::Frame;
//     fn last_frame() -> Self::Frame;
// }

// pub trait PostExec {
//     type Context: ContextWire;
//     type Frame: ContextWire;
//     type Precompile: ContextWire;

//     fn post_exec() -> Self::Frame;
// }

// pub struct InspectorValidationWire<VW: ValidationWire> {
//     pub old_wire: VW,
// }

// impl<VW: ValidationWire> ValidationWire for InspectorValidationWire<VW> {
//     type Context = VW::Context;
//     type Hardfork = VW::Hardfork;

//     fn validate_env(&self, env: &Self::Context) -> EvmResultGeneric<(), Self::Context> {
//         self.old_wire.validate_env(env)
//     }

//     fn validate_tx_against_state(
//         &self,
//         context: &Self::Context,
//     ) -> EvmResultGeneric<(), Self::Context> {
//         self.old_wire.validate_tx_against_state(context)
//     }

//     fn validate_initial_tx_gas(
//         &self,
//         context: &Self::Context,
//     ) -> EvmResultGeneric<u64, Self::Context> {
//         self.old_wire.validate_initial_tx_gas(context)
//     }
// }

// pub trait Wire {
//     type Context: ContextWire;
//     type ValidationWire: ValidationWire<Context = Self::Context>;
// }

// pub struct WireImpl<W: Wire> {
//     pub context: W::Context,
//     pub validation: W::ValidationWire,
// }

// impl<W: Wire> WireImpl<W> {
//     pub fn new(context: W::Context, validation: W::ValidationWire) -> Self {
//         Self {
//             context,
//             validation,
//         }
//     }
//     pub fn transaction(&mut self) {
//         let v = &mut self.validation;
//         let context = &mut self.context;
//         let _ = v.validate_env(context);
//         let _ = v.validate_tx_against_state(context);
//     }
// }

// pub struct InspectorWire<W: Wire> {
//     pub old_wire: WireImpl<WireImpl>,
// }

/// Handle that validates env.
pub type ValidateEnvHandle<'a, EvmWiringT> =
    Arc<dyn Fn(&EnvWiring<EvmWiringT>) -> EVMResultGeneric<(), EvmWiringT> + 'a>;

/// Handle that validates transaction environment against the state.
/// Second parametar is initial gas.
pub type ValidateTxEnvAgainstState<'a, EvmWiringT> =
    Arc<dyn Fn(&mut Context<EvmWiringT>) -> EVMResultGeneric<(), EvmWiringT> + 'a>;

/// Initial gas calculation handle
pub type ValidateInitialTxGasHandle<'a, EvmWiringT> =
    Arc<dyn Fn(&EnvWiring<EvmWiringT>) -> EVMResultGeneric<u64, EvmWiringT> + 'a>;

/// Handles related to validation.
pub struct ValidationHandler<'a, EvmWiringT: EvmWiring> {
    /// Validate and calculate initial transaction gas.
    pub initial_tx_gas: ValidateInitialTxGasHandle<'a, EvmWiringT>,
    /// Validate transactions against state data.
    pub tx_against_state: ValidateTxEnvAgainstState<'a, EvmWiringT>,
    /// Validate Env.
    pub env: ValidateEnvHandle<'a, EvmWiringT>,
}

impl<'a, EvmWiringT: EvmWiring + 'a> ValidationHandler<'a, EvmWiringT>
where
    <EvmWiringT::Transaction as Transaction>::TransactionError: From<InvalidTransaction>,
{
    /// Create new ValidationHandles
    pub fn new<SPEC: Spec + 'a>() -> Self {
        Self {
            initial_tx_gas: Arc::new(mainnet::validate_initial_tx_gas::<EvmWiringT, SPEC>),
            env: Arc::new(mainnet::validate_env::<EvmWiringT, SPEC>),
            tx_against_state: Arc::new(mainnet::validate_tx_against_state::<EvmWiringT, SPEC>),
        }
    }
}

impl<'a, EvmWiringT: EvmWiring> ValidationHandler<'a, EvmWiringT> {
    /// Validate env.
    pub fn env(&self, env: &EnvWiring<EvmWiringT>) -> EVMResultGeneric<(), EvmWiringT> {
        (self.env)(env)
    }

    /// Initial gas
    pub fn initial_tx_gas(&self, env: &EnvWiring<EvmWiringT>) -> EVMResultGeneric<u64, EvmWiringT> {
        (self.initial_tx_gas)(env)
    }

    /// Validate ttansaction against the state.
    pub fn tx_against_state(
        &self,
        context: &mut Context<EvmWiringT>,
    ) -> EVMResultGeneric<(), EvmWiringT> {
        (self.tx_against_state)(context)
    }
}
