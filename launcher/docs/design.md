# Launcher Workspace Design

## Goals
- Provide a standalone workspace for launcher-related crates.
- Separate CLI, backend orchestration, mining API types, mining program logic, and reward locking policies.
- Enable future integration with existing ORE services without impacting the root workspace.

## Workspace Layout
- `launcher-cli`: End-user CLI for starting and monitoring launcher services.
- `launcher-backend`: Core orchestration layer for managing mining and rewards workflows.
- `mining-api`: Shared API types used between the CLI and backend.
- `mining-program`: Mining estimation and execution helpers.
- `rewards-lock`: Reward locking policy models.

## Data Flow
1. `launcher-cli` collects configuration and user intent.
2. `launcher-backend` validates configuration and coordinates execution.
3. `mining-program` handles mining-specific operations.
4. `rewards-lock` enforces reward locking policies.
5. `mining-api` defines shared request/response structures between layers.

## Configuration Strategy
- TOML for operator-friendly configuration files.
- JSON for API-driven overrides and tooling integrations.
- Environment variables reserved for secrets and ephemeral overrides.

## Future Considerations
- Add persistence for backend state.
- Add gRPC interface for remote control.
- Align mining API types with on-chain program schemas.
