# Security Policy

## Known Vulnerabilities

### 1. curve25519-dalek v3.2.0 (Critical)
- **Issue**: Timing variability in `Scalar29::sub`/`Scalar52::sub` operations
- **Severity**: High
- **Status**: Awaiting upstream fix
- **Details**: This is a transitive dependency through:
  ```
  curve25519-dalek 3.2.0
  └── ed25519-zebra 3.1.0
      └── cosmwasm-crypto 1.5.9
          └── cosmwasm-std 1.5.9
  ```
- **Impact**: Potential timing side-channel vulnerability in cryptographic operations
- **Mitigation**: 
  - Awaiting update to curve25519-dalek >=4.1.3
  - Monitor upstream dependencies for updates
  - Consider pinning to a newer version if available

### 2. derivative v2.2.0 (Warning)
- **Status**: Unmaintained package warning
- **Severity**: Low
- **Details**: Transitive dependency through:
  ```
  derivative 2.2.0
  ├── cw-multi-test 0.18.1
  ├── cw-multi-test 0.16.5
  └── cosmwasm-std 1.5.9
  ```
- **Impact**: No direct security impact, but may not receive future updates
- **Mitigation**: 
  - Monitor for alternative solutions from upstream dependencies
  - Consider alternative derive macro implementations if needed
  - No immediate action required as this is a development dependency

## Dependency Management

### Version Policy
- All production dependencies are locked to specific versions
- Development dependencies are allowed to float within minor version ranges
- Security patches are prioritized for immediate updates

### Update Process
1. Regular security audits using `cargo audit`
2. Automated dependency updates through dependabot
3. Manual review of all dependency changes
4. Testing of all updates in staging environment

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please:

1. **DO NOT** create a public GitHub issue
2. Send details to [security contact email]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fixes (if any)

We will:
- Acknowledge receipt within 48 hours
- Provide a detailed response within 72 hours
- Issue security advisories for confirmed vulnerabilities
- Credit reporters in security advisories (unless anonymity is requested)

## Security Best Practices

### For Contributors
1. Always use the latest stable Rust toolchain
2. Run `cargo audit` before submitting PRs
3. Follow secure coding guidelines in CONTRIBUTING.md
4. Use strong cryptographic primitives
5. Implement proper error handling

### For Users
1. Always verify contract addresses
2. Use latest stable releases
3. Monitor security advisories
4. Follow deployment security guidelines 