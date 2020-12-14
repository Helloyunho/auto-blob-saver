import argparse
import os
import json
import asyncio
import aiohttp
import secrets
import re
from . import CONST
from pathlib import Path
from .utils import run_tsschecker


async def get_blobs(device: dict, shsh_path: Path):
    os.makedirs(shsh_path.joinpath(device["ecid"]), exist_ok=True)
    async with aiohttp.ClientSession() as session:
        async with session.get(
            f'https://api.ipsw.me/v4/device/{device["identifier"]}?type=ota'
        ) as response:
            api_response = await response.json()
            firmwares = api_response["firmwares"]

            async def get_via_firmware(firmware: dict):
                if not firmware["signed"]:
                    return

                identifier_re = re.compile(
                    r"(?P<model>iPhone|iPad|iPod)(?P<gen>\d{1,2}),\d{1}"
                )
                identifier_re_searched = identifier_re.search(device["identifier"])
                model = identifier_re_searched.group("model")
                gen = int(identifier_re_searched.group("gen"))
                length = (
                    20
                    if ((model == "iPhone" or model == "iPod") and gen < 9)
                    or (model == "iPad" and gen < 7)
                    else 32
                )

                shsh_version_path = shsh_path.joinpath(
                    device["ecid"], firmware["buildid"]
                )
                os.makedirs(shsh_version_path, exist_ok=True)

                generator_blob_savers = [
                    asyncio.ensure_future(
                        run_tsschecker(
                            device["ecid"],
                            device["identifier"],
                            firmware["buildid"],
                            firmware["releasetype"] == "Beta",
                            device.get("boardconfig"),
                            generator,
                            device["apnonce"][index]
                            if "apnonce" in device and index < len(device["apnonce"])
                            else None,
                            shsh_version_path,
                        )
                    )
                    for (index, generator) in enumerate(
                        [
                            "0x1111111111111111",
                            "0xbd34a880be0b53f3",
                        ]
                    )
                ]

                apnonce_blob_savers = map(
                    lambda x: asyncio.ensure_future(
                        run_tsschecker(
                            device["ecid"],
                            device["identifier"],
                            firmware["buildid"],
                            firmware["releasetype"] == "Beta",
                            device.get("boardconfig"),
                            None,
                            secrets.token_hex(length),
                            shsh_version_path,
                        )
                    ),
                    range(os.cpu_count()),
                )

                await asyncio.gather(*generator_blob_savers, *apnonce_blob_savers)

            blob_savers_per_firm = map(
                lambda x: asyncio.ensure_future(get_via_firmware(x)), firmwares
            )
            await asyncio.gather(*blob_savers_per_firm)


async def main():
    home_path = Path.home()
    default_devices_path = home_path.joinpath(".auto_blob_saver").joinpath(
        "devices.json"
    )
    default_shsh_path = home_path.joinpath(".shsh")
    parser = argparse.ArgumentParser(
        description="Helloyunho (@helloyunho)\nSave your blobs automatically",
        prog="auto_blob_saver",
    )
    parser.add_argument(
        "--devices",
        default=default_devices_path,
        dest="devices_path",
        metavar="DEVICES_JSON_PATH",
        help="Sets the devices.json path",
        type=Path,
    )
    parser.add_argument(
        "--shsh",
        default=default_shsh_path,
        dest="shsh_path",
        metavar="SHSH_DIRECTORY_PATH",
        help="Sets the shsh directory path",
        type=Path,
    )
    parser.add_argument(
        "--time",
        "-t",
        default=600000,
        dest="time_interval",
        metavar="MILLISECONDS",
        help="Sets shsh download time interval",
    )

    args = parser.parse_args()

    os.makedirs(args.devices_path.parent, exist_ok=True)
    if not args.devices_path.is_file():
        with open(args.devices_path, "w") as write_file:
            write_file.write(json.dumps(CONST.default_devices_settings))
            print(
                f"Config file has written. Edit config file on {str(args.devices_path)}."
            )
            exit()
    os.makedirs(args.shsh_path, exist_ok=True)

    while True:
        with open(args.devices_path, "r") as read_file:
            devices = json.load(read_file)
            proc = await asyncio.create_subprocess_shell(
                "tsschecker --nocache -o",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )
            await proc.wait()

            proc = await asyncio.create_subprocess_shell(
                "tsschecker --nocache",
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )
            await proc.wait()

            for device in devices:
                await get_blobs(device, args.shsh_path)
        await asyncio.sleep(args.time_interval / 1000)


if __name__ == "__main__":
    asyncio.run(main())
