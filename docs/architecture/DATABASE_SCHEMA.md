# 🗃️ Database Schema & Data Architecture

## 📋 **Document Overview**

**Purpose**: Define comprehensive data architecture for Crypto Bet prediction markets platform  
**Scope**: On-chain accounts, off-chain databases, analytics, and data relationships  
**Audience**: Backend developers, smart contract developers, database administrators  
**Last Updated**: December 2024  
**Version**: 1.0  

---

## 🎯 **Executive Summary**

Crypto Bet employs a hybrid data architecture combining Solana blockchain for critical transaction data with traditional databases for analytics and user experience optimization. The design prioritizes data integrity, performance, and compliance while maintaining the transparency benefits of blockchain technology.

**Key Architectural Decisions:**
- **On-Chain**: Critical market and position data via Solana Program Derived Addresses (PDAs)
- **Off-Chain**: User preferences, analytics, and cached data for performance
- **Hybrid Sync**: Real-time synchronization between blockchain and traditional databases
- **GDPR Compliance**: User data management with right to deletion

---

## ⛓️ **On-Chain Schema (Solana Program Accounts)**

### **Account Architecture Overview**

```rust
// Program Account Hierarchy
CryptoBetProgram
├── GlobalConfig          // Single instance, program configuration
├── Market[]             // Multiple instances, one per prediction market
├── Position[]           // Multiple instances, user positions in markets
├── Treasury[]           // Multiple instances, token vaults per market
└── OracleData[]         // Multiple instances, external data feeds
```

### **1. GlobalConfig Account**

```rust
#[account]
pub struct GlobalConfig {
    /// Program authority (upgrade and config authority)
    pub authority: Pubkey,                    // 32 bytes
    
    /// Platform fee percentage (basis points, e.g., 250 = 2.5%)
    pub platform_fee_bps: u16,               // 2 bytes
    
    /// Minimum market duration in seconds
    pub min_market_duration: i64,            // 8 bytes
    
    /// Maximum market duration in seconds  
    pub max_market_duration: i64,            // 8 bytes
    
    /// Minimum bet amount in lamports/smallest unit
    pub min_bet_amount: u64,                 // 8 bytes
    
    /// Maximum bet amount in lamports/smallest unit
    pub max_bet_amount: u64,                 // 8 bytes
    
    /// Platform treasury where fees are collected
    pub platform_treasury: Pubkey,           // 32 bytes
    
    /// Emergency pause flag
    pub is_paused: bool,                     // 1 byte
    
    /// Total markets created counter
    pub total_markets_created: u64,          // 8 bytes
    
    /// Total volume across all markets
    pub total_volume: u64,                   // 8 bytes
    
    /// Program version
    pub version: u8,                         // 1 byte
    
    /// PDA bump seed
    pub bump: u8,                           // 1 byte
}
// Total: 135 bytes + discriminator (8) = 143 bytes
```

**PDA Derivation:**
```rust
seeds = [b"global_config"]
```

### **2. Market Account**

```rust
#[account]
pub struct Market {
    /// Market creator authority
    pub authority: Pubkey,                   // 32 bytes
    
    /// Unique market identifier
    pub market_id: u64,                     // 8 bytes
    
    /// Market title (max 64 chars)
    pub title: String,                      // 4 + 64 = 68 bytes
    
    /// Market description (max 256 chars)
    pub description: String,                // 4 + 256 = 260 bytes
    
    /// Market category (max 32 chars)
    pub category: String,                   // 4 + 32 = 36 bytes
    
    /// Market creation timestamp
    pub created_at: i64,                    // 8 bytes
    
    /// Market resolution timestamp
    pub resolution_time: i64,               // 8 bytes
    
    /// Market state
    pub state: MarketState,                 // 1 byte (enum)
    
    /// Market type (Degen/Stable mode)
    pub market_type: MarketType,            // 1 byte (enum)
    
    /// Accepted token mint (SOL or USDC)
    pub accepted_mint: Pubkey,              // 32 bytes
    
    /// YES token vault
    pub yes_vault: Pubkey,                  // 32 bytes
    
    /// NO token vault  
    pub no_vault: Pubkey,                   // 32 bytes
    
    /// Total YES pool amount
    pub yes_pool: u64,                      // 8 bytes
    
    /// Total NO pool amount
    pub no_pool: u64,                       // 8 bytes
    
    /// Total volume (all-time)
    pub total_volume: u64,                  // 8 bytes
    
    /// Total unique participants
    pub total_participants: u32,            // 4 bytes
    
    /// Resolution result (None until resolved)
    pub resolution_result: Option<bool>,     // 1 + 1 = 2 bytes
    
    /// Resolution authority (who can resolve)
    pub resolver: Pubkey,                   // 32 bytes
    
    /// Oracle data source (optional)
    pub oracle_source: Option<Pubkey>,      // 1 + 32 = 33 bytes
    
    /// Market metadata URI (IPFS)
    pub metadata_uri: String,               // 4 + 128 = 132 bytes
    
    /// Creation fee paid
    pub creation_fee: u64,                  // 8 bytes
    
    /// PDA bump seed
    pub bump: u8,                          // 1 byte
}
// Total: ~800 bytes + discriminator (8) = ~808 bytes
```

