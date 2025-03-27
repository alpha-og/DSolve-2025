# 🚀 DSolve-2025 Hackathon

## 👥 Team Information

**Team Name:** Arkway

**Team Members:**

- Athul Anoop
- [Member 1's Github ](https://github.com/alpha-og) - Backend API implementation
- Sharon P Shajan
- [Member 2's Github](https://github.com/sharon504) - Blockchain implementation

## 💡 Project Idea

We propose a blockchain-based order verification system that ensures tracking numbers are cryptographically linked to actual orders which can also be viewed by the delivery partner, preventing fraudulent misuse. Each order is assigned a unique blockchain chain, with the first block storing order details. The seller must choose a delivery partner through our integration with the marketplace and provide necessary details such as pick up location. This approach enforces tamper-proof tracking, ensuring authenticity while reducing fraudulent disputes. To optimize scalability, hashed proofs are stored on-chain, while full metadata is kept in off-chain storage. The system integrates proof-of-pickup mechanisms (QR codes, barcodes, or NFC scans) and optional customer verification steps to further strengthen trust. By leveraging Layer 2 rollups or permissioned blockchains, the solution can be scaled to be cost-effective and efficient, making it viable for widespread adoption across marketplaces and logistics networks.

## ✨ Key Features

- Fully decentralized blockchain-based order verification system
- Cryptographically linked tracking numbers to actual orders
- Tamper-proof tracking
- Scalable and cost-effective solution

## 🎥 Product Demo

[![Watch the Demo](https://via.placeholder.com/300x200?text=Click+for+Demo+Video)](https://youtube.com/link-to-video)
_Click the image above to view our product walkthrough_

## 🛠️ Tech Stack

| Technology        | Version | Purpose                   |
| ----------------- | ------- | ------------------------- |
| Axum (Rust)       | 0.8.1   | Backend API               |
| Blockchain (Rust) | -       | Blockchain Implementation |

## 🛠️ Setup Instructions

### Prerequisites

- Rust
- PNPM
- PostgreSQL

### Installation

```bash
// {Enter the commands for installation here}
```

### Running the Project

```bash
pnpm i && pnpm run dev
```

## Acknowledgments

- Axum [https://docs.rs/axum/latest/axum/index.html]
