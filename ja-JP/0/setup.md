Setup
===

The Substrate framework provides a simple one-line command to get your machine ready to start development:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

This script will download and install different dependencies like a package manager, Rust, git, and more... If you are interested, you can check out the script by going [to the url](https://getsubstrate.io).

> Note that we are using the `--fast` flag which skips the `cargo install` of `substrate` and `subkey` which can take a while. These are not necessary to complete this workshop, however may be used in other Substrate tutorials/guides you follow.

If you are trying to develop on an operating system not supported by this script (for example Windows), you can take a look at the setup instructions [on the Substrate repository](https://github.com/paritytech/substrate#61-hacking-on-substrate).

While you're waiting for Substrate and its associated dependencies (i.e. Rust, etc) to download and install, there are other programs that are not be included in the bundle and should be installed in your development environment:

 - [Node + NPM](https://nodejs.org/en/download/)
 - [Yarn](https://yarnpkg.com)
 - [Visual Studio Code](https://code.visualstudio.com/) (or an IDE of your choice)
 - [Chrome](https://www.google.com/chrome/) or a Chromium based browser (sorry Firefox users :[ )

---

**Learn More**

The UI uses WebSockets to connect to the local Substrate node instance through an unencrypted connection. Most Browsers disallow this kind of connection for security and privacy reasons, only Chrome allows this connection _if it is connecting to localhost_. That is why we are using Chrome in this workshop. If you want to connect to the browser using a different computer in the network, it must be served through a secured connection.

If you want to use a different browser throughout this workshop, you will need to clone and run the [Polkadot.js Apps UI](https://github.com/polkadot-js/apps) locally, so that the UI is also served under a non-secure connection.

---