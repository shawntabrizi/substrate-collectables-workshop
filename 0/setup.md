Setup
===

The Substrate framework provides a set of simple commands to set up your local environment.

These commands will:
- Install a package manager for your operating system
- Install the Rust development environment
- Install the core Substrate binaries
- Install [additional helper scripts](https://github.com/paritytech/substrate-up) for quick development of new Substrate nodes

When you're ready, run the following to download and install Substrate dependencies:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

> Note that we are using the `--fast` flag which skips the `cargo install` of `substrate` and `subkey` which can take a while. These are not necessary to complete this workshop, however may be used in other Substrate tutorials/guides you follow.

Unfortunately, since the installation process can be unique per operating system and machine, this script may not yet always work out of the box. You can [take a look at the script directly](https://github.com/paritytech/scripts/blob/master/get-substrate.sh), and any [PRs](https://github.com/paritytech/scripts/pulls) that may contain improvement that you could use and adjust it for your particular development environment. We're trying to bundle installation of relevant dependencies into a single script.

Whilst you're waiting for Substrate and its associated dependencies (i.e. Rust, etc) to download and install, there are other programs that may not be included in the bundle and should be installed in your development environment:

 - [Node + NPM](https://nodejs.org/en/download/)
 - [Visual Studio Code](https://code.visualstudio.com/) (or an IDE of your choice)
 - [Chrome](https://www.google.com/chrome/) or a Chromium based browser (sorry Firefox users :[ )

If you encounter any issues or discover an improvement (such as support for a different operating system to automate the installation process), then please create an [Issue](https://github.com/paritytech/scripts/issues) and/or a [PR]((https://github.com/paritytech/scripts/pulls)) if possible.
