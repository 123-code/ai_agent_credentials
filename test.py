import pytest
import tempfile
import os
import ai_credentials

def test_password_hashing():
    password = "test_password"
    hashed = ai_credentials.hash_password(password)
    assert ai_credentials.verify_password(password, hashed)
    assert not ai_credentials.verify_password("wrong_password", hashed)

def test_master_password():
    master_pass = "master123"
    ai_credentials.set_master_password(master_pass)
    assert ai_credentials.verify_master_password(master_pass)
    assert not ai_credentials.verify_master_password("wrong")

def test_credentials_management():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.env', delete=False) as f:
        f.write("GMAIL_USERNAME=test@example.com\n")
        f.write("GMAIL_PASSWORD=testpass123\n")
        f.flush()
        
        try:
            ai_credentials.register_credentials(f.name)
            password = ai_credentials.get_credentials(f.name, "test@example.com")
            assert password == "testpass123"
        finally:
            os.unlink(f.name)

if __name__ == "__main__":
    test_password_hashing()
    test_master_password()  
    test_credentials_management()
    print("All tests passed!")