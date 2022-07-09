use std::os::raw::c_char;

use longbridge::{
    trade::{
        AccountBalance, BalanceType, CashFlow, CashFlowDirection, CashInfo, Execution,
        FundPosition, FundPositionChannel, FundPositionsResponse, Order, OrderSide, OrderStatus,
        OrderTag, OrderType, PushOrderChanged, StockPosition, StockPositionChannel,
        StockPositionsResponse, SubmitOrderResponse, TimeInForceType,
    },
    Market,
};
use time::OffsetDateTime;

use crate::{
    trade_context::enum_types::{
        CBalanceType, CCashFlowDirection, COrderSide, COrderStatus, COrderTag, COrderType,
        COutsideRTH, CTimeInForceType, CTriggerStatus,
    },
    types::{CDate, CDecimal, CMarket, CString, CVec, ToFFI},
};

/// Order changed message
#[repr(C)]
pub struct CPushOrderChanged {
    /// Order side
    pub side: COrderSide,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub submitted_quantity: i64,
    /// Order symbol
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Submitted price
    pub submitted_price: *const CDecimal,
    /// Executed quantity
    pub executed_quantity: i64,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Order ID
    pub order_id: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Submitted time
    pub submitted_at: i64,
    /// Last updated time
    pub updated_at: i64,
    /// Order trigger price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// Trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// Limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Account no
    pub account_no: *const c_char,
}

pub struct CPushOrderChangedOwned {
    side: OrderSide,
    stock_name: CString,
    submitted_quantity: i64,
    symbol: CString,
    order_type: OrderType,
    submitted_price: CDecimal,
    executed_quantity: i64,
    executed_price: Option<CDecimal>,
    order_id: CString,
    currency: CString,
    status: OrderStatus,
    submitted_at: i64,
    updated_at: i64,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    trigger_status: Option<CTriggerStatus>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    account_no: CString,
}

impl From<PushOrderChanged> for CPushOrderChangedOwned {
    fn from(order_changed: PushOrderChanged) -> Self {
        let PushOrderChanged {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
        } = order_changed;
        CPushOrderChangedOwned {
            side,
            stock_name: stock_name.into(),
            submitted_quantity,
            symbol: symbol.into(),
            order_type,
            submitted_price: submitted_price.into(),
            executed_quantity,
            executed_price: executed_price.map(Into::into),
            order_id: order_id.into(),
            currency: currency.into(),
            status,
            submitted_at: submitted_at.unix_timestamp(),
            updated_at: updated_at.unix_timestamp(),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            trigger_status: trigger_status.map(Into::into),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            account_no: account_no.into(),
        }
    }
}

impl ToFFI for CPushOrderChangedOwned {
    type FFIType = CPushOrderChanged;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPushOrderChangedOwned {
            side,
            stock_name,
            submitted_quantity,
            symbol,
            order_type,
            submitted_price,
            executed_quantity,
            executed_price,
            order_id,
            currency,
            status,
            submitted_at,
            updated_at,
            trigger_price,
            msg,
            tag,
            trigger_status,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            account_no,
        } = self;
        CPushOrderChanged {
            side: (*side).into(),
            stock_name: stock_name.to_ffi_type(),
            submitted_quantity: *submitted_quantity,
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            submitted_price: submitted_price.to_ffi_type(),
            executed_quantity: *executed_quantity,
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            order_id: order_id.to_ffi_type(),
            currency: currency.to_ffi_type(),
            status: (*status).into(),
            submitted_at: *submitted_at,
            updated_at: *updated_at,
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            account_no: account_no.to_ffi_type(),
        }
    }
}