**PDA Derivation:**
```rust
seeds = [b"market", &market_id.to_le_bytes(), authority.as_ref()]
```

**Market State Enum:**
```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MarketState {
    Active,      // 0 - Accepting bets
    Resolved,    // 1 - Resolved with outcome
    Cancelled,   // 2 - Cancelled, refunds available
    Paused,      // 3 - Temporarily paused
}
```

**Market Type Enum:**
```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MarketType {
    Degen,      // 0 - SOL betting mode
    Stable,     // 1 - USDC betting mode
}
```

### **3. Position Account**

```rust
#[account]
pub struct Position {
    /// User wallet public key
    pub user: Pubkey,                       // 32 bytes
    
    /// Market this position belongs to
    pub market: Pubkey,                     // 32 bytes
    
    /// YES shares owned
    pub yes_shares: u64,                    // 8 bytes
    
    /// NO shares owned
    pub no_shares: u64,                     // 8 bytes
    
    /// Total amount invested
    pub total_invested: u64,                // 8 bytes
    
    /// Average price paid for YES shares
    pub yes_avg_price: u64,                 // 8 bytes
    
    /// Average price paid for NO shares
    pub no_avg_price: u64,                  // 8 bytes
    
    /// Position creation timestamp
    pub created_at: i64,                    // 8 bytes
    
    /// Last update timestamp
    pub updated_at: i64,                    // 8 bytes
    
    /// Winnings claimed flag
    pub claimed: bool,                      // 1 byte
    
    /// Claimed amount (if any)
    pub claimed_amount: u64,                // 8 bytes
    
    /// Number of bet transactions
    pub bet_count: u32,                     // 4 bytes
    
    /// PDA bump seed
    pub bump: u8,                          // 1 byte
}
// Total: 142 bytes + discriminator (8) = 150 bytes
```

**PDA Derivation:**
```rust
seeds = [b"position", user.as_ref(), market.as_ref()]
```

### **4. Treasury Account (Token Vault)**

```rust
#[account]
pub struct Treasury {
    /// Associated market
    pub market: Pubkey,                     // 32 bytes
    
    /// Token mint (SOL or USDC)
    pub mint: Pubkey,                      // 32 bytes
    
    /// Vault authority (program PDA)
    pub authority: Pubkey,                  // 32 bytes
    
    /// Total deposits
    pub total_deposits: u64,                // 8 bytes
    
    /// Total withdrawals
    pub total_withdrawals: u64,             // 8 bytes
    
    /// Current balance
    pub current_balance: u64,               // 8 bytes
    
    /// Treasury type (YES or NO pool)
    pub treasury_type: TreasuryType,        // 1 byte
    
    /// PDA bump seed
    pub bump: u8,                          // 1 byte
}
// Total: 122 bytes + discriminator (8) = 130 bytes
```

**Treasury Type Enum:**
```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TreasuryType {
    YesPool,        // 0 - YES token pool
    NoPool,         // 1 - NO token pool
    Platform,       // 2 - Platform fee collection
}
```

### **5. Oracle Data Account**

```rust
#[account]
pub struct OracleData {
    /// Oracle authority
    pub authority: Pubkey,                  // 32 bytes
    
    /// Associated market
    pub market: Pubkey,                     // 32 bytes
    
    /// Data source identifier
    pub source: String,                     // 4 + 64 = 68 bytes
    
    /// Oracle data value
    pub value: String,                      // 4 + 128 = 132 bytes
    
    /// Data timestamp
    pub timestamp: i64,                     // 8 bytes
    
    /// Confidence score (0-100)
    pub confidence: u8,                     // 1 byte
    
    /// Data signature for verification
    pub signature: [u8; 64],                // 64 bytes
    
    /// PDA bump seed
    pub bump: u8,                          // 1 byte
}
// Total: 338 bytes + discriminator (8) = 346 bytes
```

