/*
 * This file is part of ActivityPub.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityPub is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityPub is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityPub.  If not, see <http://www.gnu.org/licenses/>.
 */

use serde_json;

use endpoint::Endpoint;
use link::Link;

#[derive(Clone, Debug, Default, Deserialize, Properties, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApActorProperties {
    // TODO: IRI
    #[activitystreams(ab(Link), concrete(String), functional)]
    inbox: serde_json::Value,

    // TODO: IRI
    #[activitystreams(ab(Link), concrete(String), functional)]
    outbox: serde_json::Value,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Link), functional)]
    following: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Link), functional)]
    followers: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Link), functional)]
    liked: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Link), functional)]
    shares: Option<serde_json::Value>,

    // TODO: IRI
    #[activitystreams(concrete(String), ab(Link))]
    streams: Option<serde_json::Value>,

    #[activitystreams(ab(Endpoint), functional)]
    endpoints: Option<serde_json::Value>,
}
