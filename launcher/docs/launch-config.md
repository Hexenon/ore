# Launcher Config Schema (launch.toml / launch.json)

The `launcher-cli launch` command reads a JSON or TOML config file. The schema is the same for
both formats; only the serialization changes.

## Top-level fields
- `name` *(string, optional)*: Human-friendly label used in CLI output.
- `rpc_url` *(string, required)*: Solana RPC endpoint to submit launch transactions.
- `payer_wallet` *(string, required)*: File path to the payer keypair used to sign launch transactions.
- `programs` *(object, optional)*: Program IDs to use for the launch.
- `mint` *(object, required)*: Token mint configuration.
- `lp_pool` *(object, required)*: LP pool configuration.
- `vaults` *(array, optional)*: Vaults to initialize in the launch.
- `output` *(object, optional)*: Where to write a JSON summary of the launch.

## `programs`
- `ore` *(string, optional)*: ORE program ID.
- `mining` *(string, optional)*: Mining program ID.
- `rewards_lock` *(string, optional)*: Rewards lock program ID.

If any program ID is omitted, `launcher-cli` generates a new public key and prints it.

## `mint`
- `address` *(string, optional)*: Mint address. Omit to generate a new public key.
- `symbol` *(string, required)*: Token symbol to display in summaries.
- `decimals` *(number, optional; default = 11)*: Token decimal precision.
- `authority` *(string, optional)*: Mint authority public key.

## `lp_pool`
- `address` *(string, optional)*: LP pool address. Omit to generate a new public key.
- `base_mint` *(string, optional)*: Base mint address. Defaults to the mint address.
- `quote_mint` *(string, required)*: Quote mint address.

## `vaults[]`
Each vault entry is an object with:
- `label` *(string, optional)*: Display name for the vault.
- `address` *(string, optional)*: Vault address. Omit to generate a new public key.
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

[programs]
ore = "oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv"
mining = "6b2gkN3mEVkzy7K1u7Z7hDkKB4D3k6bPSi3b8KnN1Uyh"
rewards_lock = "7j4a1j6DPFG8w6G1ZL5bR2u5T8G1w6Z9AxZ4C8v6z6Hd"

[mint]
symbol = "ORE"
decimals = 11

[lp_pool]
quote_mint = "So11111111111111111111111111111111111111112"

[[vaults]]
label = "team"
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
  "programs": {
    "ore": "oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv",
    "mining": "6b2gkN3mEVkzy7K1u7Z7hDkKB4D3k6bPSi3b8KnN1Uyh",
    "rewards_lock": "7j4a1j6DPFG8w6G1ZL5bR2u5T8G1w6Z9AxZ4C8v6z6Hd"
  },
  "mint": {
    "symbol": "ORE",
    "decimals": 11
  },
  "lp_pool": {
    "quote_mint": "So11111111111111111111111111111111111111112"
  },
  "vaults": [
    {
      "label": "team",
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
