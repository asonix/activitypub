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
