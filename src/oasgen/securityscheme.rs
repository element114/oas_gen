use crate::generator::{OAuthFlow, OAuthFlows, SecurityScheme, SecuritySchemeData};
use crate::oasgen::Oas3Builder;
use schemars::Map;

///
/// `http` – Basic, Bearer and other HTTP authentications schemes
/// `apiKey` – API key and cookie authentication
/// `openIdConnect` – `OpenID` Connect Discovery
/// `oauth2` – OAuth 2
/// See <https://swagger.io/docs/specification/authentication/>
impl Oas3Builder {
    /// Create a http bearer Auth security scheme
    #[must_use]
    pub fn create_bearer_scheme() -> SecurityScheme {
        SecurityScheme {
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("JWT".to_owned()),
            },
            description: None,
            extensions: Map::default(),
            schema_type: "http".to_owned(),
        }
    }

    /// Create a http basic Auth security scheme
    #[must_use]
    pub fn create_basic_scheme() -> SecurityScheme {
        SecurityScheme {
            data: SecuritySchemeData::Http {
                scheme: "basic".to_owned(),
                bearer_format: None,
            },
            description: None,
            extensions: Map::default(),
            schema_type: "http".to_owned(),
        }
    }

    /// Create `apikey` Auth security scheme
    #[must_use]
    pub fn create_apikey_scheme(apikey_header: String) -> SecurityScheme {
        SecurityScheme {
            data: SecuritySchemeData::ApiKey {
                location: "header".to_owned(),
                name: apikey_header,
            },
            description: None,
            extensions: Map::default(),
            schema_type: "apiKey".to_owned(),
        }
    }

    /// Create `OpenId` security scheme
    #[must_use]
    pub fn create_openid_scheme(open_id_connect_url: String) -> SecurityScheme {
        SecurityScheme {
            data: SecuritySchemeData::OpenIdConnect {
                open_id_connect_url,
            },
            description: None,
            extensions: Map::default(),
            schema_type: "openIdConnect".to_owned(),
        }
    }

    /// Create `Oauth 2` security scheme
    /// Only supports authorizationCode flow ATM
    #[must_use]
    pub fn create_oauth2_scheme(
        authorization_url: String,
        token_url: String,
        refresh_url: Option<String>,
        scopes: Map<String, String>,
    ) -> SecurityScheme {
        SecurityScheme {
            data: SecuritySchemeData::OAuth2 {
                flows: OAuthFlows {
                    authorization_code: Some(OAuthFlow {
                        authorization_url,
                        token_url,
                        refresh_url,
                        scopes,
                        ..OAuthFlow::default()
                    }),
                    ..OAuthFlows::default()
                },
            },
            description: None,
            extensions: Map::default(),
            schema_type: "openIdConnect".to_owned(),
        }
    }
}
