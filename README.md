# ESTA-TRON: TRON data sink & query

---

TRON historical data , particularly related to transaction-type decomposition  is slow to extract directly from a TRON Fullnode and not fully available in TRON explorers or Data platforms, this repo closes this gap by providing ready-to-use Substream modules and TRON blockchain DB backups for efficient Data Analytics purposes.

This project explores a couple data output (sinks) configurations:

- JSONL : Simple line based format, useful for relatively low transaction volume analysis in plain text.
- Clickhouse DB: More complex, useful for large / full TRON Blockchain data querying.

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

---

## Project Structure 
```
.
├── README.md
├── clickhouse_sink   # clickhouse DB Data sinking Substreams module
│   ├── Cargo.lock
│   ├── Cargo.toml    #Project deps, metadata and build settings
│   ├── README.md     # clickhouse sinking and config specific guidelines 
│   ├── buf.gen.yaml
│   ├── localdata
│   ├── output 
│   ├── proto
│   ├── replay.log
│   ├── schema.sql    #Your clickhouse DB schema
│   ├── src           #lib.rs home, data transformation and output config takes place here
│   ├── substreams.yaml   #Substreams configuration file, defines your architecture, used for build process
│   └── target
├── jsonl_sink        # jsonl Data sinking Substreams module
│   ├── Cargo.lock
│   ├── Cargo.toml    #Project deps, metadata and build settings
│   ├── README.md     # jsonl sinking and config specific guidelines 
│   ├── buf.gen.yaml
│   ├── jsonl-sink-v0.1.0.spkg    # pre-compiled data transformation substreams package 
│   ├── localdata
│   ├── output        # default jsonl files output dir 
│   ├── proto
│   ├── src           #lib.rs home, data transformation and output config takes place here
│   ├── state.yaml
│   ├── substreams.yaml   #Substreams configuration file, defines your architecture, used for build process
│   └── target
├── scripts
│   ├── jsonl_to_clickhouse.py    # Python utility to sink jsonl files into clickhouse DB
└── substream         # Official substream tron foundational package
    ├── bin
    └── output
```

--- 
## **Substreams: Sink Files (jsonl)**

The JSONL Sink prebuild module exports data as JSONL files based on ```src/lib.rs``` & ```substreams.yaml``` fields and contract_type filtering.

