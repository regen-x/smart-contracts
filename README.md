# RegenX Smart Contract Template

**RegenX Smart Contracts** provides the core functionality to deploy Smart Contracts to the Stellar Network using the **Stellar Soroban** framework. It includes foundational contract logic, deployment scripts, and test configurations to accelerate your development process.

## 🚀 Features

- Smart Contract with fundamental **storage, methods, and event handling**.
- **Optimized builds** for efficient deployment.
- Built-in **unit tests** to validate contract behavior.
- **Seamless deployment** to **Testnet** or **Public**.

## 📌 Prerequisites

Ensure the following dependencies are installed before starting:

- **Rust & Cargo** (via `rustup`) - [Installation Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- **Stellar SDK** (Soroban CLI) - [Installation Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup#install-the-stellar-cli)
- **Docker** _(Optional, for local test environments)_ - [QuickStart](https://github.com/stellar/quickstart)

### 🔹 Recommended VSCode Extensions _(Optional)_

For an enhanced development experience, consider using:

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - **Rust language support**
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - **Debugger for Rust**
- [BetterTOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - **TOML syntax & linting**

---

## 💻 Development Setup

### 1️⃣ Clone the Repository

#### Using HTTPS:

```bash
git clone https://github.com/regenx/template-stellar-smart-contract.git .
cd template-stellar-smart-contract
```

#### Using SSH:

```bash
git clone git@github.com:regenx/template-stellar-smart-contract.git .
cd template-stellar-smart-contract
```

---

### 2️⃣ Compile the Contract

Compile the smart contract to generate the **WASM binary**:

```bash
stellar contract build
```

The output `.wasm` file will be located at:

```text
target/wasm32-unknown-unknown/release/base_contract.wasm
```

---

## 📜 Contract Methods

### 🔹 Admin Management

- **`set_admin(admin: Address) -> Result<Address, Error>`**  
  Updates the admin address of the contract.

- **`get_admin() -> Address`**  
  Retrieves the current admin address.

- **`has_admin() -> bool`**  
  Checks if an admin is set.

### 🔹 Token Management

- **`set_reference_token(token_address: Address) -> Result<ReferenceToken, Error>`**  
  Sets the reference token for transactions.

- **`issue_token(token: Address, price: i128, supply: i128, owner: Address) -> Result<Token, Error>`**  
  Issues a new token with the specified price, supply, and owner.

- **`transfer(investor: Address, token_address: Address, amount: i128) -> Result<(), Error>`**  
  Transfers a token to an investor.

### 🔹 Offer Management

- **`create_offer(token_address: Address, amount: i128, price: i128, owner: Address) -> Result<(i128, Offer), Error>`**  
  Creates a new offer to sell tokens.

- **`update_offer(offer_id: i128, price: i128) -> Result<(i128, Offer), Error>`**  
  Updates the price of an existing offer.

- **`cancel_offer(offer_id: i128) -> Result<(i128, Offer), Error>`**  
  Cancels an active offer.

- **`buy_offer(offer_id: i128, buyer: Address) -> Result<(i128, Offer), Error>`**  
  Allows a buyer to purchase an offer.

- **`read_offer(offer_id: i128) -> Result<(i128, Offer), Error>`**  
  Reads the details of a specific offer.

---

### 3️⃣ Optimize the Contract _(Recommended for Deployment)_

For **smaller and more efficient** contracts, optimize the `.wasm` binary:

```bash
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/base_contract.wasm
```

##### Example Output:

```text
Reading: target/wasm32-unknown-unknown/release/base_contract.wasm (3452 bytes)
Optimized: target/wasm32-unknown-unknown/release/base_contract.optimized.wasm (2877 bytes)
```

> ℹ️ **Optimized contracts are required for Public deployments** to reduce network fees.

---

### 4️⃣ Run Tests

Execute unit tests to verify contract logic:

```bash
cargo test
```

---

### 5️⃣ Deploy the Contract

Ensure your environment is set up (e.g., **Testnet**) before deployment:

```bash
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/base_contract.wasm --network testnet --source S...
```

For **Mainnet** deployments, use the **optimized** version:

```bash
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/base_contract.optimized.wasm --network mainnet --source S...
```

##### Example Output:

```text
CCJGTFIZMCS7CD3D5DHDJXAF6GWLGQKO7YUVGDYDFQ5KEGCTSCWZFJY3
```

---

### 6️⃣ Install Compiled Contract _(For Multiple Instances)_

If you need to **install** the contract without immediate execution:

```bash
stellar contract install --wasm target/wasm32-unknown-unknown/release/base_contract.wasm --network testnet --source S...
```

For **Mainnet**, use:

```bash
stellar contract install --wasm target/wasm32-unknown-unknown/release/base_contract.optimized.wasm --network mainnet --source S...
```

##### Example Output:

```text
695da0050d5481fe1a1dc0edc94792223b4a152b80f8a1e360ec05a773c06196
```

---

## 📁 Project Structure

```text
template-stellar-smart-contract/
├── src/
│   ├── lib.rs          # Main entry point for the smart contract
│   ├── contract.rs     # Core smart contract logic
│   ├── events/         # Definitions for contract events
│   ├── methods/        # Contract methods and function implementations
│   ├── storage/        # Data structures & storage logic
│   ├── tests/          # Unit & integration tests
│   │   ├── config/     # Test environment setup files
│   │   └── ...         # Unit test files for individual contract functions
├── Cargo.toml          # Rust project dependencies & settings
├── README.md           # Project documentation
└── .gitignore          # Ignored files & directories
```

---

## 📝 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

### 🌱 Built for **RegenX & Stellar Soroban** 🌍

This library is designed to streamline **RegenX smart contract** development on the **Stellar Soroban network**, ensuring a seamless experience from **writing, testing, optimizing, and deploying** contracts.
