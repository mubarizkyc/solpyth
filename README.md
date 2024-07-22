# solpyth
Integration of pyth with solana smart contracts  ,demonstrating  use of on chain price fetch

This repo demonstrates fetching price on chain as well as using client side code to use that contract
```console
https://github.com/pyth-network/pyth-crosschain.git
```
In src/Cargo.toml add
``` console
 [dependencies]
anchor-lang = "0.30.1"
pyth-solana-receiver-sdk = {path="/PATH_TO/pyth_solana_receiver_sdk"}
```

```console
anchor build
anchor deploy
```
Testing
the client script is coded in tests folder
```console
npm install rpc-websockets
```
```console
anchor run test
```
You can query any token price wirh just pyth feed id
https://pyth.network/developers/price-feed-ids

Tested it with BTC/USD when it was aroung $58K
![Screenshot from 2024-07-13 16-44-10](https://github.com/user-attachments/assets/cc025fd2-a480-4c77-8562-a6d2e68c80b7)

# Support
If you benefitted from the project, show us some support by giving us a star ⭐. Open source is awesome!

# resources
https://docs.pyth.network/price-feeds/use-real-time-data/solana

