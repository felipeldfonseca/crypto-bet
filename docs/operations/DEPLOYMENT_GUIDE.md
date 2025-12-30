# 🚀 Deployment Guide

## 📋 **Document Overview**

**Purpose**: Comprehensive deployment guide for Crypto Bet prediction markets platform  
**Scope**: Infrastructure setup, environment configuration, deployment procedures, and monitoring  
**Audience**: DevOps engineers, system administrators, deployment team  
**Last Updated**: December 2024  
**Version**: 1.0  

---

## 🎯 **Executive Summary**

This deployment guide provides step-by-step instructions for deploying the Crypto Bet prediction markets platform across all environments. The platform utilizes a modern, cloud-native architecture with Solana blockchain integration, emphasizing security, scalability, and reliability.

**Deployment Architecture:**
- **Frontend**: Next.js application on Vercel with CDN
- **Backend**: Node.js APIs on containerized infrastructure
- **Database**: PostgreSQL with Redis caching
- **Blockchain**: Solana programs on mainnet/devnet
- **Monitoring**: Comprehensive observability stack

**Infrastructure Principles:**
- **Infrastructure as Code**: All resources defined declaratively
- **Zero-Downtime Deployments**: Blue-green deployment strategy
- **Automated Testing**: CI/CD with comprehensive test gates
- **Security-First**: End-to-end encryption and access controls

---

## 🏗️ **Infrastructure Architecture**

### **High-Level Deployment Diagram**

```mermaid
graph TB
    subgraph "CDN Layer"
        CDN[CloudFlare CDN]
    end
    
    subgraph "Frontend Layer"
        VERCEL[Vercel Edge Functions]
        NEXT[Next.js Application]
    end
    
    subgraph "API Layer"
        LB[Load Balancer]
        API1[API Server 1]
        API2[API Server 2]
        API3[API Server 3]
    end
    
    subgraph "Data Layer"
        PG_PRIMARY[PostgreSQL Primary]
        PG_REPLICA[PostgreSQL Replica]
        REDIS[Redis Cache]
    end
    
    subgraph "Blockchain Layer"
        SOLANA_RPC[Solana RPC Nodes]
        PROGRAM[Smart Contract Program]
    end
    
    subgraph "Monitoring"
        DATADOG[DataDog]
        SENTRY[Sentry]
        LOGS[Log Aggregation]
    end
    
    CDN --> VERCEL
    VERCEL --> NEXT
    NEXT --> LB
    LB --> API1
    LB --> API2
    LB --> API3
    
    API1 --> PG_PRIMARY
    API2 --> PG_PRIMARY
    API3 --> PG_PRIMARY
    
    PG_PRIMARY --> PG_REPLICA
    API1 --> REDIS
    API2 --> REDIS
    API3 --> REDIS
    
    API1 --> SOLANA_RPC
    SOLANA_RPC --> PROGRAM
    
    API1 --> DATADOG
    API1 --> SENTRY
    API1 --> LOGS
```

### **Environment Strategy**

| Environment | Purpose | Infrastructure | Data | Access |
|-------------|---------|----------------|------|--------|
| **Local** | Development | Docker Compose | Synthetic | Developers |
| **Development** | Integration Testing | Kubernetes (staging cluster) | Synthetic | Dev Team |
| **Staging** | Pre-production Testing | Kubernetes (production cluster) | Production-like | QA/DevOps |
| **Production** | Live System | Kubernetes (production cluster) | Real User Data | Operations |

---

## 🔧 **Prerequisites & Dependencies**

### **Required Tools & Accounts**

#### **Development Tools**
```bash
# Required CLI tools
node >= 18.0.0
npm >= 9.0.0
docker >= 24.0.0
docker-compose >= 2.0.0
kubectl >= 1.28.0
helm >= 3.12.0

# Solana tools
solana-cli >= 1.17.0
anchor-cli >= 0.29.0

# Cloud tools
aws-cli >= 2.13.0  # or gcloud, depending on cloud provider
```

#### **Service Accounts & API Keys**
```yaml
required_accounts:
  # Cloud Infrastructure
  - name: AWS/GCP Account
    purpose: Kubernetes cluster and managed services
    
  # Frontend Deployment
  - name: Vercel Account
    purpose: Frontend hosting and edge functions
    
  # CDN & Security
  - name: CloudFlare Account
    purpose: CDN, DDoS protection, WAF
    
  # Blockchain
  - name: QuickNode/Alchemy
    purpose: Solana RPC endpoints
    
  # Monitoring
  - name: DataDog Account
    purpose: Application monitoring and alerting
  - name: Sentry Account
    purpose: Error tracking and performance monitoring
    
  # External Services
  - name: Jupiter API
    purpose: Token swap integration
```

### **Environment Variables**
```bash
# Create environment configuration files

# .env.local (Development)
NEXT_PUBLIC_CLUSTER=devnet
NEXT_PUBLIC_PROGRAM_ID=CryptoBetDev...
NEXT_PUBLIC_RPC_URL=https://api.devnet.solana.com
DATABASE_URL=postgresql://localhost:5432/cryptobet_dev
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-jwt-secret-dev

# .env.staging (Staging)
NEXT_PUBLIC_CLUSTER=mainnet-beta
NEXT_PUBLIC_PROGRAM_ID=CryptoBetStaging...
NEXT_PUBLIC_RPC_URL=https://rpc.ankr.com/solana
DATABASE_URL=postgresql://staging-db:5432/cryptobet
REDIS_URL=redis://staging-redis:6379
JWT_SECRET=your-jwt-secret-staging

# .env.production (Production)
NEXT_PUBLIC_CLUSTER=mainnet-beta
NEXT_PUBLIC_PROGRAM_ID=CryptoBet...
NEXT_PUBLIC_RPC_URL=https://solana-mainnet.rpc.extrnode.com
DATABASE_URL=postgresql://prod-db:5432/cryptobet
REDIS_URL=redis://prod-redis:6379
JWT_SECRET=your-jwt-secret-prod
```

