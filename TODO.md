# 🎯 Crypto Bet MVP Development Roadmap

## 📋 **Document Overview**

**Purpose**: North star development guideline from foundation to MVP launch  
**Scope**: Granular, prioritized tasks with clear success criteria  
**Audience**: Development team, project managers, stakeholders  
**Last Updated**: December 2024  
**Version**: 2.0 (Complete rewrite)  

---

## 🎯 **Executive Summary**

This roadmap transforms the Crypto Bet project from its current documentation-complete state into a production-ready MVP. Based on our comprehensive enterprise documentation suite, this roadmap provides granular, prioritized tasks that will take us from foundational setup to MVP launch in 12 weeks.

**Current State**: ✅ **Documentation Complete** - All Phase 1 critical documentation finished  
**Target State**: 🚀 **MVP Launch** - Production-ready dual-mode prediction markets platform  
**Timeline**: 12 weeks to MVP  
**Focus**: Execution excellence without feature creep  

---

## 🏗️ **Development Strategy**

### **🎯 MVP Scope Definition**
Based on [product/PRODUCT_REQUIREMENTS.md](./docs/product/PRODUCT_REQUIREMENTS.md), our MVP includes:

**Core Features (Must Have):**
- ✅ Dual-mode betting (SOL/USDC) - *The key differentiator*
- ✅ Market creation and browsing with filtering
- ✅ Betting interface with position management  
- ✅ Wallet integration (Phantom, Solflare, Backpack)
- ✅ Real-time updates via WebSocket
- ✅ Jupiter swap integration for seamless token conversion

**Success Criteria:**
- 🎯 Support 100+ concurrent users
- 🎯 < 2 second page load times
- 🎯 99.5%+ transaction success rate
- 🎯 Mobile-responsive design
- 🎯 Zero critical security vulnerabilities

---

## 📅 **12-Week MVP Timeline**

### **Phase 1: Foundation (Weeks 1-3)**
Foundation setup and core smart contract deployment

### **Phase 2: Core Development (Weeks 4-8)**  
Core features implementation and integration

### **Phase 3: Integration & Testing (Weeks 9-11)**
System integration, testing, and optimization

### **Phase 4: Launch Preparation (Week 12)**
Final testing, deployment, and go-live

---

## 🏗️ **PHASE 1: FOUNDATION (Weeks 1-3)**

> **Goal**: Establish solid foundation for rapid development  
> **Success Criteria**: All development environments working, smart contracts deployed, CI/CD operational

### **Week 1: Development Environment & Smart Contracts**

#### **1.1 Development Environment Setup** ⚡ *Critical Path*
- [ ] **1.1.1 Clone and Setup Repository**
  ```bash
  git clone https://github.com/your-org/crypto-bet.git
  cd crypto-bet
  npm install
  ```
  - **Owner**: Lead Developer
  - **Success Criteria**: All dependencies installed, no errors
  - **Time Estimate**: 2 hours

