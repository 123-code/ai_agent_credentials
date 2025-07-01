"""
AI Credentials - A secure credential management library with Rust backend.
"""

from .ai_credentials import (
    hash_password,
    verify_password,
    register_credentials,
    get_credentials,
    set_master_password,
    verify_master_password,
)

__all__ = [
    "hash_password",
    "verify_password", 
    "register_credentials",
    "get_credentials",
    "set_master_password",
    "verify_master_password",
]

__version__ = "0.1.0" 