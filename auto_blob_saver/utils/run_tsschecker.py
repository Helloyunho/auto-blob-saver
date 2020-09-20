import asyncio
import os
import sys
from pathlib import Path
from typing import Union


async def run_tsschecker(
    ecid: str,
    identifier: str,
    buildid: str,
    ota: bool,
    boardconfig: Union[str, None],
    generator: Union[str, None],
    apnonce: Union[str, None],
    shsh_path: Path,
):
    args = ["tsschecker", "-d", identifier, "-e", ecid, "--buildid", buildid]
    if boardconfig is not None:
        args.append("-B")
        args.append(boardconfig)
    if ota:
        args.append("-o")
    if generator is not None:
        args.append("-g")
        args.append(generator)
    if apnonce is not None:
        args.append("--apnonce")
        args.append(apnonce)

    if generator is not None:
        shsh_path = shsh_path.joinpath(generator)
    elif apnonce is not None:
        shsh_path = shsh_path.joinpath(apnonce)

    args.append("--save-path")
    args.append(str(shsh_path))
    args.append("-s")

    os.makedirs(shsh_path, exist_ok=True)

    proc = await asyncio.create_subprocess_shell(
        " ".join(args), stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE
    )

    stdout, stderr = await proc.communicate()

    sys.stdout.write(stdout.decode("utf8"))
    if stderr:
        print("An error has occured while saving the blob.")
