# 🧰 Nexus SDK

This documentation aims to document the Nexus SDK, a combination of a CLI tool and Toolkit to facilitate developers building with Nexus. For more information about Nexus itself, please refer to [the Nexus Core documentation](../nexus-next/index.md).

## Actors

For the purposes of this documentation we make distinction between different user roles within the ecosystem:

* **Nexus maintainer.** Core team member that maintains the Nexus codebase.
* **Tool developer.** Outside contributor that develops Tools to be used by Agents.
* **Agent developer.** Outside contributor that creates DAGs and subsequently deploys the Agent smart contract.
* **Agent user.** End-user that interacts with the ecosystem through clients built by us or outside contributors.

## [Glossary](../nexus-next/Glossary.md)

Ubiqutously used terms. Often these terms reference specific parts of the project so it is crucial that they be clearly defined. Find them [here](../nexus-next/Glossary.md).

## [CLI](CLI.md)

This CLI can be used by both Agent Developers and Tool Developers for various tasks. Those tasks include:

1. registering Tools in the on-chain Tool Registry
2. static analysis of Nexus workflow DAGs
3. deployment of smart contracts that represent the Agent and holds its DAG

The codebase resides in [this repository](https://github.com/Talus-Network/nexus-sdk).

Docs:

* [Nexus CLI](CLI.md)
* [DAG Construction Guide](dag-construction.md)

Epics:

* https://github.com/Talus-Network/nexus-next/issues/69
* https://github.com/Talus-Network/nexus-next/issues/45
* https://github.com/Talus-Network/nexus-next/issues/15

## Toolkit

Toolkit is an SDK for Tool Developers. It helps provide boilerplate code for creating Tools that adhere to the Nexus-defined interface schema.

The codebase resides in [this repository](https://github.com/Talus-Network/nexus-sdk).

Docs:

* [Tool Development Guidelines](tool-development.md)
* [Nexus Toolkit Rust](toolkit-rust.md)

Epics:

* https://github.com/Talus-Network/nexus-next/issues/69
