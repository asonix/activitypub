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

#[macro_use]
extern crate activitystreams_derive;
extern crate activitystreams_traits;
extern crate activitystreams_types;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod activity;
pub mod actor;
pub mod collection;
mod endpoint;
pub mod link;
pub mod object;

pub use self::{
    activity::{Activity, IntransitiveActivity}, actor::Actor,
    collection::{Collection, CollectionPage}, endpoint::Endpoint, link::Link, object::Object,
};
pub use activitystreams_traits::{properties, Error, Result};
pub use activitystreams_types::{context, ContextObject, CustomLink, CustomObject};