/// Execution
#[repr(C)]
pub struct CExecution {
    /// Order ID
    pub order_id: *const c_char,
    /// Execution ID
    pub trade_id: *const c_char,
    /// Security code
    pub symbol: *const c_char,
    /// Trade done time
    pub trade_done_at: i64,
    /// Executed quantity
    pub quantity: i64,
    /// Executed price
    pub price: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CExecutionOwned {
    order_id: CString,
    trade_id: CString,
    symbol: CString,
    trade_done_at: i64,
    quantity: i64,
    price: CDecimal,
}

impl From<Execution> for CExecutionOwned {
    fn from(execution: Execution) -> Self {
        let Execution {
            order_id,
            trade_id,
            symbol,
            trade_done_at,
            quantity,
            price,
        } = execution;
        CExecutionOwned {
            order_id: order_id.into(),
            trade_id: trade_id.into(),
            symbol: symbol.into(),
            trade_done_at: trade_done_at.unix_timestamp(),
            quantity,
            price: price.into(),
        }
    }
}

impl ToFFI for CExecutionOwned {
    type FFIType = CExecution;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CExecutionOwned {
            order_id,
            trade_id,
            symbol,
            trade_done_at,
            quantity,
            price,
        } = self;
        CExecution {
            order_id: order_id.to_ffi_type(),
            trade_id: trade_id.to_ffi_type(),
            symbol: symbol.to_ffi_type(),
            trade_done_at: *trade_done_at,
            quantity: *quantity,
            price: price.to_ffi_type(),
        }
    }
}

/// Options for get histroy executions request
#[repr(C)]
pub struct CGetHistoryExecutionsOptions {
    /// Start time (can null)
    pub start_at: *const i64,
    /// End time (can null)
    pub end_at: *const i64,
    /// Security code (can null)
    pub symbol: *const c_char,
}

/// Options for get today executions request
#[repr(C)]
pub struct CGetTodayExecutionsOptions {
    /// Security code (can null)
    pub symbol: *const c_char,
    /// Order id (can null)
    pub order_id: *const c_char,
}

/// Order
#[repr(C)]
pub struct COrder {
    /// Order ID
    pub order_id: *const c_char,
    /// Order status
    pub status: COrderStatus,
    /// Stock name
    pub stock_name: *const c_char,
    /// Submitted quantity
    pub quantity: i64,
    /// Executed quantity
    pub executed_quantity: i64,
    /// Submitted price (maybe null)
    pub price: *const CDecimal,
    /// Executed price (maybe null)
    pub executed_price: *const CDecimal,
    /// Submitted time
    pub submitted_at: i64,
    /// Order side
    pub side: COrderSide,
    /// Security code
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Last done (maybe null)
    pub last_done: *const CDecimal,
    /// `LIT` / `MIT` Order Trigger Price (maybe null)
    pub trigger_price: *const CDecimal,
    /// Rejected Message or remark
    pub msg: *const c_char,
    /// Order tag
    pub tag: COrderTag,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Long term order expire date (maybe null)
    pub expire_date: *const CDate,
    /// Last updated time (maybe null)
    pub updated_at: *const i64,
    /// Conditional order trigger time (maybe null)
    pub trigger_at: *const i64,
    /// `TSMAMT` / `TSLPAMT` order trailing amount (maybe null)
    pub trailing_amount: *const CDecimal,
    /// `TSMPCT` / `TSLPPCT` order trailing percent (maybe null)
    pub trailing_percent: *const CDecimal,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount (maybe null)
    pub limit_offset: *const CDecimal,
    /// Conditional order trigger status (maybe null)
    pub trigger_status: *const CTriggerStatus,
    /// Currency
    pub currency: *const c_char,
    /// Enable or disable outside regular trading hours (maybe null)
    pub outside_rth: *const COutsideRTH,
}

#[derive(Debug)]
pub(crate) struct COrderOwned {
    order_id: CString,
    status: OrderStatus,
    stock_name: CString,
    quantity: i64,
    executed_quantity: i64,
    price: Option<CDecimal>,
    executed_price: Option<CDecimal>,
    submitted_at: OffsetDateTime,
    side: OrderSide,
    symbol: CString,
    order_type: OrderType,
    last_done: Option<CDecimal>,
    trigger_price: Option<CDecimal>,
    msg: CString,
    tag: OrderTag,
    time_in_force: TimeInForceType,
    expire_date: Option<CDate>,
    updated_at: Option<i64>,
    trigger_at: Option<i64>,
    trailing_amount: Option<CDecimal>,
    trailing_percent: Option<CDecimal>,
    limit_offset: Option<CDecimal>,
    trigger_status: Option<CTriggerStatus>,
    currency: CString,
    outside_rth: Option<COutsideRTH>,
}

