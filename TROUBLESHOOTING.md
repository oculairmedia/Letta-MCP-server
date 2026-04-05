# Troubleshooting Guide

Common issues and solutions for the Letta MCP Server.

## Table of Contents

- [Connection Issues](#connection-issues)
- [Authentication Errors](#authentication-errors)
- [Timeout Problems](#timeout-problems)
- [Memory Issues](#memory-issues)
- [Build Failures](#build-failures)
- [Runtime Errors](#runtime-errors)
- [Diagnostic Commands](#diagnostic-commands)

## Connection Issues

### Cannot connect to Letta server

**Symptoms:**
```
Error: Failed to connect to Letta server at http://localhost:8283
```

**Solutions:**

1. **Check Letta server is running:**
   ```bash
   curl http://localhost:8283/v1/health
   ```

2. **Verify LETTA_BASE_URL in .env:**
   ```bash
   cat .env | grep LETTA_BASE_URL
   ```

3. **Check network connectivity:**
   ```bash
   ping localhost
   telnet localhost 8283
   ```

4. **Check Docker container (if using Docker):**
   ```bash
   docker ps | grep letta
   docker logs letta-mcp-rust
   ```

### MCP client cannot connect to server

**Symptoms:**
- Claude Desktop shows "Server not responding"
- Connection timeout errors

**Solutions:**

1. **Check transport mode matches client expectations:**
   - Claude Desktop requires `TRANSPORT=stdio`
   - HTTP clients require `TRANSPORT=http`

2. **Verify server is running:**
   ```bash
   ps aux | grep letta-server
   ```

3. **Check logs:**
   ```bash
   # If running via systemd
   journalctl -u letta-mcp-server -n 50

   # If running in Docker
   docker logs letta-mcp-rust --tail 50
   ```

## Authentication Errors

### 401 Unauthorized from Letta API

**Symptoms:**
```
Error: Letta API returned 401 Unauthorized
```

**Solutions:**

1. **Check authentication credentials:**
   ```bash
   # Verify token is set
   echo $LETTA_API_TOKEN

   # Test token manually
   curl -H "Authorization: Bearer $LETTA_API_TOKEN" \
        http://localhost:8283/v1/agents
   ```

2. **Verify token hasn't expired:**
   - Check token expiration in Letta dashboard
   - Generate new token if needed

3. **Check authentication method:**
   - Ensure using correct auth method (Bearer vs Basic)
   - Verify credentials format in .env

## Timeout Problems

### Request timeouts

**Symptoms:**
```
Error: Request timed out after 30s
```

**Solutions:**

1. **Increase timeout:**
   ```bash
   # In .env
   REQUEST_TIMEOUT=60
   ```

2. **Check Letta server performance:**
   ```bash
   # Monitor response times
   time curl http://localhost:8283/v1/agents
   ```

3. **Reduce request complexity:**
   - Use pagination for large result sets
   - Filter results to reduce payload size

### Connection pool exhaustion

**Symptoms:**
```
Error: Connection pool timeout
```

**Solutions:**

1. **Increase pool size:**
   ```bash
   # In .env
   CONNECTION_POOL_SIZE=20
   ```

2. **Check for connection leaks:**
   ```bash
   # Monitor open connections
   lsof -i :8283
   ```

## Memory Issues

### Out of memory errors

**Symptoms:**
```
Error: Cannot allocate memory
```

**Solutions:**

1. **Check memory usage:**
   ```bash
   # Container memory
   docker stats letta-mcp-rust

   # System memory
   free -h
   ```

2. **Increase container memory limit:**
   ```yaml
   # In docker-compose.yml
   services:
     letta-mcp-rust:
       mem_limit: 512m
   ```

3. **Optimize response sizes:**
   - Use pagination
   - Request only needed fields
   - Enable response truncation

### Memory leaks

**Symptoms:**
- Memory usage grows over time
- Performance degrades

**Solutions:**

1. **Restart service:**
   ```bash
   docker restart letta-mcp-rust
   ```

2. **Monitor for leaks:**
   ```bash
   # Watch memory over time
   watch -n 5 'docker stats letta-mcp-rust --no-stream'
   ```

3. **Report issue with:**
   - Memory growth pattern
   - Operations being performed
   - Rust backtrace if available

## Build Failures

### Compilation errors

**Symptoms:**
```
error: could not compile `letta-server`
```

**Solutions:**

1. **Update Rust toolchain:**
   ```bash
   rustup update stable
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   cargo build
   ```

3. **Check Rust version:**
   ```bash
   rustc --version  # Should be 1.82+
   ```

### Dependency resolution failures

**Symptoms:**
```
error: failed to select a version for `tokio`
```

**Solutions:**

1. **Update Cargo.lock:**
   ```bash
   cargo update
   ```

2. **Clear cargo cache:**
   ```bash
   rm -rf ~/.cargo/registry
   cargo build
   ```

## Runtime Errors

### Null response errors

**Symptoms:**
```
Error: Unexpected null response from Letta API
```

**Context:**
- Known issue with attach/detach operations
- Fixed in v2.1.2+

**Solutions:**

1. **Update to latest version:**
   ```bash
   npm install -g letta-mcp-server@latest
   ```

2. **Verify version:**
   ```bash
   letta-server --version
   ```

### JSON parsing errors

**Symptoms:**
```
Error: Failed to deserialize JSON response
```

**Solutions:**

1. **Check Letta API version compatibility:**
   ```bash
   curl http://localhost:8283/v1/health
   ```

2. **Enable debug logging:**
   ```bash
   RUST_LOG=debug letta-server
   ```

3. **Report issue with:**
   - Full error message
   - Letta server version
   - MCP server version

## Diagnostic Commands

### Health check

```bash
# Basic health
curl http://localhost:6507/health

# Detailed status (if available)
curl http://localhost:6507/health/detailed
```

### Check configuration

```bash
# List environment variables
env | grep LETTA

# Verify .env loaded
cat .env
```

### Test Letta API connectivity

```bash
# Health check
curl http://localhost:8283/v1/health

# List agents
curl http://localhost:8283/v1/agents

# With authentication
curl -H "Authorization: Bearer $LETTA_API_TOKEN" \
     http://localhost:8283/v1/agents
```

### View logs

```bash
# Docker logs
docker logs letta-mcp-rust --tail 100 -f

# Systemd logs
journalctl -u letta-mcp-server -f

# File logs (if configured)
tail -f /var/log/letta-mcp-server.log
```

### Performance profiling

```bash
# CPU usage
top -p $(pgrep letta-server)

# Memory usage
ps aux | grep letta-server

# Network connections
netstat -an | grep 6507
```

## Still Having Issues?

1. **Check existing issues:**
   - https://github.com/oculairmedia/Letta-MCP-server/issues

2. **Create a new issue with:**
   - Letta MCP Server version
   - Letta API version
   - Operating system
   - Full error message
   - Steps to reproduce
   - Relevant logs

3. **Enable debug logging:**
   ```bash
   RUST_LOG=debug RUST_BACKTRACE=1 letta-server
   ```
