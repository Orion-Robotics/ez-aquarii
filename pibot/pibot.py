import os
import socket
import subprocess
import urllib.request
from typing import List
from xmlrpc.client import Boolean

from discord.ext import commands
from dotenv import load_dotenv
from pyngrok import ngrok


def get_ip_address():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect(("8.8.8.8", 80))
    return s.getsockname()[0]


load_dotenv()

bot = commands.Bot(command_prefix="$")


@bot.command(description="Get the local and public IP of the pi")
async def get_ip(ctx):
    local_ip = get_ip_address()
    external_ip = None
    try:
        external_ip = urllib.request.urlopen("https://ident.me").read().decode("utf8")
    except:
        pass
    await ctx.send(f"Local IP: {local_ip}\nExternal IP: {external_ip}")


@bot.command(name="ngrok", description="Starts an ngrok tunnel to the pi")
async def start_ngrok(ctx):
    ssh_tunnel = ngrok.connect(22, "tcp")
    host, _, port = ssh_tunnel.public_url[6:].partition(":")
    await ctx.send(
        f"SSH tunnel started: {ssh_tunnel.public_url} (`ssh pi@{host} -p {port}`)"
    )


@bot.command(description="Updates from git")
async def update(ctx):
    path = os.getenv("GIT_REPO_PATH") or "."
    user = os.getenv("GIT_USER") or ""
    password = os.getenv("GIT_PASSWORD") or ""
    subprocess.run(
        [
            "git",
            "-C",
            path,
            "pull",
            f"https://{user}:{password}@gitlab.com/princeton-soccer-robotics/ez-aquarii.git",
        ]
    )
    await ctx.send("Updated!")


def systemd_cmd(user=False) -> List[str]:
    args = ["systemctl"]
    if user:
        args.append("--user")
    return args


def journalctl_cmd(user=False) -> List[str]:
    args = ["journalctl"]
    if user:
        args.append("--user")
    return args


@bot.command(description="Restarts a user service")
async def restart(ctx, service: str, user=False):
    subprocess.run([*systemd_cmd(user), "restart", service])


@bot.command(description="Starts a user service")
async def start(ctx, service: str, user=False):
    subprocess.run([*systemd_cmd(user), "start", service])


@bot.command(description="Stops a user service")
async def stop(ctx, service: str, user=False):
    subprocess.run([*systemd_cmd(user), "stop", service])


@bot.command(description="Reads the journal")
async def journal(ctx, service: str, user=False):
    try:
        result = subprocess.run(
            [*journalctl_cmd(user), "-xeu", service, "--no-pager", "--output", "cat"],
            capture_output=True,
        ).stdout.decode("utf8")

        truncated = result[-500:] if len(result) > 500 else result

        await ctx.send(f"```\n{truncated}\n```")
    except Exception as e:
        await ctx.send(f"Error reading journal data {e}")


bot.run(os.getenv("DISCORD_TOKEN"))