impl From<Order> for COrderOwned {
    fn from(order: Order) -> Self {
        let Order {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
        } = order;
        COrderOwned {
            order_id: order_id.into(),
            status,
            stock_name: stock_name.into(),
            quantity,
            executed_quantity,
            price: price.map(Into::into),
            executed_price: executed_price.map(Into::into),
            submitted_at,
            side,
            symbol: symbol.into(),
            order_type,
            last_done: last_done.map(Into::into),
            trigger_price: trigger_price.map(Into::into),
            msg: msg.into(),
            tag,
            time_in_force,
            expire_date: expire_date.map(Into::into),
            updated_at: updated_at.map(OffsetDateTime::unix_timestamp),
            trigger_at: trigger_at.map(OffsetDateTime::unix_timestamp),
            trailing_amount: trailing_amount.map(Into::into),
            trailing_percent: trailing_percent.map(Into::into),
            limit_offset: limit_offset.map(Into::into),
            trigger_status: trigger_status.map(Into::into),
            currency: currency.into(),
            outside_rth: outside_rth.map(Into::into),
        }
    }
}

impl ToFFI for COrderOwned {
    type FFIType = COrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        let COrderOwned {
            order_id,
            status,
            stock_name,
            quantity,
            executed_quantity,
            price,
            executed_price,
            submitted_at,
            side,
            symbol,
            order_type,
            last_done,
            trigger_price,
            msg,
            tag,
            time_in_force,
            expire_date,
            updated_at,
            trigger_at,
            trailing_amount,
            trailing_percent,
            limit_offset,
            trigger_status,
            currency,
            outside_rth,
        } = self;
        COrder {
            order_id: order_id.to_ffi_type(),
            status: (*status).into(),
            stock_name: stock_name.to_ffi_type(),
            quantity: *quantity,
            executed_quantity: *executed_quantity,
            price: price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            executed_price: executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            submitted_at: submitted_at.unix_timestamp(),
            side: (*side).into(),
            symbol: symbol.to_ffi_type(),
            order_type: (*order_type).into(),
            last_done: last_done
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_price: trigger_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            msg: msg.to_ffi_type(),
            tag: (*tag).into(),
            time_in_force: (*time_in_force).into(),
            expire_date: expire_date
                .as_ref()
                .map(|value| value as *const CDate)
                .unwrap_or(std::ptr::null()),
            updated_at: updated_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trigger_at: trigger_at
                .as_ref()
                .map(|value| value as *const i64)
                .unwrap_or(std::ptr::null()),
            trailing_amount: trailing_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trailing_percent: trailing_percent
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            limit_offset: limit_offset
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            trigger_status: trigger_status
                .as_ref()
                .map(|value| value as *const CTriggerStatus)
                .unwrap_or(std::ptr::null()),
            currency: currency.to_ffi_type(),
            outside_rth: outside_rth
                .as_ref()
                .map(|value| value as *const COutsideRTH)
                .unwrap_or(std::ptr::null()),
        }
    }
}

/// Options for get history orders request
#[derive(Debug)]
#[repr(C)]
pub struct CGetHistoryOrdersOptions {
    /// Security symbol (can null)
    pub symbol: *const c_char,
    /// Order status (can null)
    pub status: *const COrderStatus,
    /// Number of order status
    pub num_status: usize,
    /// Order side (can null)
    pub side: *const COrderSide,
    /// Market (can null)
    pub market: *const CMarket,
    /// Start time (can null)
    pub start_at: *const i64,
    /// End time (can null)
    pub end_at: *const i64,
}

/// Options for get today orders request
#[derive(Debug)]
#[repr(C)]
pub struct CGetTodayOrdersOptions {
    /// Security symbol (can null)
    pub symbol: *const c_char,
    /// Order status (can null)
    pub status: *const COrderStatus,
    /// Number of order status
    pub num_status: usize,
    /// Order side (can null)
    pub side: *const COrderSide,
    /// Market (can null)
    pub market: *const CMarket,
    /// Order id (can null)
    pub order_id: *const c_char,
}

/// Options for replace order request
#[derive(Debug)]
#[repr(C)]
pub struct CReplaceOrderOptions {
    /// Order ID
    pub order_id: *const c_char,
    /// Quantity
    pub quantity: i64,
    /// Price (can null)
    pub price: *const CDecimal,
    /// Trigger price (can null)
    pub trigger_price: *const CDecimal,
    /// Limit offset (can null)
    pub limit_offset: *const CDecimal,
    /// Trailing amount (can null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (can null)
    pub trailing_percent: *const CDecimal,
    /// Remark (can null)
    pub remark: *const c_char,
}

