# 🤝 Contributing to Crypto Bet

Thank you for your interest in contributing to Crypto Bet! We welcome contributions from developers of all skill levels.

## 🚀 Quick Start

### Prerequisites
- Node.js >= 18.0.0
- npm >= 8.0.0
- Rust (latest stable)
- Solana CLI >= 1.16.0
- Anchor CLI >= 0.29.0

### Development Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/crypto-bet.git
cd crypto-bet

# Install frontend dependencies
cd app && npm install

# Install program dependencies
cd ../program && anchor build

# Start local development
solana-test-validator
anchor deploy
cd ../app && npm run dev
```

## 📋 Development Guidelines

### Code Standards

**Rust/Anchor (Smart Contract)**
- Follow Anchor best practices
- Use `checked_*` arithmetic operations
- Comprehensive error handling
- Clear documentation comments
- Security-first mindset

**TypeScript/React (Frontend)**
- Strict TypeScript mode
- ESLint + Prettier formatting
- Component-based architecture
- Responsive design principles
- Accessibility compliance

### Git Workflow

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** your changes: `git commit -m 'Add amazing feature'`
4. **Push** to the branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Commit Messages
Use conventional commits:
```
feat: add Jupiter swap integration
fix: resolve double-claiming vulnerability
docs: update README with dual-mode instructions
test: add comprehensive market resolution tests
```

## 🧪 Testing

### Smart Contract Tests
```bash
cd program
anchor test
```

### Frontend Tests
```bash
cd app
npm test
npm run test:e2e
```

### Security Testing
- Run `anchor test` for comprehensive smart contract tests
- Use `solana-security-txt` for security documentation
- Follow security best practices checklist

## 🐛 Bug Reports

When filing a bug report, please include:

1. **Environment**: OS, Node.js version, browser
2. **Steps to reproduce**: Clear, numbered steps
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Screenshots**: If applicable
6. **Console logs**: Any error messages

Use this template:
```markdown
## Bug Description
Brief description of the issue

## Environment
- OS: macOS 14.0
- Node.js: 18.17.0
- Browser: Chrome 119.0

## Steps to Reproduce
1. Go to markets page
2. Connect wallet
3. Click on market
4. See error

## Expected Behavior
Should display market details

## Actual Behavior
Shows error message "Failed to load"

## Additional Context
Console shows: "TypeError: Cannot read property..."
```

## 💡 Feature Requests

For feature requests, please:

1. **Check existing issues** to avoid duplicates
2. **Describe the problem** you're trying to solve
3. **Propose a solution** with implementation details
4. **Consider alternatives** and their trade-offs
5. **Assess impact** on existing functionality

## 🔒 Security

### Reporting Security Issues
**DO NOT** open public issues for security vulnerabilities.

Instead, email: security@cryptobet.so

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Security Guidelines
- Never commit private keys or secrets
- Use environment variables for sensitive data
- Follow smart contract security best practices
- Validate all user inputs
- Use safe math operations

## 📚 Documentation

### Code Documentation
- **Rust**: Use `///` for public functions
- **TypeScript**: Use JSDoc comments
- **README**: Keep up-to-date with changes
- **API docs**: Document all public interfaces

### Writing Style
- Clear and concise
- Include code examples
- Explain the "why" not just the "what"
- Use proper grammar and spelling

## 🎯 Areas for Contribution

### High Priority
- [ ] Jupiter swap integration
- [ ] Mobile responsiveness improvements
- [ ] Performance optimizations
- [ ] Additional test coverage

### Medium Priority
- [ ] Advanced market analytics
- [ ] Social features
- [ ] Multi-language support
- [ ] Accessibility improvements

### Low Priority
- [ ] Theme customization
- [ ] Advanced charting
- [ ] API documentation
- [ ] Developer tools

## 🏆 Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- Social media shoutouts
- Potential token rewards (future)

## 📞 Getting Help

- **Discord**: [Join our community](https://discord.gg/cryptobet)
- **GitHub Discussions**: For general questions
- **GitHub Issues**: For bugs and feature requests
- **Email**: dev@cryptobet.so

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Thank You

Every contribution, no matter how small, helps make Crypto Bet better for everyone. Thank you for being part of our community!

---

*Happy coding! 🚀* 