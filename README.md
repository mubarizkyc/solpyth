# solpyth
Integration of pyth with solana smart contracts  ,demonstrating  use of on chain price fetch

This repo demonstrates fetching price on chain as well as using client side code to use that contract
```console
cargo add pyth-solana-receiver-sdk
anchor build
anchor deploy
```
Testing
the client script is coded in tests folder
```console
anchor run test
```
You can query any token price wirh just pyth feed id
https://pyth.network/developers/price-feed-ids

Tested it with BTC/USD when it was aroung $58K
![Screenshot from 2024-07-13 16-44-10](https://github.com/user-attachments/assets/cc025fd2-a480-4c77-8562-a6d2e68c80b7)



# resources
https://github.com/pyth-network/pyth-examples/tree/main/price_feeds/solana/send_usd

