import unittest
import os
import ai_credentials

class TestAICredentials(unittest.TestCase):
    def setUp(self):
     
        with open('test.env', 'w') as f:
            f.write('TEST_USERNAME=user@example.com\n')
            f.write('TEST_PASSWORD=secret123\n')

    def tearDown(self):

        if os.path.exists('test.env'):
            os.remove('test.env')

    def test_hash_and_verify_password(self):
        password = "test123"
        hashed = ai_credentials.hash_password(password)
        self.assertTrue(ai_credentials.verify_password(password, hashed))
        self.assertFalse(ai_credentials.verify_password("wrongpass", hashed))

    def test_register_and_get_credentials(self):
        ai_credentials.register_credentials('test.env')
        password = ai_credentials.get_credentials('test.env', 'user@example.com')
        self.assertEqual(password, 'secret123')

    @unittest.skip("Skipping due to macOS keychain requiring user interaction. Run manually if needed.")
    def test_set_and_verify_master_password(self):
        master_password = "masterkey123"
        try:
            ai_credentials.set_master_password(master_password)
        except RuntimeError as e:
            if 'already exists' in str(e):

                pass
            else:
                raise
        self.assertTrue(ai_credentials.verify_master_password(master_password))
        self.assertFalse(ai_credentials.verify_master_password("wrongmaster"))

    def test_get_credentials_nonexistent_user(self):
        with self.assertRaises(KeyError):
            ai_credentials.get_credentials('test.env', 'nonexistent@example.com')

if __name__ == '__main__':
    unittest.main()