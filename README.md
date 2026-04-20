# TaskFlow-Soroban 🚀

**TaskFlow-Soroban** - Decentralized Task Management System on Stellar

## Project Description

TaskFlow-Soroban is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and immutable platform for managing daily tasks and to-do lists directly on the blockchain. By leveraging Stellar's ledger, TaskFlow ensures that your productivity data is stored without reliance on centralized servers, giving users full sovereignty over their task lists.

The system allows users to create, view, and delete tasks with high efficiency. Each task is uniquely identified, categorized by its completion status, and stored within the contract's instance storage, ensuring that your to-do list is always persistent and tamper-proof.

## Project Vision

Our vision is to empower individuals with decentralized productivity tools by:

- **Data Sovereignty**: Moving personal task management from private company servers to a public, distributed ledger.
- **Transparency**: Ensuring every addition or removal of a task is verifiable on the Stellar network.
- **Efficiency**: Utilizing Stellar’s low-latency and low-fee environment to make Web3 applications as fast as traditional ones.
- **Trustless Integrity**: Creating a system where "what you write is what stays," guaranteed by smart contract logic rather than a central authority.

## Key Features

### 1. **On-Chain Task Creation**
- Create tasks with a single function call (`add_task`).
- Each task includes a title and an automatic unique ID.
- Data is stored directly in the Stellar Testnet ledger.

### 2. **Real-Time Task Retrieval**
- Fetch your entire to-do list using the `get_tasks` function.
- Fast and structured data output, ready for frontend integration.
- Synchronized directly with the latest blockchain state.

### 3. **Task Deletion**
- Remove completed or irrelevant tasks using their unique IDs via `delete_task`.
- Ensures efficient use of contract storage by cleaning up old data.
- Immediate updates reflected on the blockchain.

### 4. **Low-Cost Operations**
- Optimized for minimal gas consumption (XLM).
- Built on the modern Soroban framework for scalable execution.

## Contract Details

- **Contract ID**: `CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M`
- **Network**: Stellar Testnet

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar CLI

## Getting Started

To interact with the TaskFlow contract, use the following functions:

1. **Add a Task**:
   ```bash
   stellar contract invoke --id [CONTRACT_ID] --source-account [YOUR_ACCOUNT] --network testnet -- add_task --title "Finish Stellar Project"