---

## 🗄️ **Off-Chain Schema (PostgreSQL)**

### **User Management Tables**

#### **users**
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_address VARCHAR(44) UNIQUE NOT NULL,
    username VARCHAR(32) UNIQUE,
    email VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    
    -- User preferences
    preferred_mode market_type DEFAULT 'stable',
    notification_settings JSONB DEFAULT '{}',
    privacy_settings JSONB DEFAULT '{}',
    
    -- KYC status (future compliance)
    kyc_status VARCHAR(20) DEFAULT 'none',
    kyc_data JSONB,
    
    -- Performance tracking
    total_bets INTEGER DEFAULT 0,
    total_volume DECIMAL(20, 8) DEFAULT 0,
    total_winnings DECIMAL(20, 8) DEFAULT 0,
    win_rate DECIMAL(5, 4) DEFAULT 0,
    
    -- Indexing
    CONSTRAINT valid_wallet_address CHECK (length(wallet_address) = 44)
);

CREATE INDEX idx_users_wallet_address ON users(wallet_address);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_last_login ON users(last_login);
```

#### **user_sessions**
```sql
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    wallet_signature TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT true
);

CREATE INDEX idx_sessions_token ON user_sessions(session_token);
CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON user_sessions(expires_at);
```

### **Market Data Tables**

#### **markets_cache**
```sql
CREATE TABLE markets_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    market_pubkey VARCHAR(44) UNIQUE NOT NULL,
    market_id BIGINT NOT NULL,
    authority_pubkey VARCHAR(44) NOT NULL,
    
    -- Market details
    title VARCHAR(64) NOT NULL,
    description TEXT,
    category VARCHAR(32) NOT NULL,
    market_type market_type NOT NULL,
    
    -- Timing
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    resolution_time TIMESTAMP WITH TIME ZONE NOT NULL,
    resolved_at TIMESTAMP WITH TIME ZONE,
    
    -- State
    state market_state NOT NULL DEFAULT 'active',
    resolution_result BOOLEAN,
    
    -- Financial data
    yes_pool DECIMAL(20, 8) DEFAULT 0,
    no_pool DECIMAL(20, 8) DEFAULT 0,
    total_volume DECIMAL(20, 8) DEFAULT 0,
    total_participants INTEGER DEFAULT 0,
    
    -- Metadata
    accepted_mint VARCHAR(44) NOT NULL,
    metadata_uri TEXT,
    tags TEXT[],
    
    -- Performance tracking
    last_sync_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    sync_block_height BIGINT,
    
    -- Indexing
    CONSTRAINT valid_market_pubkey CHECK (length(market_pubkey) = 44)
);

CREATE INDEX idx_markets_cache_pubkey ON markets_cache(market_pubkey);
CREATE INDEX idx_markets_cache_state ON markets_cache(state);
CREATE INDEX idx_markets_cache_category ON markets_cache(category);
CREATE INDEX idx_markets_cache_resolution_time ON markets_cache(resolution_time);
CREATE INDEX idx_markets_cache_created_at ON markets_cache(created_at);
```

#### **market_analytics**
```sql
CREATE TABLE market_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    market_pubkey VARCHAR(44) NOT NULL REFERENCES markets_cache(market_pubkey),
    date DATE NOT NULL,
    
    -- Daily metrics
    daily_volume DECIMAL(20, 8) DEFAULT 0,
    daily_participants INTEGER DEFAULT 0,
    daily_bets INTEGER DEFAULT 0,
    
    -- Price data
    yes_price_open DECIMAL(10, 8),
    yes_price_close DECIMAL(10, 8),
    yes_price_high DECIMAL(10, 8),
    yes_price_low DECIMAL(10, 8),
    
    no_price_open DECIMAL(10, 8),
    no_price_close DECIMAL(10, 8),
    no_price_high DECIMAL(10, 8),
    no_price_low DECIMAL(10, 8),
    
    -- Liquidity metrics
    avg_spread DECIMAL(10, 8),
    liquidity_score DECIMAL(5, 4),
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(market_pubkey, date)
);

