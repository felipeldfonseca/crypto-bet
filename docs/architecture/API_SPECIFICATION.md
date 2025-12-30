# 🔌 API Specification

## 📋 **Document Overview**

**Purpose**: Define comprehensive API interfaces for Crypto Bet prediction markets platform  
**Scope**: Smart contract instructions, REST endpoints, WebSocket APIs, and integration interfaces  
**Audience**: Frontend developers, backend developers, third-party integrators  
**Last Updated**: December 2024  
**Version**: 1.0  

---

## 🎯 **Executive Summary**

Crypto Bet provides a comprehensive API surface consisting of Solana program instructions for blockchain interactions, REST APIs for data access and caching, WebSocket APIs for real-time updates, and third-party integration endpoints. The API design prioritizes developer experience, type safety, and performance while maintaining security and compliance standards.

**Key API Components:**
- **Solana Program**: On-chain instructions for core betting functionality
- **REST API**: HTTP endpoints for data retrieval and user management
- **WebSocket API**: Real-time market updates and live data streams  
- **Integration APIs**: Jupiter swap integration and external data feeds

---

## ⛓️ **Solana Program Instructions**

### **Program Information**
```typescript
export const CRYPTO_BET_PROGRAM_ID = new PublicKey("CryptoBetXXXXXXXXXXXXXXXXXXXXXXXXXXX");
export const PROGRAM_VERSION = "1.0.0";
```

### **Instruction Set Overview**

| Instruction | Purpose | Access Level | Signer Required |
|-------------|---------|--------------|-----------------|
| `initialize_program` | Initialize global config | Authority | Program Authority |
| `create_market` | Create prediction market | Public | Market Creator |
| `place_bet` | Place bet on market outcome | Public | User |
| `resolve_market` | Resolve market outcome | Restricted | Market Authority |
| `claim_winnings` | Claim resolved winnings | Public | Position Holder |
| `claim_refund` | Claim refund for cancelled market | Public | Position Holder |
| `cancel_market` | Cancel market before resolution | Restricted | Market Authority |
| `update_market` | Update market metadata | Restricted | Market Authority |
| `emergency_pause` | Emergency pause all operations | Emergency | Program Authority |

---

## 🏗️ **Smart Contract Instructions**

### **1. Initialize Program**

**Purpose**: Initialize global program configuration

```rust
#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = GlobalConfig::LEN,
        seeds = [b"global_config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeProgramArgs {
    pub platform_fee_bps: u16,
    pub min_market_duration: i64,
    pub max_market_duration: i64,
    pub min_bet_amount: u64,
    pub max_bet_amount: u64,
    pub platform_treasury: Pubkey,
}
```

**TypeScript Interface:**
```typescript
export interface InitializeProgramInstruction {
  authority: PublicKey;
  globalConfig: PublicKey;
  args: {
    platformFeeBps: number;        // 250 = 2.5%
    minMarketDuration: number;     // seconds
    maxMarketDuration: number;     // seconds  
    minBetAmount: number;          // lamports
    maxBetAmount: number;          // lamports
    platformTreasury: PublicKey;
  };
}

// Usage example
const instruction = await program.methods
  .initializeProgram({
    platformFeeBps: 250,
    minMarketDuration: 3600,        // 1 hour
    maxMarketDuration: 31536000,    // 1 year
    minBetAmount: 1000000,          // 0.001 SOL
    maxBetAmount: 1000000000000,    // 1000 SOL
    platformTreasury: treasuryPubkey,
  })
  .accounts({
    authority: authority.publicKey,
    globalConfig: globalConfigPda,
    systemProgram: SystemProgram.programId,
  })
  .instruction();
```

### **2. Create Market**

**Purpose**: Create a new prediction market

