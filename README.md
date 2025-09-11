# Optimic Protocol

![Rust Build](https://github.com/optimic-protocol/optimic/workflows/Rust%20Build%20and%20Test/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> A High-Performance Layer 1 Blockchain for Options Trading

Optimic is a next-generation Layer 1 blockchain designed specifically for options trading, featuring:

- 🚀 **High Performance**: 1-second block finality with 100,000+ TPS
- 📊 **On-Chain Order Book**: Fully on-chain Central Limit Order Book (CLOB)
- 🔒 **Mandatory Collateral**: Fair collateral system for both buyers and sellers
- 💰 **Transparent Fees**: 100% of premiums to platform, fair penalty distribution
- ⚡ **Real-Time Greeks**: On-chain Black-Scholes and risk management
- 🌐 **24/7 Trading**: Global access to options markets

## 🏗️ Project Structure

```
optimic/
├── blockchain/          # Core blockchain implementation (Rust)
├── contracts/           # Smart contracts (CosmWasm)
├── api/                 # Backend services (Node.js)
├── frontend/           # UI application (React)
├── scripts/            # Deployment and utility scripts
└── doc/                # Documentation
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose
- Node.js 18+ (for API and frontend)

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/optimic-protocol/optimic.git
   cd optimic
   ```

2. **Build the blockchain**
   ```bash
   cd blockchain
   cargo build --release
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Start with Docker Compose**
   ```bash
   docker-compose up -d
   ```

### Running a Node

```bash
# Initialize node
./target/release/optimic-node init --chain-id optimic-1

# Start node
./target/release/optimic-node start --config config.toml --genesis genesis.json
```

## 📖 Architecture

### Core Blockchain (Rust)

The blockchain core is built in Rust with the following modules:

- **ABCI Interface**: Tendermint integration for consensus
- **State Management**: Efficient state storage and transitions
- **Trading Engine**: High-performance order matching
- **Options Engine**: Options contract management and pricing
- **Collateral Manager**: Mandatory collateral system
- **Risk Management**: Real-time portfolio monitoring

### Key Features

#### Mandatory Collateral System
- **Buyers**: Must post collateral to guarantee premium payment and exercise
- **Sellers**: Must post collateral to guarantee option fulfillment
- **Fair Penalties**: 50% to platform, 50% to counterparty for non-execution

#### Fee Structure
- **Premium Fees**: 100% collected by platform
- **Trading Fees**: Distributed among stakers, LPs, burns, and treasury
- **Penalty Fees**: Fair distribution between platform and affected parties

#### Performance
- **Block Time**: 1 second
- **Finality**: 1 second
- **Throughput**: 100,000+ TPS
- **Order Matching**: Sub-millisecond execution

## 🔧 Development

### Blockchain Development

```bash
cd blockchain

# Check code
cargo check

# Format code
cargo fmt

# Run lints
cargo clippy

# Run tests
cargo test

# Build optimized
cargo build --release
```

### Docker Development

```bash
# Build blockchain image
docker build -t optimic/blockchain ./blockchain

# Run development environment
docker-compose -f docker-compose.dev.yml up

# View logs
docker-compose logs -f blockchain
```

## 📊 Current Status

### ✅ Completed (Phase 1.1)
- [x] Core blockchain architecture
- [x] Basic ABCI interface implementation
- [x] State management foundation
- [x] Trading engine framework
- [x] Options contracts structure
- [x] Collateral management framework
- [x] CI/CD pipeline setup

### 🚧 In Progress
- [ ] Full Tendermint integration
- [ ] Order book matching engine
- [ ] Black-Scholes pricing implementation
- [ ] Collateral calculation algorithms
- [ ] Transaction processing

### 📅 Upcoming (Phase 1.2)
- [ ] Frontend development
- [ ] API backend services
- [ ] WebSocket real-time data
- [ ] Market data integration
- [ ] User interface

## 🧪 Testing

### Unit Tests
```bash
cd blockchain
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Load Testing
```bash
# Coming soon
```

## 📚 Documentation

- [Whitepaper](doc/whitepaper.md) - Complete protocol specification
- [MVP Breakdown](doc/mvp_task_breakdown.md) - Detailed development plan
- [API Documentation](doc/api.md) - API reference (coming soon)
- [Developer Guide](doc/developer-guide.md) - Development setup (coming soon)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Website**: [optimic.finance](https://optimic.finance)
- **Documentation**: [docs.optimic.finance](https://docs.optimic.finance)
- **Discord**: [Join our community](https://discord.gg/optimic)
- **Twitter**: [@OptimicProtocol](https://twitter.com/OptimicProtocol)

## ⚠️ Disclaimer

This software is in active development and not yet ready for production use. Use at your own risk. Options trading involves significant financial risk.

---

**Built with ❤️ for the decentralized derivatives future**