Set Up Substrate UI
===

It is finally time for us to use the `substratekitties-ui` folder that we got from the `substrate-package`. This folder will contain a React project from the [`substrate-ui` repo](https://github.com/paritytech/substrate-ui/tree/substrate-node-template).

## Setup the Substrate UI

For the best experience working with the Substrate UI repo, you should install `yarn` on your computer if you have not done so already:

[Install Yarn](https://yarnpkg.com/lang/en/docs/install/)

Now we can install the node packages required by the UI:

```
cd substratekitties-ui
yarn install
```

> **Note:** If you get an error using `yarn` on a debian-based system, see the debug note at the bottom.

And once the packages are done you can run the UI using:

```
yarn run dev
```

Make sure your Substrate node is up and running and open [localhost:8000](http://localhost:8000) in your Chrome browser.

----
**Debug**

You may need to remove the existing `yarn` (a legacy command line tool) and install via `npm`:
```
sudo apt remove cmdtest
sudo apt remove yarn
sudo npm install -g yarn
```
`yarn install` should work as expected now.

----

