<br/>
<p align="center">
<a href=" " target="_blank">
<img src="https://bafybeifozdhcbbfydy2rs6vbkbbtj3wc4vjlz5zg2cnqhb2g4rm2o5ldna.ipfs.w3s.link/dataverse.svg" width="180" alt="Dataverse logo">
</a >
</p >
<br/>

# idos-contracts

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
npm install
```

## Compile

```
npm run build
```

## Test

```
npm run test
```

Test the code with logs and traces:

```
npm run test -vvvv
```

## Deploy
### 1. Set .env
Please add a new file named `.env` and configure your environment as `.env.example` showed.

```
POLYGON_MUMBAI_RPC_URL=
BSC_TESTNET_RPC_URL=
PRIVATE_KEY=
```

### 2. Deploy script
```
npm run deploy:polygon_mumbai
npm run deploy:bsc_testnet
```

## 3.Deployed Contract Address
The contract addresses deployed on different blockchain networks are listed in the `addresses.json` file.