- For details on contract_types refer to: [substreams.yaml](https://github.com/simbadMarino/estaTron/blob/main/jsonl_sink/substreams.yaml#L40)  
- For details ontransaction fields refer to: [src/lib.rs](https://github.com/simbadMarino/estaTron/blob/main/jsonl_sink/src/lib.rs#L111)


### Getting Started

1. Clone this repo 
```bash
git clone https://github.com/simbadMarino/estaTron.git
```
2. Navigate to json_sink subproject
```bash
cd jsonl_sink
```

Start extracting data to output/ dir from block: ```--start-block``` to ```--stop-block```, with ```--file-block-count``` number of blocks per file.
```state.yaml``` file stores cursor state, useful to recover from latest sinked block in case of network error.  

```bash
substreams-sink-files run jsonl-sink-v0.1.0.spkg \
  jsonl_mod \
  --encoder=lines \
  --output-dir ./output \
  --state-store ./state.yaml \  #
  --start-block 81041850 \
  --stop-block  81044000 \
  --file-block-count=28880
  ```

You should start seeing jsonl files recorded in output/ folder after some time.

Tips:

```bash
# Use the following file-block-count args depending on your data aggregation needs:
# By day: 28880
# By month: 866400
# By Quarter: 2592000
```
#### Optional: Sink jsonl into a clickhouse DB (Python script)

For details please refer to the detailed guideline below:
```bash
nano jsonl_sink/README.md
```

---
## **Substreams: Sink to Clockhouse DB**

### Pre-requirements:
Install clickhouse & substreams-sink-sql

```bash
brew install streamingfast/tap/substreams-sink-sql
```

#### Install clickhouse (bare):
Optional: Run a clickhouse [docker container](https://github.com/streamingfast/substreams-sink-sql#prerequisites) instead.
```bash
curl https://clickhouse.com/ | sh
sudo ./clickhouse install
```

#### Start clickhouse server

```bash
sudo clickhouse start 
```

#### Run clickhouse client:

##### First time password creation:

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

#### 2. Create transactions TABLE from schema

```bash
clickhouse-client --password < schema.sql
```

#### 3. Setup DSN (Data Source Name) EnvVar
```bash
export CLICKHOUSE_DSN='clickhouse://default:your_clickhouse-client_password@localhost:9000/tron'
```

#### 4. Setup DB

```bash
substreams-sink-sql setup "$CLICKHOUSE_DSN" clickhouse-sink-v0.1.0.spkg
```

### Run the Substream

Run your substream and start sinking to your DB:

```bash
# The command starts sinking your filtered data from block 70937500, sinks 7220 (which accounts about 1/4 of a day), adjust starting block and end block as needed.
substreams-sink-sql run "$CLICKHOUSE_DSN" ./clickhouse-sink-v0.1.0.spkg 70937500:+7220 -e mainnet.tron.streamingfast.io:443 --undo-buffer-size 50
```



---
## **Customizing your Substream module**
For additional data transformations in your module proceed as below:

### Step 1: Adjust substreams.yaml , lib.rs or schema.sql as needed.

### Step 2: Build your new package once ready:


```bash
 substreams build
```

### Step 3: Adjust your DB if needed
For any fields changes consider updating your DB and TABLES. 

#### CLickhouse DB "RESET" (DROP DB and Schema import):

```bash
cd clickouse_sink   ##cd into clickhouse_sink dir
clickhouse-client --password --query "DROP TABLE IF EXISTS tron.transactions"
clickhouse-client --password < schema.sql
```
#### Substreams sink sql Setup:

```bash
substreams-sink-sql setup "$CLICKHOUSE_DSN" clickhouse-sink-v0.1.0.spkg
```

For advanced clickhouse DB management, please refer to clickhouse [docs](https://clickhouse.com/docs/updating-data)

---

## TROUBLESHOOTING:
```bash
ERRO (sink-sql) unable to retrieve cursor: cursor module hash mismatch, refusing to continue because flag '--on-module-hash-mismatch=error' (defaults) is set, you can change to 'warn' or 'ignore' after a module upgrade, and if cursor is stil at 0 (meaning you still didnt sink any significant data) you can reset sink state by:
```
Solution: TRUNCATE (delete all rows) from cursors TABLE

```bash
clickhouse-client --password --query "TRUNCATE TABLE tron.cursors;"
clickhouse-client --password --query "TRUNCATE TABLE tron.substreams_history;"
```
---

```bash
ERRO (sink-sql) unable to setup sql sinker: load tables: retrieving table and schemaName: getting tables from schema: querying tables from system.tables: dial tcp [::1]:9000: connect: connection refused
```
Solution: Make sure your clickhouse server is running and password was properly configured during CLICKHOUSE_DSN envar setting.

---
```bash
ERRO (sink-sql) stream auth failure: rpc error: code = Unauthenticated desc = required authorization token not found. Please provide a valid JWT token via 'authorization' header or an API key via 'x-api-key' header
```
Solution: When using streamingfast substreams endpoint, make sure you authenticate as described in the ```Getting Started``` section

---

## **Clickhouse DB Querying**

The most efficient way to analyze the resulting data is trough clickhouse SQL queries. 

Below some queries examples:

Note: Make sure to adjust the database name per your own.

```SQL
SELECT
-- Gets trx burn stats by quarter
    contract_type,
    count() AS txn_count,
    toStartOfQuarter(toDateTime(block_timestamp / 1000)) AS quarter,
    sum(total_fee_burn/1000000) AS total_fee
FROM tron.transactions
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
    toString(toYear(fromUnixTimestamp64Milli(block_timestamp, 'UTC'))),
    '-Q',
    toString(toQuarter(fromUnixTimestamp64Milli(block_timestamp, 'UTC')))
  ) AS quarter,
  sum(total_fee_burn / 1000000.0) AS burned_TRX,
  countIf(toInt32OrZero(perm_threshold) > 1) AS high_threshold_txn_count,
  uniqExact(`from`) AS unique_from_addresses
FROM tron.transactions
GROUP BY contract_type, quarter
ORDER BY quarter, burned_TRX DESC;
```


```SQL
--Gets txn count, trx burn & multi-sign stats for each contract_type
SELECT
  contract_type,
  count() AS txn_count,
  min(block_number) AS start_block,
  max(block_number) AS end_block,
  concat(
    toString(toYear(fromUnixTimestamp64Milli(block_timestamp, 'UTC'))),
    '-Q',
    toString(toQuarter(fromUnixTimestamp64Milli(block_timestamp, 'UTC')))
  ) AS quarter,
  sum(total_fee_burn / 1000000.0) AS burned_TRX,
  countIf(signature_count > 1) AS multisign_count,
  uniqExact(`from`) AS unique_from_addresses
FROM tron.transactions
GROUP BY contract_type, quarter
ORDER BY quarter, burned_TRX DESC;
```

```SQL
--Retrieves transactions table size
SELECT
    formatReadableSize(sum(data_compressed_bytes)) AS compressed,
    formatReadableSize(sum(data_uncompressed_bytes)) AS uncompressed,
    round(sum(data_uncompressed_bytes)/sum(data_compressed_bytes), 2) AS ratio
FROM system.parts
WHERE database = 'tron'
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

## **Clickhouse Database backup**

Make sure you have a propper strategy to perform backups, especially before a significant data sinking.

Bare metal server guide:

1. Run a root shell:

```bash
sudo -i # macOS
su #Linux
```

2. Navigate to clickhouse server path

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

3. configure your backup output:

```bash
sudo nano /etc/clickhouse-server/config.d/backup_disk.xml
```

```xml
<clickhouse>
  <storage_configuration>
    <disks>
      <backups>
        <type>local</type>
        <path>/path/to_your/clickhouse_backups/</path>
      </backups>
    </disks>
  </storage_configuration>

  <backups>
    <allowed_disk>backups</allowed_disk>
    <allowed_path>/path/to_your/clickhouse_backups/</allowed_path>
  </backups>
</clickhouse>
```

4. Restart your clickhouse server

```bash
sudo pkill clickhouse-server
sudo clickhouse start
```

5. Run the backup action

```bash
clickhouse-client --password --query "
BACKUP DATABASE tron_account_perm_update
TO Disk('backups', 'tron_backup_$(date +%F).zip')"
```

### Restore DB from disk

```bash
clickhouse-client --password --query "RESTORE DATABASE my_db AS my_db_restored
FROM File('/Users/YOUR_USER/clickhouse_backups/my_db_backup');"
```

## **📦 Substreams Package Management**

### **Update Pre-built Official TRON foundational Package**

NOTICE: Only execute the following steps if an updated tron-foundational spkg is available, by default, the tron foundational package is included in `/substream`

```bash
# Navigate to substreams directory
cd substream 

# Download the official TRON foundational package
substreams pack tron-foundational@v0.1.X -o bin/tron-foundational-v0.1.X.spkg

# Verify the package
substreams info bin/tron-foundational-v0.1.X.spkg
```

---

## Annex 1: TRON Transaction Type Reference:

| ID  | Contract Type                   | Description                                          | 
| --- | ------------------------------- | ---------------------------------------------------- | 
| 0   | AccountCreateContract           | Creates a new account on the network                 | 
| 1   | TransferContract                | Transfers native TRX between accounts                |
| 2   | TransferAssetContract           | Transfers TRC-10 tokens between accounts             |
| 3   | VoteAssetContract               | Votes using TRC-10 assets (deprecated/rare)          |
| 4   | VoteWitnessContract             | Votes for Super Representatives                      |
| 5   | WitnessCreateContract           | Registers as a Super Representative candidate        |
| 6   | AssetIssueContract              | Creates (issues) a new TRC-10 token                  |
| 8   | WitnessUpdateContract           | Updates Super Representative info                    |
| 9   | ParticipateAssetIssueContract   | Participates in a TRC-10 token sale                  |
| 10  | AccountUpdateContract           | Updates account name or metadata                     |
| 11  | FreezeBalanceContract           | Freezes TRX for resources or voting power            |
| 12  | UnfreezeBalanceContract         | Unfreezes previously frozen TRX                      |
| 13  | WithdrawBalanceContract         | Withdraws SR block rewards                           |
| 14  | UnfreezeAssetContract           | Unfreezes TRC-10 tokens                              |
| 15  | UpdateAssetContract             | Updates TRC-10 token parameters                      |
| 16  | ProposalCreateContract          | Creates a governance proposal                        |
| 17  | ProposalApproveContract         | Approves a governance proposal                       |
| 18  | ProposalDeleteContract          | Deletes a proposal                                   |
| 19  | SetAccountIdContract            | Sets a human-readable account ID                     |
| 20  | CustomContract                  | Reserved for custom or undefined use                 |
| 30  | CreateSmartContract             | Deploys a smart contract                             |
| 31  | TriggerSmartContract            | Calls a deployed smart contract                      |
| 32  | GetContract                     | Queries smart contract information                   |
| 33  | UpdateSettingContract           | Updates smart contract consume_user_resource_percent |
| 41  | ExchangeCreateContract          | Creates a token exchange pair                        |
| 42  | ExchangeInjectContract          | Adds liquidity to an exchange                        |
| 43  | ExchangeWithdrawContract        | Removes liquidity from an exchange                   |
| 44  | ExchangeTransactionContract     | Executes a token swap                                |
| 45  | UpdateEnergyLimitContract       | Updates contract origin_energy_limit                 |
| 46  | AccountPermissionUpdateContract | Updates account permissions (multi-signature)        |
| 48  | ClearABIContract                | Removes a contract’s ABI                             |
| 49  | UpdateBrokerageContract         | Updates SR brokerage ratio                           |
| 51  | ShieldedTransferContract        | Performs a private transfer                          |
| 52  | MarketSellAssetContract         | Places a sell order (TRC10)                          |
| 53  | MarketCancelOrderContract       | Cancels a market order (TRC10)                       |
| 54  | FreezeBalanceV2Contract         | Freezes TRX (new resource model)                     |
| 55  | UnfreezeBalanceV2Contract       | Unfreezes TRX (new model)                            |
| 56  | WithdrawExpireUnfreezeContract  | Withdraws TRX after unfreeze period                  |
| 57  | DelegateResourceContract        | Delegates resources to another account               |
| 58  | UnDelegateResourceContract      | Removes delegated resources                          |
| 59  | CancelAllUnfreezeV2Contract     | Cancels all pending unfreeze operations              

## Annex 2: TRON Block number-quarter equivalence table

```bash
TRON Starting block by quarter (Approximate):
2019-Q1	5416000
2019-Q2	8000000
2019-Q3	10613000
2019-Q4	13254000
2020-Q1	15896000
2020-Q2	18509000
2020-Q3	21121000
2020-Q4	23763000
2021-Q1	26405000
2021-Q2	28989000
2021-Q3	31601000
2021-Q4	34243000
2022-Q1	36885000
2022-Q2	39469000
2022-Q3	42081000
2022-Q4	44723000
2023-Q1	47365000
2023-Q2	49949000
2023-Q3	52562000
2023-Q4	55203000
2024-Q1	57845000
2024-Q2	60457000
2024-Q3	63070000
2024-Q4	65712000
2025-Q1	68353000
2025-Q2	70937500
2025-Q3	73550000
2025-Q4	76192000
```