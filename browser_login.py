import ai_credentials
from browser_use.llm import ChatGoogle
from browser_use import Agent
from dotenv import load_dotenv
import asyncio


load_dotenv()


llm = ChatGoogle(model='gemini-2.5-flash')


master_password = input('Enter master password: ')
if not ai_credentials.verify_master_password(master_password):
    print('Invalid master password. Access denied.')
    exit(1)


username = 'jnarcursos@gmail.com'
password = ai_credentials.get_credentials('.env', username)


agent = Agent(
    task=f"Login to the gmail using username: {username} and password: {password}",
    llm=llm
)


additional_task = input('What would you like the agent to do after login? ')
agent.task = additional_task


async def run_agent(agent):
    await agent.run()


asyncio.run(run_agent(agent))

print(f'Agent initialized with credentials for {username} and now handling task: {additional_task}')