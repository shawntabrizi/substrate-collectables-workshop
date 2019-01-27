# [Substrate Collectables][main link]
> The interactive hands-on build-your-first-blockchain with [Substrate][] workshop

![A screenshot of Substrate kitties](./media/substrate-collectables.png)

## What this is

This is an interactive hands-on self-paced workshop. You will learn how to build your first blockchain using [Substrate][], the OpenSource [Rust][] Blockchain Development Kit by [Parity][]. Through the lessons of the workshop, you will build a collectables blockchain -- a chain that creates assets, and allows you to interact with and managing ownership of them.

As such, this material will focus on the logic of building the said chain. It won't cover the networking, consensus or economic incentive aspects of blockchains. Fortunately, Substrate comes with decent networking and consensus engines built in, so we can just focus on the chain logic.

Substrate is built using [Rust][], a modern statically typed systems programming language. We won't go into the details of the language within this workshop. The language is quite easy to read and follow and if you have programmed before, you shouldn't have too much trouble following what is going on and finishing the exercises even if [Rust][] is new to you.

## How to do it

Just go through the material chapter by chapter, do one exercise at a time. While the material is meant for you to be able to do on your own, we highly recommend you to get together and work on it with others, in learning groups or hosted workshops. It is totally normal to get stuck from time to time or to not understand what the material is attempting to explain. In those situations it helps a lot to have others around to talk to about it and resolve that frustration. That said, we highly appreciate any [feedback regarding the material, and where you might got stuck][feedback].

# [Let's go](/0/0.0-introduction.md)

---
**NOTE**

Substrate is a rapidly evolving project, which means that breaking changes may cause you problems when trying to follow these instructions. Feel free to [contact us](https://substrate.readme.io/v1.0.0/docs/feedback) with any problems you encounter.

---
## How to run the workshop on your local machine and contribute

* Fork the repo: https://github.com/shawntabrizi/substrate-collectables-workshop

* Install Git
  * macOS
    ```
    brew install git;
    echo "export PATH=/usr/local/bin:$PATH" >> ~/.bash_profile;
    source ~/.bash_profile;
    ```

  * Windows
    * Download [Git for Windows](https://gitforwindows.org/)

  * Ubuntu
    * Install dependencies

      ```
      sudo apt update;
      sudo apt install -y git;
      ```

* Clone your fork

```bash
git clone https://github.com/<YOUR_GITHUB_USERNAME>/substrate-collectables-workshop;
cd substrate-collectables-workshop;
```

* Install Ruby
  * macOS

    * Copy/paste and run the following in your terminal to install the latest version using RBEnv:

    ```bash
    xcode-select --install;
    brew install rbenv;
    rbenv init;
    echo 'if which rbenv > /dev/null; then eval "$(rbenv init -)"; fi' >> ~/.bash_profile;
    source ~/.bash_profile;
    rbenv install $(rbenv install -l | grep -v - | tail -1);
    rbenv global $(rbenv install -l | grep -v - | tail -1);
    ```

  * Windows
    * [Download and install Ruby](https://rubyinstaller.org/)
    * Restart your terminal window (if necessary)
    * Run Git Bash (right-click the Git Bash icon and choose "Run as administrator") 
    * Check Ruby version `ruby --version`

  * Ubuntu
    * Install RVM and Ruby https://rvm.io/rvm/install

* Install Jekyll and plug-ins

  ```bash
  gem install github-pages
  ```

* Run the Jekyll server that watches for changes

  ```bash
  jekyll serve
  ```

* View the Jekyll website in your browser at: http://127.0.0.1:4000/

* Contribute by commiting and pushing changes to a branch of your origin fork and creating a Pull Request to the upstream repository

## Acknowledgements

Open source projects like Substrate and this workshop could not be successful without the collective minds and collaborative effort of the development community.

The Substratekitties workshop stands on the backs of giants like [Cryptokitties](https://www.cryptokitties.co/), [Cryptozombies](https://cryptozombies.io/), [Docsify](https://docsify.js.org/), [Ace Editor](https://ace.c9.io/), [David Revoy's Cat Avatar Generator](https://framagit.org/Deevad/cat-avatar-generator), and numerous guinea pigs to report errors and bugs along the way.

I hope this educational material teaches you something new, and in turn, you teach others too.

---

[main link]: https://shawntabrizi.github.io/substrate-collectables-workshop/
[feedback]: https://substrate.readme.io/v1.0.0/docs/feedback
[Substrate]: https://www.parity.io/substrate/
[Substrate docs]: https://substrate.readme.io/
[Parity]: https://www.parity.io/
[Rust]: https://www.rust-lang.org/
