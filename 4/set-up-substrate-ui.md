Set Up Substrate UI
===

Similar to the `substrate-node-new` command you used way back at the beginning of this tutorial, when you install Substrate, we also provide you with a `substrate-ui-new` command which will automatically clone the [`substrate-ui` repo](https://github.com/paritytech/substrate-ui/tree/substrate-node-template) on your computer.

## Install the Substrate UI

For the best experience working with the Substrate UI repo, you should install `yarn` on your computer following the instructions here:

[Install Yarn](https://yarnpkg.com/lang/en/docs/install/)

Run the following command in your working folder, as indicated by the usage instructions `substrate-ui-new --help`:

```
substrate-ui-new substratekitties
```

You should see that a new folder `<NAME>-ui` is created where the `substrate-ui` repo is cloned. In that folder, install the required packages using:

```
cd substratekitties-ui
yarn install
```
(If you get an error using `yarn` on a debian-based system, see the note at the bottom.)

And once the packages are done you can run the UI using:

```
yarn run dev
```

Make sure your Substrate node is up and running and open [localhost:8000](http://localhost:8000) in your Chrome browser.

----
_Note_:

The UI uses WebSockets to connect to the local Substrate node instance through an unencrypted connection. Most Browsers disallow this kind of connection for security and privacy reasons, only Chrome allows this connection _if it is connecting to localhost_. That is why we are using Chrome in this workshop. If you want to connect to the browser using a different computer in the network, it must be served through a secured connection.

_Note_:

You may need to remove the existing `yarn` (a legacy command line tool) and install via `npm`:
```
sudo apt remove cmdtest
sudo apt remove yarn
sudo npm install -g yarn
```
`yarn install` should work as expected now.

----

