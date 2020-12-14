# auto-blob-saver

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Save your blobs automatically.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

NOTE: You need to install tsschecker first.

```
1. Get the latest release by running pip install auto-blob-saver
3. Run 'auto_blob_saver' command
4. Once it's off, edit ~/.auto_blob_saver/devices.json for your phone
5. Run 'auto_blob_saver' command again and enjoy!

Automatically updates tsschecker's cached data since 'v0.1.2'.
```

## Usage

```
usage: auto_blob_saver [-h] [--devices DEVICES_JSON_PATH] [--shsh SHSH_DIRECTORY_PATH] [--time MILLISECONDS]

optional arguments:
  -h, --help            show this help message and exit
  --devices DEVICES_JSON_PATH
                        Sets the devices.json path
  --shsh SHSH_DIRECTORY_PATH
                        Sets the shsh directory path
  --time MILLISECONDS, -t MILLISECONDS
                        Sets shsh download time interval
```

## Maintainers

[@Helloyunho](https://github.com/Helloyunho)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2020 Helloyunho