---

## ⛓️ **Smart Contract Deployment**

### **Solana Program Deployment**

#### **1. Build and Test Smart Contracts**
```bash
# Clone repository
git clone https://github.com/your-org/crypto-bet.git
cd crypto-bet

# Install dependencies
npm install

# Build Anchor program
cd programs/crypto-bet
anchor build

# Run tests
anchor test

# Generate program keypair (production only)
solana-keygen new -o target/deploy/crypto_bet-keypair.json
```

#### **2. Deploy to Devnet**
```bash
# Configure Solana CLI for devnet
solana config set --url devnet
solana config set --keypair ~/.config/solana/id.json

# Airdrop SOL for deployment (devnet only)
solana airdrop 2

# Deploy program
anchor deploy --provider.cluster devnet

# Verify deployment
solana program show <PROGRAM_ID>
```

#### **3. Initialize Program State**
```bash
# Run initialization script
cd scripts
npm run initialize:devnet

# Verify initialization
anchor run test-program-state
```

#### **4. Deploy to Mainnet**
```bash
# CRITICAL: Backup all keys before mainnet deployment
cp target/deploy/crypto_bet-keypair.json backup/

# Configure for mainnet
solana config set --url mainnet-beta

# Fund deployment wallet (requires real SOL)
# Transfer ~2 SOL to deployment wallet

# Deploy with production program ID
anchor deploy --provider.cluster mainnet-beta --program-id target/deploy/crypto_bet-keypair.json

# Initialize production state
npm run initialize:mainnet

# Verify deployment
solana program show <MAINNET_PROGRAM_ID>
```

### **Smart Contract Configuration**
```typescript
// scripts/initialize-program.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoBet } from "../target/types/crypto_bet";

async function initializeProgram() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.CryptoBet as Program<CryptoBet>;
  
  // Production configuration
  const config = {
    platformFeeBps: 250,        // 2.5% platform fee
    minMarketDuration: 3600,    // 1 hour minimum
    maxMarketDuration: 31536000, // 1 year maximum
    minBetAmount: 1000000,      // 0.001 SOL minimum
    maxBetAmount: 1000000000000, // 1000 SOL maximum
    platformTreasury: new anchor.web3.PublicKey("TREASURY_WALLET_ADDRESS"),
  };
  
  const [globalConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global_config")],
    program.programId
  );
  
  try {
    await program.methods
      .initializeProgram(config)
      .accounts({
        authority: provider.wallet.publicKey,
        globalConfig: globalConfigPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
    console.log("Program initialized successfully");
    console.log("Global Config PDA:", globalConfigPda.toString());
  } catch (error) {
    console.error("Program initialization failed:", error);
    process.exit(1);
  }
}

// Run initialization
initializeProgram().catch(console.error);
```

---

## 🗄️ **Database Setup**

### **PostgreSQL Deployment**

#### **1. Production Database Setup (AWS RDS)**
```yaml
# terraform/rds.tf
resource "aws_db_instance" "crypto_bet_prod" {
  identifier = "crypto-bet-prod"
  
  engine         = "postgres"
  engine_version = "15.4"
  instance_class = "db.t3.medium"
  
  allocated_storage     = 100
  max_allocated_storage = 1000
  storage_type         = "gp3"
  storage_encrypted    = true
  
  db_name  = "cryptobet"
  username = "cryptobet_admin"
  password = var.db_password
  
  # Networking
  vpc_security_group_ids = [aws_security_group.rds.id]
  db_subnet_group_name   = aws_db_subnet_group.main.name
  
  # Backup & Maintenance
  backup_retention_period = 7
  backup_window          = "03:00-04:00"
  maintenance_window     = "Sun:04:00-Sun:05:00"
  
  # Monitoring
  monitoring_interval = 60
  monitoring_role_arn = aws_iam_role.rds_monitoring.arn
  
  # Security
  deletion_protection = true
  skip_final_snapshot = false
  final_snapshot_identifier = "crypto-bet-final-snapshot"
  
  tags = {
    Environment = "production"
    Project     = "crypto-bet"
  }
}

# Read replica for scaling
resource "aws_db_instance" "crypto_bet_replica" {
  identifier = "crypto-bet-replica"
  
  replicate_source_db = aws_db_instance.crypto_bet_prod.identifier
  instance_class      = "db.t3.medium"
  
  auto_minor_version_upgrade = false
  
  tags = {
    Environment = "production"
    Project     = "crypto-bet"
    Role        = "read-replica"
  }
}
```

#### **2. Database Schema Migration**
```bash
# Install migration tool
npm install -g db-migrate
npm install -g db-migrate-pg

# Create migration files
db-migrate create initial-schema --sql-file

# Run migrations
npm run db:migrate:production

# Verify schema
npm run db:verify
```

