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
### Setup Clickhouse sink substreams module

```bash
 substreams build
```
