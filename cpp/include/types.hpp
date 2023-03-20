#pragma once

#include "decimal.hpp"
#include <optional>
#include <vector>

namespace longbridge {

struct Date
{
  int32_t year;
  uint8_t month;
  uint8_t day;
};

struct Time
{
  uint8_t hour;
  uint8_t minute;
  uint8_t second;
};

/// Language identifer
enum class Language
{
  /// zh-CN
  ZH_CN,
  /// zh-HK
  ZH_HK,
  /// en
  EN,
};

/// Market
enum class Market
{
  /// Unknown
  Unknown,
  /// US market
  US,
  /// HK market
  HK,
  /// CN market
  CN,
  /// SG market
  SG,
};

namespace quote {

enum class TradeStatus;

/// Subscription flags
class SubFlags
{
private:
  uint8_t value_;

public:
  inline SubFlags(uint8_t value) { value_ = value; }

  inline operator uint8_t() const { return value_; }

  inline SubFlags operator|(const SubFlags& other) const
  {
    return SubFlags(value_ | other.value_);
  }

  inline SubFlags& operator|=(const SubFlags& other)
  {
    value_ |= other.value_;
    return *this;
  }

  inline SubFlags operator&(const SubFlags& other) const
  {
    return SubFlags(value_ & other.value_);
  }

  inline SubFlags& operator&=(const SubFlags& other)
  {
    value_ &= other.value_;
    return *this;
  }

  inline bool operator==(const SubFlags& other) const
  {
    return value_ == other.value_;
  }

  inline bool contains(const SubFlags& other) const
  {
    return (value_ & other.value_) > 0;
  }

  static SubFlags QUOTE();
  static SubFlags DEPTH();
  static SubFlags BROKER();
  static SubFlags TRADE();
};

/// Derivative type
struct DerivativeType
{
  uint8_t value;

