# rubi-rs

## Description

`rubi-rs` is an SDK which allows users to transact on the [Rubicon](https://rubicon.finance) protocol, a limit order book market on [Optimism](https://www.optimism.io/).

[![Crates.io][crates-badge]][crates-url]
[![Rubi documentation](https://docs.rs/rubi/badge.svg)](https://docs.rs/rubi)
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/rubi.svg
[crates-url]: https://crates.io/crates/rubi
[discord-badge]: https://img.shields.io/discord/752590582274326680.svg?logo=discord&style=flat-square
[discord-url]: https://discord.com/invite/E7pS24J

## Risk Disclaimers

### SDK Disclaimer

This codebase is in Alpha and could contain bugs or change significantly between versions. Contributing through Issues or Pull Requests is welcome!

### Protocol Disclaimer

Please refer to [this](https://docs.rubicon.finance/rubicon-docs/protocol/rubicon-pools/risks) for information on the risks associated to the Rubicon Protocol.

## Documentation

Documentation for `rubi-rs` can be found on Crates.io. Documentation for the Rubicon protocol can be found [here](https://docs.rubicon.finance)

## Features

### Feature Flags

-   `aid`: Enables support for the Market Aid contract (currently internal to the Rubicon team)
-   `full`: Enables all of the following features.
-   `streaming`: Enables event streaming via flume. Requires a websocket endpoint
-   `ierc20`: Enables ERC-20 support.

### Beta

-   [x] Basic Rubicon v1.3 Market support
-   [x] Basic Rubicon v1.3 Pair support
-   [x] Basic Rubicon v1.3 BathHouse support
-   [x] API-Wrapped Rubicon v1.3 Market support
-   [ ] API-Wrapped Rubicon v1.3 Pair support
-   [x] API-Wrapped Rubicon v1.3 BathHouse support
-   [x] Full [tracing](https://github.com/tokio-rs/tracing) support
-   [x] Optional ERC-20 support

### Future

-   [ ] Full event streaming/broadcasting support for Rubicon v1.3 events
-   [ ] Uniswap V3 support

## Versioning

This SDK is still in alpha. However, in order to maintain consistency with the Rubicon protocol, we follow the convention that the major and minor version numbers match the major and minor version numbers of the protocol, while the patch version number denotes same-version patches and upgrades to the SDK.

## License

`rubi-rs` is licensed under the Apache License (Version 2.0). See [LICENSE.txt](https://github.com/RubiconDeFi/rubi-rs/blob/master/LICENSE.txt) for details.
