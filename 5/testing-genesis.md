Testing Genesis
===

you may have noticed that the setup gets quite tededious. 

you can choose to allow a genesis configuration. 

	the main purpose is that it seeds teh genesis blog with some seed values, chainspec... 
	it also means that you can write a test to mock that genesis config. 


## setting up kitties genesis

in kitties, we might want to deploy the chain letting users already specify some accounts which owns kitties

when you run the node, you'll do it up in chainspec. but we won't go into that much detail here

### testing kitties genesis

in our test, we can simulate deploying a genesis block which already gives some accounts kitties

test setup planned
account 1: has a kitty
account 2: has 2 kitties
accoutn 3: has 0 kitties