  bool has_option();
  bool has_warrant();
};

/// Candlestick period
enum class Period
{
  /// Unknown,
  Unknown,
  /// One Minute
  Min1,
  /// Five Minutes
  Min5,
  /// Fifteen Minutes
  Min15,
  /// Thirty Minutes
  Min30,
  /// Sixty Minutes
  Min60,
  /// One Day
  Day,
  /// One Week
  Week,
  /// One Month
  Month,
  /// One Year
  Year,
};

/// Subscription
struct Subscription
{
  /// Security code
  std::string symbol;
  /// Subscription flags
  SubFlags sub_types;
  /// Candlesticks
  std::vector<Period> candlesticks;
};

/// Trade session
enum class TradeSession
{
  /// Trading
  Normal,
  /// Pre-Trading
  Pre,
  /// Post-Trading
  Post,
};

/// Quote message
struct PushQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Trade session
  TradeSession trade_session;
};

struct Depth
{
  /// Position
  int32_t position;
  /// Price
  Decimal price;
  /// Volume
  int64_t volume;
  /// Number of orders
  int64_t order_num;
};

/// Depth message
struct PushDepth
{
  /// Security code
  std::string symbol;
  /// Ask depth
  std::vector<Depth> asks;
  /// Bid depth
  std::vector<Depth> bids;
};

/// Brokers
struct Brokers
{
  /// Position
  int32_t position;
  /// Broker IDs
  std::vector<int32_t> broker_ids;
};

/// Brokers message
struct PushBrokers
{
  /// Security code
  std::string symbol;
  /// Ask brokers
  std::vector<Brokers> ask_brokers;
  /// Bid brokers
  std::vector<Brokers> bid_brokers;
};

/// Security board
enum class SecurityBoard
{
  /// Unknown
  Unknown,
  /// US Main Board
  USMain,
  /// US Pink Board
  USPink,
  /// Dow Jones Industrial Average
  USDJI,
  /// Nasdsaq Index
  USNSDQ,
  /// US Industry Board
  USSector,
  /// US Option
  USOption,
  /// US Sepecial Option
  USOptionS,
  /// Hong Kong Equity Securities
  HKEquity,
  /// HK PreIPO Security
  HKPreIPO,
  /// HK Warrant
  HKWarrant,
  /// Hang Seng Index
  HKHS,
  /// HK Industry Board
  HKSector,
  /// SH Main Board(Connect)
  SHMainConnect,
  /// SH Main Board(Non Connect)
  SHMainNonConnect,
  /// SH Science and Technology Innovation Board
  SHSTAR,
  /// CN Index
  CNIX,
  /// CN Industry Board
  CNSector,
  /// SZ Main Board(Connect)
  SZMainConnect,
  /// SZ Main Board(Non Connect)
  SZMainNonConnect,
  /// SZ Gem Board(Connect)
  SZGEMConnect,
  /// SZ Gem Board(Non Connect)
  SZGEMNonConnect,
  /// SG Main Board
  SGMain,
  /// Singapore Straits Index
  STI,
  /// SG Industry Board
  SGSector,
};

/// The basic information of securities
struct SecurityStaticInfo
{
  /// Security code
  std::string symbol;
  /// Security name (zh-CN)
  std::string name_cn;
  /// Security name (en)
  std::string name_en;
  /// Security name (zh-HK)
  std::string name_hk;
  /// Exchange which the security belongs to
  std::string exchange;
  /// Trading currency
  std::string currency;
  /// Lot size
  int32_t lot_size;
  /// Total shares
  int64_t total_shares;
  /// Circulating shares
  int64_t circulating_shares;
  /// HK shares (only HK stocks)
  int64_t hk_shares;
  /// Earnings per share
  Decimal eps;
  /// Earnings per share (TTM)
  Decimal eps_ttm;
  /// Net assets per share
  Decimal bps;
  /// Dividend yield
  Decimal dividend_yield;
  /// Types of supported derivatives
  DerivativeType stock_derivatives;
  /// Board
  SecurityBoard board;
};

/// Trade status
enum class TradeStatus
{
  /// Normal
  Normal,
  /// Suspension
  Halted,
  /// Delisted
  Delisted,
  /// Fuse
  Fuse,
  /// Papare List
  PrepareList,
  /// Code Moved
  CodeMoved,
  /// To Be Opened
  ToBeOpened,
  /// Split Stock Halts
  SplitStockHalts,
  /// Expired
  Expired,
  /// Warrant To BeListed
  WarrantPrepareList,
  /// Suspend
  SuspendTrade,
};

/// Quote of US pre/post market
struct PrePostQuote
{
  /// Latest price
  Decimal last_done;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Close of the last trade session
  Decimal prev_close;
};

/// Quote of securitity
struct SecurityQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Quote of US pre market
  std::optional<PrePostQuote> pre_market_quote;
  /// Quote of US post market
  std::optional<PrePostQuote> post_market_quote;
};

/// Option type
enum class OptionType
{
  /// Unknown
  Unknown,
  /// American
  American,
  /// Europe
  Europe,
};

/// Option direction
enum class OptionDirection
{
  /// Unknown
  Unknown,
  /// Put
  Put,
  /// Call
  Call,
};

/// Quote of option
struct OptionQuote
{ /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Implied volatility
  Decimal implied_volatility;
  /// Number of open positions
  int64_t open_interest;
  /// Exprity date
  Date expiry_date;
  /// Strike price
  Decimal strike_price;
  /// Contract multiplier
  Decimal contract_multiplier;
  /// Option type
  OptionType contract_type;
  /// Contract size
  Decimal contract_size;
  /// Option direction
  OptionDirection direction;
  /// Underlying security historical volatility of the option
  Decimal historical_volatility;
  /// Underlying security symbol of the option
  std::string underlying_symbol;
};

/// Trade direction
enum class TradeDirection
{
  /// Neutral
  Neutral,
  /// Down
  Down,
  /// Up
  Up
};

/// Trade
struct Trade
{
  Decimal price;
  int64_t volume;
  int64_t timestamp;
  std::string trade_type;
  TradeDirection direction;
  TradeSession trade_session;
};

/// Trades message
struct PushTrades
{
  /// Security code
  std::string symbol;
  /// Trades data
  std::vector<Trade> trades;
};

