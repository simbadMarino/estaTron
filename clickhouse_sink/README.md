# Clickhouse sink module

### **Historical Data Analysis: Postgresql sink (Work in progress)**

Pre-requirements:
Install postgresql & substreams-sink-sql

MacOS instructions:

```bash
brew install postgresql
brew install streamingfast/tap/substreams-sink-sql
```

#### 1. Start postgresql services

```bash
 brew services start postgresql@18  #Assuming postgresql version 18 is installed
```

#### 2. Create the database

```bash
 psql postgres -c "CREATE DATABASE tron_all_contract_types;"
 #Check your DB was created:
 psql postgres -c "\l"
```

#### 3. Apply your schema

```bash
 psql -d tron_all_contract_types -f schema.sql
```

#### 4. Build your substreams

```bash
 substreams build
```

```bash
substreams-sink-sql run \
  "postgres://$(whoami):@localhost:5432/tron?sslmode=disable" \
  "./substreams.yaml" \
  map_my_data
```