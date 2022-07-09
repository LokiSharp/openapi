use longbridge_c_macros::CEnum;

/// Topic type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TopicType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTopicType {
    /// Trading
    #[c(remote = "Private")]
    TopicPrivate,
}

/// Order side
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderSide")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderSide {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderSideUnknown,
    /// Unknown
    #[c(remote = "Buy")]
    OrderSideBuy,
    /// Unknown
    #[c(remote = "Sell")]
    OrderSideSell,
}

/// Order type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderType {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTypeUnknown,
    /// Limit Order
    #[c(remote = "LO")]
    OrderTypeLO,
    /// Enhanced Limit Order
    #[c(remote = "ELO")]
    OrderTypeELO,
    /// Market Order
    #[c(remote = "MO")]
    OrderTypeMO,
    /// At-auction Order
    #[c(remote = "AO")]
    OrderTypeAO,
    /// At-auction Limit Order
    #[c(remote = "ALO")]
    OrderTypeALO,
    /// Odd Lots
    #[c(remote = "ODD")]
    OrderTypeODD,
    /// Limit If Touched
    #[c(remote = "LIT")]
    OrderTypeLIT,
    /// Market If Touched
    #[c(remote = "MIT")]
    OrderTypeMIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[c(remote = "TSLPAMT")]
    OrderTypeTSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[c(remote = "TSLPPCT")]
    OrderTypeTSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[c(remote = "TSMAMT")]
    OrderTypeTSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[c(remote = "TSMPCT")]
    OrderTypeTSMPCT,
}

/// Order status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderStatusUnknown,
    /// Not reported
    #[c(remote = "NotReported")]
    OrderStatusNotReported,
    /// Not reported (Replaced Order)
    #[c(remote = "ReplacedNotReported")]
    OrderStatusReplacedNotReported,
    /// Not reported (Protected Order)
    #[c(remote = "ProtectedNotReported")]
    OrderStatusProtectedNotReported,
    /// Not reported (Conditional Order)
    #[c(remote = "VarietiesNotReported")]
    OrderStatusVarietiesNotReported,
    /// Filled
    #[c(remote = "Filled")]
    OrderStatusFilled,
    /// Wait To New
    #[c(remote = "WaitToNew")]
    OrderStatusWaitToNew,
    /// New
    #[c(remote = "New")]
    OrderStatusNew,
    /// Wait To Replace
    #[c(remote = "WaitToReplace")]
    OrderStatusWaitToReplace,
    /// Pending Replace
    #[c(remote = "PendingReplace")]
    OrderStatusPendingReplace,
    /// Replaced
    #[c(remote = "Replaced")]
    OrderStatusReplaced,
    /// Partial Filled
    #[c(remote = "PartialFilled")]
    OrderStatusPartialFilled,
    /// Wait To Cancel
    #[c(remote = "WaitToCancel")]
    OrderStatusWaitToCancel,
    /// Pending Cancel
    #[c(remote = "PendingCancel")]
    OrderStatusPendingCancel,
    /// Rejected
    #[c(remote = "Rejected")]
    OrderStatusRejected,
    /// Canceled
    #[c(remote = "Canceled")]
    OrderStatusCanceled,
    /// Expired
    #[c(remote = "Expired")]
    OrderStatusExpired,
    /// Partial Withdrawal
    #[c(remote = "PartialWithdrawal")]
    OrderStatusPartialWithdrawal,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderTag")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderTag {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTagUnknown,
    /// Normal Order
    #[c(remote = "Normal")]
    OrderTagNormal,
    /// Long term Order
    #[c(remote = "LongTerm")]
    OrderTagLongTerm,
    /// Grey Order
    #[c(remote = "Grey")]
    OrderTagGrey,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TriggerStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTriggerStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    TriggerStatusUnknown,
    /// Deactive
    #[c(remote = "Deactive")]
    TriggerStatusDeactive,
    /// Active
    #[c(remote = "Active")]
    TriggerStatusActive,
    /// Released
    #[c(remote = "Released")]
    TriggerStatusReleased,
}

/// Enable or disable outside regular trading hours
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OutsideRTH")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COutsideRTH {
    /// Unknown
    #[c(remote = "Unknown")]
    OutsideRTHUnknown,
    /// Regular trading hour only
    #[c(remote = "RTHOnly")]
    OutsideRTHOnly,
    /// Any time
    #[c(remote = "AnyTime")]
    OutsideRTHAnyTime,
}

/// Time in force Type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TimeInForceType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTimeInForceType {
    /// Unknown
    #[c(remote = "Unknown")]
    TimeInForceUnknown,
    /// Day Order
    #[c(remote = "Day")]
    TimeInForceDay,
    /// Good Til Canceled Order
    #[c(remote = "GoodTilCanceled")]
    TimeInForceGoodTilCanceled,
    /// Good Til Date Order
    #[c(remote = "GoodTilDate")]
    TimeInForceGoodTilDate,
}

/// Cash flow direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::CashFlowDirection ")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCashFlowDirection {
    /// Unknown
    #[c(remote = "Unknown")]
    CashFlowDirectionUnknown,
    /// Out
    #[c(remote = "Out")]
    CashFlowDirectionOutside,
    /// In
    #[c(remote = "In")]
    CashFlowDirectionIn,
}

/// Balance type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::BalanceType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBalanceType {
    /// Unknown
    #[c(remote = "Unknown")]
    BalanceTypeUnknown,
    /// Unknown
    #[c(remote = "Cash")]
    BalanceTypeCash,
    /// Unknown
    #[c(remote = "Stock")]
    BalanceTypeStock,
    /// Unknown
    #[c(remote = "Fund")]
    BalanceTypeFund,
}
