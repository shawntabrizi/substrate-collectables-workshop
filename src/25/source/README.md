# Native Balances

In our next steps, we will introduce a marketplace for buying and selling kitties.

For that, we will need to access a user's blockchain balance in addition to the logic in our pallet.

## The Balances Pallet

Every blockchain has a cryptocurrency associated with it. Bitcoin has BTC. Ethereum as ETH.

For Polkadot, that native token is the DOT token.

Polkadot is built using FRAME and Pallets just like you have been building so far. Included in the `polkadot-sdk` is `pallet_balances`.

This is a Pallet designed specifically to manage the native balance for users.

It has the ability to:

- Mint new tokens.
- Transfer tokens between users.
- Apply freezes and holds for users.
- Slash tokens from accounts.
- and much more...

Basically everything you could expect to want or need when working with the native balance of a blockchain.

## Pallet Coupling

The `polkadot-sdk` is designed to be a flexible and modular blockchain development SDK.

Part of that flexibility comes through the use of Rust traits to allow two pallets to interact with one another. We call this pallet coupling, and there are two forms of it we will briefly explain next.

### Tight Coupling

We have already been using tight coupling throughout this tutorial to give our custom Kitties pallet access to the `frame_system` pallet:

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
	// Through supertraits, we are tightly coupled to `frame_system`.
}
```

You can see our Pallet's `Config` is tightly coupled to the `frame_system::Config`. This is why we have been able to use the types coming from `frame_system` (like `T::AccountId`) and why we have been able to use functions directly from `frame_system` (like `frame_system::Pallet::<T>::block_number()`).

In fact, every Pallet built with FRAME is required to be tightly coupled to `frame_system`. But if we wanted, we could tightly couple to other pallets too!

```rust
#[pallet::config]
pub trait Config: frame_system::Config + pallet_balances:: Config {
	// Here you can see we can also tightly couple to `pallet_balances`.
}
```

The upside to tight coupling is gaining direct access to the pallet's Rust module, and all the functions, types, storage, and everything else that is included in that pallet.

With tight coupling, we are able to access the `pallet_balances` APIs like:

```rust
let total_issuance = pallet_balances::Pallet::<T>::total_issuance();
let alice_balance = pallet_balances::Pallet::<T>::total_balance(alice);
pallet_balances::Pallet::<T>::mint_into(alice, amount)?;
pallet_balances::Pallet::<T>::transfer(alice, bob, amount, Preserve)?;
```

The downside however, is that you make your pallet very rigid, forcing everyone who wants to use your pallet to use a specific version of `pallet_balances` which you import into your crate.

### Loose Coupling

Loose coupling is the more flexible approach to accessing another pallet, and will be our way of integrating the Balances Pallet in our project.

Loose coupling involves using the interface of a `trait` to access the APIs of another Pallet.

In the case of accessing the Balances Pallet, it looks exactly like this:

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
	type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	/// Access the balances pallet through the associated type `NativeBalance`.
	/// The `NativeBalance` type must implement `Inspect` and `Mutate`.
	/// Both of these traits are generic over the `AccountId` type.
	type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
}
```

You can see we introduce a new associated type called `NativeBalance`. We then require that this type must implment two traits:

- [`fungible::Inspect`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/tokens/fungible/trait.Inspect.html): A trait allowing us to read data about a fungible token.
- [`fungible::Mutate`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/tokens/fungible/trait.Mutate.html): A trait allowing us to write data about a fungible token.

So with this, we are able to access our native balance using APIs like:

```rust
// Example APIs coming from `Inspect`.
let total_issuance = T::NativeBalance::total_issuance();
let alice_balance = T::NativeBalance::total_balance(alice);
// Example APIs coming from `Mutate`.
T::NativeBalance::mint_into(alice, amount)?;
T::NativeBalance::transfer(alice, bob, amount, Preserve)?;
```

The key difference here is that we do NOT assume that these APIs must from from specifically `pallet_balances`. If you wanted to use another pallet in the `polkadot-sdk` ecosystem which provides these same functions, you can use it! Our pallet is NOT tightly coupled to which pallet provides access to the `NativeBalance`, it only requires that there is something implementing the `Inspect` and `Mutate` traits.

The power of loose coupling may not be immediately obvious, but as you get deeper into developing in the Polkadot ecosystem, you will start to realize how powerful this approach can be.

## Your Turn

Import the `Inspect` and `Mutate` traits from `frame::traits::fungible`.

Introduce the `NativeBalance` associated type to your `trait Config` using these traits.

## Learn More

To continue learning about Pallet Coupling, check out the following video from the Polkadot Blockchain Academy:

<iframe width="560" height="315" src="https://www.youtube.com/embed/mPkgG9ANNzI?si=6CrBZBMaHHBvQLYD" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
