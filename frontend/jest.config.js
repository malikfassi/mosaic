const nextJest = require('next/jest')

const createJestConfig = nextJest({
  dir: './',
})

const customJestConfig = {
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  testEnvironment: 'jest-environment-jsdom',
  coverageThreshold: {
    global: {
      statements: 70,
      branches: 40,
      functions: 50,
      lines: 70,
    },
  },
}

module.exports = createJestConfig(customJestConfig) 