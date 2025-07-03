# ai_credentials

A minimalist Python library for managing credentials securely, built in Rust.

## Installation

```bash
pip install ai-credentials
```

## Usage

### Hash a password
```python
from ai_credentials import hash_password
hashed = hash_password("my_password")
```

### Verify a password
```python
from ai_credentials import verify_password
is_valid = verify_password("my_password", hashed)
```

### Register credentials from a .env file
```python
from ai_credentials import register_credentials
register_credentials("/path/to/.env")
```

### Get credentials for a service
```python
from ai_credentials import get_credentials
username, password = get_credentials("/path/to/.env", "MY_SERVICE")
```
