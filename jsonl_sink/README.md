# JSONL Sink Optional transformation

### **Sink jsonl to clickhouse**

A script is available to sink jsonl to a clickhouse DB, reduce storage and enabling quering data

Quick Start:

Install clickhouse (bare):

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
  --password your_clickhouse_password \
  --database your_tron_db_name \
  --table transactions \
  --create-table
```

Notes: 
1. If your clickhouse server is running as a docker container you might need to configure the script port accordingly.
2. Current instructions assumes clickhouse running locally to improve querying speed on MacOS. 
