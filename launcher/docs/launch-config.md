# Launch Config Schema (launch.toml / launch.json)

The `launcher-cli launch` command reads a JSON or TOML config file. The schema is the same for
both formats; only the serialization changes. Only one set of programs (launcher + protocol) is
deployed and shared across all launches, so program IDs live in the global launcher config.

## Top-level fields
- `name` *(string, optional)*: Human-friendly label used in CLI output.
- `rpc_url` *(string, required)*: Solana RPC endpoint to submit launch transactions.
- `payer_wallet` *(string, required)*: File path to the payer keypair used to sign launch transactions.
- `mint` *(object, required)*: Token mint configuration.
- `lp_pool` *(object, required)*: LP pool configuration.
- `vaults` *(array, optional)*: Vaults to initialize in the launch.
- `output` *(object, optional)*: Where to write a JSON summary of the launch.

Program IDs are no longer part of the per-launch config. They are shared across all launches and
must be defined in the global launcher config file (see `launcher-config.md`).

## Migration note
Existing launch configs that specify per-launch `programs` must be updated to remove those fields
and to reference shared program IDs in the global launcher config.

## `mint`
- `address` *(string, required)*: Client-generated mint address (required because signing happens client-side).
- `symbol` *(string, required)*: Token symbol to display in summaries.
- `decimals` *(number, optional; default = 11)*: Token decimal precision.
- `authority` *(string, optional)*: Mint authority public key.

## `lp_pool`
- `address` *(string, required)*: Client-generated LP pool address (required because signing happens client-side).
- `base_mint` *(string, optional)*: Base mint address. Defaults to the mint address.
- `quote_mint` *(string, required)*: Quote mint address.

## `vaults[]`
Each vault entry is an object with:
- `label` *(string, optional)*: Display name for the vault.
- `address` *(string, required)*: Client-generated vault address (required because signing happens client-side).
- `beneficiary` *(string, required)*: Beneficiary public key.
- `schedule` *(object, required)*: Linear unlock schedule.

### `schedule`
- `start_ts` *(number, required)*: Unix timestamp for vesting start.
- `cliff_ts` *(number, optional)*: Unix timestamp for the cliff.
- `period_seconds` *(number, required)*: Seconds per release period.
- `release_per_period` *(number, required)*: Tokens released per period.
- `period_count` *(number, required)*: Number of periods.

## `output`
- `path` *(string, required)*: File path to write a JSON summary.

## Example (TOML)
```toml
name = "ore-launch-local"
rpc_url = "https://api.devnet.solana.com"
payer_wallet = "payer.json"

[mint]
address = "6z9j48M9N2bnCqYH1L7D4Hq2wzjQTR2rN7uS3A3w7Lx1"
symbol = "ORE"
decimals = 11

[lp_pool]
address = "9K7sUP4cZUMHiZVnnVi6dDgA4u7p9Q5d7H4rK1N8B8Vq"
quote_mint = "So11111111111111111111111111111111111111112"

[[vaults]]
label = "team"
address = "8gQnYCTkHDf4gRXG2qJY7jW9c1ytbGz6rGxH4p1cQMyR"
beneficiary = "9xQeWvG816bUx9EPKQ4kZZf1y7VvY8p2Yf1aqd8v7uQf"

[vaults.schedule]
start_ts = 1_725_000_000
cliff_ts = 1_725_100_000
period_seconds = 86_400
release_per_period = 10_000
period_count = 180

[output]
path = "launch.output.json"
```

## Example (JSON)
```json
{
  "name": "ore-launch-local",
  "rpc_url": "https://api.devnet.solana.com",
  "payer_wallet": "payer.json",
  "mint": {
    "address": "6z9j48M9N2bnCqYH1L7D4Hq2wzjQTR2rN7uS3A3w7Lx1",
    "symbol": "ORE",
    "decimals": 11
  },
  "lp_pool": {
    "address": "9K7sUP4cZUMHiZVnnVi6dDgA4u7p9Q5d7H4rK1N8B8Vq",
    "quote_mint": "So11111111111111111111111111111111111111112"
  },
  "vaults": [
    {
      "label": "team",
      "address": "8gQnYCTkHDf4gRXG2qJY7jW9c1ytbGz6rGxH4p1cQMyR",
      "beneficiary": "9xQeWvG816bUx9EPKQ4kZZf1y7VvY8p2Yf1aqd8v7uQf",
      "schedule": {
        "start_ts": 1725000000,
        "cliff_ts": 1725100000,
        "period_seconds": 86400,
        "release_per_period": 10000,
        "period_count": 180
      }
    }
  ],
  "output": {
    "path": "launch.output.json"
  }
}
```
