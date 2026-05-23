## Contract Details

- Contract Address: CAQDYKRD3F3A4FMHOUEIYGGUCA4RMFV5PFLOBL2GWJCUSPLUS4CMZK62
  (Screenshot has been removed)

# Soroban Todo List Smart Contract

Simple decentralized To-Do List smart contract built with Rust and Soroban.

## Features

- Create todo
- Get all todos
- Toggle completed status
- Delete todo
- Wallet authentication
- Owner validation

---

# Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

# Todo Structure

```rust
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub owner: Address,
}
```

---

# Contract Functions

## 1. get_todos

Get all todos stored in contract storage.

### Example

```bash
get_todos()
```

---

## 2. create_todo

Create new todo.

### Parameters

| Name | Type |
|---|---|
| owner | Address |
| title | String |

### Example

```bash
create_todo(
  owner,
  "Learn Soroban"
)
```

---

## 3. toggle_todo

Toggle todo completion status.

### Parameters

| Name | Type |
|---|---|
| owner | Address |
| id | u64 |

---

## 4. delete_todo

Delete todo by ID.

### Parameters

| Name | Type |
|---|---|
| owner | Address |
| id | u64 |

---

# Authentication

This contract uses:

```rust
owner.require_auth();
```

Meaning:
- wallet owner must approve transaction
- only owner can modify/delete their todo

---

# Storage

## TODOS

Stores all todo data.

## COUNTER

Stores incremental todo ID.

---

# Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

# Deploy Contract

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/todo_contract.wasm \
--source alice \
--network testnet
```

---

# Example Workflow

## Create Todo

```text
create_todo()
↓
todo stored on blockchain
```

## Toggle Todo

```text
toggle_todo()
↓
completed = true / false
```

## Delete Todo

```text
delete_todo()
↓
todo removed from storage
```

---

# Example Todo Data

```json
{
  "id": 0,
  "title": "Learn Soroban",
  "completed": false
}
```

---

# Security

- Uses wallet authentication
- Prevents unauthorized modification
- Prevents unauthorized deletion

---

# Future Improvements

- Edit todo title
- Per-user todo query
- Pagination
- Due date
- Priority
- Search/filter
- Better storage optimization

---

# License

MIT