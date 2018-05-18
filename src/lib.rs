#[macro_use]
extern crate activitystreams_derive;
extern crate activitystreams_traits;
extern crate activitystreams_types;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use activitystreams_traits::{Object, Collection};
use activitystreams_types::collection::OrderedCollection;
use serde::{ser::Serialize, de::DeserializeOwned};

pub trait Endpoint: DeserializeOwned + Serialize {}

#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApObjectProperties {
    #[activitystreams(ab(Object))]
    source: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApActorProperties {
    // TODO: IRI
    #[activitystreams(concrete(String, OrderedCollection), functional)]
    inbox: serde_json::Value,

    // TODO: IRI
    #[activitystreams(concrete(String, OrderedCollection), functional)]
    outbox: serde_json::Value,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Collection), functional)]
    following: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Collection), functional)]
    followers: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Collection), functional)]
    liked: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Collection), functional)]
    shares: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Collection))]
    streams: Option<serde_json::Value>,

    #[activitystreams(ab(Endpoint), functional)]
    endpoints: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApEndpointProperties {
    // TODO: IRI
    #[activitystreams(concrete(String))]
    proxy_url: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String))]
    oauth_authorization_endpoint: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String))]
    oauth_token_endpoint: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String))]
    provide_client_key: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String))]
    sign_client_key: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String))]
    shared_inbox: Option<serde_json::Value>,
}
