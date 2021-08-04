/*
 * minecraft-json: processing Minecraft JSON data
 * Copyright (C) 2021  Xie Ruifeng
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! Custom advancements in data packs of a Minecraft world store the advancement
//! data for that world as separate JSON files.
//!
//! ```
//! # use maplit::btreemap;
//! # use minecraft_json::assert_equiv_pretty;
//! # use minecraft_json::minecraft::data::advancement::{Advancement, Display, Icon, Frame, Criterion};
//! # use minecraft_json::minecraft::text::{TextComponent, TextComponentTags};
//! assert_equiv_pretty!(r#"{
//!   "display": {
//!     "icon": {
//!       "item": "minecraft:red_bed"
//!     },
//!     "title": {
//!       "translate": "advancements.adventure.sleep_in_bed.title"
//!     },
//!     "description": {
//!       "translate": "advancements.adventure.sleep_in_bed.description"
//!     }
//!   },
//!   "parent": "minecraft:adventure/root",
//!   "criteria": {
//!     "slept_in_bed": {
//!       "trigger": "minecraft:slept_in_bed",
//!       "conditions": {}
//!     }
//!   },
//!   "requirements": [
//!     [
//!       "slept_in_bed"
//!     ]
//!   ]
//! }"#, Advancement {
//!     parent: Some("minecraft:adventure/root".to_string()),
//!     display: Some(Display {
//!         icon: Some(Icon {
//!             item: "minecraft:red_bed".to_string(),
//!             nbt: None,
//!         }),
//!         title: TextComponent::Translated {
//!             translate: "advancements.adventure.sleep_in_bed.title".to_string(),
//!             with: Vec::new(),
//!             properties: TextComponentTags::default(),
//!         },
//!         description: TextComponent::Translated {
//!             translate: "advancements.adventure.sleep_in_bed.description".to_string(),
//!             with: Vec::new(),
//!             properties: TextComponentTags::default(),
//!         },
//!         frame: Frame::default(),
//!         background: None,
//!         show_toast: true,
//!         announce_to_chat: true,
//!         hidden: false,
//!     }),
//!     criteria: btreemap!{
//!         "slept_in_bed".to_string() => Criterion::SleptInBed {
//!             location: None,
//!             player: None,
//!         },
//!     },
//!     requirements: vec![vec!["slept_in_bed".to_string()]],
//!     rewards: None,
//! });
//! ```

use std::collections::BTreeMap;
use derivative::Derivative;
use serde::{Serialize, Deserialize};
use crate::defaults;
use crate::minecraft::text::TextComponent;
use crate::minecraft::data::conditions::{Location, Entity};

/// An advancement JSON file.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Advancement {
    /// The optional display data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    /// The optional parent advancement directory of this advancement. If this field is absent,
    /// this advancement is a root advancement. Circular references cause a loading failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The required criteria that have to be met.
    ///
    /// - Key: A name given to the criterion (can be any string, must be unique).
    /// - Value:
    ///   - `trigger`: The trigger and conditions for this advancement; specifies what the game
    ///     should check for the advancement.
    ///   - `conditions`: All the conditions that need to be met when the trigger gets activated.
    pub criteria: BTreeMap<String, Criterion>,
    /// An optional list of requirements (all the `<criteriaNames>`). If all criteria are required,
    /// this may be omitted. With multiple criteria: requirements contains a list of lists with
    /// criteria (all criteria need to be mentioned). If all of the lists each have any criteria
    /// met, the advancement is complete. (basically AND grouping of OR groups)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirements: Vec<Vec<String>>,
    /// An optional object representing the rewards provided when this advancement is obtained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Rewards>,
}

/// Display data for an [`Advancement`].
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Display {
    /// The data for the icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// The title for this advancement.
    pub title: TextComponent,
    /// The optional type of frame for the icon.
    #[serde(default, skip_serializing_if = "defaults::is_default")]
    pub frame: Frame,
    /// The optional directory for the background to use in this advancement tab
    /// (used only for the root advancement).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// The description of the advancement.
    pub description: TextComponent,
    /// Whether or not to show the toast pop up after completing this advancement.
    /// Defaults to `true`.
    #[serde(default = "defaults::r#true", skip_serializing_if = "defaults::is_true")]
    pub show_toast: bool,
    /// Whether or not to announce in the chat when this advancement has been completed.
    /// Defaults to `true`.
    #[serde(default = "defaults::r#true", skip_serializing_if = "defaults::is_true")]
    pub announce_to_chat: bool,
    /// Whether or not to hide this advancement and all its children from the advancement
    /// screen until this advancement have been completed. Has no effect on root advancements
    /// themselves, but still affects all their children. Defaults to `false`.
    #[serde(default, skip_serializing_if = "defaults::is_false")]
    pub hidden: bool,
}

/// An item (with NBT data) as icon.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Icon {
    /// The item id.
    pub item: String,
    /// The nbt data of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,
}

/// Type of frame for the icon.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
pub enum Frame {
    /// A tile with a more fancy spiked border as it is used for the kill all mobs advancement.
    Challenge,
    /// A tile with a rounded border as it is used for the full beacon advancement.
    Goal,
    /// A normal tile (default).
    #[derivative(Default)]
    Task,
}

/// Strongly-typed `trigger` and `conditions` for a criterion.
///
/// TODO: add all triggers here.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[serde(tag = "trigger", content = "conditions")]
#[non_exhaustive]
pub enum Criterion {
    /// Triggers when the player enters a bed.
    #[serde(rename = "minecraft:slept_in_bed")]
    SleptInBed {
        /// The location of the player.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Location>,
        /// The player that would get the advancement. May also be a list of predicates that
        /// must pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        player: Option<Entity>,
    },
}

/// An object representing the rewards provided when an [`Advancement`] is obtained.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Derivative, Serialize, Deserialize)]
pub struct Rewards {
    /// A list of recipes to unlock.
    ///
    /// Item: A namespaced ID for a recipe.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipes: Vec<String>,
    /// A list of loot tables to give to the player.
    ///
    /// Item: A namespaced ID for a loot table.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loot: Vec<String>,
    /// An amount of experience.
    #[serde(default, skip_serializing_if = "defaults::is_default")]
    pub experience: isize,
    /// A function to run. Function tags are not allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
}