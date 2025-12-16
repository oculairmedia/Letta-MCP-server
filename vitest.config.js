import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        // Test environment
        environment: 'node',

        // Global test APIs
        globals: true,

        // Coverage configuration
        coverage: {
            provider: 'v8',
            reporter: ['text', 'json', 'html', 'lcov'],
            exclude: [
                'node_modules/**',
                'dist/**',
                '**/dist/**',
                'src/test/**',
                '**/*.test.js',
                '**/*.spec.js',
                '**/index.js',
                'vitest.config.js',
                '.eslintrc.json',
                'src/tools/lettaopenapi.json',
                'src/examples/**',
            ],
            thresholds: {
                lines: 10, // Will increase as we add more tests
                functions: 10,
                branches: 10,
                statements: 10,
            },
        },

        // Test file patterns
        include: ['src/**/*.{test,spec}.js'],

        // Exclude patterns
        exclude: ['node_modules', 'dist', '.git'],

        // Test timeout
        testTimeout: 10000,

        // Hook timeout
        hookTimeout: 10000,

        // Specific timeout for integration tests
        pool: 'forks',
        poolOptions: {
            forks: {
                singleFork: true,
            },
        },

        // Reporters
        reporters: ['verbose'],

        // Watch mode exclusions
        watchExclude: ['node_modules/**', 'dist/**'],

        // Setup files
        setupFiles: ['./src/test/setup.js'],

        // Mock configuration
        mockReset: true,
        clearMocks: true,
        restoreMocks: true,
    },
});