```rust
#[derive(Accounts)]
#[instruction(args: CreateMarketArgs)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Market::LEN,
        seeds = [
            b"market",
            &args.market_id.to_le_bytes(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        seeds = [b"global_config"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    
    /// CHECK: Token mint validation in instruction
    pub accepted_mint: AccountInfo<'info>,
    
    #[account(
        init,
        payer = authority,
        token::mint = accepted_mint,
        token::authority = market,
        seeds = [b"yes_vault", market.key().as_ref()],
        bump
    )]
    pub yes_vault: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        token::mint = accepted_mint,
        token::authority = market,
        seeds = [b"no_vault", market.key().as_ref()],
        bump
    )]
    pub no_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMarketArgs {
    pub market_id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub resolution_time: i64,
    pub market_type: MarketType,
    pub metadata_uri: String,
}
```

**TypeScript Interface:**
```typescript
export interface CreateMarketInstruction {
  authority: PublicKey;
  market: PublicKey;
  globalConfig: PublicKey;
  acceptedMint: PublicKey;
  yesVault: PublicKey;
  noVault: PublicKey;
  args: {
    marketId: number;
    title: string;              // max 64 chars
    description: string;        // max 256 chars  
    category: string;           // max 32 chars
    resolutionTime: number;     // Unix timestamp
    marketType: MarketType;     // 'degen' | 'stable'
    metadataUri: string;        // IPFS URI
  };
}

// Market type enum
export enum MarketType {
  Degen = 0,    // SOL betting
  Stable = 1,   // USDC betting
}

// Usage example
const marketId = Date.now(); // Simple ID generation
const [marketPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("market"),
    new BN(marketId).toArrayLike(Buffer, "le", 8),
    authority.publicKey.toBuffer(),
  ],
  program.programId
);

const instruction = await program.methods
  .createMarket({
    marketId,
    title: "Will Bitcoin reach $100k by end of 2024?",
    description: "Market resolves YES if Bitcoin (BTC) reaches $100,000 USD...",
    category: "Crypto",
    resolutionTime: new BN(Math.floor(Date.now() / 1000) + 86400), // 24 hours
    marketType: MarketType.Degen,
    metadataUri: "https://ipfs.io/ipfs/Qm...",
  })
  .accounts({
    authority: authority.publicKey,
    market: marketPda,
    globalConfig: globalConfigPda,
    acceptedMint: NATIVE_MINT, // SOL
    yesVault: yesVaultPda,
    noVault: noVaultPda,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .instruction();
```

### **3. Place Bet**

**Purpose**: Place a bet on a market outcome

```rust
#[derive(Accounts)]
#[instruction(args: PlaceBetArgs)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut)]
    pub market: Account<'info, Market>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = Position::LEN,
        seeds = [
            b"position",
            user.key().as_ref(),
            market.key().as_ref()
        ],
        bump
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = vault.key() == if args.side == BetSide::Yes { 
            market.yes_vault 
        } else { 
            market.no_vault 
        }
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"platform_treasury"],
        bump
    )]
    pub platform_treasury: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlaceBetArgs {
    pub side: BetSide,
    pub amount: u64,
    pub max_price: u64,    // Slippage protection
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BetSide {
    Yes,
    No,
}
```

**TypeScript Interface:**
```typescript
export interface PlaceBetInstruction {
  user: PublicKey;
  market: PublicKey;
  position: PublicKey;
  userTokenAccount: PublicKey;
  vault: PublicKey;
  platformTreasury: PublicKey;
  args: {
    side: BetSide;
    amount: number;        // Amount to bet in smallest unit
    maxPrice: number;      // Max price willing to pay (slippage protection)
  };
}

export enum BetSide {
  Yes = 0,
  No = 1,
}

// Usage example - Place YES bet
const amount = 1000000000; // 1 SOL in lamports
const maxPrice = 600000000; // Willing to pay up to 0.6 SOL per share

const [positionPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("position"),
    user.publicKey.toBuffer(),
    marketPda.toBuffer(),
  ],
  program.programId
);

const instruction = await program.methods
  .placeBet({
    side: BetSide.Yes,
    amount: new BN(amount),
    maxPrice: new BN(maxPrice),
  })
  .accounts({
    user: user.publicKey,
    market: marketPda,
    position: positionPda,
    userTokenAccount: userTokenAccount,
    vault: yesVaultPda, // or noVaultPda based on bet side
    platformTreasury: platformTreasuryPda,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .instruction();
```