#### **3. Database Connection Configuration**
```typescript
// src/lib/database.ts
import { Pool } from 'pg';

const isProduction = process.env.NODE_ENV === 'production';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  ssl: isProduction ? { rejectUnauthorized: false } : false,
  
  // Connection pool settings
  min: 2,
  max: 20,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
  
  // Production settings
  ...(isProduction && {
    statement_timeout: 30000,
    query_timeout: 30000,
    application_name: 'crypto-bet-api',
  }),
});

// Health check
export async function checkDatabaseHealth(): Promise<boolean> {
  try {
    const client = await pool.connect();
    await client.query('SELECT 1');
    client.release();
    return true;
  } catch (error) {
    console.error('Database health check failed:', error);
    return false;
  }
}

export default pool;
```

### **Redis Cache Setup**

#### **1. Redis Deployment (AWS ElastiCache)**
```yaml
# terraform/redis.tf
resource "aws_elasticache_subnet_group" "main" {
  name       = "crypto-bet-cache-subnet"
  subnet_ids = var.private_subnet_ids
}

resource "aws_elasticache_replication_group" "crypto_bet" {
  replication_group_id       = "crypto-bet-cache"
  description                = "Redis cache for Crypto Bet"
  
  node_type                  = "cache.t3.micro"
  port                       = 6379
  parameter_group_name       = "default.redis7"
  
  num_cache_clusters         = 2
  automatic_failover_enabled = true
  multi_az_enabled          = true
  
  subnet_group_name = aws_elasticache_subnet_group.main.name
  security_group_ids = [aws_security_group.redis.id]
  
  at_rest_encryption_enabled = true
  transit_encryption_enabled = true
  auth_token                 = var.redis_auth_token
  
  # Backup
  snapshot_retention_limit = 3
  snapshot_window         = "03:00-05:00"
  
  tags = {
    Environment = "production"
    Project     = "crypto-bet"
  }
}
```

#### **2. Redis Configuration**
```typescript
// src/lib/redis.ts
import Redis from 'ioredis';

const redis = new Redis({
  host: process.env.REDIS_HOST,
  port: parseInt(process.env.REDIS_PORT || '6379'),
  password: process.env.REDIS_PASSWORD,
  
  // Production settings
  retryDelayOnFailover: 100,
  maxRetriesPerRequest: 3,
  lazyConnect: true,
  
  // TLS for production
  ...(process.env.NODE_ENV === 'production' && {
    tls: {
      rejectUnauthorized: false,
    },
  }),
});

// Cache utility functions
export class CacheManager {
  static async get<T>(key: string): Promise<T | null> {
    try {
      const value = await redis.get(key);
      return value ? JSON.parse(value) : null;
    } catch (error) {
      console.error('Cache get error:', error);
      return null;
    }
  }
  
  static async set(key: string, value: any, ttlSeconds = 300): Promise<void> {
    try {
      await redis.setex(key, ttlSeconds, JSON.stringify(value));
    } catch (error) {
      console.error('Cache set error:', error);
    }
  }
  
  static async del(key: string): Promise<void> {
    try {
      await redis.del(key);
    } catch (error) {
      console.error('Cache delete error:', error);
    }
  }
}

export default redis;
```

---

## 🌐 **Frontend Deployment**

### **Vercel Deployment Configuration**

#### **1. Project Setup**
```json
// vercel.json
{
  "framework": "nextjs",
  "buildCommand": "npm run build",
  "outputDirectory": ".next",
  "installCommand": "npm install",
  
  "env": {
    "NEXT_PUBLIC_CLUSTER": "mainnet-beta",
    "NEXT_PUBLIC_PROGRAM_ID": "CryptoBetMainnet...",
    "NEXT_PUBLIC_RPC_URL": "https://solana-mainnet.rpc.extrnode.com"
  },
  
  "build": {
    "env": {
      "NODE_ENV": "production"
    }
  },
  
  "functions": {
    "pages/api/**/*.ts": {
      "runtime": "nodejs18.x",
      "maxDuration": 30
    }
  },
  
  "headers": [
    {
      "source": "/api/(.*)",
      "headers": [
        {
          "key": "Access-Control-Allow-Origin",
          "value": "https://crypto-bet.io"
        },
        {
          "key": "Access-Control-Allow-Methods",
          "value": "GET, POST, PUT, DELETE, OPTIONS"
        },
        {
          "key": "Access-Control-Allow-Headers",
          "value": "Content-Type, Authorization"
        }
      ]
    }
  ],
  
  "rewrites": [
    {
      "source": "/api/v1/(.*)",
      "destination": "https://api.crypto-bet.io/v1/$1"
    }
  ]
}
```

#### **2. Next.js Production Configuration**
```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  
  // Performance optimizations
  compiler: {
    removeConsole: process.env.NODE_ENV === 'production',
  },
  
  // Image optimization
  images: {
    domains: ['crypto-bet.io', 'ipfs.io'],
    formats: ['image/webp', 'image/avif'],
  },
  
  // Security headers
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff',
          },
          {
            key: 'X-Frame-Options',
            value: 'DENY',
          },
          {
            key: 'X-XSS-Protection',
            value: '1; mode=block',
          },
          {
            key: 'Strict-Transport-Security',
            value: 'max-age=63072000; includeSubDomains; preload',
          },
          {
            key: 'Content-Security-Policy',
            value: "default-src 'self'; script-src 'self' 'unsafe-eval' 'unsafe-inline' *.vercel-analytics.com; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https: wss:;",
          },
        ],
      },
    ];
  },
  
  // Webpack optimization
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.resolve.fallback = {
        fs: false,
        net: false,
        tls: false,
        crypto: false,
      };
    }
    
    // Bundle analyzer in development
    if (process.env.ANALYZE) {
      const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
      config.plugins.push(
        new BundleAnalyzerPlugin({
          analyzerMode: 'static',
          openAnalyzer: false,
        })
      );
    }
    
    return config;
  },
};

module.exports = nextConfig;
```

