# Security Policy

## Reporting a Vulnerability

The SkillCert team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings and will make every effort to acknowledge your contributions.

### Reporting Process

1. **DO NOT** file a public issue to report a security vulnerability.
2. Email your findings to `security@skillcert.com`. Encrypt your email using our PGP key (see below).
3. Include detailed information about the vulnerability:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggested fixes (if available)

### What to Expect

- A response acknowledging receipt of your report within 24 hours
- An assessment and initial response within 72 hours
- Regular updates about our progress if the issue is accepted
- Credit for responsible disclosure, if desired

## Scope

### In Scope
- Smart contracts in the `/contracts` directory
- Core protocol implementations
- Authentication mechanisms
- Access control systems
- Token handling mechanisms

### Out of Scope
- Issues already reported
- Issues in dependencies (report to respective projects)
- Theoretical vulnerabilities without proof of exploitability
- Issues requiring physical access to a system

## Security Best Practices

### For Contributors
- All code must pass our automated security checks
- Follow our [secure development guidelines](./docs/CONTRIBUTING.md)
- Use safe Rust practices and avoid unsafe code blocks
- Thoroughly test all changes, especially those affecting state management
- Review the [Soroban security guidelines](https://soroban.stellar.org/docs/security-guide)

### For Users
- Always verify contract addresses
- Keep private keys secure and never share them
- Use official deployment channels
- Report any suspicious activity immediately

## Bug Bounty Program

We currently handle bug reports on a case-by-case basis. Rewards are determined based on:
- Severity of the vulnerability
- Quality of the report
- Potential impact on users
- Novelty of the finding

## Security Measures

Our contracts undergo:
- Regular security audits
- Automated testing
- Static analysis
- Formal verification where applicable

## Contact Information

### Primary Channels
- **Security Team Email:** security@skillcert.com
- **Telegram Community:** [https://t.me/skillcert_community](https://t.me/skillcert_community)
  - For urgent security matters, please message an admin directly on Telegram
  - Identify your message clearly as a security report

### PGP Key
```
-----BEGIN PGP PUBLIC KEY BLOCK-----
[Your PGP key here]
-----END PGP PUBLIC KEY BLOCK-----
```

## Recent Security Updates

| Date | Description | Status |
|------|-------------|--------|
| 2025-09-18 | Initial Security Policy Creation | Active |

## Acknowledgments

We'd like to thank all security researchers who have contributed to the security of this project. A list of acknowledged researchers can be found here.

## Updates

This security policy will be updated as our security practices evolve. Check the git history of this file for changes.