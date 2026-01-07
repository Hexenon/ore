# Launcher Global Config (launcher-config.toml / launcher-config.json)

The launcher config stores shared settings that apply to every launch. Only one set of programs
(launcher + protocol) is deployed and reused across all launches.

`launcher-cli launch` loads this file from `launcher-config.toml` by default. Override the
location with `--launcher-config PATH`.

## Top-level fields
- `programs` *(object, required)*: Program IDs shared across all launches.

## `programs`
- `ore` *(string, required)*: ORE program ID.
- `mining` *(string, required)*: Mining program ID.
- `rewards_lock` *(string, required)*: Rewards lock program ID.

If any program ID is missing, `launcher-cli` fails validation.

## Example (TOML)
```toml
[programs]
ore = "oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv"
mining = "6b2gkN3mEVkzy7K1u7Z7hDkKB4D3k6bPSi3b8KnN1Uyh"
rewards_lock = "7j4a1j6DPFG8w6G1ZL5bR2u5T8G1w6Z9AxZ4C8v6z6Hd"
```

## Example (JSON)
```json
{
  "programs": {
    "ore": "oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv",
    "mining": "6b2gkN3mEVkzy7K1u7Z7hDkKB4D3k6bPSi3b8KnN1Uyh",
    "rewards_lock": "7j4a1j6DPFG8w6G1ZL5bR2u5T8G1w6Z9AxZ4C8v6z6Hd"
  }
}
```