#### **3. Deployment Script**
```bash
#!/bin/bash
# scripts/deploy-frontend.sh

set -e

echo "🚀 Starting frontend deployment..."

# Environment validation
if [[ -z "$VERCEL_TOKEN" ]]; then
  echo "❌ VERCEL_TOKEN environment variable required"
  exit 1
fi

# Build optimizations
echo "📦 Building application..."
npm run build

# Bundle analysis
echo "📊 Analyzing bundle size..."
npm run analyze

# Type checking
echo "🔍 Running type checks..."
npm run type-check

# Linting
echo "🧹 Running linter..."
npm run lint

# Deploy to Vercel
echo "🌐 Deploying to Vercel..."
if [[ "$ENVIRONMENT" == "production" ]]; then
  npx vercel --prod --token $VERCEL_TOKEN
else
  npx vercel --token $VERCEL_TOKEN
fi

echo "✅ Frontend deployment completed!"
```

---

## 🔧 **Backend API Deployment**

### **Kubernetes Deployment**

#### **1. Kubernetes Manifests**
```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: crypto-bet
  labels:
    environment: production
---
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: crypto-bet-api
  namespace: crypto-bet
  labels:
    app: crypto-bet-api
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: crypto-bet-api
  template:
    metadata:
      labels:
        app: crypto-bet-api
    spec:
      containers:
      - name: api
        image: crypto-bet/api:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_ENV
          value: "production"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: crypto-bet-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: crypto-bet-secrets
              key: redis-url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: crypto-bet-secrets
              key: jwt-secret
        
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: crypto-bet-api-service
  namespace: crypto-bet
spec:
  selector:
    app: crypto-bet-api
  ports:
  - port: 80
    targetPort: 3000
    protocol: TCP
  type: ClusterIP
---
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: crypto-bet-api-ingress
  namespace: crypto-bet
  annotations:
    kubernetes.io/ingress.class: "nginx"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  tls:
  - hosts:
    - api.crypto-bet.io
    secretName: crypto-bet-api-tls
  rules:
  - host: api.crypto-bet.io
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: crypto-bet-api-service
            port:
              number: 80
```

#### **2. Docker Configuration**
```dockerfile
# Dockerfile
FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY tsconfig.json ./

# Install dependencies
RUN npm ci --only=production && npm cache clean --force

# Copy source code
COPY src/ ./src/

# Build application
RUN npm run build

# Production image
FROM node:18-alpine AS production

# Security: Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nextjs -u 1001

# Set working directory
WORKDIR /app

# Copy built application
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package*.json ./

# Change ownership to non-root user
USER nextjs

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
  CMD node dist/health-check.js

# Start application
CMD ["node", "dist/index.js"]
```

#### **3. Deployment Pipeline**
```yaml
# .github/workflows/deploy-backend.yml
name: Deploy Backend API

on:
  push:
    branches: [main]
    paths: ['src/**', 'package.json']

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Run tests
      run: npm run test:ci
    
    - name: Build Docker image
      run: |
        docker build -t crypto-bet/api:${{ github.sha }} .
        docker tag crypto-bet/api:${{ github.sha }} crypto-bet/api:latest
    
    - name: Configure AWS credentials
      uses: aws-actions/configure-aws-credentials@v2
      with:
        aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
        aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
        aws-region: us-east-1
    
    - name: Login to Amazon ECR
      id: login-ecr
      uses: aws-actions/amazon-ecr-login@v1
    
    - name: Push image to ECR
      env:
        ECR_REGISTRY: ${{ steps.login-ecr.outputs.registry }}
        ECR_REPOSITORY: crypto-bet-api
      run: |
        docker tag crypto-bet/api:latest $ECR_REGISTRY/$ECR_REPOSITORY:latest
        docker tag crypto-bet/api:latest $ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }}
        docker push $ECR_REGISTRY/$ECR_REPOSITORY:latest
        docker push $ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }}
    
    - name: Deploy to Kubernetes
      run: |
        aws eks get-token --cluster-name crypto-bet-cluster | kubectl apply -f k8s/
        kubectl set image deployment/crypto-bet-api crypto-bet-api=$ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }} -n crypto-bet
        kubectl rollout status deployment/crypto-bet-api -n crypto-bet
```

---

## 📊 **Monitoring & Observability**

### **DataDog Integration**

