# Assistant Behavior Guide

## Core Principles
1. Keep files and functions small and focused
2. Follow Single Responsibility Principle
3. Maintain clear documentation
4. Prioritize testability
5. Ensure security best practices

## Workflow Steps
1. **Before Making Changes**
   - Review existing code
   - Plan changes in small, manageable chunks
   - Consider impact on existing functionality

2. **During Implementation**
   - Create/modify one file at a time
   - Keep functions under 20 lines
   - Add comments for complex logic
   - Include error handling
   - Add tests for new code

3. **After Changes**
   - Review changes for simplicity
   - Ensure test coverage
   - Update documentation
   - Create focused commits

## TODO Management
1. Never delete tasks unless explicitly requested
2. Group tasks by priority:
   - Immediate (current sprint)
   - Short-term (next sprint)
   - Long-term (backlog)
3. Add new tasks with clear acceptance criteria
4. Mark completed tasks with date

## Code Review Guidelines
1. Check for:
   - Function size and complexity
   - Error handling
   - Test coverage
   - Security considerations
   - Documentation
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
2. Keep commits focused and atomic
3. Write clear descriptions

## Response Format
1. Start with action summary
2. Show relevant code changes
3. Explain modifications
4. List next steps
5. Ask for confirmation when needed

## Security Practices
1. Never expose sensitive data
2. Use environment variables
3. Validate inputs
4. Follow least privilege principle
5. Keep dependencies updated

## Testing Requirements
1. Unit tests for functions
2. Integration tests for features
3. E2E tests for critical paths
4. Mock external dependencies
5. Maintain test coverage > 70%

## Documentation Updates
1. Keep README current
2. Update API documentation
3. Maintain clear comments
4. Document breaking changes
5. Keep TODO.md updated

## Prompt Updates
1. Only modify when requested
2. Keep changes focused
3. Document modifications
4. Maintain core principles 