ifdef::env-github,env-browser[:outfilesuffix: .adoc]
ifndef::rootdir[:rootdir: ..]
:imagesdir: {rootdir}/images
:toc:

# JSON-RPC API

JSON-RPC is a remote procedure call protocol encoded in JSON.
We use the [version 2 of the protocol](http://www.jsonrpc.org/specification).

## Methods

### emerald_heartbeat

Identify if and when the originator fails or is no longer available.

*Parameters*: none

*Result*: `timestamp` (Number) - seconds since Jan 01 1970 (UTC)

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_heartbeat", "params": [], "id": 1}
<-- {"jsonrpc": "2.0", "result": 1497439590, "id": 1}
```

### emerald_currentVersion

Returns the client current version.

*Parameters*: none

*Result*: `version` (String) - current version according [Semantic Versioning](http://semver.org/)

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_currentVersion", "params": [], "id": 1}
<-- {"jsonrpc": "2.0", "result": "0.9", "id": 1}
```

### emerald_listAccounts

Return the list of all not hidden (by default) accounts from the keystore.

*Parameters*:

    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`
    ** `show_hidden` (Boolean, optional) - also show hidden accounts

*Result*:

    *`accounts` (Array)
    ** `account` (Object) - an account
    *** `address` (String) - hex-encoded 20 bytes public address
    *** `hardware_wallet` (Boolean) - flag to distinguish normal accounts from HD wallet accounts
    *** `name` (String, optional) - account name
    *** `description` (String, optional) - account description

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_listAccounts", "params": [{"chain": "testnet", "show_hidden": true}], "id": 1}
<-- {"jsonrpc": "2.0", "result":
      [{"address": "0x5e97870f263700f46aa00d967821199b9bc5a120"},
       {"name": "main",
        "address": "0x3d80b31a78c30fc628f20b2c89d7ddbf6e53cedc"}],
       {"hardware_wallet": "true"
        "name": "test",
        "description": "A test account",
        "address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9"}],
     "id": 1}
```

### emerald_hideAccount

Hide an account from the list returned by default by `emerald_listAccounts`.

*Parameters*:

    * `account` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required account exists

*Examples*:

If required account exists
```
--> {"jsonrpc": "2.0", "method": "emerald_hideAccount", "params": [{"address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required account doesn't exist
```
--> {"jsonrpc": "2.0", "method": "emerald_hideAccount", "params": [{"address": "0x3d80b31a78c30fc628f20b2c89d7ddbf6e53cedc"}], "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Account doesn't exist"}, "id": "1"}
```

### emerald_unhideAccount

Show an account that was hidden before by the command `emerald_hideAccount`.

*Parameters*:

    * `account` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required account exists

*Examples*:

If required account exists
```
--> {"jsonrpc": "2.0", "method": "emerald_unhideAccount", "params": [{"address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required account doesn't exist
```
--> {"jsonrpc": "2.0", "method": "emerald_unhideAccount", "params": [{"address": "0x3d80b31a78c30fc628f20b2c89d7ddbf6e53cedc"}], "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Account doesn't exist"}, "id": "1"}
```

### emerald_newAccount

Creates a new account and stores it locally as a passphrase-encoded keystore file.

*Parameters*:

    * `account` (Object)
    ** `name` (String, optional) - account name
    ** `description` (String, optional) - account description
    ** `passphrase` (String) - passphrase used to encode keyfile (recommend to use 8+ words with good entropy)
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `address` (String) - hex-encoded 20 bytes public address

*Examples*:

.Simple format, only `passphrase`
```
--> {"jsonrpc": "2.0", "method": "emerald_newAccount", "params": [{"passphrase": "1234567890"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9", "id": 1}
```

.Full format with all optional parameters for `testnet` (id: `62`)
```
--> {"jsonrpc": "2.0",
     "method": "emerald_newAccount",
     "params":
       [{"name": "test",
         "description": "A test account"
         "passphrase": "1234567890"},
        {"chain": "testnet"}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9", "id": 1}
```

### emerald_shakeAccount

Recreate account with the same public address, but with a different passphrase.

*Parameters*:

    * `account` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    ** `old_passphrase` (String) - old passphrase used to encode keyfile
    ** `new_passphrase` (String) - new passphrase to recreate keyfile (recommend to use 8+ words with good entropy)
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required account exists

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_shakeAccount", "params": [{"address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9", "old_passphrase": "1234567890", "new_passphrase": "123"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

### emerald_updateAccount

Update not secured by passphrase account metadata, like `name` and `description`.

*Parameters*:

    * `account` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    ** `name` (String, optional) - account name
    ** `description` (String, optional) - account description
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required account exists

*Examples*:

If required account exists
```
--> {"jsonrpc": "2.0", "method": "emerald_updateAccount", "params": [{"name": "new", "address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required account doesn't exist
```
--> {"jsonrpc": "2.0", "method": "emerald_updateAccount", "params": [{"address": "0x3d80b31a78c30fc628f20b2c89d7ddbf6e53cedc"}], "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Account doesn't exist"}, "id": "1"}
```

### emerald_importAccount

Import a new account from an external keyfile. Handle both cases: normal account & HD wallet account,

*Parameters*:

    - Normal account:
        * `keyfile` (Object) - should be totally comply with the [Web3 UTC / JSON format](https://github.com/ethereum/wiki/wiki/Web3-Secret-Storage-Definition)
        * `additional` (Object, optional)
        ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
        ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

    - HD wallet:
        * `keyfile` (Object) - should be totally comply with format specified in example
        * `additional` (Object, optional)
        ** `chain`, `chain_id` - same as for normal account


*Result*: `address` (String) - successfully imported hex-encoded 20 bytes public address

*Examples*:

.Normal account:
```
--> {"jsonrpc": "2.0",
     "method": "emerald_importAccount",
     "params":
       [{"version": 3,
         "id": "f7ab2bfa-e336-4f45-a31f-beb3dd0689f3",
         "address": "0047201aed0b69875b24b614dda0270bcd9f11cc",
         "crypto": {
           "ciphertext": "c3dfc95ca91dce73fe8fc4ddbaed33bad522e04a6aa1af62bba2a0bb90092fa1",
           "cipherparams": {
             "iv": "9df1649dd1c50f2153917e3b9e7164e9"
           },
           "cipher": "aes-128-ctr",
           "kdf": "scrypt",
           "kdfparams": {
             "dklen": 32,
             "salt": "fd4acb81182a2c8fa959d180967b374277f2ccf2f7f401cb08d042cc785464b4",
             "n": 1024,
             "r": 8,
             "p": 1
           },
           "mac": "9f8a85347fd1a81f14b99f69e2b401d68fb48904efe6a66b357d8d1d61ab14e5"}}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": "0x0047201aed0b69875b24b614dda0270bcd9f11cc", "id": 1}
```

.HD wallet account:
```
--> {"jsonrpc": "2.0",
     "method": "emerald_importAccount",
     "params":
       [{"version": 3,
         "id": "f7ab2bfa-e336-4f45-a31f-beb3dd0689f3",
         "address": "8f5201aed0b69875b24b6accounaccoun14dda0e",
         "crypto": {
            "cipher": "hardware",
            "hardware": "ledger-nano-s:v1",
            "hd_path": "44'/61'/0'/0/0"},
     "id": 1}
<-- {"jsonrpc": "2.0", "result": "0x8f5201aed0b69875b24b6accounaccoun14dda0e", "id": 1}
```

### emerald_exportAccount

Returns an account keyfile associated with the account.

*Parameters*:

    * `account` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `keyfile` (Object) - normal account in [Web3 UTC / JSON format](https://github.com/ethereum/wiki/wiki/Web3-Secret-Storage-Definition),
 or HD wallet account (see example)

*Examples*:

Normal account:
```
--> {"jsonrpc": "2.0", "method": "emerald_exportAccount", "params": [{"address": "0x0047201aed0b69875b24b614dda0270bcd9f11cc"}, {"chain_id": 62}], "id": 1}
<-- {"jsonrpc": "2.0",
     "result":
       [{"version": 3,
         "id": "f7ab2bfa-e336-4f45-a31f-beb3dd0689f3",
         "address": "0047201aed0b69875b24b614dda0270bcd9f11cc",
         "crypto": {
           "ciphertext": "c3dfc95ca91dce73fe8fc4ddbaed33bad522e04a6aa1af62bba2a0bb90092fa1",
           "cipherparams": {
             "iv": "9df1649dd1c50f2153917e3b9e7164e9"
           },
           "cipher": "aes-128-ctr",
           "kdf": "scrypt",
           "kdfparams": {
             "dklen": 32,
             "salt": "fd4acb81182a2c8fa959d180967b374277f2ccf2f7f401cb08d042cc785464b4",
             "n": 1024,
             "r": 8,
             "p": 1
           },
           "mac": "9f8a85347fd1a81f14b99f69e2b401d68fb48904efe6a66b357d8d1d61ab14e5"}}],
     "id": 1}
```

HD wallet account:
```
--> {"jsonrpc": "2.0", "method": "emerald_exportAccount", "params": [{"address": "0x8f5201aed0b69875b24b6accounaccoun14dda0e"}, {"chain_id": 62}], "id": 1}
<-- {"jsonrpc": "2.0",
     "method": "emerald_importAccount",
     "params":
       [{"version": 3,
         "id": "f7ab2bfa-e336-4f45-a31f-beb3dd0689f3",
         "address": "8f5201aed0b69875b24b6accounaccoun14dda0e",
         "crypto": {
            cipher: "hardware",
            type: "ledger-nano-s:v1",
            hd: "0'/0/0"},
     "id": 1}
```

### emerald_listContracts

Return the list of all not hidden (by default) smart contracts from the local storage.

*Parameters*:

    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`
    ** `show_hidden` (Boolean, optional) - also show hidden accounts

*Result*:

    * `contracts` (Array)
    ** `contract` (Object) - a smart contract
    *** `address` (String) - hex-encoded 20 bytes smart contract address
    *** `name` (String, optional) - smart contract name
    *** `description` (String, optional) - smart contract name

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_listContracts", "params": [{"chain": "testnet", "show_hidden": true}], "id": 1}
<-- {"jsonrpc": "2.0", "result":
      [{"name": "BitEther",
        "description": "BitEther ERC20 token",
        "address": "0x085fb4f24031eaedbc2b611aa528f22343eb52db"},
       {"name": "DexNS",
        "description": "Dexaran Naming service",
        "address": "0x2906797a0a56a0c60525245c01788ecd34063b80"}],
     "id": 1}
```

### emerald_hideContract

Hide a smart contract from the list returned by default by `emerald_listContracts`.

*Parameters*:

    * `contract` (Object)
    ** `address` (String) - hex-encoded 20 bytes smart contract public address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required smart contract exists

*Examples*:

If required contract exists
```
--> {"jsonrpc": "2.0", "method": "emerald_hideContract", "params": [{"address": "0xe9a7e26bf5c05fe3bae272d4c940bd7158611ce9"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required contract doesn't exist
```
--> {"jsonrpc": "2.0", "method": "emerald_hideContract", "params": [{"address": "0x085fb4f24031eaedbc2b611aa528f22343eb52db"}], "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Contract doesn't exist"}, "id": "1"}
```

### emerald_unhideContract

Show a smart contract that was hidden before by the command `emerald_hideContract`.

*Parameters*:

    * `contract` (Object)
    ** `address` (String) - hex-encoded 20 bytes smart contract public address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required smart contract exists

*Examples*:

If required contract exists
```
--> {"jsonrpc": "2.0", "method": "emerald_unhideContract", "params": [{"address": "0x085fb4f24031eaedbc2b611aa528f22343eb52db"}], "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required contract doesn't exist
```
--> {"jsonrpc": "2.0", "method": "emerald_unhideContract", "params": [{"address": "0x085fb4f24031eaedbc2b611aa528f22343eb52db"}], "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Contract doesn't exist"}, "id": "1"}
```

### emerald_updateContract

Update contract metadata. Contract address and chain information are used to identify the contract, and may not be updated. 

*Parameters*:

    * `contract` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    ** `name` (String, optional) - contract name
    ** `description` (String, optional) - contract description
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if required contract exists

*Examples*:

If required contract exists
```
--> {"jsonrpc": "2.0", 
     "method": "emerald_updateContract", 
     "params": [{"address": "0x085fb4f24031eaedbc2b611aa528f22343eb52db",
         "name": "ERC223 token",
         "description": "Bit Ether"}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

If required contract doesn't exist
```
--> {"jsonrpc": "2.0", 
     "method": "emerald_updateContract", 
     "params": [{"address": "0x0047201aed0b69875b24b614dda0270bcd9f11cc",
         "name": "ERC20 token",
         "description": "Bit Ether"}], 
     "id": 1}
<-- {"jsonrpc": "2.0", "error": {"code": -32000, "message": "Contract doesn't exist"}, "id": "1"}
```


### emerald_importContract

Import a new smart contract Application Binary Interface (ABI) locally.

*Parameters*:

    * `contract` (Object)
    ** `address` (String) - hex-encoded 20 bytes public address
    ** `name` (String, optional) - contract name
    ** `description` (String, optional) - contract description
    ** `bytecode` (String, optional) - hex-encoded compiled contract
    ** `contract` (Array) - JSON format for a contract ABI, should be an array of function and/or event descriptions as defined https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI[here]. Each operator should have the following properties:
    *** `name` (String) - the name of the function
    *** `inputs` (Array) - an array of objects, each of which contains a name and a type
    *** `outputs` (Array) - an array of objects, each of which contains a name and a type
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `accept` (Boolean) - `true` if successful

*Examples*:

```
--> {"jsonrpc": "2.0",
     "method": "emerald_importContract",
     "params":
       [{"address": "0x0047201aed0b69875b24b614dda0270bcd9f11cc",
         "name": "ERC20 token",
         "contract":
           [{"constant":true,
             "inputs":[],
             "name":"name",
             "outputs":[{"name":"",
                         "type":"string"}],
             "payable":false,
             "type":"function"},
            {"constant":false,
             "inputs":[{"name":"_spender",
                        "type":"address"},
                       {"name":"_value",
                        "type":"uint256"}],
             "name":"approve",
             "outputs":[{"name":"success",
                         "type":"bool"}],
             "payable":false,
             "type":"function"},
            {"constant":true,
             "inputs":[],
             "name":"totalSupply",
             "outputs":[{"name":"",
                         "type":"uint256"}],
             "payable":false,
             "type":"function"},
            ...
            {"inputs":[{"name":"initialSupply",
                        "type":"uint256"},
                       {"name":"tokenName",
                        "type":"string"},
                       {"name":"decimalUnits",
                        "type":"uint8"},
                       {"name":"tokenSymbol",
                        "type":"string"}],
             "payable":false,
             "type":"constructor"},
            {"anonymous":false,
             "inputs":[{"indexed":true,
                        "name":"from",
                        "type":"address"},
                       {"indexed":true,
                        "name":"to",
                        "type":"address"},
                       {"indexed":false,
                        "name":"value",
                        "type":"uint256"}],
             "name":"Transfer",
             "type":"event"}]}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": true, "id": 1}
```

### emerald_exportContract

Returns contract object associated with the contract.

*Parameters*:

    * `contractt` (Object)
    ** `address` (String) - hex-encoded 20 bytes publ/usr/local/bin/ic address
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `contract` (Object) - JSON format for a contract ABI, as defined [here](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI).

*Examples*:

```
--> {"jsonrpc": "2.0", "method": "emerald_exportContract", "params": [{"address": "0x0047201aed0b69875b24b614dda0270bcd9f11cc"}, {"chain_id": 62}], "id": 1}
<-- {"jsonrpc": "2.0",
     "result":
       [{"address": "0x0047201aed0b69875b24b614dda0270bcd9f11cc",
         "name": "ERC20 token",
         "abi":
           [{"constant":true,
             "inputs":[],
             "name":"name",
             "outputs":[{"name":"",
                         "type":"string"}],
             "payable":false,
             "type":"function"},
            ...
            {"anonymous":false,
             "inputs":[{"indexed":true,
                        "name":"from",
                        "type":"address"},
                       {"indexed":true,
                        "name":"to",
                        "type":"address"},
                       {"indexed":false,
                        "name":"value",
                        "type":"uint256"}],
             "name":"Transfer",
             "type":"event"}]}],
     "id": 1}
```

### emerald_signTransaction

Signs transaction offline with private key from keystore file with given passphrase.
If `function` and `arguments` are provided, they will be encoded according smart contract ABI and used in the `data` field of the transaction.

*Parameters*:

    * `transaction` (Object)
    ** `from` (String) - the address the transaction is sent from (hex-encoded 20 Bytes)
    ** `to` (String, optional when creating new contract) - the address the transaction is directed to (hex-encoded 20 Bytes)
    ** `gas` (String) - Hex-encoded integer of the gas provided for the transaction execution, it will return unused gas
    ** `gasPrice` (String) - Hex-encoded integer of the gasPrice used for each paid gas
    ** `value` (String, optional) - Hex-encoded integer of the value sent with this transaction
    ** `data` (String, optional) - The compiled code of a contract OR the hash of the invoked method signature and encoded parameters (smart contract ABI)
    ** `function` (String, optional) - Name of a not-constant smart contract function to encode and use as `data`
    *** `name` (String) - an smart contract function name 
    *** `inputs` (Array, optional) - an array of smart contract input arguments
    **** `name` (String) - an smart contract function argument name 
    **** `value` (String) - an smart contract function argument value
    ** `nonce` (String) - Hex-encoded integer of a nonce, this allows to overwrite your own pending transactions that use the same nonce
    ** `passphrase` (String) - passphrase used to encode keyfile
    * `additional` (Object, optional)
    ** `chain` (String, optional) - chain name, by default `mainnet`, other possible variant `testnet`
    ** `chain_id` (Number, optional) - chain id number, by default for `mainnet` it equals `61`

*Result*: `data` (String) - hex-encoded signed raw transaction data

*Examples*:

```
--> {"jsonrpc": "2.0",
     "method": "emerald_signTransaction",
     "params":
       [{"from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
         "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
         "gas": "0x76c0",
         "gasPrice": "0x9184e72a000",
         "value": "0x9184e72a",
         "data": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675",
         "nonce": "0x1000",
         "passphrase": 1234567890"},
        {"chain": "testnet"}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675", "id": 1}
```

```
--> {"jsonrpc": "2.0",
     "method": "emerald_signTransaction",
     "params":
       [{"from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
         "to": "0x085fb4f24031eaedbc2b611aa528f22343eb52db",
         "gas": "0x0186a0",
         "gasPrice": "0x04e3b29200",         
         "function":
           {"name": "transfer",
            "inputs": [{"name": "_to",
                        "value": "0x3d80b31a78c30fc628f20b2c89d7ddbf6e53cedc"},
                       {"name": "_value",
                        "value": 10}]}}],
     "id": 1}
<-- {"jsonrpc": "2.0", "result": "0x085fb4f24031eaedbc2b611aa528f22343eb52dba9059cbb000000000000000000000000aa00000000bbbb000000000000000000000000aa000000000000000000000000000000000000000000000000000000000000000a", "id": 1}
```

## Custom Errors


|Code   |Message |Meaning|
|---   |:-------------:|:-----:|
|-32000 | Account doesn't exist|Nothing is found at the specified account address|

