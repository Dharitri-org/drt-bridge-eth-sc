# Initial Setup

The bridge suite is comprised of multiple smart contracts. First and foremost, you'll need to set up an `Aggregator` smart contract, which will be used to get the approximate cost of transactions on Ethereum at a certain point in time.  

Additionally, you will have to issue at least two DCT tokens (suggested parameters in parentheses):  
- Wrapped MOAX (name: "WrappedMoax", ticker: "MOAX", initial supply: 1, decimals: 18)
- Wrapped ETH (name: "WrappedEth", ticker: "ETH", initial supply: 1, decimals: 18)

IMPORTANT: Ticker should always be chosen so that the aggregator has a mapping for it. "WMOAX" would not work at all, since the aggregator wouldn't know about the token, but "MOAX" works perfectly)  

You can find more about how to issue an DCT token here: https://docs.dharitri.com/developers/dct-tokens/#issuance-of-fungible-dct-tokens  

Next, we're going to setup the main "controller" contract, which will be a multisig-style SC. You can find more details about this type of smart contract here: https://github.com/dharitri/mx-sdk-rs/blob/master/contracts/examples/multisig/README.md  

Basically, we will have a certain number of board members (in this case we will call them "relayers") which will validate transactions and take the appropriate actions. As this is a multisig contract, at least a certain number of members must agree to the action, otherwise it cannot be performed.  

Once the multisig contract is deployed and setup properly, the owner must call the `deployChildContracts` to deploy `MoaxDctSwap`, `DctSafe` and `MultiTransferDct` contracts.  The endpoint looks like this:  

```
#[endpoint(deployChildContracts)]
fn deploy_child_contracts(
    &self,
    moax_dct_swap_code: ManagedBuffer,
    multi_transfer_dct_code: ManagedBuffer,
    dct_safe_code: ManagedBuffer,
    price_aggregator_contract_address: ManagedAddress,
    dct_safe_eth_tx_gas_limit: BigUint,
    multi_transfer_dct_eth_tx_gas_limit: BigUint,
    wrapped_moax_token_id: TokenIdentifier,
     token_whitelist: VarArgs<TokenIdentifier>,
)
```

The `_code` arguments are the compiled wasm bytecode of the respective contracts. `price_aggregator_contract_address` is the aggregator described in the intro (has to be deployed already). 

`dct_safe_eth_tx_gas_limit` and `multi_transfer_dct_eth_tx_gas_limit` are gas limits used for Dharitri -> Ethereum tx, and Ethereum -> Dharitri tx respectively. This is the gas limit used for processing on the Ethereum side (briding over to Dharitri or from Dharitri). This cost is used to calculate the fees taken from the bridged token, to be then used as payment/incentive for the relayers.  

`wrapped_moax_token_id` is the token identifier of the previously issued "WrappedMoax" token (Note: identifier format is ticker + '-' + 6 random characters). For WrappedMoax, it might look something like "MOAX-123456".  

`token_whitelist` is a list of tokens already issued that will be used by the bridge, in our case, that will be only one: The "WrappedEth" token.  

Once those are setup, the individual contracts will need a bit more setup themselves.   

# MoaxDctSwap 

Requires having local MINT and local BURN roles for the WrappedMoax token. More info about how to set local roles can be found here: https://docs.dharitri.com/tokens/dct-tokens/#setting-and-unsetting-special-roles

# MultiTransferDct

Requires local MINT role set for every token added to the whitelist.  

# DctSafe

Requires local BURN role set for every token added to the whitelist.  

# Erc20 to TokenIdentifier mapping

The relayers will need to know the mapping between Erc20 tokens on Ethereum and their respective representation as DCT on Dharitri. This mapping can be added by using the following function:  

```
#[endpoint(addMapping)]
fn add_mapping(&self, erc20_address: EthAddress, token_id: TokenIdentifier)
```

Where `EthAddress` is a 20 byte address.  

# End of Setup 

Setup is now complete! Now let's discuss the use-cases, workflows and more, in the [readme](../README.md) document.
