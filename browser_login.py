import ai_credentials
from browser_use.llm import ChatGoogle
from browser_use import Agent
from dotenv import load_dotenv
import asyncio
import os
import re 

load_dotenv()

def extract_services(env_content):
    pattern = r'^(\w+)_USERNAME'
    matches = re.findall(pattern,env_content,re.MULTILINE)
    return matches


llm = ChatGoogle(model='gemini-2.5-flash', api_key='YOUR GEMINI API KEY')


task = "Login to gmail"
service_match = re.search(r'(?i)Login to (\w+)', task)
if service_match:
    service = service_match.group(1).upper()
    print(f"Extracted service: {service}")
    print("Note: Ensure your .env file has keys in the format SERVICE_USERNAME, not SERVICE_USRRNAME")
else:
    service = None
    print("Could not extract service from task")



ai_credentials.register_credentials('.env')


username, password = ai_credentials.get_credentials('.env', service)


agent = Agent(task=f"Login to {service} using username: {username} and password: {password}",llm=llm)

async def run_agent(agent):
    await agent.run()




asyncio.run(run_agent(agent))

print(f'Agent initialized with credentials for {username} and now handling task:')