CREATE INDEX idx_market_analytics_market_date ON market_analytics(market_pubkey, date);
```

### **Position Tracking Tables**

#### **positions_cache**
```sql
CREATE TABLE positions_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    position_pubkey VARCHAR(44) UNIQUE NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    market_pubkey VARCHAR(44) NOT NULL REFERENCES markets_cache(market_pubkey),
    
    -- Position data
    yes_shares DECIMAL(20, 8) DEFAULT 0,
    no_shares DECIMAL(20, 8) DEFAULT 0,
    total_invested DECIMAL(20, 8) DEFAULT 0,
    yes_avg_price DECIMAL(10, 8) DEFAULT 0,
    no_avg_price DECIMAL(10, 8) DEFAULT 0,
    
    -- Status
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    claimed BOOLEAN DEFAULT false,
    claimed_amount DECIMAL(20, 8) DEFAULT 0,
    
    -- Performance
    current_value DECIMAL(20, 8) DEFAULT 0,
    unrealized_pnl DECIMAL(20, 8) DEFAULT 0,
    realized_pnl DECIMAL(20, 8) DEFAULT 0,
    
    -- Sync tracking
    last_sync_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    bet_count INTEGER DEFAULT 0,
    
    CONSTRAINT valid_position_pubkey CHECK (length(position_pubkey) = 44)
);

CREATE INDEX idx_positions_cache_pubkey ON positions_cache(position_pubkey);
CREATE INDEX idx_positions_cache_user_id ON positions_cache(user_id);
CREATE INDEX idx_positions_cache_market ON positions_cache(market_pubkey);
CREATE INDEX idx_positions_cache_updated_at ON positions_cache(updated_at);
```

### **Transaction History Tables**

#### **transactions**
```sql
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    signature VARCHAR(88) UNIQUE NOT NULL,
    user_id UUID REFERENCES users(id),
    market_pubkey VARCHAR(44) REFERENCES markets_cache(market_pubkey),
    
    -- Transaction details
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    side bet_side,
    price DECIMAL(10, 8),
    
    -- Blockchain data
    block_height BIGINT NOT NULL,
    slot BIGINT NOT NULL,
    block_time TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Fee information
    fee_amount DECIMAL(20, 8) DEFAULT 0,
    platform_fee DECIMAL(20, 8) DEFAULT 0,
    
    -- Status
    status transaction_status DEFAULT 'confirmed',
    error_message TEXT,
    
    -- Metadata
    instruction_index INTEGER,
    raw_transaction JSONB,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_signature CHECK (length(signature) = 88)
);

CREATE INDEX idx_transactions_signature ON transactions(signature);
CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_market ON transactions(market_pubkey);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);
CREATE INDEX idx_transactions_block_time ON transactions(block_time);
```

### **Custom Types (ENUMs)**

```sql
-- Market related enums
CREATE TYPE market_type AS ENUM ('degen', 'stable');
CREATE TYPE market_state AS ENUM ('active', 'resolved', 'cancelled', 'paused');

-- Transaction related enums
CREATE TYPE transaction_type AS ENUM (
    'place_bet', 'claim_winnings', 'claim_refund', 
    'create_market', 'resolve_market', 'cancel_market'
);
CREATE TYPE transaction_status AS ENUM ('pending', 'confirmed', 'failed');
CREATE TYPE bet_side AS ENUM ('yes', 'no');
```

---

## 🔄 **Data Synchronization Strategy**

### **Real-Time Sync Architecture**

```typescript
// Blockchain Event Listener Service
class BlockchainSyncService {
  private connection: Connection;
  private programId: PublicKey;
  
  async startSyncing() {
    // Subscribe to program account changes
    this.connection.onProgramAccountChange(
      this.programId,
      (accountInfo, context) => {
        this.handleAccountChange(accountInfo, context);
      },
      'confirmed'
    );
    
    // Subscribe to transaction logs
    this.connection.onLogs(
      this.programId,
      (logs, context) => {
        this.handleTransactionLogs(logs, context);
      },
      'confirmed'
    );
  }
  
