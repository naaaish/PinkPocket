# PinkPocket

**PinkPocket** — Blockchain-Based Personal Expense Tracker on Stellar Soroban

## Project Description

PinkPocket is a decentralized personal finance tracking application built on the Stellar blockchain using the Soroban Smart Contract SDK. 

The application allows users to record, monitor, and manage personal expenses securely on-chain through a smart contract system. Instead of relying on traditional centralized databases, PinkPocket stores expense data directly within blockchain contract storage, ensuring transparency, persistence, and reliability. 

Users can:

* Add expense records
* View all stored expenses
* Delete expenses
* Update expense notes
* Monitor spending summaries

Each expense is uniquely identified and stored within the smart contract instance storage.

---

# Project Vision

PinkPocket aims to provide a simple, transparent, and decentralized solution for personal financial tracking.

Our vision includes:

* **Decentralized Financial Records**
  Moving personal expense management from centralized systems to blockchain-based storage.

* **Data Ownership**
  Giving users complete ownership and control over their financial records.

* **Transparency and Reliability**
  Ensuring expense records remain consistent and accessible through blockchain technology.

* **Secure Smart Contract Storage**
  Leveraging Soroban smart contracts to securely manage expense data.

* **Modern Financial Awareness**
  Encouraging users to monitor and understand their spending habits effectively.

---

# Key Features

## 1. Expense Management

* Create expense records with:

  * Item name
  * Amount
  * Category
  * Additional notes
* Automatically generates unique IDs for every expense
* Persistent blockchain storage

---

## 2. Expense Retrieval

* Retrieve all recorded expenses
* Structured data format for frontend integration
* Expense data retrieval through smart contract functions

---

## 3. Expense Deletion

* Delete expense records using unique IDs
* Immediate updates to stored expense lists
* Efficient contract storage management

---

## 4. Expense Note Update

* Modify additional notes for existing expenses
* Flexible record management
* Simple update mechanism through smart contract functions

---

## 5. Financial Summary

* Calculate total expenses
* Count recorded transactions
* Generate simple spending activity summaries

---

## 6. Stellar Soroban Integration

* Built using Soroban Smart Contracts
* Powered by the Stellar blockchain network
* Low transaction cost and efficient execution
* Compatible with Stellar ecosystem tools

---

# Contract Functions

The smart contract provides the following functions:

| Function                  | Description                         |
| ------------------------- | ----------------------------------- |
| `create_expense()`        | Create a new expense record         |
| `get_expenses()`          | Retrieve all expenses               |
| `delete_expense()`        | Delete an expense by ID             |
| `update_expense_note()`   | Update expense notes                |
| `get_total_expense()`     | Calculate total expenses            |
| `get_expense_count()`     | Count total transactions            |
| `get_financial_summary()` | Generate financial activity summary |

---

# Technical Requirements

* Rust Programming Language
* Soroban SDK
* Stellar Blockchain Network

---

# Project Status

| Component | Status |
|---|---|
| Smart Contract | Completed and deployed on Stellar Soroban Testnet |
| Frontend UI | Completed as a prototype interface |
| Frontend-Contract Integration | In development |
| Wallet Integration | Planned |

---

# Future Enhancements

## Short-Term Development

1. Expense categories with analytics visualization
2. Monthly financial reports
3. Budget planning system
4. Search and filtering features
5. Transaction history export

---

## Medium-Term Development

6. Wallet-based authentication
7. Frontend integration with Freighter Wallet
8. Real-time smart contract interaction
9. Spending trend analysis
10. Multi-user expense management

---

## Long-Term Vision

11. AI-based spending recommendations
12. Cross-device synchronization
13. Decentralized cloud backup
14. Mobile application support
15. Advanced financial dashboards

---

# Getting Started

Deploy the PinkPocket smart contract to the Stellar Soroban network and interact with the provided contract functions.

The project also includes a web-based frontend prototype designed to represent user interaction with the PinkPocket smart contract.

---

# Contract Information

| Item | Detail |
|---|---|
| Contract Alias | `PinkPocket` |
| Blockchain Network | Stellar Soroban Testnet |
| Contract Status | Deployed |
| Contract ID | `CDJYVWVQGG5HYCPRY2VQR6WKUHMECRGEOUTS6UTM2TP26XF7PJ2G4RGD` |

---

# Testnet Environment

PinkPocket currently operates on the Stellar Soroban Testnet environment for development and testing purposes.

---

# Tech Stack

* Soroban Smart Contract
* Rust
* Stellar Blockchain
* HTML
* CSS
* JavaScript

---

**PinkPocket** — Cute Interface, Smarter Spending.