#### **1. Application Monitoring Setup**
```typescript
// src/lib/monitoring.ts
import { StatsD } from 'hot-shots';
import tracer from 'dd-trace';

// Initialize DataDog tracer
tracer.init({
  service: 'crypto-bet-api',
  env: process.env.NODE_ENV || 'development',
  version: process.env.APP_VERSION || '1.0.0',
  
  // Performance settings
  profiling: process.env.NODE_ENV === 'production',
  runtimeMetrics: true,
  
  // Sampling
  sampleRate: process.env.NODE_ENV === 'production' ? 0.1 : 1.0,
});

// StatsD client for custom metrics
const statsD = new StatsD({
  host: process.env.DATADOG_AGENT_HOST || 'localhost',
  port: 8125,
  prefix: 'cryptobet.',
  
  // Error handling
  errorHandler: (error) => {
    console.warn('StatsD error:', error);
  },
});

export class Metrics {
  // Business metrics
  static recordBetPlaced(amount: number, market: string) {
    statsD.increment('bets.placed', 1, [`market:${market}`]);
    statsD.histogram('bets.amount', amount, [`market:${market}`]);
  }
  
  static recordMarketCreated(category: string) {
    statsD.increment('markets.created', 1, [`category:${category}`]);
  }
  
  static recordUserRegistration() {
    statsD.increment('users.registered');
  }
  
  // Performance metrics
  static recordDatabaseQuery(table: string, operation: string, duration: number) {
    statsD.histogram('database.query.duration', duration, [
      `table:${table}`,
      `operation:${operation}`,
    ]);
  }
  
  static recordAPIResponse(endpoint: string, statusCode: number, duration: number) {
    statsD.histogram('api.response.duration', duration, [
      `endpoint:${endpoint}`,
      `status_code:${statusCode}`,
    ]);
    statsD.increment('api.requests', 1, [
      `endpoint:${endpoint}`,
      `status_code:${statusCode}`,
    ]);
  }
  
  // Blockchain metrics
  static recordBlockchainTransaction(type: string, success: boolean, duration: number) {
    statsD.histogram('blockchain.transaction.duration', duration, [
      `type:${type}`,
      `success:${success}`,
    ]);
    statsD.increment('blockchain.transactions', 1, [
      `type:${type}`,
      `success:${success}`,
    ]);
  }
}

export default tracer;
```

#### **2. DataDog Configuration**
```yaml
# k8s/datadog-agent.yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: datadog-agent
  namespace: crypto-bet
spec:
  selector:
    matchLabels:
      app: datadog-agent
  template:
    metadata:
      labels:
        app: datadog-agent
    spec:
      containers:
      - image: gcr.io/datadoghq/agent:latest
        name: datadog-agent
        env:
        - name: DD_API_KEY
          valueFrom:
            secretKeyRef:
              name: datadog-secret
              key: api-key
        - name: DD_SITE
          value: "datadoghq.com"
        - name: DD_COLLECT_KUBERNETES_EVENTS
          value: "true"
        - name: DD_APM_ENABLED
          value: "true"
        - name: DD_PROCESS_AGENT_ENABLED
          value: "true"
        - name: DD_LOGS_ENABLED
          value: "true"
        - name: DD_LOGS_CONFIG_CONTAINER_COLLECT_ALL
          value: "true"
        - name: DD_AC_EXCLUDE
          value: "name:datadog-agent"
        
        resources:
          requests:
            memory: "256Mi"
            cpu: "200m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        
        volumeMounts:
        - name: dockersocket
          mountPath: /var/run/docker.sock
        - name: procdir
          mountPath: /host/proc
          readOnly: true
        - name: cgroups
          mountPath: /host/sys/fs/cgroup
          readOnly: true
        
      volumes:
      - name: dockersocket
        hostPath:
          path: /var/run/docker.sock
      - name: procdir
        hostPath:
          path: /proc
      - name: cgroups
        hostPath:
          path: /sys/fs/cgroup
```

### **Sentry Error Tracking**

#### **1. Sentry Configuration**
```typescript
// src/lib/sentry.ts
import * as Sentry from '@sentry/node';
import { ProfilingIntegration } from '@sentry/profiling-node';

Sentry.init({
  dsn: process.env.SENTRY_DSN,
  environment: process.env.NODE_ENV || 'development',
  
  // Performance monitoring
  tracesSampleRate: process.env.NODE_ENV === 'production' ? 0.1 : 1.0,
  profilesSampleRate: process.env.NODE_ENV === 'production' ? 0.1 : 1.0,
  
  integrations: [
    new Sentry.Integrations.Http({ tracing: true }),
    new Sentry.Integrations.Express({ app: require('./app') }),
    new ProfilingIntegration(),
  ],
  
  // Error filtering
  beforeSend(event, hint) {
    // Filter out known non-critical errors
    if (event.exception) {
      const error = hint.originalException;
      if (error instanceof Error) {
        // Skip validation errors
        if (error.name === 'ValidationError') return null;
        // Skip rate limiting errors
        if (error.message.includes('rate limit')) return null;
      }
    }
    
    return event;
  },
  
  // Additional context
  initialScope: {
    tags: {
      component: 'api-server',
    },
    user: {
      id: process.env.INSTANCE_ID || 'unknown',
    },
  },
});

// Custom error handling
export function captureError(error: Error, context?: any) {
  Sentry.withScope((scope) => {
    if (context) {
      scope.setContext('additional', context);
    }
    Sentry.captureException(error);
  });
}

export function captureMessage(message: string, level: Sentry.SeverityLevel = 'info') {
  Sentry.captureMessage(message, level);
}

export default Sentry;
```

---

## 🔐 **Security & Secrets Management**

### **AWS Secrets Manager Integration**

