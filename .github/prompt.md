# Assistant Behavior Guide

## Core Principles
1. Keep changes atomic and focused
2. Test every change (build, lint, unit tests)
3. Follow Single Responsibility Principle
4. Maintain clear documentation
5. Ensure security best practices

## Change Validation Steps
1. **Before ANY Change**
   - Run current tests: `npm test`
   - Check build: `npm run build`
   - Verify linting: `npm run lint`
   - Run type check: `npm run type-check`

2. **During Implementation**
   - Make ONE change at a time
   - Test EACH change immediately
   - Keep functions under 20 lines
   - Add tests for new code
   - Run validation steps after each change

3. **After Changes**
   - Run ALL validation steps again
   - Review changes for simplicity
   - Update documentation if needed
   - Create focused commit
   - Verify CI pipeline success

## Atomic Development Process
1. **Single Unit of Work**
   - One feature/fix at a time
   - One file change when possible
   - One function modification
   - One test addition

2. **Validation Sequence**
   ```bash
   # Run for EVERY change
   npm run type-check
   npm run lint
   npm run test
   npm run build
   ```

3. **Error Handling**
   - Fix errors before proceeding
   - Document any workarounds
   - Update tests for edge cases

## TODO Management
1. Never delete tasks unless explicitly requested
2. Group tasks by priority:
   - Immediate (current sprint)
   - Short-term (next sprint)
   - Long-term (backlog)
3. Add new tasks with clear acceptance criteria
4. Mark completed tasks with date and test status

## Code Review Guidelines
1. Check for:
   - Function size and complexity
   - Test coverage
   - Build success
   - Lint compliance
   - Type safety
2. Suggest improvements
3. Look for code duplication

## Commit Standards
1. Use conventional commits:
   - feat: new feature
   - fix: bug fix
   - test: testing
   - docs: documentation
   - refactor: code improvement
   - style: formatting
   - chore: maintenance
2. One logical change per commit
3. Include test status in commit message

## Response Format
1. Start with validation status
2. Show single change
3. Show test results
4. List next atomic step
5. Ask for confirmation

## Security Practices
1. Never expose sensitive data
2. Use environment variables
3. Validate inputs
4. Follow least privilege principle
5. Keep dependencies updated

## Testing Requirements
1. Test EVERY change
2. Unit tests for ALL functions
3. Integration tests for features
4. E2E tests for critical paths
5. Maintain test coverage > 70%

## Documentation Updates
1. Keep README current
2. Document test procedures
3. Maintain clear comments
4. Document breaking changes
5. Keep TODO.md updated

## Prompt Updates
1. Only modify when requested
2. Keep changes focused
3. Document modifications
4. Maintain core principles