### **4. Resolve Market**

**Purpose**: Resolve a market with final outcome

```rust
#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = market.authority == authority.key() || 
                    market.resolver == authority.key()
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        seeds = [b"global_config"],
        bump = global_config.bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ResolveMarketArgs {
    pub outcome: bool,     // true = YES wins, false = NO wins
    pub final_data: String, // Resolution explanation/data
}
```

**TypeScript Interface:**
```typescript
export interface ResolveMarketInstruction {
  authority: PublicKey;
  market: PublicKey;
  globalConfig: PublicKey;
  args: {
    outcome: boolean;      // true = YES, false = NO
    finalData: string;     // Resolution explanation
  };
}

// Usage example
const instruction = await program.methods
  .resolveMarket({
    outcome: true, // YES wins
    finalData: "Bitcoin reached $100,847 on December 5, 2024",
  })
  .accounts({
    authority: authority.publicKey,
    market: marketPda,
    globalConfig: globalConfigPda,
  })
  .instruction();
```

### **5. Claim Winnings**

**Purpose**: Claim winnings from a resolved market

```rust
#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        constraint = market.state == MarketState::Resolved
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        mut,
        constraint = position.user == user.key() &&
                    position.market == market.key() &&
                    !position.claimed
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = winning_vault.key() == if market.resolution_result.unwrap() {
            market.yes_vault
        } else {
            market.no_vault
        }
    )]
    pub winning_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}
```

**TypeScript Interface:**
```typescript
export interface ClaimWinningsInstruction {
  user: PublicKey;
  market: PublicKey;
  position: PublicKey;
  userTokenAccount: PublicKey;
  winningVault: PublicKey;
}

// Usage example
const instruction = await program.methods
  .claimWinnings()
  .accounts({
    user: user.publicKey,
    market: marketPda,
    position: positionPda,
    userTokenAccount: userTokenAccount,
    winningVault: yesVaultPda, // or noVaultPda based on market outcome
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .instruction();
```

---

## 🌐 **REST API Endpoints**

### **Base Configuration**
```typescript
export const API_CONFIG = {
  BASE_URL: process.env.NEXT_PUBLIC_API_URL || 'https://api.crypto-bet.io',
  VERSION: 'v1',
  TIMEOUT: 30000,
  RATE_LIMIT: '100 requests per minute',
};

// Base response interface
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: any;
  };
  meta?: {
    timestamp: string;
    requestId: string;
    version: string;
  };
}
```

### **Authentication Endpoints**

#### **POST /auth/wallet/connect**
```typescript
// Connect wallet and create session
interface WalletConnectRequest {
  walletAddress: string;
  signature: string;
  message: string;
}

interface WalletConnectResponse {
  sessionToken: string;
  expiresAt: string;
  user: {
    id: string;
    walletAddress: string;
    username?: string;
    createdAt: string;
  };
}

// Example usage
const connectWallet = async (
  walletAddress: string,
  signature: string,
  message: string
): Promise<WalletConnectResponse> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/auth/wallet/connect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ walletAddress, signature, message }),
  });
  
  const result: ApiResponse<WalletConnectResponse> = await response.json();
  if (!result.success) throw new Error(result.error?.message);
  return result.data!;
};
```

#### **POST /auth/wallet/disconnect**
```typescript
// Disconnect wallet and invalidate session
interface WalletDisconnectRequest {
  sessionToken: string;
}

const disconnectWallet = async (sessionToken: string): Promise<void> => {
  await fetch(`${API_CONFIG.BASE_URL}/v1/auth/wallet/disconnect`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${sessionToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ sessionToken }),
  });
};
```

### **Market Endpoints**

