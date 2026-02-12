# Accel Week 1 Challenge

A Solana Token-2022 vault program with whitelist-based transfer hooks and memo-verified deposits.

## Features

- **Token-2022 Integration**: Custom mint with transfer hook and metadata extensions
- **Whitelist System**: Admin-controlled user whitelist enforcement via transfer hooks
- **Secure Deposits**: Memo-based deposit verification with nonce protection
- **Vault Management**: Deposit/withdraw functionality with balance tracking
- **Transfer Hook**: Automatic validation of whitelisted users on all transfers

## Prerequisites

- Rust 1.75+
- Solana CLI 1.18+
- Anchor 0.32.1
- Node.js & Yarn

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd accel_week1_challenge

# Install dependencies
yarn install

# Build the program
anchor build
```

## Testing

```bash
anchor test
```

## Program Structure

### Instructions

- `create_mint` - Create Token-2022 mint with transfer hook
- `init_vault_config` - Initialize vault configuration PDA
- `init_extra_account_meta_list` - Setup transfer hook metadata
- `mint_token` - Mint tokens to users
- `add_to_whitelist` - Add user to whitelist (admin only)
- `remove_from_whitelist` - Remove user from whitelist (admin only)
- `deposit` - Deposit tokens with memo verification
- `withdraw` - Withdraw tokens from vault
- `transfer_hook` - Validate whitelist on transfers

### State Accounts

- **VaultConfig**: Stores admin, mint, and vault ATA addresses
- **User**: Tracks user balance and whitelist status


## Key Concepts

**Memo Format**: Deposits require memo in format

**Transfer Hook**: Automatically validates that the token sender is whitelisted before allowing any transfer.

## Withdraw + Transfer Hook Reentrancy Note

With the current architecture, making `withdraw` perform a real Token-2022 transfer from `vault_ta` causes a runtime failure:

- `withdraw` currently updates internal vault balance only (no token CPI transfer).
- Transfer-hook logic is implemented in the same on-chain program.
- If `withdraw` calls Token-2022 `TransferChecked`, the token program attempts to invoke the configured transfer hook.
- Because the hook points back to the same program that is already executing, Solana rejects it with:
	`Cross-program invocation reentrancy not allowed`.

### Current Status

- The repository is currently restored to the passing baseline (`6 passing, 1 pending`).

### Recommended Path for Real Withdraw Transfers

- Best option: split transfer-hook logic into a separate hook program ID, while keeping vault business logic in this program.
- Then `withdraw` can safely CPI-transfer from the vault PDA, and Token-2022 can invoke the external hook program without reentry conflict.

### Alternative

- Remove/rotate transfer-hook enforcement for vault-withdraw flows (usually less desirable for policy consistency).

## Program ID

```
7cwdqRZ1Ap8ano7Vsdwk9NfkB26tWf8bSba3Bvb2G6JM
```

## License

ISC
