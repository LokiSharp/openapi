# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [2.0.6] 2024-11-16

- Add Serialize/Deserialize to response types.

# [2.0.4] 2024-11-15

- Add `LONGPORT_PRINT_QUOTE_PACKAGES` environment variable to enable printing the opened quote packages when connected to the server, default is `true`.

# [2.0.3] 2024-11-14

- Changed the `time` parameter of `Quote.history_candlesticks_by_offset` method to be optional.

# [2.0.2] 2024-10-31

- [python] Change `TradeStatus.SuspendTrade` to `TradeStatus.Suspend` in pyi.

# [2.0.1] 2024-10-22

- Returns the most recent historical candlesticks after subscribing to the candlesticks.

# [2.0.0] 2024-10-09

### Added

- Print the opened quote packages when connected to the server.
- Add `EstimateMaxPurchaseQuantityOptions.fractional_shares` field, sets to `true` to get the maximum fractional share buying power.
- The quantity type in the trading API has changed from `int` to `Decimal`.

# [1.0.32] 2024-08-28

- make Depth.price to optional type