#### **GET /markets**
```typescript
// Get paginated list of markets
interface MarketListRequest {
  page?: number;
  limit?: number;
  category?: string;
  state?: 'active' | 'resolved' | 'cancelled';
  search?: string;
  sortBy?: 'volume' | 'created' | 'ending';
  sortOrder?: 'asc' | 'desc';
  marketType?: 'degen' | 'stable';
}

interface MarketListResponse {
  markets: Market[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
    hasNext: boolean;
    hasPrev: boolean;
  };
}

interface Market {
  pubkey: string;
  id: number;
  title: string;
  description: string;
  category: string;
  state: 'active' | 'resolved' | 'cancelled';
  marketType: 'degen' | 'stable';
  createdAt: string;
  resolutionTime: string;
  resolvedAt?: string;
  resolutionResult?: boolean;
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
    totalVolume: number;
    dailyVolume: number;
    participants: number;
    volume24h: number;
  };
  metadata: {
    uri?: string;
    tags?: string[];
  };
}

// Example usage
const getMarkets = async (params: MarketListRequest = {}): Promise<MarketListResponse> => {
  const queryParams = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined) queryParams.append(key, value.toString());
  });
  
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/markets?${queryParams}`);
  const result: ApiResponse<MarketListResponse> = await response.json();
  return result.data!;
};
```

#### **GET /markets/:pubkey**
```typescript
// Get detailed market information
interface MarketDetailResponse extends Market {
  authority: string;
  acceptedMint: string;
  vaults: {
    yes: string;
    no: string;
  };
  priceHistory: {
    timestamp: string;
    yesPrice: number;
    noPrice: number;
    volume: number;
  }[];
  recentBets: {
    id: string;
    user: string;
    side: 'yes' | 'no';
    amount: number;
    price: number;
    timestamp: string;
  }[];
}

const getMarketDetail = async (pubkey: string): Promise<MarketDetailResponse> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/markets/${pubkey}`);
  const result: ApiResponse<MarketDetailResponse> = await response.json();
  return result.data!;
};
```

#### **POST /markets**
```typescript
// Create new market (requires authentication)
interface CreateMarketRequest {
  title: string;
  description: string;
  category: string;
  resolutionTime: string;
  marketType: 'degen' | 'stable';
  metadataUri?: string;
  tags?: string[];
}

const createMarket = async (
  marketData: CreateMarketRequest,
  sessionToken: string
): Promise<{ marketPubkey: string; transactionSignature: string }> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/markets`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${sessionToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(marketData),
  });
  
  const result = await response.json();
  return result.data;
};
```

### **Position Endpoints**

#### **GET /positions**
```typescript
// Get user's positions
interface PositionListRequest {
  page?: number;
  limit?: number;
  marketState?: 'active' | 'resolved' | 'cancelled';
  claimed?: boolean;
}

interface Position {
  pubkey: string;
  user: string;
  market: {
    pubkey: string;
    title: string;
    category: string;
    state: 'active' | 'resolved' | 'cancelled';
    resolutionResult?: boolean;
  };
  shares: {
    yes: number;
    no: number;
  };
  invested: number;
  currentValue: number;
  unrealizedPnl: number;
  realizedPnl: number;
  claimed: boolean;
  claimedAmount: number;
  createdAt: string;
  updatedAt: string;
}

interface PositionListResponse {
  positions: Position[];
  summary: {
    totalInvested: number;
    currentValue: number;
    totalPnl: number;
    winRate: number;
  };
  pagination: {
    page: number;
    limit: number;
    total: number;
  };
}

const getPositions = async (
  params: PositionListRequest,
  sessionToken: string
): Promise<PositionListResponse> => {
  const queryParams = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined) queryParams.append(key, value.toString());
  });
  
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/positions?${queryParams}`, {
    headers: { 'Authorization': `Bearer ${sessionToken}` },
  });
  
  const result: ApiResponse<PositionListResponse> = await response.json();
  return result.data!;
};
```

### **Transaction Endpoints**

