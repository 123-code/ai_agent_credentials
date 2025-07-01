# ai_credentials

A Python library for managing credentials securely using a Rust backend.

## Installation

```bash
pip install ai_credentials
```

## Usage

```python
import ai_credentials

# Register credentials from .env file
ai_credentials.register_credentials('.env')

# Retrieve credentials for a specific username
password = ai_credentials.get_credentials('.env', 'user@example.com')
print(f'Retrieved password: {password}')
``` # Force refresh
