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

//! Activity traits and types

pub use activitystreams_traits::{Activity, IntransitiveActivity};
pub use activitystreams_types::activity::{kind, properties};

use self::{kind::*, properties::*};
use object::{
    properties::{ApObjectProperties, ObjectProperties}, Object,
};

/// Indicates that the actor accepts the object.
///
/// The target property can be used in certain circumstances to indicate the context into which the
/// object has been accepted.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
    #[serde(rename = "type")]
    kind: AcceptType,

    #[serde(flatten)]
    pub accept_props: AcceptProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Accept {}
impl Activity for Accept {}

/// Indicates that the actor has added the object to the target.
///
/// If the target property is not explicitly specified, the target would need to be determined
/// implicitly by context. The origin can be used to identify the context from which the object
/// originated.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Add {
    #[serde(rename = "type")]
    kind: AddType,

    #[serde(flatten)]
    pub add_props: AddProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Add {}
impl Activity for Add {}

/// Indicates that the actor has moved object from origin to target.
///
/// If the origin or target are not specified, either can be determined by context.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AMove {
    #[serde(rename = "type")]
    kind: MoveType,

    #[serde(flatten)]
    pub move_props: MoveProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for AMove {}
impl Activity for AMove {}

/// Indicates that the actor is calling the target's attention the object.
///
/// The origin typically has no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Announce {
    #[serde(rename = "type")]
    kind: AnnounceType,

    #[serde(flatten)]
    pub announce_props: AnnounceProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Announce {}
impl Activity for Announce {}

/// An IntransitiveActivity that indicates that the actor has arrived at the location.
///
/// The origin can be used to identify the context from which the actor originated. The target
/// typically has no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrive {
    #[serde(rename = "type")]
    kind: ArriveType,

    #[serde(flatten)]
    pub arrive_props: ArriveProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Arrive {}
impl Activity for Arrive {}
impl IntransitiveActivity for Arrive {}

/// Indicates that the actor is blocking the object.
///
/// Blocking is a stronger form of Ignore. The typical use is to support social systems that allow
/// one user to block activities or content of other users. The target and origin typically have no
/// defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    kind: BlockType,

    #[serde(flatten)]
    pub block_props: BlockProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Block {}
impl Activity for Block {}

/// Indicates that the actor has created the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    #[serde(rename = "type")]
    kind: CreateType,

    #[serde(flatten)]
    pub create_props: CreateProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Create {}
impl Activity for Create {}

/// Indicates that the actor has deleted the object.
///
/// If specified, the origin indicates the context from which the object was deleted.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    #[serde(rename = "type")]
    kind: DeleteType,

    #[serde(flatten)]
    pub delete_props: DeleteProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Delete {}
impl Activity for Delete {}

/// Indicates that the actor dislikes the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dislike {
    #[serde(rename = "type")]
    kind: DislikeType,

    #[serde(flatten)]
    pub dislike_props: DislikeProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Dislike {}
impl Activity for Dislike {}

/// Indicates that the actor is "flagging" the object.
///
/// Flagging is defined in the sense common to many social platforms as reporting content as being
/// inappropriate for any number of reasons.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    #[serde(rename = "type")]
    kind: FlagType,

    #[serde(flatten)]
    pub flag_props: FlagProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Flag {}
impl Activity for Flag {}

/// Indicates that the actor is "following" the object.
///
/// Following is defined in the sense typically used within Social systems in which the actor is
/// interested in any activity performed by or on the object. The target and origin typically have
/// no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    #[serde(rename = "type")]
    kind: FollowType,

    #[serde(flatten)]
    pub follow_props: FollowProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Follow {}
impl Activity for Follow {}

/// Indicates that the actor is ignoring the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ignore {
    #[serde(rename = "type")]
    kind: IgnoreType,

    #[serde(flatten)]
    pub ignore_props: IgnoreProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Ignore {}
impl Activity for Ignore {}

/// A specialization of Offer in which the actor is extending an invitation for the object to the
/// target.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    #[serde(rename = "type")]
    kind: InviteType,

    #[serde(flatten)]
    pub invite_props: InviteProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Invite {}
impl Activity for Invite {}