#### **GET /transactions**
```typescript
// Get transaction history
interface TransactionListRequest {
  page?: number;
  limit?: number;
  type?: 'place_bet' | 'claim_winnings' | 'claim_refund';
  marketPubkey?: string;
  startDate?: string;
  endDate?: string;
}

interface Transaction {
  id: string;
  signature: string;
  type: 'place_bet' | 'claim_winnings' | 'claim_refund' | 'create_market';
  amount: number;
  side?: 'yes' | 'no';
  price?: number;
  market?: {
    pubkey: string;
    title: string;
  };
  status: 'confirmed' | 'failed' | 'pending';
  blockTime: string;
  fees: {
    transaction: number;
    platform: number;
  };
}

const getTransactions = async (
  params: TransactionListRequest,
  sessionToken: string
): Promise<{ transactions: Transaction[]; pagination: any }> => {
  const queryParams = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined) queryParams.append(key, value.toString());
  });
  
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/transactions?${queryParams}`, {
    headers: { 'Authorization': `Bearer ${sessionToken}` },
  });
  
  const result = await response.json();
  return result.data;
};
```

### **Analytics Endpoints**

#### **GET /analytics/platform**
```typescript
// Get platform-wide analytics
interface PlatformAnalytics {
  totalVolume: number;
  totalUsers: number;
  totalMarkets: number;
  activeMarkets: number;
  volume24h: number;
  volume7d: number;
  volume30d: number;
  topCategories: {
    category: string;
    volume: number;
    markets: number;
  }[];
  recentActivity: {
    timestamp: string;
    volume: number;
    transactions: number;
  }[];
}

const getPlatformAnalytics = async (): Promise<PlatformAnalytics> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/analytics/platform`);
  const result: ApiResponse<PlatformAnalytics> = await response.json();
  return result.data!;
};
```

#### **GET /analytics/user**
```typescript
// Get user-specific analytics
interface UserAnalytics {
  totalBets: number;
  totalVolume: number;
  winRate: number;
  totalPnl: number;
  averageBetSize: number;
  favoriteCategories: string[];
  performanceByCategory: {
    category: string;
    winRate: number;
    volume: number;
    pnl: number;
  }[];
  activityHistory: {
    date: string;
    bets: number;
    volume: number;
    pnl: number;
  }[];
}

const getUserAnalytics = async (sessionToken: string): Promise<UserAnalytics> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/analytics/user`, {
    headers: { 'Authorization': `Bearer ${sessionToken}` },
  });
  
  const result: ApiResponse<UserAnalytics> = await response.json();
  return result.data!;
};
```

---

## 🔄 **WebSocket API**

### **Connection & Authentication**
```typescript
interface WebSocketConfig {
  url: string;
  protocols?: string[];
  options?: {
    maxReconnectAttempts?: number;
    reconnectInterval?: number;
  };
}

class CryptoBetWebSocket {
  private ws: WebSocket | null = null;
  private sessionToken?: string;
  private subscriptions = new Map<string, Function>();
  
  constructor(private config: WebSocketConfig) {}
  
  connect(sessionToken?: string): Promise<void> {
    return new Promise((resolve, reject) => {
      this.sessionToken = sessionToken;
      this.ws = new WebSocket(this.config.url, this.config.protocols);
      
      this.ws.onopen = () => {
        if (sessionToken) {
          this.authenticate(sessionToken);
        }
        resolve();
      };
      
      this.ws.onerror = reject;
      this.ws.onmessage = this.handleMessage.bind(this);
    });
  }
  
  private authenticate(token: string) {
    this.send({
      type: 'auth',
      data: { token }
    });
  }
  
  private send(message: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }
  }
  
  private handleMessage(event: MessageEvent) {
    try {
      const message = JSON.parse(event.data);
      const handler = this.subscriptions.get(message.type);
      if (handler) {
        handler(message.data);
      }
    } catch (error) {
      console.error('WebSocket message parsing error:', error);
    }
  }
}
```

### **Market Data Subscriptions**

#### **Subscribe to Market Updates**
```typescript
interface MarketUpdateSubscription {
  marketPubkey: string;
  callback: (update: MarketUpdate) => void;
}