  private async handleAccountChange(accountInfo: any, context: any) {
    const accountType = this.identifyAccountType(accountInfo.accountId);
    
    switch (accountType) {
      case 'Market':
        await this.syncMarketData(accountInfo);
        break;
      case 'Position':
        await this.syncPositionData(accountInfo);
        break;
      case 'Treasury':
        await this.syncTreasuryData(accountInfo);
        break;
    }
  }
}
```

### **Sync Frequency Matrix**

| Data Type | Sync Method | Frequency | Priority |
|-----------|-------------|-----------|----------|
| **Market Creation** | Real-time | Immediate | Critical |
| **Bet Placement** | Real-time | Immediate | Critical |
| **Position Updates** | Real-time | Immediate | High |
| **Market Resolution** | Real-time | Immediate | Critical |
| **Price Calculations** | Polling | 5 seconds | High |
| **User Analytics** | Batch | Hourly | Medium |
| **Historical Data** | Batch | Daily | Low |

### **Data Consistency Patterns**

#### **1. Eventually Consistent Pattern**
```sql
-- Store blockchain data with sync status
ALTER TABLE markets_cache ADD COLUMN sync_status VARCHAR(20) DEFAULT 'syncing';
ALTER TABLE positions_cache ADD COLUMN sync_status VARCHAR(20) DEFAULT 'syncing';

-- Update sync status after successful sync
UPDATE markets_cache 
SET sync_status = 'synced', last_sync_at = NOW() 
WHERE market_pubkey = $1;
```

#### **2. Conflict Resolution Strategy**
```typescript
// Blockchain data is always source of truth
async resolveDataConflict(blockchainData: any, dbData: any) {
  if (blockchainData.lastUpdateSlot > dbData.lastSyncSlot) {
    // Blockchain is newer, update database
    await this.updateDatabaseFromBlockchain(blockchainData);
  } else {
    // Database might have newer cached calculations
    await this.validateAndSync(blockchainData, dbData);
  }
}
```

---

## 🔍 **Analytics Schema**

### **User Behavior Analytics**

#### **user_events**
```sql
CREATE TABLE user_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    session_id UUID REFERENCES user_sessions(id),
    
    -- Event details
    event_type VARCHAR(50) NOT NULL,
    event_category VARCHAR(30) NOT NULL,
    event_action VARCHAR(50) NOT NULL,
    
    -- Context
    page_url TEXT,
    referrer TEXT,
    user_agent TEXT,
    
    -- Custom properties
    properties JSONB DEFAULT '{}',
    
    -- Timing
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Analytics
    market_pubkey VARCHAR(44),
    amount DECIMAL(20, 8),
    duration_seconds INTEGER
);

CREATE INDEX idx_user_events_user_id ON user_events(user_id);
CREATE INDEX idx_user_events_timestamp ON user_events(timestamp);
CREATE INDEX idx_user_events_type ON user_events(event_type);
```

### **Platform Analytics Tables**

#### **daily_platform_metrics**
```sql
CREATE TABLE daily_platform_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE UNIQUE NOT NULL,
    
    -- User metrics
    total_users INTEGER DEFAULT 0,
    new_users INTEGER DEFAULT 0,
    active_users INTEGER DEFAULT 0,
    returning_users INTEGER DEFAULT 0,
    
    -- Market metrics
    total_markets INTEGER DEFAULT 0,
    new_markets INTEGER DEFAULT 0,
    active_markets INTEGER DEFAULT 0,
    resolved_markets INTEGER DEFAULT 0,
    
    -- Volume metrics
    total_volume DECIMAL(20, 8) DEFAULT 0,
    degen_volume DECIMAL(20, 8) DEFAULT 0,
    stable_volume DECIMAL(20, 8) DEFAULT 0,
    
    -- Transaction metrics
    total_transactions INTEGER DEFAULT 0,
    successful_transactions INTEGER DEFAULT 0,
    failed_transactions INTEGER DEFAULT 0,
    avg_transaction_size DECIMAL(20, 8) DEFAULT 0,
    
    -- Revenue metrics
    platform_fees_collected DECIMAL(20, 8) DEFAULT 0,
    swap_fees_collected DECIMAL(20, 8) DEFAULT 0,
    total_revenue DECIMAL(20, 8) DEFAULT 0,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

---

## 🚀 **Performance Optimization**

### **Indexing Strategy**

#### **Composite Indexes for Common Queries**
```sql
-- Market listing with filters
CREATE INDEX idx_markets_state_category_time ON markets_cache(state, category, resolution_time);

-- User portfolio queries
CREATE INDEX idx_positions_user_state ON positions_cache(user_id, claimed) 
WHERE claimed = false;

-- Transaction history pagination
CREATE INDEX idx_transactions_user_time ON transactions(user_id, block_time DESC);

-- Analytics queries
CREATE INDEX idx_events_user_type_time ON user_events(user_id, event_type, timestamp);
```