/// Options for submit order request
#[derive(Debug)]
#[repr(C)]
pub struct CSubmitOrderOptions {
    /// Security symbol
    pub symbol: *const c_char,
    /// Order type
    pub order_type: COrderType,
    /// Order side
    pub side: COrderSide,
    /// Submitted price
    pub submitted_quantity: i64,
    /// Time in force type
    pub time_in_force: CTimeInForceType,
    /// Submitted price (can null)
    pub submitted_price: *const CDecimal,
    /// Trigger price (`LIT` / `MIT` Required) (can null)
    pub trigger_price: *const CDecimal,
    /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required) (can null)
    pub limit_offset: *const CDecimal,
    /// Trailing amount (`TSLPAMT` / `TSMAMT` Required) (can null)
    pub trailing_amount: *const CDecimal,
    /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required) (can null)
    pub trailing_percent: *const CDecimal,
    /// Long term order expire date (Required when `time_in_force` is
    /// `GoodTilDate`) (can null)
    pub expire_date: *const CDate,
    /// Enable or disable outside regular trading hours (can null)
    pub outside_rth: *const COutsideRTH,
    /// Remark (Maximum 64 characters) (can null)
    pub remark: *const c_char,
}

#[repr(C)]
pub struct CSubmitOrderResponse {
    pub order_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSubmitOrderResponseOwned {
    order_id: CString,
}

impl From<SubmitOrderResponse> for CSubmitOrderResponseOwned {
    fn from(resp: SubmitOrderResponse) -> Self {
        CSubmitOrderResponseOwned {
            order_id: resp.order_id.into(),
        }
    }
}

impl ToFFI for CSubmitOrderResponseOwned {
    type FFIType = CSubmitOrderResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CSubmitOrderResponse {
            order_id: self.order_id.to_ffi_type(),
        }
    }
}

/// Account balance
#[repr(C)]
pub struct CCashInfo {
    /// Withdraw cash
    pub withdraw_cash: *const CDecimal,
    /// Available cash
    pub available_cash: *const CDecimal,
    /// Frozen cash
    pub frozen_cash: *const CDecimal,
    /// Cash to be settled
    pub settling_cash: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCashInfoOwned {
    withdraw_cash: CDecimal,
    available_cash: CDecimal,
    frozen_cash: CDecimal,
    settling_cash: CDecimal,
    currency: CString,
}

impl From<CashInfo> for CCashInfoOwned {
    fn from(info: CashInfo) -> Self {
        let CashInfo {
            withdraw_cash,
            available_cash,
            frozen_cash,
            settling_cash,
            currency,
        } = info;
        Self {
            withdraw_cash: withdraw_cash.into(),
            available_cash: available_cash.into(),
            frozen_cash: frozen_cash.into(),
            settling_cash: settling_cash.into(),
            currency: currency.into(),
        }
    }
}

impl ToFFI for CCashInfoOwned {
    type FFIType = CCashInfo;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCashInfoOwned {
            withdraw_cash,
            available_cash,
            frozen_cash,
            settling_cash,
            currency,
        } = self;
        CCashInfo {
            withdraw_cash: withdraw_cash.to_ffi_type(),
            available_cash: available_cash.to_ffi_type(),
            frozen_cash: frozen_cash.to_ffi_type(),
            settling_cash: settling_cash.to_ffi_type(),
            currency: currency.to_ffi_type(),
        }
    }
}

/// Account balance
#[repr(C)]
pub struct CAccountBalance {
    /// Total cash
    pub total_cash: *const CDecimal,
    /// Maximum financing amount
    pub max_finance_amount: *const CDecimal,
    /// Remaining financing amount
    pub remaining_finance_amount: *const CDecimal,
    /// Risk control level
    pub risk_level: i32,
    /// Margin call
    pub margin_call: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Cash details
    pub cash_infos: *const CCashInfo,
    /// Number of cash details
    pub num_cash_infos: usize,
    /// Net assets
    pub net_assets: *const CDecimal,
    /// Initial margin
    pub init_margin: *const CDecimal,
    /// Maintenance margin
    pub maintenance_margin: *const CDecimal,
}