interface MarketUpdate {
  marketPubkey: string;
  timestamp: string;
  update: {
    pools?: { yes: number; no: number };
    prices?: { yes: number; no: number };
    volume?: number;
    participants?: number;
    state?: 'active' | 'resolved' | 'cancelled';
    resolutionResult?: boolean;
  };
}

// Subscribe to real-time market updates
const subscribeToMarket = (ws: CryptoBetWebSocket, subscription: MarketUpdateSubscription) => {
  ws.send({
    type: 'subscribe',
    channel: 'market_updates',
    data: { marketPubkey: subscription.marketPubkey }
  });
  
  ws.subscriptions.set('market_update', (data: MarketUpdate) => {
    if (data.marketPubkey === subscription.marketPubkey) {
      subscription.callback(data);
    }
  });
};
```

#### **Subscribe to Live Trades**
```typescript
interface TradeSubscription {
  marketPubkey?: string; // Optional: specific market or all markets
  callback: (trade: LiveTrade) => void;
}

interface LiveTrade {
  marketPubkey: string;
  marketTitle: string;
  user: string;
  side: 'yes' | 'no';
  amount: number;
  price: number;
  timestamp: string;
  signature: string;
}

const subscribeToTrades = (ws: CryptoBetWebSocket, subscription: TradeSubscription) => {
  ws.send({
    type: 'subscribe',
    channel: 'live_trades',
    data: { marketPubkey: subscription.marketPubkey }
  });
  
  ws.subscriptions.set('live_trade', subscription.callback);
};
```

#### **Subscribe to User Updates**
```typescript
interface UserUpdateSubscription {
  callback: (update: UserUpdate) => void;
}

interface UserUpdate {
  type: 'position_update' | 'balance_update' | 'notification';
  data: {
    positionPubkey?: string;
    marketPubkey?: string;
    newBalance?: number;
    message?: string;
    timestamp: string;
  };
}

const subscribeToUserUpdates = (ws: CryptoBetWebSocket, subscription: UserUpdateSubscription) => {
  ws.send({
    type: 'subscribe',
    channel: 'user_updates',
    data: {}
  });
  
  ws.subscriptions.set('user_update', subscription.callback);
};
```

---

## 🔗 **Integration APIs**

### **Jupiter Swap Integration**

#### **Get Swap Quote**
```typescript
interface SwapQuoteRequest {
  inputMint: string;      // Source token mint
  outputMint: string;     // Destination token mint  
  amount: number;         // Input amount in smallest unit
  slippageBps?: number;   // Slippage tolerance in basis points (default: 50)
}

interface SwapQuoteResponse {
  inputMint: string;
  outputMint: string;
  inAmount: number;
  outAmount: number;
  otherAmountThreshold: number;
  swapMode: 'ExactIn' | 'ExactOut';
  slippageBps: number;
  platformFee?: {
    amount: number;
    feeBps: number;
  };
  priceImpactPct: number;
  routePlan: {
    swapInfo: {
      ammKey: string;
      label: string;
      inputMint: string;
      outputMint: string;
      inAmount: number;
      outAmount: number;
      feeAmount: number;
      feeMint: string;
    };
    percent: number;
  }[];
  contextSlot: number;
  timeTaken: number;
}

const getSwapQuote = async (params: SwapQuoteRequest): Promise<SwapQuoteResponse> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/swap/quote`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  
  const result: ApiResponse<SwapQuoteResponse> = await response.json();
  return result.data!;
};
```

#### **Execute Swap**
```typescript
interface SwapExecutionRequest {
  quoteResponse: SwapQuoteResponse;
  userPublicKey: string;
  wrapUnwrapSOL?: boolean;
  useSharedAccounts?: boolean;
}

interface SwapExecutionResponse {
  swapTransaction: string;  // Base64 encoded transaction
  lastValidBlockHeight: number;
}

const executeSwap = async (params: SwapExecutionRequest, sessionToken: string): Promise<SwapExecutionResponse> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/swap/execute`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${sessionToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(params),
  });
  
  const result: ApiResponse<SwapExecutionResponse> = await response.json();
  return result.data!;
};
```

### **Price Feed Integration**

#### **Get Token Prices**
```typescript
interface TokenPriceRequest {
  mints: string[];        // Array of token mints
  currency?: 'USD' | 'SOL'; // Price currency (default: USD)
}

