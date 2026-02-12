"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.LettaServer = void 0;
const letta_client_1 = require("@letta-ai/letta-client");
const axios_1 = __importDefault(require("axios"));
const http_1 = __importDefault(require("http"));
const https_1 = __importDefault(require("https"));
const dotenv_1 = __importDefault(require("dotenv"));
// Load environment variables
dotenv_1.default.config();
class LettaServer {
    constructor() {
        var _a, _b;
        // Simple logger for now
        this.logger = console;
        // Validate environment variables
        this.apiBase = (_a = process.env.LETTA_BASE_URL) !== null && _a !== void 0 ? _a : '';
        this.password = (_b = process.env.LETTA_PASSWORD) !== null && _b !== void 0 ? _b : '';
        if (!this.apiBase) {
            throw new Error('Missing required environment variable: LETTA_BASE_URL');
        }
        // Initialize axios instance (keep for backward compatibility)
        if (!this.apiBase.endsWith('/v1')) {
            this.apiBase = `${this.apiBase}/v1`;
        }
        // Configure HTTP/HTTPS agents with connection pooling
        const httpAgent = new http_1.default.Agent({
            keepAlive: true,
            maxSockets: 50,
            maxFreeSockets: 10,
            timeout: 60000,
        });
        const httpsAgent = new https_1.default.Agent({
            keepAlive: true,
            maxSockets: 50,
            maxFreeSockets: 10,
            timeout: 60000,
        });
        this.api = axios_1.default.create({
            baseURL: this.apiBase,
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            httpAgent,
            httpsAgent,
            timeout: 30000,
        });
        // Initialize Letta SDK client
        try {
            this.client = new letta_client_1.LettaClient({
                token: this.password,
                baseUrl: this.apiBase.replace('/v1', ''), // SDK adds /v1 automatically
                maxRetries: 2,
                timeoutInSeconds: 30,
            });
            this.logger.info('Letta SDK client initialized successfully');
        }
        catch (error) {
            this.logger.error('Failed to initialize Letta SDK client:', error);
            throw new Error(`SDK initialization failed: ${error.message}`);
        }
    }
    /**
     * Get standard headers for API requests
     */
    getApiHeaders() {
        return {
            'Content-Type': 'application/json',
            Accept: 'application/json',
            'X-BARE-PASSWORD': `password ${this.password}`,
            Authorization: `Bearer ${this.password}`,
        };
    }
    /**
     * Map HTTP status codes to descriptive messages
     */
    mapErrorCode(statusCode) {
        switch (statusCode) {
            case 400:
                return 'Invalid parameters';
            case 401:
            case 403:
                return 'Authentication/Authorization error';
            case 404:
                return 'Resource not found';
            case 422:
                return 'Validation error';
            case 429:
                return 'Rate limit exceeded';
            case 500:
                return 'Internal server error';
            case 502:
                return 'Bad gateway';
            case 503:
                return 'Service unavailable';
            case 504:
                return 'Gateway timeout';
            default:
                return 'Unknown error';
        }
    }
    /**
     * Wrapper for SDK calls that provides better error handling
     */
    handleSdkCall(sdkFunction, context) {
        return __awaiter(this, void 0, void 0, function* () {
            try {
                return yield sdkFunction();
            }
            catch (error) {
                this.logger.error('SDK call failed:', { error, context });
                let errorMessage = '';
                // Handle Letta SDK errors
                if (error instanceof letta_client_1.LettaError || error instanceof letta_client_1.LettaTimeoutError) {
                    const statusCode = error.statusCode || 500;
                    const errorType = this.mapErrorCode(statusCode);
                    errorMessage = `${errorType}: ${error.message || 'SDK request failed'}`;
                    if (error.body) {
                        const bodyStr = typeof error.body === 'string' ? error.body : JSON.stringify(error.body);
                        errorMessage += `\nBody: ${bodyStr}`;
                    }
                }
                // Handle axios errors
                else if (axios_1.default.isAxiosError(error) && error.response) {
                    const statusCode = error.response.status || 500;
                    const errorType = this.mapErrorCode(statusCode);
                    errorMessage = `${errorType}: ${error.message || 'Request failed'}`;
                    if (error.response.data) {
                        const dataStr = typeof error.response.data === 'string'
                            ? error.response.data
                            : JSON.stringify(error.response.data);
                        errorMessage += ` - ${dataStr}`;
                    }
                }
                // Handle generic errors
                else if (error instanceof Error) {
                    errorMessage = error.message || 'Unknown error occurred';
                }
                else {
                    errorMessage = 'Unknown SDK error occurred';
                }
                // Add context if provided
                if (context) {
                    errorMessage = `${context}: ${errorMessage}`;
                }
                throw new Error(errorMessage);
            }
        });
    }
}
exports.LettaServer = LettaServer;
