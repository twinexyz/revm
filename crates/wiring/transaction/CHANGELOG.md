# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0](https://github.com/bluealloy/revm/releases/tag/revm-transaction-v1.0.0) - 2024-10-24

### Added

- restructuring Part6 transaction crate ([#1814](https://github.com/bluealloy/revm/pull/1814))
- *(examples)* generate block traces ([#895](https://github.com/bluealloy/revm/pull/895))
- implement EIP-4844 ([#668](https://github.com/bluealloy/revm/pull/668))
- *(Shanghai)* All EIPs: push0, warm coinbase, limit/measure initcode ([#376](https://github.com/bluealloy/revm/pull/376))
- Migrate `primitive_types::U256` to `ruint::Uint<256, 4>` ([#239](https://github.com/bluealloy/revm/pull/239))
- Introduce ByteCode format, Update Readme ([#156](https://github.com/bluealloy/revm/pull/156))

### Fixed

- fix typos ([#620](https://github.com/bluealloy/revm/pull/620))

### Other

- Bump new logo ([#1735](https://github.com/bluealloy/revm/pull/1735))
- *(README)* add rbuilder to used-by ([#1585](https://github.com/bluealloy/revm/pull/1585))
- added simular to used-by ([#1521](https://github.com/bluealloy/revm/pull/1521))
- add Trin to used by list ([#1393](https://github.com/bluealloy/revm/pull/1393))
- Fix typo in readme ([#1185](https://github.com/bluealloy/revm/pull/1185))
- Add Hardhat to the "Used by" list ([#1164](https://github.com/bluealloy/revm/pull/1164))
- Add VERBS to used by list ([#1141](https://github.com/bluealloy/revm/pull/1141))
- license date and revm docs ([#1080](https://github.com/bluealloy/revm/pull/1080))
- *(docs)* Update the benchmark docs to point to revm package ([#906](https://github.com/bluealloy/revm/pull/906))
- *(docs)* Update top-level benchmark docs ([#894](https://github.com/bluealloy/revm/pull/894))
- clang requirement ([#784](https://github.com/bluealloy/revm/pull/784))
- Readme Updates ([#756](https://github.com/bluealloy/revm/pull/756))
- Logo ([#743](https://github.com/bluealloy/revm/pull/743))
- book workflow ([#537](https://github.com/bluealloy/revm/pull/537))
- add example to revm crate ([#468](https://github.com/bluealloy/revm/pull/468))
- Update README.md ([#424](https://github.com/bluealloy/revm/pull/424))
- add no_std to primitives ([#366](https://github.com/bluealloy/revm/pull/366))
- revm-precompiles to revm-precompile
- Bump v20, changelog ([#350](https://github.com/bluealloy/revm/pull/350))
- typos ([#232](https://github.com/bluealloy/revm/pull/232))
- Add support for old forks. ([#191](https://github.com/bluealloy/revm/pull/191))
- revm bump 1.8. update libs. snailtracer rename ([#159](https://github.com/bluealloy/revm/pull/159))
- typo fixes
- fix readme typo
- Big Refactor. Machine to Interpreter. refactor instructions. call/create struct ([#52](https://github.com/bluealloy/revm/pull/52))
- readme. debuger update
- Bump revm v0.3.0. README updated
- readme
- Add time elapsed for tests
- readme updated
- Include Basefee into cost calc. readme change
- Initialize precompile accounts
- Status update. Taking a break
- Merkle calc. Tweaks and debugging for eip158
- Replace aurora bn lib with parity's. All Bn128Add/Mul/Pair tests passes
- TEMP
- one tab removed
- readme
- README Example simplified
- Gas calculation for Call/Create. Example Added
- readme usage
- README changes
- Static gas cost added
- Subroutine changelogs and reverts
- Readme postulates
- Spelling
- Restructure project
- First iteration. Machine is looking okay
