# Spike web3 compatibility layer


    1
    +------------------------+
    | Web3 Client            |
    | (Metamask, Truffle)    |
    +------------------------+
               |
    2          v
    +------------------------+
    | Server implements      |
    | EVM JSON-RPC           |
    +------------------------+
               |
    3          v
    +------------------------+
    | RPC Substrate Client   |
    +------------------------+
               |
    4          v
    +------------------------+
    | Substrate Node         |
    +------------------------+


1. TODO
2. [Server](./2_evm_rpc_server.go) and [endpoints](./2_evm_rpc_endpoints.go)
3. [File](./3_substrate_client.go) with examples: transfer and get owner of collection
4. run evolution node code from `feature/ethereumSignature_no_benchmark` branch
