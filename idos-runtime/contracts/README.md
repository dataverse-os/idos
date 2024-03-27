<br/>
<p align="center">
<a href=" " target="_blank">
<img src="https://bafybeifozdhcbbfydy2rs6vbkbbtj3wc4vjlz5zg2cnqhb2g4rm2o5ldna.ipfs.w3s.link/dataverse.svg" width="180" alt="Dataverse logo">
</a >
</p >
<br/>

# dataverse-contracts

**⚠️ Warning: This Smart Contract has not been professionally audited.**

## Overview

## Setup

Install Foundry:

```
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

Install dependencies:

```
forge install
```

## Compile

```
forge build
```

## Test

```
forge test
```

Test the code with logs and traces:

```
forge test -vvvv
```

## Deploy

Anvil is a local testnet node that comes with Foundry. You can use it to test your contracts from frontends or to interact over RPC. Here, we deploy contracts to the local chain Anvil as an example.

### 1. Start Anvil

```
anvil
```

The service will then be listening on `127.0.0.1:8545`.

### 2. Set .env

Please add a new file named .env and configure your environment as `.env.example` showed.

```
ANVIL_RPC_URL=
PRIVATE_KEY=
```

`PRIVATE_KEY` could be copied from Anvil's shell dashboard.

Then source `.env` in shell:

```
source .env
```

## 3.Deploy

Please add a new file named `.env` and configure your environment as `.env.example` showed.

```
npm run deploy:polygon_mumbai
npm run deploy:bsc_testnet
```

## 4.Deployed Contract Address

The contract addresses deployed on different blockchain networks are listed in the `addresses.json` file.

### Deployed Contract Address

```json
{
  "Mumbai": {
    "GlobalConfig": "0x1d5122b2293Edb9bbCc03F55da110588cEAf657b",
    "DappTableRegistry": "0xAf0a7C64ecEB3525390e4A7289189c8B06051DB5",
    "DataUnion": {
      "DataUnion": "0x2AE67993019275E140fa01B47e6f32d1AecFF1ca",
      "LitACL": "0xa420D0Fee98b242e494A15D6143e068c1D16b72C",

      "BlockSubscribeModule": "0x20b591ed9Ce58013B7765Ea5Cc8Df943B78f2DaC",
      "TimeSegmentSubscribeModule": "0x78BeEc5D57eB43F14D91645AB817644BeF7acCa3"
    },
  },
  "BSCT": {},
  "Polygon": {},
  "BSC": {},
}
```