/// Candlestick
struct Candlestick
{
  /// Close price
  Decimal close;
  /// Open price
  Decimal open;
  /// Low price
  Decimal low;
  /// High price
  Decimal high;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Timestamp
  int64_t timestamp;
};

/// Candlestick updated message
struct PushCandlestick
{
  /// Security code
  std::string symbol;
  /// Period type
  Period period;
  /// Candlestick
  Candlestick candlestick;
};

/// Warrant type
enum class WarrantType
{
  /// Unknown
  Unknown,
  /// Call
  Call,
  /// Put
  Put,
  /// Bull
  Bull,
  /// Bear
  Bear,
  /// Inline
  Inline
};

/// Quote of warrant
struct WarrantQuote
{ /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Implied volatility
  Decimal implied_volatility;
  /// Exprity date
  Date expiry_date;
  /// Last tradalbe date
  Date last_trade_date;
  /// Outstanding ratio
  Decimal outstanding_ratio;
  /// Outstanding quantity
  int64_t outstanding_quantity;
  /// Conversion ratio
  Decimal conversion_ratio;
  /// Warrant type
  WarrantType category;
  /// Strike price
  Decimal strike_price;
  /// Upper bound price
  Decimal upper_strike_price;
  /// Lower bound price
  Decimal lower_strike_price;
  /// Call price
  Decimal call_price;
  /// Underlying security symbol of the warrant
  std::string underlying_symbol;
};

/// Security depth
struct SecurityDepth
{
  /// Ask depth
  std::vector<Depth> asks;
  /// Bid depth
  std::vector<Depth> bids;
};

/// Security brokers
struct SecurityBrokers
{
  /// Ask brokers
  std::vector<Brokers> ask_brokers;
  /// Bid brokers
  std::vector<Brokers> bid_brokers;
};

struct ParticipantInfo
{
  /// Broker IDs
  std::vector<int32_t> broker_ids;
  /// Participant name (zh-CN)
  std::string name_cn;
  /// Participant name (en)
  std::string name_en;
  /// Participant name (zh-HK)
  std::string name_hk;
};

/// Intraday line
struct IntradayLine
{
  Decimal price;
  int64_t timestamp;
  int64_t volume;
  Decimal turnover;
  Decimal avg_price;
};

/// Adjust type
enum class AdjustType
{
  NoAdjust,
  ForwardAdjust
};

/// Strike price info
struct StrikePriceInfo
{
  /// Strike price
  Decimal price;
  /// Security code of call option
  std::string call_symbol;
  /// Security code of put option
  std::string put_symbol;
  /// Is standard
  bool standard;
};

/// Issuer info
struct IssuerInfo
{
  /// Issuer ID
  int32_t issuer_id;
  /// Issuer name (zh-CN)
  std::string name_cn;
  /// Issuer name (en)
  std::string name_en;
  /// Issuer name (zh-HK)
  std::string name_hk;
};

struct TradingSessionInfo
{
  /// Being trading time
  Time begin_time;
  /// End trading time
  Time end_time;
  /// Trading session
  TradeSession trade_session;
};

/// Market trading session
struct MarketTradingSession
{
  /// Market
  Market market;
  /// Trading session
  std::vector<TradingSessionInfo> trade_session;
};

/// Market trading days
struct MarketTradingDays
{
  /// Trading days
  std::vector<Date> trading_days;
  /// Half trading days
  std::vector<Date> half_trading_days;
};

/// Capital flow line
struct CapitalFlowLine
{
  /// Inflow capital data
  Decimal inflow;
  /// Time
  int64_t timestamp;
};

/// Capital distribution
struct CapitalDistribution
{
  /// Large order
  Decimal large;
  /// Medium order
  Decimal medium;
  /// Small order
  Decimal small;
};

/// Capital distribution response
struct CapitalDistributionResponse
{
  /// Time
  int64_t timestamp;
  /// Inflow capital data
  CapitalDistribution capital_in;
  /// Outflow capital data
  CapitalDistribution capital_out;
};

/// Watch list security
struct WatchListSecurity
{
  /// Security symbol
  std::string symbol;
  /// Market
  Market market;
  /// Security name
  std::string name;
  /// Watched price
  std::optional<Decimal> watched_price;
  /// Watched time
  int64_t watched_at;
};

