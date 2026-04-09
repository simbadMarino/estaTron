# accountperm Substreams modules

This package was initialized via `substreams init`, using the `tron-transactions` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

### `map_my_data`

This module outputs filtered transactions based on the filter provided. Take a look at the input parameters in `substreams.yaml` to configure the filter.

**NOTE:** We do NOT recommend outputting the full Transaction object. Instead, you should filter on the specific parts that your application needs.
