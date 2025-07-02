import click
import ai_credentials

@click.group()
def cli():
    """AI Credentials Manager - Manage your credentials securely."""
    pass

@cli.command()
@click.argument('password')
def hash_password(password):
    """Hash a password using bcrypt."""
    hashed = ai_credentials.hash_password(password)
    click.echo(f'Hashed password: {hashed}')

@cli.command()
@click.argument('password')
@click.argument('hashed_password')
def verify_password(password, hashed_password):
    """Verify a password against its hash."""
    is_valid = ai_credentials.verify_password(password, hashed_password)
    click.echo(f'Password valid: {is_valid}')

@cli.command()
@click.argument('env_path')
def register_credentials(env_path):
    """Register credentials from a .env file."""
    ai_credentials.register_credentials(env_path)
    click.echo('Credentials registered successfully.')

@cli.command()
@click.argument('env_path')
@click.argument('username')
def get_credentials(env_path, username):
    """Get credentials for a specific username."""
    password = ai_credentials.get_credentials(env_path, username)
    click.echo(f'Password for {username}: {password}')

@cli.command()
@click.argument('master_password')
def set_master_password(master_password):
    """Set the master password."""
    ai_credentials.set_master_password(master_password)
    click.echo('Master password set successfully.')

@cli.command()
@click.argument('master_password')
def verify_master_password(master_password):
    """Verify the master password."""
    is_valid = ai_credentials.verify_master_password(master_password)
    click.echo(f'Master password valid: {is_valid}')

if __name__ == '__main__':
    cli() 