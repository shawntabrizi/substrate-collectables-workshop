# substrate-collectables-workshop-mdbook

[Substrate Collectables Workshop tutorial](https://github.com/shawntabrizi/substrate-collectables-workshop) rendered by [mdBook](https://github.com/rust-lang/mdBook).

## Generate

To re-generate the book:

1. Install node dependencies:

	```sh
	yarn --cwd generate
	```

2. Run the `generate` script:

	```sh
	yarn --cwd generate substrate-collectables-workshop
	```

3. Remove the existing `src` folder:

	```sh
	rm -rf src
	```

4. Move the generated `substrate-collectables-workshop` folder into the root with the name `src`.

	```sh
	mv generate/substrate-collectables-workshop src
	```