#[derive(Debug)]
pub(crate) struct CAccountBalanceOwned {
    total_cash: CDecimal,
    max_finance_amount: CDecimal,
    remaining_finance_amount: CDecimal,
    risk_level: i32,
    margin_call: CDecimal,
    currency: CString,
    cash_infos: CVec<CCashInfoOwned>,
    net_assets: CDecimal,
    init_margin: CDecimal,
    maintenance_margin: CDecimal,
}

impl From<AccountBalance> for CAccountBalanceOwned {
    fn from(info: AccountBalance) -> Self {
        let AccountBalance {
            total_cash,
            max_finance_amount,
            remaining_finance_amount,
            risk_level,
            margin_call,
            currency,
            cash_infos,
            net_assets,
            init_margin,
            maintenance_margin,
        } = info;
        Self {
            total_cash: total_cash.into(),
            max_finance_amount: max_finance_amount.into(),
            remaining_finance_amount: remaining_finance_amount.into(),
            risk_level,
            margin_call: margin_call.into(),
            currency: currency.into(),
            cash_infos: cash_infos.into(),
            net_assets: net_assets.into(),
            init_margin: init_margin.into(),
            maintenance_margin: maintenance_margin.into(),
        }
    }
}

impl ToFFI for CAccountBalanceOwned {
    type FFIType = CAccountBalance;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAccountBalanceOwned {
            total_cash,
            max_finance_amount,
            remaining_finance_amount,
            risk_level,
            margin_call,
            currency,
            cash_infos,
            net_assets,
            init_margin,
            maintenance_margin,
        } = self;
        CAccountBalance {
            total_cash: total_cash.to_ffi_type(),
            max_finance_amount: max_finance_amount.to_ffi_type(),
            remaining_finance_amount: remaining_finance_amount.to_ffi_type(),
            risk_level: *risk_level,
            margin_call: margin_call.to_ffi_type(),
            currency: currency.to_ffi_type(),
            cash_infos: cash_infos.to_ffi_type(),
            num_cash_infos: cash_infos.len(),
            net_assets: net_assets.to_ffi_type(),
            init_margin: init_margin.to_ffi_type(),
            maintenance_margin: maintenance_margin.to_ffi_type(),
        }
    }
}

/// Cash flow
#[repr(C)]
pub struct CCashFlow {
    /// Cash flow name
    pub transaction_flow_name: *const c_char,
    /// Outflow direction
    pub direction: CCashFlowDirection,
    /// Balance type
    pub business_type: CBalanceType,
    /// Cash amount
    pub balance: *const CDecimal,
    /// Cash currency
    pub currency: *const c_char,
    /// Business time
    pub business_time: i64,
    /// Associated Stock code information (maybe null)
    pub symbol: *const c_char,
    /// Cash flow description
    pub description: *const c_char,
}

/// Cash flow
#[repr(C)]
pub(crate) struct CCashFlowOwned {
    transaction_flow_name: CString,
    direction: CashFlowDirection,
    business_type: BalanceType,
    balance: CDecimal,
    currency: CString,
    business_time: i64,
    symbol: Option<CString>,
    description: CString,
}

impl From<CashFlow> for CCashFlowOwned {
    fn from(cash_flow: CashFlow) -> Self {
        let CashFlow {
            transaction_flow_name,
            direction,
            business_type,
            balance,
            currency,
            business_time,
            symbol,
            description,
        } = cash_flow;
        CCashFlowOwned {
            transaction_flow_name: transaction_flow_name.into(),
            direction,
            business_type,
            balance: balance.into(),
            currency: currency.into(),
            business_time: business_time.unix_timestamp(),
            symbol: symbol.map(Into::into),
            description: description.into(),
        }
    }
}

impl ToFFI for CCashFlowOwned {
    type FFIType = CCashFlow;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CCashFlowOwned {
            transaction_flow_name,
            direction,
            business_type,
            balance,
            currency,
            business_time,
            symbol,
            description,
        } = self;
        CCashFlow {
            transaction_flow_name: transaction_flow_name.to_ffi_type(),
            direction: (*direction).into(),
            business_type: (*business_type).into(),
            balance: balance.to_ffi_type(),
            currency: currency.to_ffi_type(),
            business_time: *business_time,
            symbol: match symbol {
                Some(symbol) => symbol.to_ffi_type(),
                None => std::ptr::null(),
            },
            description: description.to_ffi_type(),
        }
    }
}

