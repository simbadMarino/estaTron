# JSONL Sink module

### **Historical Data Analysis: JSONL Sink**

```bash
# Analyze a specific date range (block start is specified in substreams.yaml config file)

#Split files by day
substreams-sink-files run allparams-v0.1.0.spkg \
  map_my_data \
  --encoder=lines \
  --e mainnet.tron.streamingfast.io:443 \
  --output-dir ./output \
  --state-store ./state.yaml \
  --file-block-count=28880

#Split files by month
  substreams-sink-files run allparams-v0.1.0.spkg \               
  map_my_data \  --encoder=lines \
  --output-dir ./output \                 
  --state-store ./state.yaml \
  --file-block-count=866400
  
 #Split files by quarter
  substreams-sink-files run allparams-v0.1.0.spkg \
  map_my_data \
  jsonl_out \
  --encoder=lines \
  --output-dir ./output \
  --state-store ./state.yaml \
  --file-block-count=2592000
```


### **Sink jsonl to clickhouse**

A script is available to sink jsonl to a clickhouse DB, reduce storage and enabling quering data

Quick Start:

Install clickhouse:

```bash
curl https://clickhouse.com/ | sh
sudo ./clickhouse install
```

Start clickhouse server

```bash

sudo clickhouse start 
```

Run clickhouse client and Create DB:

First time password creation:

```bash

clickhouse-client --password 
```

Install script deps:

```bash
python3 -m venv .venv    # Creates virtual Python environmen
source .venv/bin/activate   #Activates Virtual environment
pip install lz4 clickhouse-cityhash clickhouse-driver   #Install script deps
```

Execute script:

```bash
python jsonl_to_clickhouse.py \
  --host localhost \
  --user default \
  --password yourpassword \
  --database tron_db \
  --table transactions \
  --create-table
```