interface TokenPrice {
  mint: string;
  symbol: string;
  name: string;
  price: number;
  change24h: number;
  volume24h: number;
  marketCap?: number;
  timestamp: string;
}

const getTokenPrices = async (params: TokenPriceRequest): Promise<TokenPrice[]> => {
  const response = await fetch(`${API_CONFIG.BASE_URL}/v1/prices/tokens`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  
  const result: ApiResponse<TokenPrice[]> = await response.json();
  return result.data!;
};
```

---

## 📊 **Rate Limiting & Error Handling**

### **Rate Limiting**
```typescript
interface RateLimit {
  endpoint: string;
  limit: number;           // Requests per window
  window: number;          // Window duration in seconds
  burst?: number;          // Burst capacity
}

const RATE_LIMITS: RateLimit[] = [
  { endpoint: '/markets', limit: 100, window: 60 },
  { endpoint: '/positions', limit: 200, window: 60 },
  { endpoint: '/transactions', limit: 50, window: 60 },
  { endpoint: '/swap/*', limit: 20, window: 60 },
  { endpoint: '/auth/*', limit: 10, window: 60 },
];

// Rate limit headers in response
interface RateLimitHeaders {
  'X-RateLimit-Limit': string;
  'X-RateLimit-Remaining': string;
  'X-RateLimit-Reset': string;
  'X-RateLimit-Window': string;
}
```

### **Error Codes**
```typescript
enum ApiErrorCode {
  // Authentication errors (4000-4099)
  UNAUTHORIZED = 'AUTH_001',
  INVALID_TOKEN = 'AUTH_002',
  TOKEN_EXPIRED = 'AUTH_003',
  INSUFFICIENT_PERMISSIONS = 'AUTH_004',
  
  // Validation errors (4100-4199)
  INVALID_INPUT = 'VAL_001',
  MISSING_FIELD = 'VAL_002',
  INVALID_FORMAT = 'VAL_003',
  VALUE_OUT_OF_RANGE = 'VAL_004',
  
  // Market errors (4200-4299)
  MARKET_NOT_FOUND = 'MKT_001',
  MARKET_INACTIVE = 'MKT_002',
  MARKET_RESOLVED = 'MKT_003',
  MARKET_CANCELLED = 'MKT_004',
  INSUFFICIENT_LIQUIDITY = 'MKT_005',
  
  // Position errors (4300-4399)
  POSITION_NOT_FOUND = 'POS_001',
  INSUFFICIENT_BALANCE = 'POS_002',
  ALREADY_CLAIMED = 'POS_003',
  NOTHING_TO_CLAIM = 'POS_004',
  
  // System errors (5000-5099)
  INTERNAL_ERROR = 'SYS_001',
  BLOCKCHAIN_ERROR = 'SYS_002',
  DATABASE_ERROR = 'SYS_003',
  EXTERNAL_SERVICE_ERROR = 'SYS_004',
  
  // Rate limiting (4290-4299)
  RATE_LIMIT_EXCEEDED = 'RL_001',
  TOO_MANY_REQUESTS = 'RL_002',
}

interface ApiError {
  code: ApiErrorCode;
  message: string;
  details?: any;
  timestamp: string;
  requestId: string;
  documentation?: string;
}
```

### **Error Handling Utilities**
```typescript
class ApiClient {
  async handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
    const data: ApiResponse<T> = await response.json();
    
    if (!response.ok) {
      throw new ApiClientError(
        data.error?.code || 'UNKNOWN_ERROR',
        data.error?.message || 'An unknown error occurred',
        response.status,
        data.error?.details
      );
    }
    
    return data;
  }
}