/// Options for get cash flow request
#[repr(C)]
pub struct CGetCashFlowOptions {
    /// Start time
    pub start_at: i64,
    /// End time
    pub end_at: i64,
    /// Business type (can null)
    pub business_type: *const CBalanceType,
    /// Security symbol
    pub symbol: *const c_char,
    /// Page number
    pub page: *const usize,
    /// Page size
    pub size: *const usize,
}

/// Options for get fund positions request
#[repr(C)]
pub struct CGetFundPositionsOptions {
    /// Fund symbols (can null)
    pub symbols: *const *const c_char,
    /// Number of fund symbols
    pub num_symbols: usize,
}

/// Fund positions response
#[repr(C)]
pub struct CFundPositionsResponse {
    pub channels: *const CFundPositionChannel,
    pub num_channels: usize,
}

pub(crate) struct CFundPositionsResponseOwned {
    pub channels: CVec<CFundPositionChannelOwned>,
}

impl From<FundPositionsResponse> for CFundPositionsResponseOwned {
    fn from(resp: FundPositionsResponse) -> Self {
        let FundPositionsResponse { channels } = resp;
        Self {
            channels: channels.into(),
        }
    }
}

impl ToFFI for CFundPositionsResponseOwned {
    type FFIType = CFundPositionsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionsResponseOwned { channels } = self;
        CFundPositionsResponse {
            channels: channels.to_ffi_type(),
            num_channels: channels.len(),
        }
    }
}

/// Fund position channel
#[repr(C)]
pub struct CFundPositionChannel {
    /// Account type
    pub account_channel: *const c_char,
    /// Fund positions
    pub positions: *const CFundPosition,
    /// Number of fund positions
    pub num_positions: usize,
}

pub(crate) struct CFundPositionChannelOwned {
    account_channel: CString,
    positions: CVec<CFundPositionOwned>,
}

impl From<FundPositionChannel> for CFundPositionChannelOwned {
    fn from(channel: FundPositionChannel) -> Self {
        let FundPositionChannel {
            account_channel,
            positions,
        } = channel;
        CFundPositionChannelOwned {
            account_channel: account_channel.into(),
            positions: positions.into(),
        }
    }
}

impl ToFFI for CFundPositionChannelOwned {
    type FFIType = CFundPositionChannel;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionChannelOwned {
            account_channel,
            positions,
        } = self;
        CFundPositionChannel {
            account_channel: account_channel.to_ffi_type(),
            positions: positions.to_ffi_type(),
            num_positions: positions.len(),
        }
    }
}

/// Fund position
#[repr(C)]
pub struct CFundPosition {
    /// Fund ISIN code
    pub symbol: *const c_char,
    /// Current equity
    pub current_net_asset_value: *const CDecimal,
    /// Current equity time
    pub net_asset_value_day: i64,
    /// Fund name
    pub symbol_name: *const c_char,
    /// Currency
    pub currency: *const c_char,
    /// Net cost
    pub cost_net_asset_value: *const CDecimal,
    /// Holding units
    pub holding_units: *const CDecimal,
}

pub(crate) struct CFundPositionOwned {
    symbol: CString,
    current_net_asset_value: CDecimal,
    net_asset_value_day: i64,
    symbol_name: CString,
    currency: CString,
    cost_net_asset_value: CDecimal,
    holding_units: CDecimal,
}

impl From<FundPosition> for CFundPositionOwned {
    fn from(position: FundPosition) -> Self {
        let FundPosition {
            symbol,
            current_net_asset_value,
            net_asset_value_day,
            symbol_name,
            currency,
            cost_net_asset_value,
            holding_units,
        } = position;
        Self {
            symbol: symbol.into(),
            current_net_asset_value: current_net_asset_value.into(),
            net_asset_value_day: net_asset_value_day.unix_timestamp(),
            symbol_name: symbol_name.into(),
            currency: currency.into(),
            cost_net_asset_value: cost_net_asset_value.into(),
            holding_units: holding_units.into(),
        }
    }
}

