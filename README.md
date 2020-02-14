# auto-blob-saver

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Save your blobs automatically.

**IMPORTANT: Due to a bug in rustc, async in loop causing crash. That means this project will crash if you turn this thing over about 30 minutes.**

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

NOTE: You need to install tsschecker first.

```
1. Get the latest release
2. Move it to /bin or PATH dir
3. Run 'auto_blob_saver' command
4. Once it's off, edit ~/.auto_blob_saver/config.json for your phone
5. Run 'auto_blob_saver' command again and enjoy!

NOTE: if you're getting trouble because of 'curlcode=28', run 'tsschecker --nocache -o' and try again.
```

## Usage

```
USAGE:
    auto_blob_saver [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --devices <DEVICES FILE>    Sets the devices.json path [default: ~/.auto_blob_saver/devices.json]
        --shsh <SHSH FOLDER>        Sets shsh folder path [default: ~/.shsh]
    -t, --time <MILLISECOND>        Sets shsh download time interval [default: 600000]
```

## Maintainers

[@Helloyunho](https://github.com/Helloyunho)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2020 Helloyunho