/// Indicates that the actor has joined the object.
///
/// The target and origin typically have no defined meaning
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Join {
    #[serde(rename = "type")]
    kind: JoinType,

    #[serde(flatten)]
    pub join_props: JoinProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Join {}
impl Activity for Join {}

/// Indicates that the actor has left the object.
///
/// The target and origin typically have no meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Leave {
    #[serde(rename = "type")]
    kind: LeaveType,

    #[serde(flatten)]
    pub leave_props: LeaveProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Leave {}
impl Activity for Leave {}

/// Indicates that the actor likes, recommends or endorses the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    #[serde(rename = "type")]
    kind: LikeType,

    #[serde(flatten)]
    pub like_props: LikeProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Like {}
impl Activity for Like {}

/// Indicates that the actor has listened to the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Listen {
    #[serde(rename = "type")]
    kind: ListenType,

    #[serde(flatten)]
    pub listen_props: ListenProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Listen {}
impl Activity for Listen {}

/// Indicates that the actor is offering the object.
///
/// If specified, the target indicates the entity to which the object is being offered.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    #[serde(rename = "type")]
    kind: OfferType,

    #[serde(flatten)]
    pub offer_props: OfferProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Offer {}
impl Activity for Offer {}

/// Represents a question being asked.
///
/// Question objects are an extension of IntransitiveActivity. That is, the Question object is an
/// Activity, but the direct object is the question itself and therefore it would not contain an
/// object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers, but a
/// Question object MUST NOT have both properties.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "type")]
    kind: QuestionType,

    #[serde(flatten)]
    pub question_props: QuestionProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Question {}
impl Activity for Question {}
impl IntransitiveActivity for Question {}

/// Indicates that the actor has read the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    #[serde(rename = "type")]
    kind: ReadType,

    #[serde(flatten)]
    pub read_props: ReadProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Read {}
impl Activity for Read {}

/// Indicates that the actor is rejecting the object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reject {
    #[serde(rename = "type")]
    kind: RejectType,

    #[serde(flatten)]
    pub reject_props: RejectProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Reject {}
impl Activity for Reject {}

/// Indicates that the actor is removing the object.
///
/// If specified, the origin indicates the context from which the object is being removed.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Remove {
    #[serde(rename = "type")]
    kind: RemoveType,

    #[serde(flatten)]
    pub remove_props: RemoveProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Remove {}
impl Activity for Remove {}

/// A specialization of Accept indicating that the acceptance is tentative.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TentativeAccept {
    #[serde(rename = "type")]
    kind: TentativeAcceptType,

    #[serde(flatten)]
    pub tentative_accept_props: TentativeAcceptProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for TentativeAccept {}
impl Activity for TentativeAccept {}

/// A specialization of Reject in which the rejection is considered tentative.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TentativeReject {
    #[serde(rename = "type")]
    kind: TentativeRejectType,

    #[serde(flatten)]
    pub tentative_reject_props: TentativeRejectProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for TentativeReject {}
impl Activity for TentativeReject {}

/// Indicates that the actor is traveling to target from origin.
///
/// Travel is an IntransitiveObject whose actor specifies the direct object. If the target or
/// origin are not specified, either can be determined by context.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Travel {
    #[serde(rename = "type")]
    kind: TravelType,

    #[serde(flatten)]
    pub travel_props: TravelProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Travel {}
impl Activity for Travel {}
impl IntransitiveActivity for Travel {}

/// Indicates that the actor is undoing the object.
///
/// In most cases, the object will be an Activity describing some previously performed action (for
/// instance, a person may have previously "liked" an article but, for whatever reason, might
/// choose to undo that like at some later point in time).
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Undo {
    #[serde(rename = "type")]
    kind: UndoType,

    #[serde(flatten)]
    pub undo_props: UndoProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Undo {}
impl Activity for Undo {}

/// Indicates that the actor has updated the object.
///
/// Note, however, that this vocabulary does not define a mechanism for describing the actual set
/// of modifications made to object.
///
/// The target and origin typically have no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[serde(rename = "type")]
    kind: UpdateType,

    #[serde(flatten)]
    pub update_props: UpdateProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Update {}
impl Activity for Update {}

/// Indicates that the actor has viewed the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    #[serde(rename = "type")]
    kind: ViewType,

    #[serde(flatten)]
    pub view_props: ViewProperties,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub ap_object_props: ApObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for View {}
impl Activity for View {}