impl ToFFI for CFundPositionOwned {
    type FFIType = CFundPosition;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CFundPositionOwned {
            symbol,
            current_net_asset_value,
            net_asset_value_day,
            symbol_name,
            currency,
            cost_net_asset_value,
            holding_units,
        } = self;
        CFundPosition {
            symbol: symbol.to_ffi_type(),
            current_net_asset_value: current_net_asset_value.to_ffi_type(),
            net_asset_value_day: *net_asset_value_day,
            symbol_name: symbol_name.to_ffi_type(),
            currency: currency.to_ffi_type(),
            cost_net_asset_value: cost_net_asset_value.to_ffi_type(),
            holding_units: holding_units.to_ffi_type(),
        }
    }
}

/// Stock position
#[repr(C)]
pub struct CStockPosition {
    /// Stock code
    pub symbol: *const c_char,
    /// Stock name
    pub symbol_name: *const c_char,
    /// The number of holdings
    pub quantity: i64,
    /// Available quantity
    pub available_quantity: i64,
    /// Currency
    pub currency: *const c_char,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    pub cost_price: *const CDecimal,
    /// Market
    pub market: CMarket,
}

pub(crate) struct CStockPositionOwned {
    /// Stock code
    symbol: CString,
    /// Stock name
    symbol_name: CString,
    /// The number of holdings
    quantity: i64,
    /// Available quantity
    available_quantity: i64,
    /// Currency
    currency: CString,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    cost_price: CDecimal,
    /// Market
    market: Market,
}

impl From<StockPosition> for CStockPositionOwned {
    fn from(position: StockPosition) -> Self {
        let StockPosition {
            symbol,
            symbol_name,
            quantity,
            available_quantity,
            currency,
            cost_price,
            market,
        } = position;
        Self {
            symbol: symbol.into(),
            symbol_name: symbol_name.into(),
            quantity,
            available_quantity,
            currency: currency.into(),
            cost_price: cost_price.into(),
            market,
        }
    }
}

impl ToFFI for CStockPositionOwned {
    type FFIType = CStockPosition;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionOwned {
            symbol,
            symbol_name,
            quantity,
            available_quantity,
            currency,
            cost_price,
            market,
        } = self;
        CStockPosition {
            symbol: symbol.to_ffi_type(),
            symbol_name: symbol_name.to_ffi_type(),
            quantity: *quantity,
            available_quantity: *available_quantity,
            currency: currency.to_ffi_type(),
            cost_price: cost_price.to_ffi_type(),
            market: (*market).into(),
        }
    }
}

/// Stock position channel
#[repr(C)]
pub struct CStockPositionChannel {
    /// Account type
    pub account_channel: *const c_char,
    /// Stock positions
    pub positions: *const CStockPosition,
    /// Number of stock positions
    pub num_positions: usize,
}

pub(crate) struct CStockPositionChannelOwned {
    account_channel: CString,
    positions: CVec<CStockPositionOwned>,
}

impl From<StockPositionChannel> for CStockPositionChannelOwned {
    fn from(channel: StockPositionChannel) -> Self {
        let StockPositionChannel {
            account_channel,
            positions,
        } = channel;
        Self {
            account_channel: account_channel.into(),
            positions: positions.into(),
        }
    }
}

impl ToFFI for CStockPositionChannelOwned {
    type FFIType = CStockPositionChannel;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionChannelOwned {
            account_channel,
            positions,
        } = self;
        CStockPositionChannel {
            account_channel: account_channel.to_ffi_type(),
            positions: positions.to_ffi_type(),
            num_positions: positions.len(),
        }
    }
}

/// Stock positions response
#[repr(C)]
pub struct CStockPositionsResponse {
    pub channels: *const CStockPositionChannel,
    pub num_channels: usize,
}

pub(crate) struct CStockPositionsResponseOwned {
    channels: CVec<CStockPositionChannelOwned>,
}

impl From<StockPositionsResponse> for CStockPositionsResponseOwned {
    fn from(resp: StockPositionsResponse) -> Self {
        let StockPositionsResponse { channels } = resp;
        CStockPositionsResponseOwned {
            channels: channels.into(),
        }
    }
}

impl ToFFI for CStockPositionsResponseOwned {
    type FFIType = CStockPositionsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CStockPositionsResponseOwned { channels } = self;
        CStockPositionsResponse {
            channels: channels.to_ffi_type(),
            num_channels: channels.len(),
        }
    }
}

/// Options for get stock positions request
#[repr(C)]
pub struct CGetStockPositionsOptions {
    /// Fund symbols (can null)
    pub symbols: *const *const c_char,
    /// Number of fund symbols
    pub num_symbols: usize,
}
