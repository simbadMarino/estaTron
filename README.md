# ESTA-TRON: TRON data sink & query

---

TRON historical data , particularly related to transaction-type decomposition  is slow to extract directly from a TRON Fullnode and not fully available in TRON explorers, this repo closes this gap by providing ready-to-use Substream modules and TRON blockchain DB backups for efficient Data Analytics purposes.

This project explores a couple data output (sinks)configurations:

- JSONL : Simple line based format, useful for relatively low transaction volume analysis.
- Clickhouse DB: More complex, useful for large / full TRON data querying.

---

## **The underlying Infra: Substreams**

We are leveraging Substreams technology to transform and sink TRON data in an efficient way.

Substreams allow you to:

- **Extract blockchain data** either historical or real-time as blocks are produced
- **Transform and filter** transaction data based on your requirements
- **Build composable modules** that can be combined for complex data processing
- **Extract data efficiently** with parallel processing and caching

Further information can de consulted in Substreams official [docs](https://docs.substreams.dev/)

### Getting Started

If this is your first time using Substreams follow the tutorial below to get started:

1. **Install Substreams CLI**

```bash
# Useful for quick tests and streaming
brew install streamingfast/tap/substreams   
# Useful for historical data extraction
brew install streamingfast/tap/substreams-sink-files
# Useful to sink data in sql or clickhouse
brew install streamingfast/tap/substreams-sink-sql
```

2. **Get your StreamingFast API key from:**

- [https://app.streamingfast.io/](https://app.streamingfast.io/dashboard) - Sign up for free tier
- Create a new token and KEY (Copy your API KEY and API TOKEN)
- Now, export your API key and create authentication function:

```bash
# Set your StreamingFast API key
export STREAMINGFAST_KEY="your-streamingfast-api-key-here"

# Create the authentication function
function sftoken {
  export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
  echo "Token set in SUBSTREAMS_API_TOKEN"
}

# Get authentication token
sftoken
```

3. Optional: Install Rust (Only if willing to transform and build new substreams modules)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env # to configure your current shell
```

Ensure you have the wasm32-unknown-unknown target installed on your Rust installation, if unsure, you can install it with:

```bash
rustup target add wasm32-unknown-unknown
```




### **Example Queries**

```SQL
SELECT
    contract_type,
    count() AS txn_count,
    toStartOfQuarter(toDateTime(block_timestamp / 1000)) AS quarter,
    sum(total_fee_burn/1000000) AS total_fee
FROM tron_account_perm_update.transactions
GROUP BY
    contract_type,
    quarter
ORDER BY
    quarter,
    total_fee DESC
```

```SQL
--Gets trx burn and usage stats around AccountPermissionUpdate contract_type
SELECT
    contract_type,
    count() AS txn_count,
    min(block_number) AS start_block,
    max(block_number) AS end_block,
    concat(
        toString(toYear(toDateTime(block_timestamp / 1000))),
        '-Q',
        toString(toQuarter(toDateTime(block_timestamp / 1000)))
    ) AS quarter,
    sum(total_fee_burn/1000000) AS burned_TRX,
    countIf(toInt32OrZero(perm_threshold) > 1) AS high_threshold_txn_count,
    uniqExact(`from`) AS unique_from_addresses
FROM tron_account_perm_update.transactions
GROUP BY
    contract_type,
    quarter
ORDER BY
    quarter,
    burned_TRX DESC;
```

```SQL
--Retrieves transactions table size
SELECT
    formatReadableSize(sum(data_compressed_bytes)) AS compressed,
    formatReadableSize(sum(data_uncompressed_bytes)) AS uncompressed,
    round(sum(data_uncompressed_bytes)/sum(data_compressed_bytes), 2) AS ratio
FROM system.parts
WHERE database = 'tron_account_perm_update'
  AND table = 'transactions';
```

```SQL
---Find duplicated txns (Approximation )
SELECT
    count() AS total_rows,
    uniq(tx_id) AS unique_tx,
    total_rows - unique_tx AS duplicate_rows
FROM tron_account_perm_update.transactions;
```

```SQL
--Find txn duplicates in between a block range
SELECT
    tx_id,
    count()
FROM tron_account_perm_update.transactions
WHERE block_number BETWEEN 60818393 AND 68944620
GROUP BY tx_id
HAVING count() > 1;
```

### **Database backup**

Make sure you have a propper strategy to perform backups, especially before a significant data sinking.

1. Run a root shell:

```bash
sudo -i # macOS
su #Linux
```

1. Navigate to clickhouse server path

```bash
cd /etc/clickhouse-server
ls
```

You should see something like:

```bash
config.xml
users.xml
config.d/
users.d/
```

1. configure your backup output:

```bash
sudo nano /etc/clickhouse-server/config.d/backup_disk.xml
```

```xml
<clickhouse>
  <storage_configuration>
    <disks>
      <backups>
        <type>local</type>
        <path>/Users/YOUR_USER/clickhouse_backups/</path>
      </backups>
    </disks>
  </storage_configuration>

  <backups>
    <allowed_disk>backups</allowed_disk>
    <allowed_path>/Users/YOUR_USER/clickhouse_backups/</allowed_path>
  </backups>
</clickhouse>
```

1. Restart your clickhouse server

```bash
sudo pkill clickhouse-server
sudo clickhouse start
```

1. Run the backup action

```bash
clickhouse-client --password --query "
BACKUP DATABASE tron_account_perm_update
TO Disk('backups', 'tron_backup_$(date +%F).zip')"
```

#### Restore DB from disk

```bash
clickhouse-client --password --query "RESTORE DATABASE my_db AS my_db_restored
FROM File('/Users/YOUR_USER/clickhouse_backups/my_db_backup');"
```

### **📦 Substreams Package Management**

#### **Update Pre-built Official TRON foundational Package**

NOTICE: Only execute the following steps if an updated tron-foundational spkg is available, by default, the tron foundational package is included in `/substream`

```bash
# Navigate to substreams directory
cd substream 

# Download the official TRON foundational package
substreams pack tron-foundational@v0.1.X -o bin/tron-foundational-v0.1.X.spkg

# Verify the package
substreams info bin/tron-foundational-v0.1.X.spkg
```

**Package Contents:**

- `**map_transactions`** - Extracts all non-failed transactions with full details
- `**index_transactions`** - Creates searchable transaction indices
- `**filtered_transactions`** - Filters transactions by type, contract, or other parameters

### Supported Contract Types in package:

|compressed  | uncompressed | ratio |
│ 66.80 GiB  │ 152.53 GiB   │  2.28 │


| ID  | Contract Type                   | Description                                          | Include in Module? |
| --- | ------------------------------- | ---------------------------------------------------- | ------------------ |
| 0   | AccountCreateContract           | Creates a new account on the network                 | No                 |
| 1   | TransferContract                | Transfers native TRX between accounts                | No                 |
| 2   | TransferAssetContract           | Transfers TRC-10 tokens between accounts             | No                 |
| 3   | VoteAssetContract               | Votes using TRC-10 assets (deprecated/rare)          | No                 |
| 4   | VoteWitnessContract             | Votes for Super Representatives                      | Yes                |
| 5   | WitnessCreateContract           | Registers as a Super Representative candidate        | Yes                |
| 6   | AssetIssueContract              | Creates (issues) a new TRC-10 token                  | No                 |
| 8   | WitnessUpdateContract           | Updates Super Representative info                    | Yes                |
| 9   | ParticipateAssetIssueContract   | Participates in a TRC-10 token sale                  | No                 |
| 10  | AccountUpdateContract           | Updates account name or metadata                     | Yes                |
| 11  | FreezeBalanceContract           | Freezes TRX for resources or voting power            | Yes                |
| 12  | UnfreezeBalanceContract         | Unfreezes previously frozen TRX                      | Yes                |
| 13  | WithdrawBalanceContract         | Withdraws SR block rewards                           | Yes                |
| 14  | UnfreezeAssetContract           | Unfreezes TRC-10 tokens                              | No                 |
| 15  | UpdateAssetContract             | Updates TRC-10 token parameters                      | No                 |
| 16  | ProposalCreateContract          | Creates a governance proposal                        | Yes                |
| 17  | ProposalApproveContract         | Approves a governance proposal                       | Yes                |
| 18  | ProposalDeleteContract          | Deletes a proposal                                   | Yes                |
| 19  | SetAccountIdContract            | Sets a human-readable account ID                     | Yes                |
| 20  | CustomContract                  | Reserved for custom or undefined use                 | Yes                |
| 30  | CreateSmartContract             | Deploys a smart contract                             | Yes                |
| 31  | TriggerSmartContract            | Calls a deployed smart contract                      | No                 |
| 32  | GetContract                     | Queries smart contract information                   | No                 |
| 33  | UpdateSettingContract           | Updates smart contract consume_user_resource_percent | Yes                |
| 41  | ExchangeCreateContract          | Creates a token exchange pair                        | No                 |
| 42  | ExchangeInjectContract          | Adds liquidity to an exchange                        | No                 |
| 43  | ExchangeWithdrawContract        | Removes liquidity from an exchange                   | No                 |
| 44  | ExchangeTransactionContract     | Executes a token swap                                | No                 |
| 45  | UpdateEnergyLimitContract       | Updates contract origin_energy_limit                 | Yes                |
| 46  | AccountPermissionUpdateContract | Updates account permissions (multi-signature)        | Yes                |
| 48  | ClearABIContract                | Removes a contract’s ABI                             | Yes                |
| 49  | UpdateBrokerageContract         | Updates SR brokerage ratio                           | Yes                |
| 51  | ShieldedTransferContract        | Performs a private transfer                          | Yes                |
| 52  | MarketSellAssetContract         | Places a sell order (TRC10)                          | No                 |
| 53  | MarketCancelOrderContract       | Cancels a market order (TRC10)                       | No                 |
| 54  | FreezeBalanceV2Contract         | Freezes TRX (new resource model)                     | Yes                |
| 55  | UnfreezeBalanceV2Contract       | Unfreezes TRX (new model)                            | Yes                |
| 56  | WithdrawExpireUnfreezeContract  | Withdraws TRX after unfreeze period                  | Yes                |
| 57  | DelegateResourceContract        | Delegates resources to another account               | No                 |
| 58  | UnDelegateResourceContract      | Removes delegated resources                          | No                 |
| 59  | CancelAllUnfreezeV2Contract     | Cancels all pending unfreeze operations              | Yes                |


