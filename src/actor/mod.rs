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

//! Actor traits and types

pub use activitystreams_traits::Actor;
pub use activitystreams_types::actor::kind;

pub mod properties;

use self::{kind::*, properties::*};
use object::{
    properties::{ApObjectProperties, ObjectProperties}, Object,
};

/// Describes a software application.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    #[serde(rename = "type")]
    kind: ApplicationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    pub ap_actor_props: ApActorProperties,
}

impl Object for Application {}
impl Actor for Application {}

/// Represents a formal or informal collective of Actors.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "type")]
    kind: GroupType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    pub ap_actor_props: ApActorProperties,
}

impl Object for Group {}
impl Actor for Group {}

/// Represents an organization.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "type")]
    kind: OrganizationType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    pub ap_actor_props: ApActorProperties,
}

impl Object for Organization {}
impl Actor for Organization {}

/// Represents an individual person.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    pub ap_actor_props: ApActorProperties,
}

impl Object for Person {}
impl Actor for Person {}

/// Represents a service of any kind.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "type")]
    kind: ServiceType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activitypub object properties to this struct
    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    /// Adds all valid activitypub actor properties to this struct
    #[serde(flatten)]
    pub ap_actor_props: ApActorProperties,
}

impl Object for Service {}
impl Actor for Service {}
