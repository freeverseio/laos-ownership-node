# LAOS Ownership Parachain

The LAOS Ownership Parachain is a specialized chain built on Polkadot. 

It offers several functionalities that enable the management and transfer of LAOS native utility tokens, as well as the ownership of all LA (Living Assets) created directly within LAOS. Additionally, the Parachain handles the runtime upgrades of Evolution Chains (Evochains) and stores state roots of Evochains, providing asset attribute certification methods and rewarding Evochains validators upon receiving new correct roots. Trustless transfers of Living Assets and LAOS tokens between LAOS and other Parachains can be performed using the Cross-Chain Message (XCM) protocol.

## LAOS Ownership Node

Run an Arrakis testnet node
```
$ docker run freeverseio/laos-ownership-node:v0.0.5 --chain=arrakis
```

### Networks
#### Arrakis (testnet)

The Arrakis network serves as the testnet for the LAOS Ownership Parachain. It can be accessed and interacted with using either the Substrate RPC (Polkadot JS extension) or the Ethereum RPC wallet (Metamask).


##### Substrate RPC
* **RPC URL**: https://arrakis.gorengine.com/own

##### Ethereum RPC
* **Network ID**: Arrakis
* **Chain ID**: 667
* **RPC URL**: https://arrakis.gorengine.com/own
* **Currency Symbol**: DROP

## Installation and Setup

To get started with the LAOS Ownership Parachain, follow the steps below:

1. Install the necessary dependencies for interacting with the Parachain.

2. Connect to the Arrakis network using either the Substrate RPC or Ethereum RPC.

3. Use the provided RPC URL and network/chain IDs to configure your connection.

4. Access the LAOS Ownership Parachain functionalities through the chosen interface (Substrate RPC or Ethereum RPC) to manage LAOS tokens, Living Assets, and perform trustless transfers.

## Contributing

Contributions to the LAOS Ownership Parachain project are welcome.