- [ ] **1.1.2 Solana Development Environment**
  ```bash
  # Install Solana CLI
  sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
  
  # Install Anchor
  npm install -g @coral-xyz/anchor-cli
  
  # Configure for devnet
  solana config set --url devnet
  ```
  - **Owner**: Blockchain Developer
  - **Success Criteria**: Solana and Anchor CLI working, connected to devnet
  - **Time Estimate**: 1 hour
  - **Documentation**: [operations/DEPLOYMENT_GUIDE.md](./docs/operations/DEPLOYMENT_GUIDE.md#smart-contract-deployment)

- [ ] **1.1.3 Local Database Setup**
  ```bash
  # Start PostgreSQL and Redis locally
  docker-compose -f docker-compose.dev.yml up -d
  
  # Run database migrations
  npm run db:migrate:dev
  npm run db:seed:dev
  ```
  - **Owner**: Backend Developer  
  - **Success Criteria**: PostgreSQL and Redis running, schema applied
  - **Time Estimate**: 3 hours
  - **Documentation**: [architecture/DATABASE_SCHEMA.md](./docs/architecture/DATABASE_SCHEMA.md)

#### **1.2 Smart Contract Deployment** ⚡ *Critical Path*
- [ ] **1.2.1 Build and Test Smart Contract**
  ```bash
  cd programs/crypto-bet
  anchor build
  anchor test
  ```
  - **Owner**: Blockchain Developer
  - **Success Criteria**: All tests pass, program builds successfully
  - **Time Estimate**: 4 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#smart-contract-instructions)

- [ ] **1.2.2 Deploy to Devnet**
  ```bash
  # Deploy program
  anchor deploy --provider.cluster devnet
  
  # Initialize program state
  npm run initialize:devnet
  ```
  - **Owner**: Blockchain Developer
  - **Success Criteria**: Program deployed, global config initialized
  - **Time Estimate**: 2 hours
  - **Blockers**: Requires 1.1.2 completion

- [ ] **1.2.3 Verify Smart Contract Integration**
  ```typescript
  // Test basic program interactions
  const program = anchor.workspace.CryptoBet;
  const config = await program.account.globalConfig.fetch(configPda);
  console.log("Platform fee:", config.platformFeeBps);
  ```
  - **Owner**: Blockchain Developer  
  - **Success Criteria**: Can read program state from devnet
  - **Time Estimate**: 1 hour
  - **Blockers**: Requires 1.2.2 completion

### **Week 2: Project Structure & CI/CD**

#### **1.3 Next.js Project Setup** ⚡ *Critical Path*
- [ ] **1.3.1 Initialize Next.js Application**
  ```bash
  # Create new Next.js project with recommended configuration
  npx create-next-app@latest crypto-bet-frontend \
    --typescript --tailwind --app --src-dir --import-alias "@/*"
  
  # Install required dependencies
  npm install @solana/wallet-adapter-react @solana/wallet-adapter-wallets
  npm install @coral-xyz/anchor @solana/web3.js
  npm install @radix-ui/react-dialog @radix-ui/react-dropdown-menu
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Next.js app running locally, all dependencies installed
  - **Time Estimate**: 3 hours
  - **Documentation**: [architecture/SYSTEM_ARCHITECTURE.md](./docs/architecture/SYSTEM_ARCHITECTURE.md#frontend-architecture)

- [ ] **1.3.2 Configure TypeScript and ESLint**
  ```json
  // tsconfig.json configuration per documentation standards
  {
    "compilerOptions": {
      "target": "es5",
      "lib": ["dom", "dom.iterable", "esnext"],
      "allowJs": true,
      "skipLibCheck": true,
      "strict": true,
      "forceConsistentCasingInFileNames": true,
      "noEmit": true,
      "esModuleInterop": true,
      "module": "esnext",
      "moduleResolution": "node",
      "resolveJsonModule": true,
      "isolatedModules": true,
      "jsx": "preserve",
      "incremental": true,
      "plugins": [{ "name": "next" }],
      "baseUrl": ".",
      "paths": { "@/*": ["./src/*"] }
    }
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: TypeScript strict mode working, no lint errors
  - **Time Estimate**: 2 hours

- [ ] **1.3.3 Setup Tailwind CSS and Component Library**
  ```bash
  # Install shadcn/ui components
  npx shadcn-ui@latest init
  npx shadcn-ui@latest add button card dialog dropdown-menu
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Component library working, consistent styling
  - **Time Estimate**: 2 hours

#### **1.4 CI/CD Pipeline Setup**
- [ ] **1.4.1 GitHub Actions Workflow**
  ```yaml
  # .github/workflows/main.yml
  name: CI/CD Pipeline
  on: [push, pull_request]
  jobs:
    test:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v3
        - uses: actions/setup-node@v3
        - run: npm ci
        - run: npm run test
        - run: npm run build
  ```
  - **Owner**: DevOps Lead
  - **Success Criteria**: CI/CD pipeline running on all commits
  - **Time Estimate**: 4 hours
  - **Documentation**: [operations/DEPLOYMENT_GUIDE.md](./docs/operations/DEPLOYMENT_GUIDE.md#cicd-pipeline)

- [ ] **1.4.2 Testing Framework Setup**
  ```bash
  # Install testing dependencies
  npm install --save-dev jest @testing-library/react @testing-library/jest-dom
  npm install --save-dev @playwright/test
  ```
  - **Owner**: QA Lead
  - **Success Criteria**: Unit and E2E test frameworks configured
  - **Time Estimate**: 3 hours
  - **Documentation**: [operations/TESTING_STRATEGY.md](./docs/operations/TESTING_STRATEGY.md)

### **Week 3: Core Infrastructure**

#### **1.5 Wallet Integration Foundation**
- [ ] **1.5.1 Wallet Context Provider**
  ```typescript
  // src/components/providers/WalletContextProvider.tsx
  import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
  import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
  import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
  import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
  
  export function WalletContextProvider({ children }: { children: React.ReactNode }) {
    const wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];
    return (
      <ConnectionProvider endpoint={process.env.NEXT_PUBLIC_RPC_URL!}>
        <WalletProvider wallets={wallets} autoConnect>
          <WalletModalProvider>{children}</WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    );
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Wallet connection working with Phantom and Solflare
  - **Time Estimate**: 4 hours
  - **Documentation**: [product/PRODUCT_REQUIREMENTS.md](./docs/product/PRODUCT_REQUIREMENTS.md#wallet-integration)

- [ ] **1.5.2 Dual-Mode Context Provider** ⚡ *Key Differentiator*
  ```typescript
  // src/components/providers/BettingModeProvider.tsx
  type BettingMode = 'degen' | 'stable';
  
  interface BettingModeContext {
    mode: BettingMode;
    toggleMode: () => void;
    acceptedMint: PublicKey;
    tokenSymbol: string;
  }
  
  export function BettingModeProvider({ children }: { children: React.ReactNode }) {
    const [mode, setMode] = useState<BettingMode>('stable');
    // Implementation per product requirements
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Mode switching works, UI updates appropriately
  - **Time Estimate**: 3 hours
  - **Priority**: HIGH - This is our core differentiator

#### **1.6 Backend API Foundation**
- [ ] **1.6.1 Next.js API Routes Structure**
  ```typescript
  // src/app/api/v1/markets/route.ts
  import { NextRequest, NextResponse } from 'next/server';
  
  export async function GET(request: NextRequest) {
    // Market listing endpoint implementation
    return NextResponse.json({ success: true, data: markets });
  }
  
  export async function POST(request: NextRequest) {
    // Market creation endpoint implementation
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: Basic API routes responding correctly
  - **Time Estimate**: 5 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#rest-api-endpoints)

- [ ] **1.6.2 Database Connection and ORM**
  ```typescript
  // src/lib/database.ts
  import { Pool } from 'pg';
  
  const pool = new Pool({
    connectionString: process.env.DATABASE_URL,
    ssl: process.env.NODE_ENV === 'production' ? { rejectUnauthorized: false } : false,
  });
  
  export async function query(text: string, params?: any[]) {
    const client = await pool.connect();
    try {
      const result = await client.query(text, params);
      return result;
    } finally {
      client.release();
    }
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: Database queries working, connection pool healthy
  - **Time Estimate**: 3 hours
  - **Blockers**: Requires 1.1.3 completion

---

## 🔥 **PHASE 2: CORE DEVELOPMENT (Weeks 4-8)**

> **Goal**: Implement all MVP core features  
> **Success Criteria**: All user flows working end-to-end, basic UI complete

### **Week 4: Market System Foundation**

#### **2.1 Market Creation Backend** ⚡ *Critical Path*
- [ ] **2.1.1 Market Creation API Endpoint**
  ```typescript
  // src/app/api/v1/markets/route.ts - POST method
  export async function POST(request: NextRequest) {
    const { title, description, category, resolutionTime, marketType } = await request.json();
    
    // Validate input
    if (!title || title.length > 64) {
      return NextResponse.json({ error: 'Invalid title' }, { status: 400 });
    }
    
    // Create market on blockchain
    const signature = await createMarketOnChain({
      title, description, category, resolutionTime, marketType
    });
    
    // Store in database
    await storeMarketInDB({ title, description, signature, /* ... */ });
    
    return NextResponse.json({ success: true, data: { signature } });
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: API creates markets on blockchain and stores in DB
  - **Time Estimate**: 6 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#post-markets)
  - **Blockers**: Requires smart contract deployed (1.2.2)

- [ ] **2.1.2 Market Listing API with Filters**
  ```typescript
  // Complex query with filters, pagination, and sorting
  export async function GET(request: NextRequest) {
    const url = new URL(request.url);
    const category = url.searchParams.get('category');
    const state = url.searchParams.get('state') || 'active';
    const page = parseInt(url.searchParams.get('page') || '1');
    const limit = Math.min(parseInt(url.searchParams.get('limit') || '20'), 100);
    
    const markets = await getMarketsFromDB({ category, state, page, limit });
    return NextResponse.json({ success: true, data: { markets, pagination } });
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: Efficient market queries with all required filters
  - **Time Estimate**: 4 hours
  - **Documentation**: [architecture/DATABASE_SCHEMA.md](./docs/architecture/DATABASE_SCHEMA.md#market-listing-query)

#### **2.2 Blockchain Integration Service**
- [ ] **2.2.1 Smart Contract Integration Layer**
  ```typescript
  // src/lib/blockchain/program.ts
  import * as anchor from '@coral-xyz/anchor';
  
  export class CryptoBetProgram {
    private program: anchor.Program;
    
    constructor(connection: Connection, wallet: Wallet) {
      const provider = new anchor.AnchorProvider(connection, wallet, {});
      this.program = new anchor.Program(IDL, PROGRAM_ID, provider);
    }
    
    async createMarket(params: CreateMarketParams): Promise<string> {
      // Implementation per API specification
    }
    
    async placeBet(params: PlaceBetParams): Promise<string> {
      // Implementation per API specification
    }
  }
  ```
  - **Owner**: Blockchain Developer
  - **Success Criteria**: All smart contract interactions working
  - **Time Estimate**: 8 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#smart-contract-instructions)

### **Week 5: Frontend Core Components**

#### **2.3 Core UI Components** ⚡ *Critical Path*
- [ ] **2.3.1 Market Card Component**
  ```typescript
  // src/components/features/markets/MarketCard.tsx
  interface MarketCardProps {
    market: Market;
    onBetClick: (marketId: string, side: 'yes' | 'no') => void;
    mode: BettingMode;
  }
  
  export function MarketCard({ market, onBetClick, mode }: MarketCardProps) {
    const { yes: yesPrice, no: noPrice } = calculatePrices(market, mode);
    
    return (
      <Card className="p-6 hover:shadow-lg transition-shadow">
        <CardHeader>
          <h3 className="text-lg font-semibold">{market.title}</h3>
          <Badge variant="secondary">{market.category}</Badge>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground mb-4">{market.description}</p>
          <div className="flex gap-2">
            <Button onClick={() => onBetClick(market.id, 'yes')} className="flex-1">
              YES {yesPrice}¢
            </Button>
            <Button onClick={() => onBetClick(market.id, 'no')} variant="outline" className="flex-1">
              NO {noPrice}¢
            </Button>
          </div>
        </CardContent>
      </Card>
    );
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Market cards display correctly, responsive design
  - **Time Estimate**: 6 hours
  - **Testing**: Visual regression tests required
  - **Documentation**: [product/PRODUCT_REQUIREMENTS.md](./docs/product/PRODUCT_REQUIREMENTS.md#market-browsing--filtering)

- [ ] **2.3.2 Market Creation Form**
  ```typescript
  // src/components/features/markets/CreateMarketForm.tsx
  export function CreateMarketForm() {
    const { mode } = useBettingMode();
    const form = useForm<CreateMarketSchema>({
      resolver: zodResolver(createMarketSchema),
    });
    
    async function onSubmit(values: CreateMarketSchema) {
      try {
        const response = await fetch('/api/v1/markets', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ ...values, marketType: mode }),
        });
        // Handle response
      } catch (error) {
        // Error handling
      }
    }
    
    return (
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          {/* Form fields per product requirements */}
        </form>
      </Form>
    );
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Form validation working, successful market creation
  - **Time Estimate**: 8 hours
  - **Blockers**: Requires 2.1.1 completion

#### **2.4 Betting Interface** ⚡ *Critical Path*
- [ ] **2.4.1 Bet Placement Modal**
  ```typescript
  // src/components/features/betting/BetModal.tsx
  interface BetModalProps {
    market: Market;
    side: 'yes' | 'no';
    isOpen: boolean;
    onClose: () => void;
  }
  
  export function BetModal({ market, side, isOpen, onClose }: BetModalProps) {
    const { mode, acceptedMint } = useBettingMode();
    const { balance } = useWallet();
    const [amount, setAmount] = useState('');
    
    const { shares, potentialReturn } = useMemo(() => 
      calculateBetOutcome(market, side, parseFloat(amount), mode), 
      [market, side, amount, mode]
    );
    
    async function handlePlaceBet() {
      try {
        const signature = await placeBet({
          marketId: market.id,
          side,
          amount: parseFloat(amount),
          mode
        });
        // Handle success
      } catch (error) {
        // Error handling
      }
    }
    
    return (
      <Dialog open={isOpen} onOpenChange={onClose}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bet {side.toUpperCase()} on {market.title}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label htmlFor="amount">Amount ({mode === 'degen' ? 'SOL' : 'USDC'})</Label>
              <Input
                id="amount"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="0.00"
              />
            </div>
            <div className="text-sm text-muted-foreground">
              <p>You'll receive: {shares} shares</p>
              <p>Potential return: {potentialReturn} {mode === 'degen' ? 'SOL' : 'USDC'}</p>
            </div>
            <Button onClick={handlePlaceBet} className="w-full">
              Place Bet
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    );
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Betting modal works, calculates shares correctly
  - **Time Estimate**: 10 hours
  - **Priority**: HIGHEST - Core user interaction
  - **Blockers**: Requires wallet integration (1.5.1)

### **Week 6: Jupiter Integration & Token Swapping**

#### **2.5 Jupiter Swap Integration** ⚡ *Key Differentiator*
- [ ] **2.5.1 Swap Service Implementation**
  ```typescript
  // src/lib/jupiter/swapService.ts
  export class JupiterSwapService {
    private readonly API_URL = 'https://quote-api.jup.ag/v6';
    
    async getSwapQuote(params: SwapQuoteParams): Promise<SwapQuoteResponse> {
      const response = await fetch(`${this.API_URL}/quote`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        // Build query params from SwapQuoteParams
      });
      
      if (!response.ok) {
        throw new Error(`Jupiter API error: ${response.statusText}`);
      }
      
      return response.json();
    }
    
    async executeSwap(quote: SwapQuoteResponse, wallet: Wallet): Promise<string> {
      const { swapTransaction } = await fetch(`${this.API_URL}/swap`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          quoteResponse: quote,
          userPublicKey: wallet.publicKey.toString(),
          wrapUnwrapSOL: true,
        }),
      }).then(res => res.json());
      
      const transaction = Transaction.from(Buffer.from(swapTransaction, 'base64'));
      const signature = await wallet.sendTransaction(transaction);
      
      return signature;
    }
  }
  ```
  - **Owner**: Blockchain Developer
  - **Success Criteria**: SOL ↔ USDC swaps working seamlessly
  - **Time Estimate**: 12 hours
  - **Priority**: HIGH - Enables dual-mode betting
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#jupiter-swap-integration)

- [ ] **2.5.2 Swap + Bet Combined Flow**
  ```typescript
  // src/hooks/useSwapAndBet.ts
  export function useSwapAndBet() {
    const { mode } = useBettingMode();
    const { balance, publicKey } = useWallet();
    const swapService = new JupiterSwapService();
    
    async function swapAndBet(params: SwapAndBetParams): Promise<string> {
      // 1. Check if user has sufficient balance in target token
      const requiredToken = mode === 'degen' ? 'SOL' : 'USDC';
      const userBalance = balance[requiredToken];
      
      if (userBalance < params.betAmount) {
        // 2. Get swap quote for the difference
        const swapAmount = params.betAmount - userBalance;
        const quote = await swapService.getSwapQuote({
          inputMint: mode === 'degen' ? 'USDC' : 'SOL',
          outputMint: requiredToken,
          amount: swapAmount,
        });
        
        // 3. Execute swap
        const swapSignature = await swapService.executeSwap(quote, wallet);
        await confirmTransaction(swapSignature);
      }
      
      // 4. Place bet
      return await placeBet(params);
    }
    
    return { swapAndBet };
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Users can bet without manually swapping tokens first
  - **Time Estimate**: 8 hours
  - **Priority**: HIGH - Major UX improvement
  - **Blockers**: Requires 2.5.1 completion

### **Week 7: Real-time Updates & WebSocket**

#### **2.6 WebSocket Integration** ⚡ *Critical Path*
- [ ] **2.6.1 WebSocket Server Setup**
  ```typescript
  // src/app/api/ws/route.ts
  import { Server } from 'socket.io';
  
  export async function GET(request: NextRequest) {
    if (!global.io) {
      const io = new Server(3001, {
        cors: { origin: process.env.NEXT_PUBLIC_APP_URL }
      });
      
      io.on('connection', (socket) => {
        socket.on('subscribe_market', (marketId) => {
          socket.join(`market:${marketId}`);
        });
        
        socket.on('unsubscribe_market', (marketId) => {
          socket.leave(`market:${marketId}`);
        });
      });
      
      global.io = io;
    }
    
    return new Response('WebSocket server initialized');
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: WebSocket connections stable, real-time updates working
  - **Time Estimate**: 6 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#websocket-api)

- [ ] **2.6.2 Real-time Market Updates**
  ```typescript
  // src/hooks/useMarketUpdates.ts
  export function useMarketUpdates(marketId: string) {
    const [market, setMarket] = useState<Market | null>(null);
    const socket = useSocket();
    
    useEffect(() => {
      if (!socket || !marketId) return;
      
      socket.emit('subscribe_market', marketId);
      
      socket.on('market_update', (update: MarketUpdate) => {
        if (update.marketId === marketId) {
          setMarket(prev => prev ? { ...prev, ...update.changes } : null);
        }
      });
      
      return () => {
        socket.emit('unsubscribe_market', marketId);
        socket.off('market_update');
      };
    }, [socket, marketId]);
    
    return market;
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Market prices update in real-time without refresh
  - **Time Estimate**: 4 hours
  - **Blockers**: Requires 2.6.1 completion

### **Week 8: Position Management & Portfolio**

#### **2.7 User Portfolio System**
- [ ] **2.7.1 Position Tracking Backend**
  ```typescript
  // src/app/api/v1/positions/route.ts
  export async function GET(request: NextRequest) {
    const userId = await getUserFromToken(request);
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }
    
    const positions = await query(`
      SELECT 
        p.*,
        m.title,
        m.category,
        m.state,
        m.resolution_result
      FROM positions_cache p
      JOIN markets_cache m ON m.market_pubkey = p.market_pubkey
      WHERE p.user_id = $1
        AND (p.yes_shares > 0 OR p.no_shares > 0 OR p.claimed = false)
      ORDER BY p.updated_at DESC
    `, [userId]);
    
    return NextResponse.json({ success: true, data: positions.rows });
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: Efficient position queries, accurate P&L calculations
  - **Time Estimate**: 5 hours
  - **Documentation**: [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md#position-endpoints)

- [ ] **2.7.2 Portfolio Dashboard Component**
  ```typescript
  // src/components/features/portfolio/PortfolioDashboard.tsx
  export function PortfolioDashboard() {
    const { data: positions, isLoading } = usePositions();
    const { totalValue, totalPnl, winRate } = usePortfolioSummary(positions);
    
    if (isLoading) return <PortfolioSkeleton />;
    
    return (
      <div className="space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <Card>
            <CardContent className="p-6">
              <h3 className="text-sm font-medium text-muted-foreground">Total Value</h3>
              <p className="text-2xl font-bold">{formatCurrency(totalValue)}</p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-6">
              <h3 className="text-sm font-medium text-muted-foreground">Total P&L</h3>
              <p className={`text-2xl font-bold ${totalPnl >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                {formatCurrency(totalPnl)}
              </p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="p-6">
              <h3 className="text-sm font-medium text-muted-foreground">Win Rate</h3>
              <p className="text-2xl font-bold">{(winRate * 100).toFixed(1)}%</p>
            </CardContent>
          </Card>
        </div>
        
        <Card>
          <CardHeader>
            <h2 className="text-xl font-semibold">Your Positions</h2>
          </CardHeader>
          <CardContent>
            {positions.length === 0 ? (
              <EmptyState />
            ) : (
              <PositionsList positions={positions} />
            )}
          </CardContent>
        </Card>
      </div>
    );
  }
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: Portfolio shows accurate position data and P&L
  - **Time Estimate**: 8 hours
  - **Blockers**: Requires 2.7.1 completion

---

## 🔗 **PHASE 3: INTEGRATION & TESTING (Weeks 9-11)**

> **Goal**: System integration, comprehensive testing, and optimization  
> **Success Criteria**: All systems working together, performance targets met, tests passing

### **Week 9: System Integration**

#### **3.1 End-to-End User Flows** ⚡ *Critical Path*
- [ ] **3.1.1 Complete User Journey Testing**
  ```typescript
  // tests/e2e/user-journey.spec.ts
  test('complete user journey: connect wallet → create market → place bet → view portfolio', async ({ page }) => {
    // 1. Connect wallet
    await page.goto('/');
    await page.click('[data-testid="connect-wallet"]');
    await mockWalletConnection(page);
    await expect(page.locator('[data-testid="wallet-connected"]')).toBeVisible();
    
    // 2. Create market
    await page.goto('/markets/create');
    await page.fill('[data-testid="market-title"]', 'Test Market');
    await page.fill('[data-testid="market-description"]', 'Test description');
    await page.selectOption('[data-testid="market-category"]', 'Test');
    await page.click('[data-testid="create-market"]');
    await expect(page.locator('[data-testid="creation-success"]')).toBeVisible();
    
    // 3. Place bet
    await page.goto('/markets');
    await page.click('[data-testid="market-card"]:first-child [data-testid="bet-yes"]');
    await page.fill('[data-testid="bet-amount"]', '0.1');
    await page.click('[data-testid="confirm-bet"]');
    await expect(page.locator('[data-testid="bet-success"]')).toBeVisible();
    
    // 4. View portfolio
    await page.goto('/portfolio');
    await expect(page.locator('[data-testid="position-item"]')).toBeVisible();
  });
  ```
  - **Owner**: QA Lead
  - **Success Criteria**: All user flows work without errors
  - **Time Estimate**: 12 hours
  - **Documentation**: [operations/TESTING_STRATEGY.md](./docs/operations/TESTING_STRATEGY.md#end-to-end-testing)

#### **3.2 Data Synchronization** ⚡ *Critical Path*
- [ ] **3.2.1 Blockchain ↔ Database Sync Service**
  ```typescript
  // src/lib/sync/blockchainSync.ts
  export class BlockchainSyncService {
    private connection: Connection;
    private program: Program;
    
    async syncMarketData(marketPubkey: PublicKey): Promise<void> {
      const marketAccount = await this.program.account.market.fetch(marketPubkey);
      
      await query(`
        UPDATE markets_cache 
        SET 
          yes_pool = $1,
          no_pool = $2,
          total_volume = $3,
          state = $4,
          resolution_result = $5,
          last_sync_at = NOW()
        WHERE market_pubkey = $6
      `, [
        marketAccount.yesPool.toString(),
        marketAccount.noPool.toString(),
        marketAccount.totalVolume.toString(),
        marketAccount.state,
        marketAccount.resolutionResult,
        marketPubkey.toString()
      ]);
    }
    
    async startRealTimeSync(): Promise<void> {
      this.connection.onProgramAccountChange(
        this.program.programId,
        (accountInfo, context) => {
          this.handleAccountChange(accountInfo, context);
        },
        'confirmed'
      );
    }
  }
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: Real-time sync working, no data inconsistencies
  - **Time Estimate**: 8 hours
  - **Priority**: HIGH - Data accuracy critical

### **Week 10: Performance Optimization**

#### **3.3 Performance Optimization** ⚡ *Critical Path*
- [ ] **3.3.1 Frontend Performance Optimization**
  ```bash
  # Run performance analysis
  npm run analyze
  npm run lighthouse
  
  # Optimize based on results:
  # - Code splitting
  # - Image optimization  
  # - Bundle size reduction
  # - Lazy loading
  ```
  - **Owner**: Frontend Developer
  - **Success Criteria**: < 2s page load times, Lighthouse score > 90
  - **Time Estimate**: 10 hours
  - **Documentation**: [performance/PERFORMANCE.md](./docs/performance/PERFORMANCE.md)

- [ ] **3.3.2 Database Query Optimization**
  ```sql
  -- Add indexes for common queries
  CREATE INDEX CONCURRENTLY idx_markets_state_category 
    ON markets_cache(state, category) 
    WHERE state = 'active';
  
  CREATE INDEX CONCURRENTLY idx_positions_user_unclaimed 
    ON positions_cache(user_id) 
    WHERE claimed = false;
  
  -- Analyze query performance
  EXPLAIN ANALYZE SELECT * FROM markets_cache 
    WHERE state = 'active' 
    ORDER BY total_volume DESC 
    LIMIT 20;
  ```
  - **Owner**: Backend Developer
  - **Success Criteria**: All queries < 100ms, database performing under load
  - **Time Estimate**: 6 hours
  - **Documentation**: [architecture/DATABASE_SCHEMA.md](./docs/architecture/DATABASE_SCHEMA.md#indexing-strategy)

### **Week 11: Security & Load Testing**

#### **3.4 Security Testing** ⚡ *Critical Path*
- [ ] **3.4.1 Smart Contract Security Review**
  ```bash
  # Run security analysis tools
  anchor test --skip-deploy
  
  # Manual security review checklist:
  # - Authority validation
  # - Overflow protection
  # - Reentrancy protection
  # - State validation
  # - PDA verification
  ```
  - **Owner**: Security Lead
  - **Success Criteria**: No critical vulnerabilities, security review passed
  - **Time Estimate**: 12 hours
  - **Priority**: CRITICAL - Security cannot be compromised
  - **Documentation**: [security/SECURITY.md](./docs/security/SECURITY.md)

#### **3.5 Load Testing**
- [ ] **3.5.1 Application Load Testing**
  ```bash
  # Install load testing tools
  npm install -g artillery
  
  # Run load tests
  artillery run load-test-config.yml
  
  # Performance targets:
  # - 100 concurrent users
  # - 95% of requests < 2s response time
  # - < 1% error rate
  ```
  - **Owner**: QA Lead
  - **Success Criteria**: System handles target load without degradation
  - **Time Estimate**: 8 hours
  - **Documentation**: [operations/TESTING_STRATEGY.md](./docs/operations/TESTING_STRATEGY.md#performance-testing)

---

## 🚀 **PHASE 4: LAUNCH PREPARATION (Week 12)**

> **Goal**: Final preparation and go-live  
> **Success Criteria**: Production deployment successful, monitoring operational

### **Week 12: Production Deployment**

#### **4.1 Production Environment Setup** ⚡ *Critical Path*
- [ ] **4.1.1 Mainnet Smart Contract Deployment**
  ```bash
  # Deploy to Solana mainnet
  solana config set --url mainnet-beta
  anchor deploy --provider.cluster mainnet-beta --program-keypair target/deploy/crypto_bet-mainnet.json
  
  # Initialize production program state
  npm run initialize:mainnet
  
  # Verify deployment
  solana program show <MAINNET_PROGRAM_ID>
  ```
  - **Owner**: DevOps Lead
  - **Success Criteria**: Smart contract deployed and initialized on mainnet
  - **Time Estimate**: 4 hours
  - **Priority**: CRITICAL - Production requirement
  - **Documentation**: [operations/DEPLOYMENT_GUIDE.md](./docs/operations/DEPLOYMENT_GUIDE.md#smart-contract-deployment)

- [ ] **4.1.2 Production Infrastructure Deployment**
  ```bash
  # Deploy to Vercel (frontend)
  vercel --prod
  
  # Deploy database and Redis (managed services)
  terraform apply -var="environment=production"
  
  # Deploy API services
  kubectl apply -f k8s/production/
  
  # Configure monitoring
  kubectl apply -f k8s/monitoring/
  ```
  - **Owner**: DevOps Lead
  - **Success Criteria**: All production services operational
  - **Time Estimate**: 6 hours
  - **Documentation**: [operations/DEPLOYMENT_GUIDE.md](./docs/operations/DEPLOYMENT_GUIDE.md)

#### **4.2 Launch Checklist** ⚡ *Critical Path*
- [ ] **4.2.1 Pre-Launch Validation**
  ```typescript
  // Production readiness checklist
  const launchChecklist = {
    // Infrastructure
    frontendDeployed: await checkUrl('https://crypto-bet.io'),
    apiHealthy: await checkUrl('https://api.crypto-bet.io/health'),
    databaseConnected: await checkDbConnection(),
    smartContractDeployed: await checkProgramExists(MAINNET_PROGRAM_ID),
    
    // Security
    sslCertificateValid: await checkSSL('crypto-bet.io'),
    securityHeaders: await checkSecurityHeaders(),
    rateLimitingActive: await checkRateLimiting(),
    
    // Performance
    pageLoadTime: await measurePageLoad(),
    apiResponseTime: await measureApiResponse(),
    
    // Business Logic
    walletConnection: await testWalletConnection(),
    marketCreation: await testMarketCreation(),
    betting: await testBetting(),
    swapIntegration: await testJupiterSwap(),
    
    // Monitoring
    errorTracking: await checkSentry(),
    analyticsActive: await checkAnalytics(),
    alertsConfigured: await checkAlerts(),
  };
  ```
  - **Owner**: QA Lead
  - **Success Criteria**: All checklist items passing
  - **Time Estimate**: 8 hours
  - **Priority**: CRITICAL - Go/no-go decision

- [ ] **4.2.2 Launch Day Execution**
  ```bash
  # Launch sequence
  echo "🚀 Starting Crypto Bet MVP Launch"
  
  # 1. Final smoke tests
  npm run test:smoke:production
  
  # 2. Enable production traffic
  kubectl scale deployment/crypto-bet-api --replicas=3
  
  # 3. Update DNS to production
  # (DNS changes as per deployment guide)
  
  # 4. Monitor for 1 hour
  kubectl get pods -w
  
  # 5. Announce launch
  echo "✅ Crypto Bet MVP is live!"
  ```
  - **Owner**: DevOps Lead
  - **Success Criteria**: Successful public launch
  - **Time Estimate**: 4 hours
  - **Priority**: CRITICAL - Public milestone

---

## 📊 **Success Metrics & KPIs**

### **MVP Launch Success Criteria**

#### **Technical Metrics** ⚡ *Critical*
- [ ] **Performance**: < 2 second average page load time
- [ ] **Reliability**: 99.5%+ uptime in first week
- [ ] **Transaction Success**: 99%+ blockchain transaction success rate
- [ ] **Security**: Zero critical security vulnerabilities
- [ ] **Mobile**: Fully responsive design working on all devices

#### **Business Metrics** 🎯 *Target*
- [ ] **User Adoption**: 100+ registered users in first week
- [ ] **Market Activity**: 10+ active markets at launch
- [ ] **Trading Volume**: $10K+ trading volume in first month
- [ ] **User Retention**: 30%+ users return within 7 days
- [ ] **Feature Usage**: 80%+ of users try both betting modes

#### **Product Quality Metrics** ✅ *Essential*
- [ ] **User Experience**: < 5% bounce rate on core pages
- [ ] **Error Rate**: < 1% application error rate
- [ ] **Support Tickets**: < 10 support requests per 100 users
- [ ] **Feature Completion**: 100% of MVP features working
- [ ] **Browser Compatibility**: Works on Chrome, Safari, Firefox, Edge

---

## 🚨 **Risk Mitigation & Contingency Plans**

### **Critical Risks & Mitigations**

#### **Technical Risks** ⚠️
1. **Smart Contract Bug**
   - **Mitigation**: Comprehensive testing, security review
   - **Contingency**: Emergency pause functionality, rollback plan

2. **Solana Network Issues**
   - **Mitigation**: Multiple RPC endpoints, graceful degradation
   - **Contingency**: Display network status, queue transactions

3. **Database Performance**
   - **Mitigation**: Query optimization, connection pooling
   - **Contingency**: Read replicas, caching layer

#### **Business Risks** ⚠️
1. **Low User Adoption**
   - **Mitigation**: Strong onboarding, clear value proposition
   - **Contingency**: Marketing campaign, user incentives

2. **Regulatory Issues**
   - **Mitigation**: Legal review, compliance framework
   - **Contingency**: Geographic restrictions, feature modifications

---

## 🏃‍♂️ **Daily Development Workflow**

### **Daily Standups** (15 minutes)
- **Format**: What did I do yesterday? What will I do today? Any blockers?
- **Focus**: Progress on current week's tasks
- **Escalation**: Blockers addressed immediately

### **Weekly Reviews** (1 hour)
- **Progress Review**: Compare actual vs. planned progress
- **Risk Assessment**: Identify upcoming risks and blockers
- **Scope Adjustment**: Adjust scope if needed to hit MVP target

### **Quality Gates**
- **No commits to main without PR review**
- **All tests must pass before merge**
- **Security review for smart contract changes**
- **Performance regression testing**

---

## 📚 **Documentation Cross-References**

This roadmap is built upon our comprehensive documentation suite:

### **Architecture Foundation**
- [architecture/SYSTEM_ARCHITECTURE.md](./docs/architecture/SYSTEM_ARCHITECTURE.md) - Technical decisions
- [architecture/DATABASE_SCHEMA.md](./docs/architecture/DATABASE_SCHEMA.md) - Data structures
- [architecture/API_SPECIFICATION.md](./docs/architecture/API_SPECIFICATION.md) - Interface contracts

### **Product Requirements**
- [product/PRODUCT_REQUIREMENTS.md](./docs/product/PRODUCT_REQUIREMENTS.md) - Feature specifications
- [business/MARKET_ANALYSIS.md](./docs/business/MARKET_ANALYSIS.md) - Market opportunity

### **Quality Assurance**
- [operations/TESTING_STRATEGY.md](./docs/operations/TESTING_STRATEGY.md) - Testing approach
- [operations/DEPLOYMENT_GUIDE.md](./docs/operations/DEPLOYMENT_GUIDE.md) - Deployment procedures
- [security/SECURITY.md](./docs/security/SECURITY.md) - Security requirements

---

## ✅ **Getting Started Checklist**

Before beginning development, ensure:

- [ ] **Documentation Review**: All team members have read core documentation
- [ ] **Development Environment**: All developers have working local environments
- [ ] **Tool Access**: All required accounts and tools provisioned
- [ ] **Team Alignment**: Everyone understands MVP scope and timeline
- [ ] **Risk Understanding**: Team aware of critical risks and mitigations

---

## 🎯 **Conclusion**

This roadmap provides a clear path from our current documentation-complete state to a production-ready MVP in 12 weeks. Every task is designed to build toward our core differentiator: the world's first dual-mode prediction markets platform.

**Key Success Factors:**
- ⚡ **Focus**: No feature creep - stick to MVP scope
- 🔄 **Iteration**: Test and iterate quickly
- 🎯 **Quality**: No compromise on security or performance
- 📊 **Data-Driven**: Make decisions based on metrics
- 👥 **Team**: Clear ownership and accountability

**Remember**: This roadmap is our north star. When in doubt, refer back to this document and our comprehensive documentation suite. Every decision should move us closer to our MVP launch goal.

---

**Document Owner**: Technical Lead  
**Review Cycle**: Weekly during development  
**Last Updated**: December 2024  
**Next Review**: Weekly sprint planning