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
//!     display: Some(Box::new(Display {
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
//!     })),
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
use crate::minecraft::data::conditions::{Location, Entity, Item};
use crate::minecraft::common::Either;

/// An advancement JSON file.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Advancement {
    /// The optional display data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Box<Display>>,
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
    /// Triggers when the player breaks a bee nest or beehive.
    ///
    /// ```
    /// # use minecraft_json::assert_equiv_pretty;
    /// # use minecraft_json::minecraft::data::conditions::Item;
    /// # use minecraft_json::minecraft::data::advancement::Criterion;
    /// assert_equiv_pretty!(r#"{
    ///   "trigger": "minecraft:bee_nest_destroyed",
    ///   "conditions": {
    ///     "block": "minecraft:beehive",
    ///     "item": {
    ///       "items": [
    ///         "minecraft:wooden_axe"
    ///       ]
    ///     },
    ///     "num_bees_inside": 3
    ///   }
    /// }"#, Criterion::BeeNestDestroyed {
    ///     block: Some("minecraft:beehive".to_string()),
    ///     item: Some(Box::new(Item {
    ///         items: vec!["minecraft:wooden_axe".to_string()],
    ///         ..Item::default()
    ///     })),
    ///     num_bees_inside: Some(3),
    ///     player: None,
    /// });
    /// ```
    #[serde(rename = "minecraft:bee_nest_destroyed")]
    BeeNestDestroyed {
        /// The block that was destroyed. Accepts block IDs.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<String>,
        /// The item used to break the block. See also [`Item`].
        #[serde(default, skip_serializing_if = "Option::is_none")]
        item: Option<Box<Item>>,
        /// The number of bees that were inside the bee nest/beehive before it was broken.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        num_bees_inside: Option<isize>,
        /// The player that would get the advancement. May also be a list of predicates that must
        /// pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        player: Option<Either<Vec<String>, Box<Entity>>>,
    },
    /// Triggers after the player breeds 2 animals.
    ///
    /// ```
    /// # use maplit::btreemap;
    /// # use minecraft_json::assert_equiv_pretty_protected;
    /// # use minecraft_json::minecraft::data::conditions::{Item, Entity, Location, Effect};
    /// # use minecraft_json::minecraft::data::advancement::Criterion;
    /// # use minecraft_json::minecraft::common::{Ranged, Either};
    /// assert_equiv_pretty_protected!(r#"{
    ///   "trigger": "minecraft:bred_animals",
    ///   "conditions": {
    ///     "child": {
    ///       "type": "minecraft:mule"
    ///     },
    ///     "parent": {
    ///       "location": {
    ///         "biome": "minecraft:beach"
    ///       }
    ///     },
    ///     "partner": {
    ///       "effects": {
    ///         "minecraft:speed": {
    ///           "amplifier": {
    ///             "min": 2
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }"#, Criterion::BredAnimals {
    ///     child: Some(Either::Right(Box::new(Entity {
    ///         r#type: Some("minecraft:mule".to_string()),
    ///         ..Entity::default()
    ///     }))),
    ///     parent: Some(Either::Right(Box::new(Entity {
    ///         location: Some(Box::new(Location {
    ///             biome: Some("minecraft:beach".to_string()),
    ///             ..Location::default()
    ///         })),
    ///         ..Entity::default()
    ///     }))),
    ///     partner: Some(Either::Right(Box::new(Entity {
    ///         effects: btreemap! {
    ///             "minecraft:speed".to_string() => Effect {
    ///                 amplifier: Some(Ranged::Range {
    ///                     min: Some(2),
    ///                     max: None,
    ///                 }),
    ///                 ..Effect::default()
    ///             },
    ///         },
    ///         ..Entity::default()
    ///     }))),
    ///     player: None,
    /// });
    /// ```
    #[serde(rename = "minecraft:bred_animals")]
    BredAnimals {
        /// The child that results from the breeding. May also be a list of predicates that must
        /// pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        child: Option<Either<Vec<String>, Box<Entity>>>,
        /// The parent. May also be a list of predicates that must pass in order for the trigger
        /// to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent: Option<Either<Vec<String>, Box<Entity>>>,
        /// The partner. (The entity the parent was bred with) May also be a list of predicates
        /// that must pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        partner: Option<Either<Vec<String>, Box<Entity>>>,
        /// The player that would get the advancement. May also be a list of predicates that must
        /// pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        player: Option<Either<Vec<String>, Box<Entity>>>,
    },
    /// Triggers when the player enters a bed.
    ///
    /// ```
    /// # use minecraft_json::assert_equiv_pretty_protected;
    /// # use minecraft_json::minecraft::data::conditions::{Item, Location};
    /// # use minecraft_json::minecraft::data::advancement::Criterion;
    /// # use minecraft_json::minecraft::common::{Vector3d, Ranged};
    /// assert_equiv_pretty_protected!(r#"{
    ///   "trigger": "minecraft:slept_in_bed",
    ///   "conditions": {
    ///     "location": {
    ///       "biome": "minecraft:desert",
    ///       "feature": "village",
    ///       "position": {
    ///         "y": {
    ///           "min": 50,
    ///           "max": 100
    ///         }
    ///       }
    ///     }
    ///   }
    /// }"#, Criterion::SleptInBed {
    ///     location: Some(Box::new(Location {
    ///         biome: Some("minecraft:desert".to_string()),
    ///         feature: Some("village".to_string()),
    ///         position: Some(Box::new(Vector3d {
    ///             y: Some(Ranged::Range {
    ///                 min: Some(50),
    ///                 max: Some(100),
    ///             }),
    ///             ..Vector3d::default()
    ///         })),
    ///         ..Location::default()
    ///     })),
    ///     player: None,
    /// });
    /// ```
    #[serde(rename = "minecraft:slept_in_bed")]
    SleptInBed {
        /// The location of the player.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<Box<Location>>,
        /// The player that would get the advancement. May also be a list of predicates that
        /// must pass in order for the trigger to activate.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        player: Option<Either<Vec<String>, Box<Entity>>>,
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
