# Development Guidelines

## Validation Requirements
1. Run validation script before any commit:
   ```bash
   ./scripts/validate.sh
   ```
2. All changes must pass:
   - Frontend:
     - Type checking
     - Linting
     - Tests (with coverage thresholds)
     - Build
   - Contract:
     - Cargo check
     - Clippy (no warnings)
     - Tests
     - Build

## Code Standards
1. Frontend:
   - Use latest stable versions:
     - Next.js 14.0.3 (15.0.4 planned)
     - React 18.3.1 (19.0.0 planned)
     - @cosmjs 0.31.3
   - TypeScript strict mode
   - Test coverage requirements:
     - Components: 85%+
     - Hooks: 90%+
     - Utils: 95%+

2. Contract:
   - Use latest stable CosmWasm (2.2.0-rc.3)
   - Use `to_json_binary` instead of `to_binary`
   - All functions must have tests
   - Coverage requirement: 90%+

## Development Process
1. Before starting work:
   - Pull latest changes
   - Run validation script
   - Check TODO.md for priorities

2. During development:
   - Write tests first (TDD)
   - Run validation frequently
   - Keep changes atomic
   - Update documentation

3. Before committing:
   - Run full validation
   - Update TODO.md if needed
   - Update README.md if needed
   - Follow commit message format:
     ```
     <type>[optional scope]: <description>
     
     [optional body]
     [optional footer(s)]
     ```

4. After committing:
   - Verify CI/CD pipeline passes
   - Review test coverage
   - Check documentation is up to date

## Documentation
1. Code Documentation:
   - All public functions must have docs
   - Complex logic needs inline comments
   - Update CHANGELOG.md for changes

2. Project Documentation:
   - README.md must be current
   - TODO.md must reflect priorities
   - API documentation must be complete

## Testing
1. Unit Tests:
   - Required for all new code
   - Must cover edge cases
   - Must be deterministic

2. Integration Tests:
   - Required for critical paths
   - Must cover main workflows
   - Must use mocks appropriately

3. E2E Tests:
   - Required for main user flows
   - Must run in CI/CD
   - Must be reliable

## Commit Messages
1. Types:
   - feat: new feature
   - fix: bug fix
   - docs: documentation only
   - style: formatting, missing semi colons, etc
   - refactor: code change that neither fixes a bug nor adds a feature
   - perf: code change that improves performance
   - test: adding missing tests
   - chore: maintain

2. Format:
   ```
   <type>[optional scope]: <description>
   
   [optional body]
   [optional footer(s)]
   ```

3. Examples:
   ```
   feat(contract): add pixel rental functionality
   fix(frontend): handle wallet connection errors
   test(hooks): improve useContract coverage
   ```