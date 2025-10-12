#!/usr/bin/env node
/**
 * Quick test script to verify Letta SDK installation
 */
import { LettaClient } from '@letta-ai/letta-client';
import * as dotenv from 'dotenv';

dotenv.config();

async function testSdk() {
    try {
        console.log('Testing Letta SDK initialization...');

        const client = new LettaClient({
            token: process.env.LETTA_PASSWORD,
            baseUrl: process.env.LETTA_BASE_URL,
            maxRetries: 2,
            timeoutInSeconds: 30
        });

        console.log('✅ SDK client initialized successfully');
        console.log('SDK Configuration:');
        console.log('  - Base URL:', process.env.LETTA_BASE_URL);
        console.log('  - Max Retries: 2');
        console.log('  - Timeout: 30s');

        // Try a simple API call to verify connectivity
        console.log('\nTesting API connectivity...');
        const agents = await client.agents.list({ limit: 1 });
        console.log('✅ API call successful');
        console.log(`  - Found ${agents.length} agent(s)`);

        process.exit(0);
    } catch (error) {
        console.error('❌ SDK test failed:', error.message);
        if (error.statusCode) {
            console.error('  - Status Code:', error.statusCode);
        }
        if (error.body) {
            console.error('  - Response:', JSON.stringify(error.body, null, 2));
        }
        process.exit(1);
    }
}

testSdk();
