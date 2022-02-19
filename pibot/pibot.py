import os
import socket
import urllib.request

from discord.ext import commands
from dotenv import load_dotenv
from pyngrok import ngrok

load_dotenv()

bot = commands.Bot(command_prefix="$")


@bot.command()
async def get_ip(ctx):
    local_ip = socket.gethostbyname(socket.gethostname())
    external_ip = urllib.request.urlopen("https://ident.me").read().decode("utf8")
    await ctx.send(f"Local IP: {local_ip}\nExternal IP: {external_ip}")


@bot.command()
async def start_ngrok(ctx):
    ssh_tunnel = ngrok.connect(22, "tcp")
    host, _, port = ssh_tunnel.public_url[6:].partition(":")
    await ctx.send(
        f"SSH tunnel started: {ssh_tunnel.public_url} (`ssh pi@{host} -p {port}`)"
    )


bot.run(os.getenv("DISCORD_TOKEN"))