#### **Partial Indexes for Performance**
```sql
-- Only index active markets
CREATE INDEX idx_active_markets ON markets_cache(created_at DESC) 
WHERE state = 'active';

-- Only index unclaimed positions
CREATE INDEX idx_unclaimed_positions ON positions_cache(user_id, market_pubkey) 
WHERE claimed = false;

-- Recent transactions index
CREATE INDEX idx_recent_transactions ON transactions(block_time DESC) 
WHERE block_time > (NOW() - INTERVAL '30 days');
```

### **Partitioning Strategy**

#### **Time-Based Partitioning for Analytics**
```sql
-- Partition user_events by month
CREATE TABLE user_events (
    -- columns as above
) PARTITION BY RANGE (timestamp);

-- Create monthly partitions
CREATE TABLE user_events_2024_12 PARTITION OF user_events
    FOR VALUES FROM ('2024-12-01') TO ('2025-01-01');

CREATE TABLE user_events_2025_01 PARTITION OF user_events
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
```

### **Caching Strategy**

#### **Redis Cache Schema**
```typescript
// Cache key patterns
interface CacheKeys {
  // Market data (5 minute TTL)
  MARKET_DETAILS: `market:${marketPubkey}`;
  MARKET_LIST: `markets:${category}:${state}:page:${page}`;
  
  // User data (1 hour TTL)  
  USER_PROFILE: `user:${walletAddress}`;
  USER_POSITIONS: `positions:${userId}`;
  
  // Analytics (24 hour TTL)
  PLATFORM_STATS: 'stats:platform:daily';
  MARKET_ANALYTICS: `analytics:market:${marketPubkey}:${timeframe}`;
  
  // Real-time data (30 second TTL)
  MARKET_PRICES: `prices:${marketPubkey}`;
  USER_PORTFOLIO: `portfolio:${userId}`;
}
```

---

## 🔒 **Data Security & Compliance**

### **Data Classification**

| Data Type | Classification | Storage | Encryption | Retention |
|-----------|----------------|---------|------------|-----------|
| **Wallet Addresses** | Public | On-chain + DB | Transit only | Permanent |
| **Transaction Data** | Public | On-chain + DB | Transit only | Permanent |
| **User Preferences** | Private | DB only | At rest + transit | User controlled |
| **Analytics Data** | Internal | DB only | At rest + transit | 2 years |
| **Session Data** | Sensitive | DB only | At rest + transit | 30 days |
| **KYC Data** | Confidential | Encrypted DB | Full encryption | 7 years |

### **GDPR Compliance Schema**

#### **gdpr_requests**
```sql
CREATE TABLE gdpr_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    request_type gdpr_request_type NOT NULL,
    status gdpr_request_status DEFAULT 'pending',
    
    -- Request details
    requested_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    processed_at TIMESTAMP WITH TIME ZONE,
    processed_by UUID REFERENCES users(id),
    
    -- Data export (for portability requests)
    export_data JSONB,
    export_url TEXT,
    
    -- Deletion tracking (for erasure requests)
    deletion_log JSONB DEFAULT '{}',
    
    -- Legal basis
    legal_basis TEXT,
    notes TEXT
);

CREATE TYPE gdpr_request_type AS ENUM ('access', 'portability', 'erasure', 'rectification');
CREATE TYPE gdpr_request_status AS ENUM ('pending', 'processing', 'completed', 'rejected');
```

### **Audit Trail Schema**

#### **data_audit_log**
```sql
CREATE TABLE data_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name VARCHAR(50) NOT NULL,
    record_id UUID,
    operation audit_operation NOT NULL,
    
    -- Change tracking
    old_values JSONB,
    new_values JSONB,
    changed_fields TEXT[],
    
    -- Context
    user_id UUID REFERENCES users(id),
    session_id UUID REFERENCES user_sessions(id),
    ip_address INET,
    user_agent TEXT,
    
    -- Timing
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Compliance
    reason TEXT,
    legal_basis TEXT
);

CREATE TYPE audit_operation AS ENUM ('INSERT', 'UPDATE', 'DELETE', 'SELECT');
CREATE INDEX idx_audit_log_table_record ON data_audit_log(table_name, record_id);
CREATE INDEX idx_audit_log_user_time ON data_audit_log(user_id, timestamp);
```