#### **1. Secrets Configuration**
```typescript
// src/lib/secrets.ts
import { SecretsManagerClient, GetSecretValueCommand } from '@aws-sdk/client-secrets-manager';

const client = new SecretsManagerClient({
  region: process.env.AWS_REGION || 'us-east-1',
});

export class SecretsManager {
  private static cache = new Map<string, any>();
  
  static async getSecret(secretName: string): Promise<any> {
    // Check cache first
    if (this.cache.has(secretName)) {
      return this.cache.get(secretName);
    }
    
    try {
      const command = new GetSecretValueCommand({
        SecretId: secretName,
      });
      
      const response = await client.send(command);
      const secret = JSON.parse(response.SecretString || '{}');
      
      // Cache for 5 minutes
      this.cache.set(secretName, secret);
      setTimeout(() => this.cache.delete(secretName), 5 * 60 * 1000);
      
      return secret;
    } catch (error) {
      console.error(`Failed to get secret ${secretName}:`, error);
      throw error;
    }
  }
  
  // Specific secret getters
  static async getDatabaseCredentials() {
    return this.getSecret('crypto-bet/database');
  }
  
  static async getJWTSecret() {
    const secrets = await this.getSecret('crypto-bet/jwt');
    return secrets.JWT_SECRET;
  }
  
  static async getExternalAPIKeys() {
    return this.getSecret('crypto-bet/external-apis');
  }
}
```

#### **2. Kubernetes Secrets**
```yaml
# k8s/secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: crypto-bet-secrets
  namespace: crypto-bet
type: Opaque
data:
  # Base64 encoded values
  database-url: <base64-encoded-db-url>
  redis-url: <base64-encoded-redis-url>
  jwt-secret: <base64-encoded-jwt-secret>
---
# External secrets operator configuration
apiVersion: external-secrets.io/v1beta1
kind: SecretStore
metadata:
  name: aws-secrets-manager
  namespace: crypto-bet
spec:
  provider:
    aws:
      service: SecretsManager
      region: us-east-1
      auth:
        secretRef:
          accessKeyID:
            name: aws-credentials
            key: access-key-id
          secretAccessKey:
            name: aws-credentials
            key: secret-access-key
---
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: crypto-bet-external-secret
  namespace: crypto-bet
spec:
  refreshInterval: 1h
  secretStoreRef:
    name: aws-secrets-manager
    kind: SecretStore
  target:
    name: crypto-bet-secrets
    creationPolicy: Owner
  data:
  - secretKey: database-url
    remoteRef:
      key: crypto-bet/database
      property: DATABASE_URL
  - secretKey: redis-url
    remoteRef:
      key: crypto-bet/redis
      property: REDIS_URL
```

---

## 🔄 **CI/CD Pipeline**

### **Complete GitHub Actions Workflow**

```yaml
# .github/workflows/deploy.yml
name: Complete Deployment Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  NODE_VERSION: '18'
  SOLANA_VERSION: '1.17.0'

jobs:
  # Test jobs
  test-smart-contracts:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Solana
        uses: ./.github/actions/setup-solana
        with:
          solana-version: ${{ env.SOLANA_VERSION }}
      
      - name: Install Anchor
        run: |
          npm install -g @coral-xyz/anchor-cli
          anchor --version
      
      - name: Start local validator
        run: solana-test-validator --reset &
      
      - name: Build programs
        working-directory: programs
        run: anchor build
      
      - name: Run smart contract tests
        working-directory: programs
        run: anchor test --skip-local-validator

  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run type checking
        run: npm run type-check
      
      - name: Run linting
        run: npm run lint
      
      - name: Run unit tests
        run: npm run test:unit
      
      - name: Run E2E tests
        run: npm run test:e2e

  test-api:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: ${{ env.NODE_VERSION }}
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Setup test database
        run: |
          npm run db:setup:test
          npm run db:migrate:test
      
      - name: Run API tests
        run: npm run test:api
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test
          REDIS_URL: redis://localhost:6379

  # Security scanning
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run Semgrep
        uses: returntocorp/semgrep-action@v1
        with:
          config: >-
            p/security-audit
            p/secrets
            p/typescript
      
      - name: Run npm audit
        run: npm audit --audit-level=high
      
      - name: Run Snyk
        uses: snyk/actions/node@master
        env:
          SNYK_TOKEN: ${{ secrets.SNYK_TOKEN }}

  # Build and deploy jobs
  build-and-push:
    needs: [test-smart-contracts, test-frontend, test-api, security-scan]
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    outputs:
      image-digest: ${{ steps.build.outputs.digest }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v2
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: us-east-1
      
      - name: Login to Amazon ECR
        id: login-ecr
        uses: aws-actions/amazon-ecr-login@v1
      
      - name: Build and push Docker image
        id: build
        env:
          ECR_REGISTRY: ${{ steps.login-ecr.outputs.registry }}
          ECR_REPOSITORY: crypto-bet-api
        run: |
          docker build -t $ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }} .
          docker push $ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }}
          echo "digest=$(docker inspect --format='{{index .RepoDigests 0}}' $ECR_REGISTRY/$ECR_REPOSITORY:${{ github.sha }})" >> $GITHUB_OUTPUT

  deploy-staging:
    needs: build-and-push
    if: github.ref == 'refs/heads/develop'
    runs-on: ubuntu-latest
    environment: staging
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Deploy to staging
        run: |
          # Update Kubernetes deployment
          kubectl set image deployment/crypto-bet-api \
            crypto-bet-api=${{ needs.build-and-push.outputs.image-digest }} \
            -n crypto-bet-staging
          
          # Wait for rollout
          kubectl rollout status deployment/crypto-bet-api -n crypto-bet-staging

  deploy-production:
    needs: build-and-push
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    environment: production
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Deploy smart contracts
        if: github.event_name == 'push'
        run: |
          # Deploy to mainnet
          cd programs
          anchor deploy --provider.cluster mainnet-beta
      
      - name: Deploy to production
        run: |
          # Blue-green deployment
          kubectl patch deployment crypto-bet-api \
            -p '{"spec":{"template":{"spec":{"containers":[{"name":"crypto-bet-api","image":"${{ needs.build-and-push.outputs.image-digest }}"}]}}}}' \
            -n crypto-bet
          
          # Wait for rollout
          kubectl rollout status deployment/crypto-bet-api -n crypto-bet --timeout=600s
      
      - name: Run smoke tests
        run: |
          # Basic health checks
          curl -f https://api.crypto-bet.io/health
          curl -f https://crypto-bet.io
      
      - name: Notify deployment
        uses: 8398a7/action-slack@v3
        with:
          status: ${{ job.status }}
          channel: '#deployments'
          webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

---

## 🚦 **Health Checks & Monitoring**

### **Application Health Endpoints**

```typescript
// src/routes/health.ts
import express from 'express';
import { checkDatabaseHealth } from '../lib/database';
import { checkRedisHealth } from '../lib/redis';
import { checkBlockchainHealth } from '../lib/blockchain';

