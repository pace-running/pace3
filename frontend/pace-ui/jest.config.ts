import type { Config } from 'jest';

const config: Config = {
  collectCoverage: true,
  coverageReporters: ['json', 'html', 'text-summary'],
  collectCoverageFrom: ['**/*.tsx', '!**/node_modules/**'],
  verbose: true,
  setupFilesAfterEnv: ['./jest-setup.ts'],
  testEnvironment: 'jest-environment-jsdom',
  transform: {
    '\\.[jt]sx?$': 'babel-jest',
    '\\.svg$': './testing/fileTransformer.js'
  }
  // moduleDirectories: ['node_modules', '.'],
  // roots: ['.'],
  // modulePaths: ['.'],
};

export default config;
