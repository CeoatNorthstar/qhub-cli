# QHub Configuration Guide

QHub uses a flexible configuration system that supports both configuration files and environment variables.

## Configuration File Location

- **macOS/Linux**: `~/.qhub/config.toml`
- **Windows**: `%USERPROFILE%\.qhub\config.toml`

The configuration file is automatically created on first run with sensible defaults.

## Configuration Structure

```toml
version = 1  # Config format version

# AI Provider Configuration
[ai]
provider = "deepseek"                    # AI provider: deepseek, openai, anthropic
model = "deepseek/deepseek-chat"        # Model to use
max_tokens = 4096                        # Maximum response tokens
api_key = "your_key_here"               # Optional: API key (prefer env var)

# Quantum Provider Configuration
[quantum]
provider = "ibm"                         # Quantum provider: ibm, simulator
api_key = "your_ibm_token"              # Optional: IBM Quantum token
default_backend = "ibmq_qasm_simulator" # Optional: Default backend

# UI Configuration
[ui]
scroll_speed = 3                         # Lines to scroll per keypress
show_timestamps = true                   # Show message timestamps
syntax_highlighting = true               # Enable code highlighting

# User Configuration (managed by login/register)
[user]
email = "user@example.com"
tier = "free"                            # Subscription tier: free, pro, enterprise
```

## Environment Variables

Environment variables take precedence over config file values:

### AI Configuration
- `CLOUDFLARE_AI_TOKEN` - AI API key (required for AI features)
- `QHUB_AI_PROVIDER` - AI provider override
- `QHUB_AI_MODEL` - AI model override

### Quantum Configuration
- `IBM_QUANTUM_TOKEN` - IBM Quantum API key
- `QHUB_QUANTUM_PROVIDER` - Quantum provider override
- `QHUB_QUANTUM_BACKEND` - Default quantum backend

## Configuration Precedence

QHub loads configuration in the following order (highest precedence first):

1. **Environment variables** - Takes highest precedence
2. **Configuration file** - `~/.qhub/config.toml`
3. **Defaults** - Built-in sensible defaults

## Getting API Keys

### Cloudflare AI Gateway (for DeepSeek)
1. Sign up at https://dash.cloudflare.com
2. Create an AI Gateway
3. Get your API token
4. Set it: `export CLOUDFLARE_AI_TOKEN=your_token`

### IBM Quantum
1. Sign up at https://quantum.ibm.com
2. Go to Account Settings
3. Copy your API token
4. Set it: `export IBM_QUANTUM_TOKEN=your_token`

## Configuration Commands

Within QHub TUI:
- `/status` - View current configuration and API key status
- `/help` - Show all available commands

## Example: Setting Up QHub

### Option 1: Using Environment Variables (Recommended)

```bash
# Set AI key
export CLOUDFLARE_AI_TOKEN="your_cloudflare_token"

# Set quantum key (optional)
export IBM_QUANTUM_TOKEN="your_ibm_token"

# Run QHub
qhub
```

### Option 2: Using Configuration File

```bash
# Edit config file
vim ~/.qhub/config.toml

# Add your keys:
# [ai]
# api_key = "your_cloudflare_token"
# 
# [quantum]
# api_key = "your_ibm_token"

# Run QHub
qhub
```

## Security Best Practices

1. **Never commit API keys** to version control
2. **Use environment variables** for sensitive data
3. **Restrict config file permissions**: `chmod 600 ~/.qhub/config.toml`
4. **Rotate keys regularly** if compromised

## Troubleshooting

### "AI service error: 401"
- Your API key is invalid or not set
- Check `CLOUDFLARE_AI_TOKEN` environment variable
- Verify key in config file

### "Config file version X is newer than supported"
- Update QHub: `cargo install --path .`
- Or remove config: `rm ~/.qhub/config.toml`

### Config file not loading
- Check file permissions: `ls -l ~/.qhub/config.toml`
- Validate TOML syntax: `cat ~/.qhub/config.toml`
- Check logs for parse errors

## Advanced Configuration

### Custom AI Models

```toml
[ai]
provider = "openai"
model = "gpt-4"
api_key = "sk-..."
```

### Multiple Quantum Backends

Edit your quantum workflow to specify backends programmatically.

### UI Customization

```toml
[ui]
scroll_speed = 5        # Faster scrolling
show_timestamps = false # Cleaner chat view
```