const router = express.Router();

interface HealthCheck {
  service: string;
  status: 'healthy' | 'unhealthy' | 'degraded';
  responseTime: number;
  details?: any;
}

// Basic liveness probe
router.get('/health', (req, res) => {
  res.status(200).json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    version: process.env.APP_VERSION || '1.0.0',
  });
});

// Comprehensive readiness probe
router.get('/ready', async (req, res) => {
  const checks: HealthCheck[] = [];
  let overallStatus: 'healthy' | 'unhealthy' | 'degraded' = 'healthy';
  
  // Database check
  const dbStart = Date.now();
  try {
    const dbHealthy = await checkDatabaseHealth();
    checks.push({
      service: 'database',
      status: dbHealthy ? 'healthy' : 'unhealthy',
      responseTime: Date.now() - dbStart,
    });
    
    if (!dbHealthy) overallStatus = 'unhealthy';
  } catch (error) {
    checks.push({
      service: 'database',
      status: 'unhealthy',
      responseTime: Date.now() - dbStart,
      details: error.message,
    });
    overallStatus = 'unhealthy';
  }
  
  // Redis check
  const redisStart = Date.now();
  try {
    const redisHealthy = await checkRedisHealth();
    checks.push({
      service: 'redis',
      status: redisHealthy ? 'healthy' : 'degraded',
      responseTime: Date.now() - redisStart,
    });
    
    if (!redisHealthy && overallStatus === 'healthy') {
      overallStatus = 'degraded'; // Redis is not critical
    }
  } catch (error) {
    checks.push({
      service: 'redis',
      status: 'degraded',
      responseTime: Date.now() - redisStart,
      details: error.message,
    });
  }
  
  // Blockchain connectivity check
  const blockchainStart = Date.now();
  try {
    const blockchainHealthy = await checkBlockchainHealth();
    checks.push({
      service: 'blockchain',
      status: blockchainHealthy ? 'healthy' : 'degraded',
      responseTime: Date.now() - blockchainStart,
    });
    
    if (!blockchainHealthy && overallStatus === 'healthy') {
      overallStatus = 'degraded';
    }
  } catch (error) {
    checks.push({
      service: 'blockchain',
      status: 'degraded',
      responseTime: Date.now() - blockchainStart,
      details: error.message,
    });
  }
  
  const statusCode = overallStatus === 'healthy' ? 200 : 
                    overallStatus === 'degraded' ? 200 : 503;
  
  res.status(statusCode).json({
    status: overallStatus,
    timestamp: new Date().toISOString(),
    checks,
  });
});

