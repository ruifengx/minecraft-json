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

//! Predicates are technical JSON files that represent the conditions for loot tables, `/execute
//! if predicate` command, or predicate target selector argument. They are a part of data packs.
//!
//! Inside a data pack, a predicate is located within data/<namespace>/predicates.
//!
//! A predicate file may also contain an array of multiple predicate objects, in which case the
//! predicate passes only if all sub-predicates pass.

use std::collections::BTreeMap;
use derivative::Derivative;
use serde::{Serialize, Deserialize};
use serde_json::Number;
use crate::minecraft::data::conditions::{DamageSource, Entity, Location, Item};
use crate::minecraft::common::{Ranged, Ranged2, NumberProviderValue};
use crate::defaults;

/// Predicate.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "condition", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Predicate {
    /// Joins conditions from parameter terms with "or".
    Alternative {
        /// A list of conditions to join using 'or'.
        terms: Vec<Predicate>,
    },
    /// Check properties of a block state.
    BlockStateProperty {
        /// A block ID. The test fails if the block doesn't match.
        block: String,
        /// A map of block property names to values. All values are strings. The test fails if
        /// the block doesn't match.
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        properties: BTreeMap<String, String>,
    },
    /// Check properties of damage source.
    DamageSourceProperties {
        /// Predicate applied to the damage source.
        predicate: DamageSource,
    },
    /// Test properties of an entity.
    EntityProperties {
        /// Specifies the entity to check for the condition.
        entity: WhichEntity,
        /// Predicate applied to entity, uses same structure as advancements.
        predicate: Box<Entity>,
    },
    /// Test the scoreboard scores of an entity.
    EntityScores {
        /// Specifies the entity to check for the condition.
        entity: WhichEntity,
        /// Scores to check. All specified scores must pass for the condition to pass.
        ///
        /// Item: Key name is the objective while the value is the exact score value (or a range
        /// of score values) required for the condition to pass.
        scores: BTreeMap<String, Ranged<isize>>,
    },
    /// Inverts condition from parameter term.
    Inverted {
        /// The condition to be negated.
        term: Box<Predicate>,
    },
    /// Test if a [`WhichEntity::KillerPlayer`] entity is available.
    KilledByPlayer {
        /// If true, the condition passes if [`WhichEntity::KillerPlayer`] is *not* available.
        #[serde(default, skip_serializing_if = "defaults::is_default")]
        inverse: bool,
    },
    /// Checks if the current location matches.
    #[serde(rename_all = "camelCase")]
    LocationCheck {
        /// Optional offsets to location.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset_x: Option<isize>,
        /// Optional offsets to location.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset_y: Option<isize>,
        /// Optional offsets to location.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset_z: Option<isize>,
        /// Predicate applied to location, uses same structure as advancements.
        predicate: Box<Location>,
    },
    /// Checks tool.
    MatchTool {
        /// Predicate applied to item, uses same structure as advancements.
        predicate: Box<Item>,
    },
    /// Test if a random number 0.0–1.0 is less than a specified value.
    RandomChance {
        /// Success rate as a number 0.0–1.0.
        chance: Number,
    },
    /// Test if a random number 0.0–1.0 is less than a specified value, affected by the level of
    /// `Looting` on the killer entity.
    RandomChanceWithLooting {
        /// Base success rate.
        chance: Number,
        /// Looting adjustment to the base success rate.
        /// Formula is `chance + (looting_level * looting_multiplier)`.
        looting_multiplier: Number,
    },
    /// Test if another referred condition (predicate) passes.
    Reference {
        /// The namespaced ID of the condition (predicate) referred to. A cyclic reference
        /// causes a parsing failure.
        name: String,
    },
    /// Returns true with 1/explosion radius probability.
    SurvivesExplosion,
    /// Passes with probability picked from table, indexed by enchantment level.
    TableBonus {
        /// Id of enchantment.
        enchantment: isize,
        /// List of probabilities for enchantment level, indexed from 0.
        chances: Vec<Number>,
    },
    /// Checks the current time.
    TimeCheck {
        /// The time value in ticks.
        value: Ranged2<isize, NumberProviderValue<isize>>,
        /// If present, time gets modulo-divided by this value (for example, if set to `24000`,
        /// value operates on a time period of daytime ticks just like `/time query daytime`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        period: Option<isize>,
    },
    /// Checks for a current weather state.
    WeatherCheck {
        /// If true, the condition evaluates to true only if it's raining or thundering.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        raining: Option<bool>,
        /// If true, the condition evaluates to true only if it's thundering.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thundering: Option<bool>,
    },
    /// Checks for range of value.
    ValueCheck {
        /// Number Provider. The value to test.
        value: NumberProviderValue<isize>,
        /// The exact value to check, or the range to check the value.
        range: Ranged<NumberProviderValue<isize>>,
    },
}

/// Specifies the entity to check for the condition.
#[derive(Eq, PartialEq, Debug)]
#[derive(Derivative, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WhichEntity {
    /// The entity that died or the player that gained the advancement.
    This,
    /// The killer.
    Killer,
    /// A killer that is a player.
    KillerPlayer,
}