/// Watch list group
struct WatchListGroup
{
  /// Group id
  int64_t id;
  /// Group name
  std::string name;
  /// Securities
  std::vector<WatchListSecurity> securities;
};

/// Real-time quote
struct RealtimeQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
};

} // namespace quote

namespace trade {

/// Topic type
enum class TopicType
{
  /// Private notification for trade
  Private,
};

/// Exexution
struct Execution
{
  std::string order_id;
  std::string trade_id;
  std::string symbol;
  int64_t trade_done_at;
  int64_t quantity;
  Decimal price;
};

/// Options for get histroy executions request
struct GetHistoryExecutionsOptions
{
  /// Start time
  std::optional<int64_t> start_at;
  /// End time
  std::optional<int64_t> end_at;
  /// Security code
  std::optional<std::string> symbol;
};

/// Options for get today executions request
struct GetTodayExecutionsOptions
{
  /// Security code
  std::optional<std::string> symbol;
  /// Order id
  std::optional<std::string> order_id;
};

/// Order status
enum class OrderStatus
{
  /// Unknown
  Unknown,
  /// Not reported
  NotReported,
  /// Not reported (Replaced Order)
  ReplacedNotReported,
  /// Not reported (Protected Order)
  ProtectedNotReported,
  /// Not reported (Conditional Order)
  VarietiesNotReported,
  /// Filled
  Filled,
  /// Wait To New
  WaitToNew,
  /// New
  New,
  /// Wait To Replace
  WaitToReplace,
  /// Pending Replace
  PendingReplace,
  /// Replaced
  Replaced,
  /// Partial Filled
  PartialFilled,
  /// Wait To Cancel
  WaitToCancel,
  /// Pending Cancel
  PendingCancel,
  /// Rejected
  Rejected,
  /// Canceled
  Canceled,
  /// Expired
  Expired,
  /// Partial Withdrawal
  PartialWithdrawal,
};

/// Order side
enum class OrderSide
{
  /// Unknown
  Unknown,
  /// Buy
  Buy,
  /// Sell
  Sell,
};

/// Order type
enum class OrderType
{
  /// Unknown
  Unknown,
  /// Limit Order
  LO,
  /// Enhanced Limit Order
  ELO,
  /// Market Order
  MO,
  /// At-auction Order
  AO,
  /// At-auction Limit Order
  ALO,
  /// Odd Lots
  ODD,
  /// Limit If Touched
  LIT,
  /// Market If Touched
  MIT,
  /// Trailing Limit If Touched (Trailing Amount)
  TSLPAMT,
  /// Trailing Limit If Touched (Trailing Percent)
  TSLPPCT,
  /// Trailing Market If Touched (Trailing Amount)
  TSMAMT,
  /// Trailing Market If Touched (Trailing Percent)
  TSMPCT,
  /// Special Limit Order
  SLO,
};

/// Order tag
enum class OrderTag
{
  /// Unknown
  Unknown,
  /// Normal Order
  Normal,
  /// Long term Order
  LongTerm,
  /// Grey Order
  Grey,
};

/// Time in force Type
enum class TimeInForceType
{
  /// Unknown
  Unknown,
  /// Day Order
  Day,
  /// Good Til Canceled Order
  GoodTilCanceled,
  /// Good Til Date Order
  GoodTilDate,
};

/// Trigger status
enum class TriggerStatus
{
  /// Unknown
  Unknown,
  /// Deactive
  Deactive,
  /// Active
  Active,
  /// Released
  Released,
};

/// Enable or disable outside regular trading hours
enum class OutsideRTH
{
  /// Unknown
  Unknown,
  /// Regular trading hour only
  RTHOnly,
  /// Any time
  AnyTime,
};

/// Order
struct Order
{
  /// Order ID
  std::string order_id;
  /// Order status
  OrderStatus status;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  int64_t quantity;
  /// Executed quantity
  int64_t executed_quantity;
  /// Submitted price
  std::optional<Decimal> price;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Submitted time
  int64_t submitted_at;
  /// Order side
  OrderSide side;
  /// Security code
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Last done
  std::optional<Decimal> last_done;
  /// `LIT` / `MIT` Order Trigger Price
  std::optional<Decimal> trigger_price;
  /// Rejected Message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Time in force type
  TimeInForceType time_in_force;
  /// Long term order expire date
  std::optional<Date> expire_date;
  /// Last updated time
  std::optional<int64_t> updated_at;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// `TSMAMT` / `TSLPAMT` order trailing amount
  std::optional<Decimal> trailing_amount;
  /// `TSMPCT` / `TSLPPCT` order trailing percent
  std::optional<Decimal> trailing_percent;
  /// `TSLPAMT` / `TSLPPCT` order limit offset amount
  std::optional<Decimal> limit_offset;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Currency
  std::string currency;
  /// Enable or disable outside regular trading hours
  std::optional<OutsideRTH> outside_rth;
  /// Remark
  std::string remark;
};

/// Order changed message
struct PushOrderChanged
{
  /// Order side
  OrderSide side;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  int64_t submitted_quantity;
  /// Order symbol
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Submitted price
  Decimal submitted_price;
  /// Executed quantity
  int64_t executed_quantity;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Order ID
  std::string order_id;
  /// Currency
  std::string currency;
  /// Order status
  OrderStatus status;
  /// Submitted time
  int64_t submitted_at;
  /// Last updated time
  int64_t updated_at;
  /// Order trigger price
  std::optional<Decimal> trigger_price;
  /// Rejected message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// Trailing amount
  std::optional<Decimal> trailing_amount;
  /// Trailing percent
  std::optional<Decimal> trailing_percent;
  /// Limit offset amount
  std::optional<Decimal> limit_offset;
  /// Account no
  std::string account_no;
};

/// Options for get history orders request
struct GetHistoryOrdersOptions
{
  /// Security symbol
  std::optional<std::string> symbol;
  /// Order status
  std::optional<std::vector<OrderStatus>> status;
  /// Order side
  std::optional<OrderSide> side;
  /// Market
  std::optional<Market> market;
  /// Start time
  std::optional<int64_t> start_at;
  /// End time
  std::optional<int64_t> end_at;
};

/// Options for get today orders request
struct GetTodayOrdersOptions
{
  /// Security symbol
  std::optional<std::string> symbol;
  /// Order status
  std::optional<std::vector<OrderStatus>> status;
  /// Order side
  std::optional<OrderSide> side;
  /// Market
  std::optional<Market> market;
  /// Order id
  std::optional<std::string> order_id;
};

/// Options for replace order request
struct ReplaceOrderOptions
{
  /// Order ID
  std::string order_id;
  /// Quantity
  int64_t quantity;
  /// Price
  std::optional<Decimal> price;
  /// Trigger price
  std::optional<Decimal> trigger_price;
  /// Limit offset
  std::optional<Decimal> limit_offset;
  /// Trailing amount
  std::optional<Decimal> trailing_amount;
  /// Trailing percent
  std::optional<Decimal> trailing_percent;
  /// Remark
  std::optional<std::string> remark;
};

/// Options for submit order request
struct SubmitOrderOptions
{
  /// Security symbol
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Order side
  OrderSide side;
  /// Submitted price
  int64_t submitted_quantity;
  /// Time in force type
  TimeInForceType time_in_force;
  /// Submitted price
  std::optional<Decimal> submitted_price;
  /// Trigger price (`LIT` / `MIT` Required)
  std::optional<Decimal> trigger_price;
  /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
  std::optional<Decimal> limit_offset;
  /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
  std::optional<Decimal> trailing_amount;
  /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
  std::optional<Decimal> trailing_percent;
  /// Long term order expire date (Required when `time_in_force` is
  /// `GoodTilDate`)
  std::optional<Date> expire_date;
  /// Enable or disable outside regular trading hours
  std::optional<OutsideRTH> outside_rth;
  /// Remark (Maximum 64 characters)
  std::optional<std::string> remark;
};

/// Response for submit order request
struct SubmitOrderResponse
{
  /// Order id
  std::string order_id;
};

/// Cash info
struct CashInfo
{
  /// Withdraw cash
  Decimal withdraw_cash;
  /// Available cash
  Decimal available_cash;
  /// Frozen cash
  Decimal frozen_cash;
  /// Cash to be settled
  Decimal settling_cash;
  /// Currency
  std::string currency;
};

/// Account balance
struct AccountBalance
{
  /// Total cash
  Decimal total_cash;
  /// Maximum financing amount
  Decimal max_finance_amount;
  /// Remaining financing amount
  Decimal remaining_finance_amount;
  /// Risk control level
  int32_t risk_level;
  /// Margin call
  Decimal margin_call;
  /// Currency
  std::string currency;
  /// Cash details
  std::vector<CashInfo> cash_infos;
  /// Net assets
  Decimal net_assets;
  /// Initial margin
  Decimal init_margin;
  /// Maintenance margin
  Decimal maintenance_margin;
};

/// Cash flow direction
enum class CashFlowDirection
{
  /// Unknown
  Unknown,
  /// Out
  Out,
  /// In
  In,
};

/// Balance type
enum class BalanceType
{
  /// Unknown
  Unknown,
  /// Cash
  Cash,
  /// Stock
  Stock,
  /// Fund
  Fund,
};

/// Cash flow
struct CashFlow
{
  /// Cash flow name
  std::string transaction_flow_name;
  /// Outflow direction
  CashFlowDirection direction;
  /// Balance type
  BalanceType business_type;
  /// Cash amount
  Decimal balance;
  /// Cash currency
  std::string currency;
  /// Business time
  int64_t business_time;
  /// Associated Stock code information
  std::optional<std::string> symbol;
  /// Cash flow description
  std::string description;
};

/// Options for submit order request
struct GetCashFlowOptions
{
  /// Start time
  int64_t start_at;
  /// End time
  int64_t end_at;
  /// Business type
  std::optional<BalanceType> business_type;
  /// Security symbol
  std::optional<std::string> symbol;
  /// Page number
  std::optional<uintptr_t> page;
  /// Page size
  std::optional<uintptr_t> size;
};

/// Options for get fund positions request
struct GetFundPositionsOptions
{
  /// Fund symbols
  std::optional<std::vector<std::string>> symbols;
};

/// Options for get stock positions request
struct GetStockPositionsOptions
{
  /// Stock symbols
  std::optional<std::vector<std::string>> symbols;
};

/// Fund position
struct FundPosition
{
  /// Fund ISIN code
  std::string symbol;
  /// Current equity
  Decimal current_net_asset_value;
  /// Current equity time
  int64_t net_asset_value_day;
  /// Fund name
  std::string symbol_name;
  /// Currency
  std::string currency;
  /// Net cost
  Decimal cost_net_asset_value;
  /// Holding units
  Decimal holding_units;
};

/// Fund position channel
struct FundPositionChannel
{
  /// Account type
  std::string account_channel;
  /// Fund positions
  std::vector<FundPosition> positions;
};

/// Fund positions response
struct FundPositionsResponse
{
  /// Channels
  std::vector<FundPositionChannel> channels;
};

/// Stock position
struct StockPosition
{
  /// Stock code
  std::string symbol;
  /// Stock name
  std::string symbol_name;
  /// The number of holdings
  int64_t quantity;
  /// Available quantity
  int64_t available_quantity;
  /// Currency
  std::string currency;
  /// Cost Price(According to the client's choice of average purchase or diluted
  /// cost)
  Decimal cost_price;
  /// Market
  Market market;
};

/// Stock position channel
struct StockPositionChannel
{
  /// Account type
  std::string account_channel;
  /// Stock positions
  std::vector<StockPosition> positions;
};

/// Stock positions response
struct StockPositionsResponse
{
  /// Channels
  std::vector<StockPositionChannel> channels;
};

/// Margin ratio
struct MarginRatio
{
  /// Initial margin ratio
  Decimal im_factor;
  /// Maintain the initial margin ratio
  Decimal mm_factor;
  /// Forced close-out margin ratio
  Decimal fm_factor;
};

} // namespace trade

} // namespace longbridge