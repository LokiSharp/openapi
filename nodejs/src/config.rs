use napi::Result;

use crate::error::ErrorNewType;

/// Configuration parameters
#[napi_derive::napi(object)]
pub struct ConfigParams {
    /// App Key
    pub app_key: String,
    /// App Secret
    pub app_secret: String,
    /// Access Token
    pub access_token: String,
    /// HTTP API url (default: "https://openapi.longbridgeapp.com")
    pub http_url: Option<String>,
    /// Websocket url for quote API (default:
    /// "wss://openapi-quote.longbridgeapp.com")
    pub quote_ws_url: Option<String>,
    /// Websocket url for trade API (default:
    /// "wss://openapi-trade.longbridgeapp.com")
    pub trade_ws_url: Option<String>,
}

/// Configuration for Longbridge sdk
#[napi_derive::napi]
pub struct Config(pub(crate) longbridge::Config);

#[napi_derive::napi]
impl Config {
    /// Create a new `Config`
    #[napi(constructor)]
    pub fn new(params: ConfigParams) -> Self {
        let mut config =
            longbridge::Config::new(params.app_key, params.app_secret, params.access_token);

        if let Some(http_url) = params.http_url {
            config = config.http_url(http_url);
        }

        if let Some(quote_ws_url) = params.quote_ws_url {
            config = config.quote_ws_url(quote_ws_url);
        }

        if let Some(trade_ws_url) = params.trade_ws_url {
            config = config.trade_ws_url(trade_ws_url);
        }

        Self(config)
    }

    /// Create a new `Config` from the given environment variables
    ///
    /// It first gets the environment variables from the `.env` file in the
    /// current directory.
    ///
    /// # Variables
    ///
    /// - `LONGBRIDGE_APP_KEY` - App key
    /// - `LONGBRIDGE_APP_SECRET` - App secret
    /// - `LONGBRIDGE_ACCESS_TOKEN` - Access token
    /// - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url
    /// - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url
    /// - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url
    #[napi(factory)]
    pub fn from_env() -> Result<Self> {
        Ok(Self(longbridge::Config::from_env().map_err(ErrorNewType)?))
    }
}