export default router;
```

### **Alerting Configuration**

```yaml
# monitoring/alerts.yaml
groups:
  - name: crypto-bet-alerts
    rules:
    # Application alerts
    - alert: HighErrorRate
      expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
      for: 5m
      labels:
        severity: critical
      annotations:
        summary: "High error rate detected"
        description: "Error rate is {{ $value }} for the last 5 minutes"
    
    - alert: HighResponseTime
      expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 2
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "High response time detected"
        description: "95th percentile response time is {{ $value }}s"
    
    # Database alerts
    - alert: DatabaseConnectionsHigh
      expr: pg_stat_database_numbackends / pg_settings_max_connections > 0.8
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "Database connection pool usage high"
    
    - alert: DatabaseDown
      expr: pg_up == 0
      for: 1m
      labels:
        severity: critical
      annotations:
        summary: "Database is down"
    
    # Blockchain alerts
    - alert: BlockchainRPCDown
      expr: solana_rpc_up == 0
      for: 2m
      labels:
        severity: critical
      annotations:
        summary: "Solana RPC endpoint is down"
    
    - alert: TransactionFailureRate
      expr: rate(blockchain_transactions_total{success="false"}[5m]) > 0.05
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "High blockchain transaction failure rate"
```

---

## 📋 **Deployment Checklist**

### **Pre-Deployment Checklist**

#### **Smart Contracts**
- [ ] Smart contract code reviewed and audited
- [ ] Program deployed to devnet and thoroughly tested
- [ ] Program authority keys secured (hardware wallet/multisig)
- [ ] Program upgrade authority configured correctly
- [ ] Initial program state configured and verified
- [ ] Emergency pause mechanism tested

#### **Infrastructure**
- [ ] All infrastructure provisioned via Terraform
- [ ] Database migrations tested on staging environment
- [ ] SSL certificates configured and valid
- [ ] DNS records properly configured
- [ ] Load balancer health checks working
- [ ] Auto-scaling policies configured

#### **Security**
- [ ] All secrets stored securely (AWS Secrets Manager)
- [ ] API keys rotated and permissions minimized
- [ ] Network security groups configured
- [ ] WAF rules implemented and tested
- [ ] Rate limiting configured
- [ ] DDoS protection enabled

#### **Monitoring**
- [ ] DataDog dashboards configured
- [ ] Alerting rules tested and validated
- [ ] Log aggregation working
- [ ] Error tracking (Sentry) configured
- [ ] Uptime monitoring enabled
- [ ] Performance baselines established

#### **Testing**
- [ ] All test suites passing (unit, integration, E2E)
- [ ] Load testing completed successfully
- [ ] Security scanning passed
- [ ] Dependency vulnerabilities resolved
- [ ] Performance benchmarks met
- [ ] Disaster recovery procedures tested

### **Post-Deployment Checklist**

#### **Immediate (0-30 minutes)**
- [ ] All services are healthy and responding
- [ ] Database connections working
- [ ] Smart contract interactions functional
- [ ] Real-time updates working (WebSocket)
- [ ] Authentication flow working
- [ ] Basic user journeys tested

#### **Short-term (30 minutes - 4 hours)**
- [ ] Monitor error rates and response times
- [ ] Verify all background jobs running
- [ ] Check data synchronization between blockchain and database
- [ ] Validate all external integrations (Jupiter, price feeds)
- [ ] Monitor resource usage and auto-scaling
- [ ] Verify backup processes

#### **Medium-term (4-24 hours)**
- [ ] Monitor user registration and activity
- [ ] Verify market creation and betting flows
- [ ] Check financial calculations and balances
- [ ] Monitor blockchain transaction success rates
- [ ] Review security logs for anomalies
- [ ] Validate performance under normal load

#### **Long-term (24+ hours)**
- [ ] Analyze user behavior and adoption metrics
- [ ] Review cost optimization opportunities
- [ ] Plan capacity scaling based on growth
- [ ] Schedule regular security audits
- [ ] Update documentation based on deployment learnings
- [ ] Conduct post-deployment retrospective

---

## 🔄 **Rollback Procedures**

### **Emergency Rollback Plan**

```bash
#!/bin/bash
# scripts/emergency-rollback.sh

set -e

echo "🚨 EMERGENCY ROLLBACK INITIATED"

# Get previous working version
PREVIOUS_VERSION=$(kubectl rollout history deployment/crypto-bet-api -n crypto-bet | tail -2 | head -1 | awk '{print $1}')

echo "Rolling back to revision: $PREVIOUS_VERSION"

# Frontend rollback (Vercel)
echo "Rolling back frontend..."
npx vercel rollback --token $VERCEL_TOKEN

# API rollback (Kubernetes)
echo "Rolling back API..."
kubectl rollout undo deployment/crypto-bet-api -n crypto-bet --to-revision=$PREVIOUS_VERSION

# Wait for rollback completion
kubectl rollout status deployment/crypto-bet-api -n crypto-bet

# Verify health
echo "Verifying system health..."
curl -f https://api.crypto-bet.io/health
curl -f https://crypto-bet.io

# Database rollback (if needed)
read -p "Run database rollback? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  npm run db:rollback:production
fi

echo "✅ Emergency rollback completed"
echo "🔍 Review logs and investigate root cause"
```

---

## 📚 **Operational Runbooks**

### **Common Operational Tasks**

#### **Scaling the Application**
```bash
# Scale API horizontally
kubectl scale deployment crypto-bet-api --replicas=5 -n crypto-bet

# Scale database vertically (AWS RDS)
aws rds modify-db-instance \
  --db-instance-identifier crypto-bet-prod \
  --db-instance-class db.t3.large \
  --apply-immediately

# Scale Redis cache
aws elasticache modify-replication-group \
  --replication-group-id crypto-bet-cache \
  --cache-node-type cache.t3.small
```

#### **Database Maintenance**
```bash
# Create database backup
aws rds create-db-snapshot \
  --db-instance-identifier crypto-bet-prod \
  --db-snapshot-identifier crypto-bet-maintenance-$(date +%Y%m%d)

# Run maintenance scripts
npm run db:maintenance:production

# Update database statistics
psql $DATABASE_URL -c "ANALYZE;"
```

#### **Certificate Renewal**
```bash
# Check certificate expiration
openssl x509 -in /path/to/cert.pem -text -noout | grep "Not After"

# Renew Let's Encrypt certificates
certbot renew --nginx

# Update Kubernetes TLS secrets
kubectl create secret tls crypto-bet-api-tls \
  --cert=path/to/cert.pem \
  --key=path/to/private.key \
  -n crypto-bet
```

---

*This comprehensive deployment guide provides all necessary information and procedures for successfully deploying and operating the Crypto Bet prediction markets platform in production.*

**Next Review**: Monthly deployment process review  
**Document Owner**: DevOps Team  
**Approval**: Infrastructure Committee