---

## 🔧 **Database Administration**

### **Backup Strategy**

#### **PostgreSQL Backup Configuration**
```sql
-- Full backup schedule (daily)
SELECT pg_start_backup('crypto-bet-daily-' || to_char(now(), 'YYYY-MM-DD'));

-- Point-in-time recovery setup
ALTER SYSTEM SET wal_level = 'replica';
ALTER SYSTEM SET max_wal_senders = 10;
ALTER SYSTEM SET archive_mode = 'on';
ALTER SYSTEM SET archive_command = 'test ! -f /backup/archive/%f && cp %p /backup/archive/%f';
```

#### **Backup Retention Policy**
```yaml
backup_retention:
  daily_backups: 30_days
  weekly_backups: 12_weeks  
  monthly_backups: 12_months
  yearly_backups: 7_years
  
replication:
  streaming_replication: enabled
  standby_servers: 2
  async_replication: true
```

### **Monitoring Queries**

#### **Database Health Checks**
```sql
-- Table size monitoring
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size,
    pg_total_relation_size(schemaname||'.'||tablename) as size_bytes
FROM pg_tables 
WHERE schemaname = 'public'
ORDER BY size_bytes DESC;

-- Index usage monitoring  
SELECT 
    schemaname,
    tablename,
    attname,
    n_distinct,
    correlation
FROM pg_stats
WHERE schemaname = 'public'
ORDER BY n_distinct;

-- Connection monitoring
SELECT 
    state,
    count(*) 
FROM pg_stat_activity 
GROUP BY state;
```

---

## 📊 **Data Migration & Versioning**

### **Schema Version Management**

#### **schema_migrations**
```sql
CREATE TABLE schema_migrations (
    version VARCHAR(20) PRIMARY KEY,
    description TEXT NOT NULL,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    applied_by VARCHAR(100) NOT NULL,
    execution_time_ms INTEGER,
    checksum VARCHAR(64),
    
    -- Migration details
    migration_type migration_type NOT NULL,
    rollback_sql TEXT,
    is_reversible BOOLEAN DEFAULT false,
    
    -- Dependencies
    depends_on VARCHAR(20)[],
    conflicts_with VARCHAR(20)[]
);

CREATE TYPE migration_type AS ENUM ('schema', 'data', 'index', 'constraint');
```

### **Data Migration Procedures**

#### **Blockchain Data Backfill**
```sql
-- Procedure to backfill missing blockchain data
CREATE OR REPLACE FUNCTION backfill_market_data(
    start_slot BIGINT,
    end_slot BIGINT
) RETURNS INTEGER AS $$
DECLARE
    processed_count INTEGER := 0;
    current_slot BIGINT;
BEGIN
    -- Process each slot for market account changes
    FOR current_slot IN start_slot..end_slot LOOP
        -- Fetch and process market data for this slot
        -- Implementation would call blockchain sync service
        processed_count := processed_count + 1;
        
        IF processed_count % 1000 = 0 THEN
            RAISE NOTICE 'Processed % slots', processed_count;
        END IF;
    END LOOP;
    
    RETURN processed_count;
END;
$$ LANGUAGE plpgsql;
```

---

## 🎯 **Query Patterns & API Design**

### **Common Query Patterns**

#### **Market Listing Query**
```sql
-- Get paginated active markets with analytics
SELECT 
    m.market_pubkey,
    m.title,
    m.category,
    m.resolution_time,
    m.yes_pool,
    m.no_pool,
    m.total_volume,
    m.total_participants,
    COALESCE(ma.yes_price_close, 0.5) as current_yes_price,
    COALESCE(ma.daily_volume, 0) as daily_volume
FROM markets_cache m
LEFT JOIN market_analytics ma ON ma.market_pubkey = m.market_pubkey 
    AND ma.date = CURRENT_DATE
WHERE m.state = 'active'
    AND m.resolution_time > NOW()
    AND ($1 IS NULL OR m.category = $1)
ORDER BY m.total_volume DESC, m.created_at DESC
LIMIT $2 OFFSET $3;
```