class ApiClientError extends Error {
  constructor(
    public code: string,
    message: string,
    public statusCode: number,
    public details?: any
  ) {
    super(message);
    this.name = 'ApiClientError';
  }
  
  isRateLimitError(): boolean {
    return this.code === ApiErrorCode.RATE_LIMIT_EXCEEDED;
  }
  
  isAuthError(): boolean {
    return this.code.startsWith('AUTH_');
  }
  
  isRetryable(): boolean {
    return this.statusCode >= 500 || this.isRateLimitError();
  }
}
```

---

## 🔧 **SDK Implementation**

### **TypeScript SDK**
```typescript
// Main SDK class
export class CryptoBetSDK {
  private apiClient: ApiClient;
  private wsClient: CryptoBetWebSocket;
  private program: Program;
  
  constructor(
    private config: {
      apiUrl: string;
      wsUrl: string;
      programId: string;
      connection: Connection;
    }
  ) {
    this.apiClient = new ApiClient(config.apiUrl);
    this.wsClient = new CryptoBetWebSocket({ url: config.wsUrl });
    this.program = new Program(/* Anchor program */);
  }
  
  // Authentication
  async connectWallet(wallet: WalletAdapter): Promise<AuthSession> {
    const message = `Sign in to Crypto Bet: ${Date.now()}`;
    const signature = await wallet.signMessage(new TextEncoder().encode(message));
    
    return this.apiClient.connectWallet({
      walletAddress: wallet.publicKey!.toString(),
      signature: bs58.encode(signature),
      message,
    });
  }
  
  // Markets
  async getMarkets(filters?: MarketFilters): Promise<Market[]> {
    return this.apiClient.getMarkets(filters);
  }
  
  async createMarket(marketData: CreateMarketData): Promise<string> {
    const instruction = await this.program.methods
      .createMarket(marketData)
      .instruction();
      
    const transaction = new Transaction().add(instruction);
    const signature = await this.wallet.sendTransaction(transaction);
    
    return signature;
  }
  
  async placeBet(
    marketPubkey: string,
    side: 'yes' | 'no',
    amount: number
  ): Promise<string> {
    const instruction = await this.program.methods
      .placeBet({ side, amount })
      .accounts({
        market: new PublicKey(marketPubkey),
        // ... other accounts
      })
      .instruction();
      
    const transaction = new Transaction().add(instruction);
    return this.wallet.sendTransaction(transaction);
  }
  
  // Real-time subscriptions
  subscribeToMarket(
    marketPubkey: string,
    callback: (update: MarketUpdate) => void
  ): () => void {
    this.wsClient.subscribe('market_updates', { marketPubkey }, callback);
    return () => this.wsClient.unsubscribe('market_updates', { marketPubkey });
  }
}
```

---

## 📋 **Implementation Checklist**

### **Phase 1: Core API (Weeks 1-2)**
- [ ] Implement Solana program instructions
- [ ] Create REST API endpoints for markets and positions
- [ ] Set up authentication and session management
- [ ] Implement basic error handling and validation
- [ ] Create TypeScript interfaces and types

### **Phase 2: Advanced Features (Weeks 3-4)**
- [ ] Implement WebSocket API for real-time updates
- [ ] Add Jupiter swap integration
- [ ] Create analytics and reporting endpoints
- [ ] Implement rate limiting and security measures
- [ ] Add comprehensive error handling

### **Phase 3: SDK & Documentation (Weeks 5-6)**
- [ ] Build TypeScript SDK
- [ ] Create API documentation and examples
- [ ] Implement testing suite for all endpoints
- [ ] Set up monitoring and logging
- [ ] Performance optimization and caching

---

*This API specification provides the complete interface definition for the Crypto Bet platform, enabling seamless integration between frontend applications, blockchain interactions, and third-party services.*

**Next Review**: Weekly API review during development  
**Document Owner**: API Development Team  
**Approval**: Technical Architecture Team