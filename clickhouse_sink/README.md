# Clickhouse sink module
---

## **Historical Data Analysis: Clickhouse sink (Work in progress)**
---

### Pre-requirements:
Install clickhouse & substreams-sink-sql

```bash
brew install streamingfast/tap/substreams-sink-sql
```

Install clickhouse (bare):
Optional: Run a clickhouse docker container instead
```bash
curl https://clickhouse.com/ | sh
sudo ./clickhouse install
```

Start clickhouse server

```bash

sudo clickhouse start 
```

Run clickhouse client:

First time password creation:

```bash

clickhouse-client --password 
```
---
### Configure DB

#### 1. Create the database

```bash
clickhouse-client --password --query "
CREATE DATABASE IF NOT EXISTS tron"
```

#### 2. Create transactions TABLE in your DB

```bash
clickhouse-client --password < schema.sql
```
---
### Setup Clickhouse sink substreams module (only if changes in substreams.yaml or lib.rs)

```bash
 substreams build
```
#### Setup DSN (Data Source Name) EnvVar
```bash
export CLICKHOUSE_DSN='clickhouse://default:your_clickhouse-client_password@localhost:9000/tron'
```
CLickhouse DB "RESET" (DROP DB and Schema import):

```bash
cd clickouse_sink   ##cd into clickhouse_sink dir
clickhouse-client --password --query "DROP TABLE IF EXISTS tron.transactions"
clickhouse-client --password < schema.sql
```
then setup again:

```bash
substreams-sink-sql setup "$CLICKHOUSE_DSN" clickhouse-sink-v0.1.0.spkg
```

TROUBLESHOOTING:

If you face an error like this: ERRO (sink-sql) unable to retrieve cursor: cursor module hash mismatch, refusing to continue because flag '--on-module-hash-mismatch=error' (defaults) is set, you can change to 'warn' or 'ignore' after a module upgrade, and if cursor is stil at 0 (meaning you still didnt sink any significant data) you can reset sink state by:

```bash
clickhouse-client --password --query "TRUNCATE TABLE tron.cursors;"
clickhouse-client --password --query "TRUNCATE TABLE tron.substreams_history;"
```


Run your substream and start sinking to your DB:

```bash
# The command starts sinking your filtered data from block 70937500, sinks 7220 which accounts about 1/4 of a day for data egress estimation purposes, adjust starting block and end block as needed.
substreams-sink-sql run "$CLICKHOUSE_DSN" ./clickhouse-sink-v0.1.0.spkg 70937500:+7220 -e mainnet.tron.streamingfast.io:443 --undo-buffer-size 50```

| ID  | Contract Type                   | Description                                          | Enabled in Module? |
| --- | ------------------------------- | ---------------------------------------------------- | ------------------ |
| 0   | AccountCreateContract           | Creates a new account on the network                 | No                 |
| 1   | TransferContract                | Transfers native TRX between accounts                | Yes                 |
| 2   | TransferAssetContract           | Transfers TRC-10 tokens between accounts             | Yes                 |
| 3   | VoteAssetContract               | Votes using TRC-10 assets (deprecated/rare)          | No                 |
| 4   | VoteWitnessContract             | Votes for Super Representatives                      | No                |
| 5   | WitnessCreateContract           | Registers as a Super Representative candidate        | No                |
| 6   | AssetIssueContract              | Creates (issues) a new TRC-10 token                  | No                 |
| 8   | WitnessUpdateContract           | Updates Super Representative info                    | No                |
| 9   | ParticipateAssetIssueContract   | Participates in a TRC-10 token sale                  | No                 |
| 10  | AccountUpdateContract           | Updates account name or metadata                     | No                |
| 11  | FreezeBalanceContract           | Freezes TRX for resources or voting power            | No                |
| 12  | UnfreezeBalanceContract         | Unfreezes previously frozen TRX                      | No                |
| 13  | WithdrawBalanceContract         | Withdraws SR block rewards                           | No                |
| 14  | UnfreezeAssetContract           | Unfreezes TRC-10 tokens                              | No                 |
| 15  | UpdateAssetContract             | Updates TRC-10 token parameters                      | No                 |
| 16  | ProposalCreateContract          | Creates a governance proposal                        | No                |
| 17  | ProposalApproveContract         | Approves a governance proposal                       | No                |
| 18  | ProposalDeleteContract          | Deletes a proposal                                   | No                |
| 19  | SetAccountIdContract            | Sets a human-readable account ID                     | No                |
| 20  | CustomContract                  | Reserved for custom or undefined use                 | No                |
| 30  | CreateSmartContract             | Deploys a smart contract                             | No                |
| 31  | TriggerSmartContract            | Calls a deployed smart contract                      | Yes                 |
| 32  | GetContract                     | Queries smart contract information                   | No                 |
| 33  | UpdateSettingContract           | Updates smart contract consume_user_resource_percent | No                |
| 41  | ExchangeCreateContract          | Creates a token exchange pair                        | No                 |
| 42  | ExchangeInjectContract          | Adds liquidity to an exchange                        | No                 |
| 43  | ExchangeWithdrawContract        | Removes liquidity from an exchange                   | No                 |
| 44  | ExchangeTransactionContract     | Executes a token swap                                | No                 |
| 45  | UpdateEnergyLimitContract       | Updates contract origin_energy_limit                 | No                |
| 46  | AccountPermissionUpdateContract | Updates account permissions (multi-signature)        | No                |
| 48  | ClearABIContract                | Removes a contract’s ABI                             | No                |
| 49  | UpdateBrokerageContract         | Updates SR brokerage ratio                           | No                |
| 51  | ShieldedTransferContract        | Performs a private transfer                          | No                |
| 52  | MarketSellAssetContract         | Places a sell order (TRC10)                          | No                 |
| 53  | MarketCancelOrderContract       | Cancels a market order (TRC10)                       | No                 |
| 54  | FreezeBalanceV2Contract         | Freezes TRX (new resource model)                     | No                |
| 55  | UnfreezeBalanceV2Contract       | Unfreezes TRX (new model)                            | No                |
| 56  | WithdrawExpireUnfreezeContract  | Withdraws TRX after unfreeze period                  | No                |
| 57  | DelegateResourceContract        | Delegates resources to another account               | No                 |
| 58  | UnDelegateResourceContract      | Removes delegated resources                          | No                 |
| 59  | CancelAllUnfreezeV2Contract     | Cancels all pending unfreeze operations              | No                |