#### **User Portfolio Query**
```sql
-- Get user's positions with current values
SELECT 
    p.position_pubkey,
    p.market_pubkey,
    m.title,
    m.category,
    m.state,
    p.yes_shares,
    p.no_shares,
    p.total_invested,
    p.current_value,
    p.unrealized_pnl,
    p.claimed,
    m.resolution_result
FROM positions_cache p
JOIN markets_cache m ON m.market_pubkey = p.market_pubkey
WHERE p.user_id = $1
    AND (p.yes_shares > 0 OR p.no_shares > 0 OR p.claimed = false)
ORDER BY p.updated_at DESC;
```

### **API Response Schemas**

#### **Market List Response**
```typescript
interface MarketListResponse {
  markets: {
    pubkey: string;
    id: number;
    title: string;
    category: string;
    resolutionTime: string;
    state: 'active' | 'resolved' | 'cancelled';
    pools: {
      yes: number;
      no: number;
      total: number;
    };
    prices: {
      yes: number;
      no: number;
    };
    analytics: {
      participants: number;
      dailyVolume: number;
      totalVolume: number;
    };
  }[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    hasMore: boolean;
  };
}
```

---

## 🔮 **Future Schema Enhancements**

### **Phase 2: Advanced Features**

#### **Multi-Outcome Markets**
```sql
-- Support for markets with more than YES/NO
CREATE TABLE market_outcomes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    market_pubkey VARCHAR(44) NOT NULL REFERENCES markets_cache(market_pubkey),
    outcome_id INTEGER NOT NULL,
    title VARCHAR(64) NOT NULL,
    description TEXT,
    probability DECIMAL(5, 4) DEFAULT 0,
    pool_amount DECIMAL(20, 8) DEFAULT 0,
    
    UNIQUE(market_pubkey, outcome_id)
);
```

#### **Social Features Schema**
```sql
-- User following/followers
CREATE TABLE user_follows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id),
    following_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(follower_id, following_id),
    CONSTRAINT no_self_follow CHECK (follower_id != following_id)
);

-- Market comments/discussions
CREATE TABLE market_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    market_pubkey VARCHAR(44) NOT NULL REFERENCES markets_cache(market_pubkey),
    user_id UUID NOT NULL REFERENCES users(id),
    parent_comment_id UUID REFERENCES market_comments(id),
    
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Moderation
    is_deleted BOOLEAN DEFAULT false,
    deleted_at TIMESTAMP WITH TIME ZONE,
    deleted_by UUID REFERENCES users(id)
);
```

### **Phase 3: Machine Learning Features**

#### **Prediction Accuracy Tracking**
```sql
CREATE TABLE user_prediction_accuracy (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    category VARCHAR(32),
    timeframe prediction_timeframe,
    
    -- Accuracy metrics
    total_predictions INTEGER DEFAULT 0,
    correct_predictions INTEGER DEFAULT 0,
    accuracy_rate DECIMAL(5, 4) DEFAULT 0,
    
    -- Confidence calibration
    avg_confidence DECIMAL(5, 4) DEFAULT 0,
    calibration_score DECIMAL(5, 4) DEFAULT 0,
    
    -- Time period
    period_start DATE,
    period_end DATE,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id, category, timeframe, period_start)
);

CREATE TYPE prediction_timeframe AS ENUM ('daily', 'weekly', 'monthly', 'quarterly', 'yearly');
```

---

## 📋 **Implementation Checklist**

### **Phase 1: Core Schema (Weeks 1-2)**
- [ ] Set up PostgreSQL with proper configuration
- [ ] Create base user and session management tables
- [ ] Implement market and position cache tables
- [ ] Set up basic indexing strategy
- [ ] Create blockchain sync service foundation

### **Phase 2: Analytics & Performance (Weeks 3-4)**
- [ ] Implement analytics tables and partitioning
- [ ] Set up Redis caching layer
- [ ] Create monitoring and health check queries
- [ ] Implement backup and recovery procedures
- [ ] Set up audit logging

### **Phase 3: Advanced Features (Weeks 5-6)**
- [ ] Add GDPR compliance schemas
- [ ] Implement data migration procedures
- [ ] Create comprehensive API query patterns
- [ ] Set up performance monitoring
- [ ] Implement data retention policies

---

*This database schema serves as the comprehensive data foundation for the Crypto Bet platform, balancing performance, security, compliance, and scalability requirements.*

**Next Review**: Weekly schema review during development  
**Document Owner**: Backend Architecture Team  
**Approval**: Database Administration Team