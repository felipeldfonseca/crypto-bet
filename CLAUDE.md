# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **pre-development repository** with comprehensive enterprise-grade documentation for building a Solana-based dual-mode prediction markets platform. The repository contains extensive planning and a single implemented smart contract, but **no frontend or backend implementation** has begun.

**Current Status**: Phase 1 (Documentation) ✅ Complete | Phase 2 (Core Development) 🚀 Ready to Begin

## Repository Architecture

### What Actually Exists
- **Smart Contract**: Complete Solana program (`/program/`) with dual-mode betting (SOL/USDC)
- **Documentation Suite**: 28+ enterprise-grade documents across 8 organized folders (`/docs/`)
- **Development Roadmap**: Comprehensive 12-week MVP plan (`TODO.md`)
- **Project Configuration**: Basic Anchor setup for Solana development

### What's Missing (Ready for Implementation)
- **Frontend Application**: No `src/` directory or Next.js implementation
- **API Layer**: No backend routes or database connections
- **Integration Layer**: No Jupiter swap or wallet connectivity
- **Testing Suite**: Smart contract tests exist as templates only

## Smart Contract Development

### Current Implementation
The Solana program (`/program/programs/crypto-bet/src/lib.rs`) is fully implemented with:
- Dual-mode market support (SOL "Degen Mode" + USDC "Stable Mode")
- Market creation, betting, resolution, and claiming functionality
- Comprehensive security measures and error handling
- Event emission for real-time updates

### Development Commands
```bash
# Smart contract development
cd program
anchor build                    # Build the program
anchor test                     # Run tests (templates only)
anchor deploy                   # Deploy to configured cluster
solana-test-validator          # Start local validator

# Program validation
anchor test --provider.cluster localnet --provider.wallet ~/.config/solana/id.json
```

### Program Configuration
- **Program ID**: `4Gd64thyhLeqyLxDz8Ae5Z98qXdqwJrcAYkS6g3Yzy5V`
- **USDC Mint**: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
- **Cluster**: Configured for devnet deployment

## Technology Stack (Planned)

### Frontend (To Be Implemented)
- **Framework**: Next.js 14 with App Router
- **Language**: TypeScript with strict mode
- **Styling**: Tailwind CSS + shadcn/ui components
- **State Management**: React Context
- **Wallet Integration**: Solana Wallet Adapter

### Backend (To Be Implemented)
- **Database**: PostgreSQL with Redis caching
- **API**: REST endpoints + WebSocket for real-time updates
- **Integration**: Jupiter Aggregator for token swaps

### Development Prerequisites
When implementation begins, ensure these are installed:
```bash
node >= 18.0.0
npm >= 8.0.0
anchor >= 0.29.0
solana-cli >= 1.16.0
```

## Documentation Architecture

The `/docs/` folder contains enterprise-grade specifications organized into 8 categories:

### Critical Documents for Development
- **`docs/architecture/SYSTEM_ARCHITECTURE.md`**: Complete technical architecture
- **`docs/architecture/DATABASE_SCHEMA.md`**: Detailed database design
- **`docs/architecture/API_SPECIFICATION.md`**: REST and WebSocket API specs
- **`docs/product/PRODUCT_REQUIREMENTS.md`**: Feature specifications and user stories
- **`docs/operations/TESTING_STRATEGY.md`**: Comprehensive testing approach
- **`docs/security/SECURITY.md`**: Security requirements and measures

### Development Workflow Documents
- **`TODO.md`**: 12-week MVP development roadmap with granular tasks
- **`docs/operations/DEPLOYMENT_GUIDE.md`**: Environment setup and deployment procedures
- **`docs/guides/`**: User guides and tutorials (templates ready)

## Development Phases

### Phase 1: Foundation ✅ Complete
- Enterprise documentation suite complete
- Smart contract implemented and ready for testing
- Technical specifications defined
- Security framework established

### Phase 2: Core Development 🚀 Next Phase
**Priority Implementation Order:**
1. **Smart Contract Testing**: Implement comprehensive test suite
2. **Frontend Foundation**: Set up Next.js 14 with TypeScript
3. **Database Setup**: PostgreSQL + Redis implementation
4. **API Development**: REST endpoints and WebSocket connections
5. **Wallet Integration**: Solana wallet connectivity

### Phase 3: Integration & Testing
- Jupiter swap integration
- UI/UX implementation per design specifications
- End-to-end testing suite
- Security audits and optimization

## Key Architectural Decisions

### Dual-Mode Betting System
- **Degen Mode**: Native SOL betting for volatility-embracing users
- **Stable Mode**: USDC betting for price-stable prediction markets
- **1:1 Share Mechanics**: Simple proportional payout system
- **Token Swap Integration**: Convert any token to betting assets via Jupiter

### Smart Contract Security
- Authority-controlled market resolution
- Checked arithmetic operations throughout
- Proper PDA derivation and validation
- Event emission for transparency

### Frontend Architecture (Planned)
- Aave-style navigation with hover animations
- Mobile-responsive design throughout
- Real-time market updates via WebSocket
- Persistent dual-mode state management

## Development Standards

### Code Quality
- **Rust**: Follow Anchor framework best practices
- **TypeScript**: Strict mode enabled with comprehensive typing
- **Testing**: Unit, integration, and E2E coverage required
- **Security**: Security-first development approach

### Git Workflow
- Feature branch development
- Pull request reviews required
- Comprehensive commit messages
- No direct commits to main branch

## Critical Notes for Implementation

1. **This is a planning repository**: All code must be implemented following the detailed specifications in `/docs/`

2. **Smart contract is production-ready**: The Solana program is fully implemented but requires comprehensive testing

3. **Follow the roadmap**: The `TODO.md` contains a week-by-week plan with specific tasks, success criteria, and time estimates

4. **Documentation-driven development**: Always reference the technical specifications before implementing features

5. **Security-first approach**: Review `/docs/security/SECURITY.md` before implementing any security-sensitive functionality

6. **Dual-mode support**: All betting functionality must support both SOL and USDC modes

7. **Jupiter integration required**: Token swap functionality is core to the platform's value proposition

## Quick Navigation

- **Development Roadmap**: `TODO.md`
- **Technical Architecture**: `docs/architecture/`
- **API Specifications**: `docs/architecture/API_SPECIFICATION.md`
- **Security Requirements**: `docs/security/SECURITY.md`
- **Smart Contract**: `program/programs/crypto-bet/src/lib.rs`
- **Testing Strategy**: `docs/operations/TESTING_STRATEGY.md`