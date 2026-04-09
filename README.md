# ESTA-TRON:  TRON Historical Stats

### Powered by Substreams

### **🔍 What are Substreams?**

Substreams allow you to:

- **Extract blockchain data** in real-time as blocks are produced
- **Transform and filter** transaction data based on your requirements
- **Build composable modules** that can be combined for complex data processing
- **Stream data efficiently** with parallel processing and caching

---

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

1. **Get your StreamingFast API key from:**

- [https://app.streamingfast.io/](https://app.streamingfast.io/dashboard) - Sign up for free tier
- Create a new token and KEY (Copy your API KEY and API TOKEN)
- Now, in a terminal export your API key and create authentication function:

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

- For publishing and downloading packages from substreams.dev:

```bash
# Login with your registry token
substreams registry login #This will prompt for your API token
# Paste your token when prompted

# Or set environment variable
export SUBSTREAMS_REGISTRY_TOKEN="your-registry-token-here"
```

### **📦 Package Management**

#### **Download Pre-built Package**

The fastest way to get started is downloading the official TRON Foundational Modules:

```bash
# NOTICE: Only execute this step if an updated tron-foundational spkg is available, by default Im including this in the repo
# Navigate to substreams directory
cd substream 

# Download the official TRON foundational package
substreams pack tron-foundational@v0.1.2 -o bin/tron-foundational-v0.1.2.spkg

# Verify the package
substreams info bin/tron-foundational-v0.1.2.spkg
```

**Package Contents:**

- `**map_transactions`** - Extracts all non-failed transactions with full details
- `**index_transactions`** - Creates searchable transaction indices
- `**filtered_transactions**` - Filters transactions by type, contract, or other parameters

### **Historical Data Analysis**

```bash
# Analyze a specific date range (block start is specified in substreams.yaml config file)


substreams-sink-files run accountperm-v0.1.0.spkg \
  map_my_data \
   --encoder=lines \
  --output-dir ./output \
  --state-store ./state.yaml \
  --file-block-count=28880
  
 #WORKING command , bundled by quarter per file
  substreams-sink-files run accountperm-v0.1.0.spkg \
  map_my_data \
  jsonl_out \
  --encoder=lines \
  --output-dir ./output \
  --state-store ./state.yaml \
  --file-block-count=2592000
```

### Supported Contract Types in package:


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
| 31  | TriggerSmartContract            | Calls a deployed smart contract                      | Yes                |
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
| 57  | DelegateResourceContract        | Delegates resources to another account               | Yes                |
| 58  | UnDelegateResourceContract      | Removes delegated resources                          | Yes                |
| 59  | CancelAllUnfreezeV2Contract     | Cancels all pending unfreeze operations              | Yes                |


