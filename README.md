# 🚀 Crypto Bet - Solana Prediction Markets

> **The first dual-mode prediction market platform on Solana**  
> Bet with SOL for maximum degen energy or USDC for stable precision

[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)](https://nextjs.org/)
[![Anchor](https://img.shields.io/badge/Anchor-663399?style=for-the-badge&logo=anchor&logoColor=white)](https://www.anchor-lang.com/)

📚 **[Complete Documentation](./docs/)** | 🗺️ **[Development Roadmap](./TODO.md)** | 🏗️ **[Architecture Guide](./docs/architecture/)** | 🔒 **[Security Audit](./docs/security/)**

---

## 🎯 **What is Crypto Bet?**

Crypto Bet is a revolutionary prediction market platform built on Solana that bridges the gap between serious prediction markets and the vibrant Solana degen culture.

### **🔥 Unique Value Proposition**
- **Dual-Mode Betting**: Choose between SOL (Degen Mode) and USDC (Stable Mode)
- **Built-in Swap**: Convert any token to betting assets seamlessly
- **Solana Speed**: Lightning-fast transactions and low fees
- **Degen-Friendly**: Embrace the meme coin energy while offering institutional-grade stability

---

## 🏗️ **Planned Architecture**

### **Smart Contract (Anchor/Rust)**
*To be implemented in Phase 2*
```
crypto-bet/program/src/lib.rs (planned)
├── Dual-mode market support (SOL + USDC)
├── Secure 1:1 share mechanics
├── Authority-controlled market resolution
├── Anti-manipulation safeguards
└── Comprehensive error handling
```

### **Frontend (Next.js 14 + TypeScript)**
*To be implemented in Phases 2-3*
```
src/ (planned)
├── Landing page with hero section and navigation
├── Markets app with comprehensive wallet integration  
├── Dual-mode UI with persistent state (Degen 🚀 vs Stable 🏦)
├── Dedicated swap page with Jupiter integration
├── Market creation interface with validation
├── Aave-style navigation with hover animations
├── Mobile-responsive design throughout
└── Real-time market updates and swap quotes
```

---

## 🎮 **How It Works**

### **1. Choose Your Mode**
```
🚀 DEGEN MODE (SOL)
- Bet with native SOL
- Embrace volatility
- Meme coin trader vibes
- "I won 50 SOL on Bitcoin!"

💰 STABLE MODE (USDC)  
- Bet with USDC stablecoin
- Price stability
- Professional trading
- "I made $1,000 profit"
```

### **2. Simple Betting Mechanics**
```
1 SOL bet = 1 SOL share
1 USDC bet = 1 USDC share

Winner takes all, proportionally distributed:
Your winnings = (Your shares / Total winning shares) × Total pool
```

### **3. Built-in Token Swap**
```
Have USDT? → Swap to USDC → Bet on stable markets
Have BONK? → Swap to SOL → Bet on degen markets
Any token → Betting token (with 0.3% platform fee)
```

---

## 🔧 **Planned Technical Features**

### **Smart Contract Security** *(Phase 2 Implementation)*
- 📋 **Authority Controls**: Only designated authorities can resolve markets
- 📋 **Math Safety**: All operations use checked arithmetic
- 📋 **Double-Claim Prevention**: Shares zeroed after claiming
- 📋 **Time Controls**: Betting deadlines and resolution windows
- 📋 **Account Security**: Secure PDA derivation and validation

### **Market Mechanics** *(Phase 2-3 Implementation)*
- 📋 **Market States**: Active → Resolved/Cancelled
- 📋 **Position Tracking**: Individual user positions per market
- 📋 **Event Emission**: Real-time updates via Solana events
- 📋 **Refund System**: Full refunds for cancelled markets

### **Implementation Roadmap**

**📋 Phase 2: Core Development (Weeks 4-8)**
- Smart contract development with security measures
- Database and API implementation
- Frontend foundation setup
- Wallet integration framework

**📋 Phase 3: Integration & Testing (Weeks 9-11)**
- Jupiter swap integration
- UI/UX implementation
- Comprehensive testing suite
- Security audits and optimization

---

## 🚀 **Getting Started**

### **Repository Overview**
```bash
# Clone the repository
git clone https://github.com/felipeldfonseca/crypto-bet.git
cd crypto-bet

# Explore the documentation
ls docs/          # Browse organized documentation
cat TODO.md       # View 12-week development roadmap
```

### **Development Prerequisites** *(For Phase 2 Implementation)*
```bash
# Required for development when implementation begins
node >= 18.0.0
npm >= 8.0.0
anchor >= 0.29.0
solana-cli >= 1.16.0
```

### **Next Steps**
1. **Study Documentation**: Review the complete [documentation suite](./docs/)
2. **Understand Architecture**: Read [technical specifications](./docs/architecture/)
3. **Follow Roadmap**: Check the [12-week development plan](./TODO.md)
4. **Begin Development**: Start with Phase 2 implementation when ready

### **📊 Current Status**

**✅ Documentation & Planning Complete:**
- Enterprise documentation suite (28 documents across 8 folders)
- 12-week MVP development roadmap
- Technical specifications and architecture plans
- Security framework and testing strategies
- Database schema and API specifications

**🚀 Ready for Development:**
- All foundational planning completed
- Development guidelines and standards established  
- Clear roadmap from concept to MVP launch
- Repository structure and documentation organized

**⚠️ Implementation Status: Pre-Development**
- No smart contracts implemented yet
- No frontend development completed
- No backend infrastructure built
- Ready to begin Phase 1 development following the comprehensive roadmap

*This repository contains the complete planning foundation for building the Solana prediction markets platform. See [Development Roadmap](./TODO.md) for the detailed 12-week implementation plan.*

---

## 📊 **Market Examples**

### **Crypto Markets (Degen Mode - SOL)**
```
🚀 "Will Bitcoin hit $100,000 by Dec 31, 2024?"
- Current pool: 150 SOL YES, 75 SOL NO
- Your bet: 10 SOL on YES
- Potential payout: 15 SOL (1.5x multiplier)
```

### **Traditional Markets (Stable Mode - USDC)**
```
💰 "Will the Fed cut rates in Q1 2025?"
- Current pool: $50,000 YES, $30,000 NO  
- Your bet: $1,000 on NO
- Potential payout: $2,667 (2.67x multiplier)
```

---

## 🔐 **Security Framework**

### **Planned Security Measures** *(Phase 2-3 Implementation)*
- 📋 **Security Audits**: Professional third-party security reviews
- 📋 **Vulnerability Testing**: Comprehensive penetration testing
- 📋 **Code Reviews**: Multi-developer security validation
- 📋 **Continuous Monitoring**: Ongoing security assessments

### **Planned Security Features**
- **Authority Validation**: `has_one` constraints on all privileged functions
- **Overflow Protection**: `checked_add/sub/mul/div` throughout
- **Reentrancy Prevention**: Proper account constraints  
- **State Management**: Secure market state transitions

*See [Security Documentation](./docs/security/) for complete security specifications*

---

## 💰 **Planned Tokenomics & Revenue**

### **Initial Model (V1.0)** *(Phase 2 Implementation)*
- **No Platform Fees**: Pure peer-to-peer betting
- **Swap Fees**: 0.3% on token conversions  
- **Gas Optimization**: Minimal transaction costs

### **Future Revenue Streams** *(Post-MVP)* (See [TODO.md](./TODO.md))
- **Platform Fees**: 2-5% of betting pools
- **Market Creation Fees**: Fixed amounts for market creation
- **Premium Features**: Advanced analytics, priority support
- **API Access**: B2B prediction market data

---

## 🛠️ **Development Roadmap**

### **Phase 1: Foundation (Weeks 1-3)** ✅ **Planning Complete**
- [x] **Documentation Suite**: Enterprise-grade documentation with 28 documents
- [x] **Technical Specifications**: Database schema, API specs, architecture plans
- [x] **Security Framework**: Comprehensive security measures and audit procedures
- [x] **Development Guidelines**: Standards, testing strategies, deployment guides
- [x] **Project Structure**: Organized repository with clear development paths

### **Phase 2: Core Development (Weeks 4-8)** 🚀 **Ready to Begin**
- [ ] **Smart Contract Development**: Implement dual-mode prediction markets
- [ ] **Database Setup**: PostgreSQL with Redis caching implementation
- [ ] **API Development**: REST endpoints and real-time WebSocket connections
- [ ] **Frontend Foundation**: Next.js 14 setup with TypeScript and Tailwind
- [ ] **Wallet Integration**: Solana wallet connectivity and transaction handling

### **Phase 3: Integration & Testing (Weeks 9-11)** 
- [ ] **Jupiter Integration**: Token swap functionality implementation
- [ ] **UI/UX Implementation**: Complete betting interface and user flows
- [ ] **Testing Suite**: Unit, integration, and end-to-end testing
- [ ] **Security Audits**: Smart contract and system security validation
- [ ] **Performance Optimization**: Load testing and performance tuning

### **Phase 4: Launch Preparation (Week 12)**
- [ ] **Production Deployment**: Mainnet deployment and monitoring setup
- [ ] **User Documentation**: Guides, tutorials, and help resources
- [ ] **Marketing Materials**: Landing pages, social media, community setup

### **V1.1 - Revenue Optimization**
- [ ] Platform fee implementation
- [ ] Direct DEX integrations
- [ ] Advanced swap features
- [ ] Analytics dashboard

### **V2.0 - Ecosystem Expansion**
- [ ] Multi-asset support (USDT, meme coins)
- [ ] Advanced market types
- [ ] Mobile app
- [ ] Social features

*See [TODO.md](./TODO.md) for complete roadmap*

---

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guidelines](./CONTRIBUTING.md) for details.

### **Development Process**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### **Code Standards**
- **Rust**: Follow Anchor best practices
- **TypeScript**: Strict mode enabled
- **Testing**: Comprehensive test coverage
- **Documentation**: Clear code comments

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

## 🔗 **Links & Resources**

### **Official Links**
- **Website**: [cryptobet.so](https://cryptobet.so) *(coming soon)*
- **Twitter**: [@CryptoBetSol](https://twitter.com/CryptoBetSol) *(coming soon)*
- **Discord**: [Join Community](https://discord.gg/cryptobet) *(coming soon)*
- **Documentation**: [docs.cryptobet.so](https://docs.cryptobet.so) *(coming soon)*

### **Technical Resources**
- **Solana Docs**: [docs.solana.com](https://docs.solana.com)
- **Anchor Framework**: [anchor-lang.com](https://anchor-lang.com)
- **Jupiter Aggregator**: [jup.ag](https://jup.ag)
- **Next.js**: [nextjs.org](https://nextjs.org)

---

## 🙏 **Acknowledgments**

- **Solana Foundation** for the incredible blockchain infrastructure
- **Anchor Team** for the amazing development framework
- **Jupiter** for best-in-class swap aggregation
- **Solana Community** for inspiration and support

---

## 📞 **Contact**

For questions, suggestions, or partnerships:

- **Email**: hello@cryptobet.so *(coming soon)*
- **Telegram**: [@CryptoBetSupport](https://t.me/CryptoBetSupport) *(coming soon)*
- **GitHub Issues**: [Create an issue](https://github.com/yourusername/crypto-bet/issues)

---

<div align="center">

**Built with ❤️ on Solana**

*Empowering the future of decentralized prediction markets*

</div>
