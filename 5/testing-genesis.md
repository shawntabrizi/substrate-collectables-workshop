Testing Genesis
===

By meow, you may have noticed that the test setup can get quite tededious,
i.e. having to create new kitties for each unit test.

Wouldn't it be great to deploy our blockchain with some existing kitties in the genesis block?

Substrate lets you deploy your chain with preconfigured storage.
You can allow a genesis configuration for your Substrate modules.

In this section, we'll walk you through: 
- Extending `decl_storage` to add the extra genesis
- Mocking the genesis configuration in tests
- Testing that we have the correct genesis set up

## setting up kitties genesis

Let's start by importing the following types and traits from runtime_io at the top of your file.

**substratekitties<span>.</span>rs**
```rust
use runtime_io::{with_storage, StorageOverlay, ChildrenStorageOverlay};
```
in kitties, we might want to deploy the chain already specifying: 
- some accounts
- having those accounts already own kitties

you can see an example of this in...
when you run the node, you'll do it up in chainspec. but we won't go into that much detail here

### testing kitties genesis

in our test, we can simulate deploying a genesis block which already gives some accounts kitties

test setup planned
account 1: has a kitty
account 2: has 2 kitties
accoutn 3: has 0 kitties

### configuring genesis in the node
<!-- address this later -->

# Your Turn!

Set up your genesis specs as specified above and get your tests to pass.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->