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

//! Conditions common to all [`Criterion`](crate::minecraft::data::advancement::Criterion)s.

use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::minecraft::common::{Ranged, Vector3d, PlainValue};

/// Tags common to all locations.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Location {
    /// The biome the entity is currently in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biome: Option<String>,
    /// The block at the location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<Block>>,
    /// The dimension the entity is currently in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// Name of a structure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// The fluid at the location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fluid: Option<Box<Fluid>>,
    /// The light Level of visible light. Calculated using: `max(sky - darkening, block)`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<Ranged<isize>>,
    /// Block coordinate of this location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<Vector3d<Ranged<isize>>>>,
    /// True if the block is closely above a campfire or soul campfire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smokey: Option<bool>,
}

/// Block conditions.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    /// A list of block IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    /// The block Tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The block NBT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,
    /// A map of block property names to values. Test will fail if the block doesn't match.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub state: BTreeMap<String, PlainValue>,
}

/// Fluid conditions.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Fluid {
    /// The fluid ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fluid: Option<String>,
    /// The fluid Tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// A map of fluid property names to values. Test will fail if the fluid doesn't match.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub state: BTreeMap<String, PlainValue>,
}

/// Tags common to all entities.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Entity {
    /// The player that would get the advancement. May also be a list of predicates that must
    /// pass in order for the trigger to activate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Box<Distance>>,
    /// A list of status effects.
    ///
    /// Item: A status effect with the key name being the status effect name.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub effects: BTreeMap<String, Effect>,
    /// Equipments on this entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Equipment>,
    /// Predicate Flags to be checked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EntityFlags>,
    /// Lightning bolt properties to be checked. Fails when entity is not a lightning bolt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lightning_bolt: Option<Box<LightningBolt>>,
    /// Location of this entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    /// An NBT string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,
    /// The entity directly riding this entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passenger: Option<Box<Entity>>,
    /// Player properties to be checked. Fails when entity is not a player.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub player: Option<Box<Player>>,
    /// Location predicate for the block the entity is standing on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stepping_on: Option<Box<Location>>,
    /// The team the entity belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    /// An entity ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The entity which this entity is targeting for attacks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targeted_entity: Option<Box<Entity>>,
    /// The vehicle that the entity is riding on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Box<Entity>>,
}

/// World distance.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Distance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absolute: Option<Ranged<isize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<Ranged<isize>>,
    #[serde(flatten)]
    pub components: Vector3d<Ranged<isize>>,
}

/// Effect properties.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Effect {
    /// Whether the effect is from a beacon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambient: Option<bool>,
    /// The effect amplifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<Ranged<isize>>,
    /// The effect duration in ticks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<Ranged<isize>>,
    /// Whether the effect has visible particles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

/// Entity equipments.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Equipment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mainhand: Option<Box<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offhand: Option<Box<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<Box<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chest: Option<Box<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legs: Option<Box<Item>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feet: Option<Box<Item>>,
}

/// Tags common to all items.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Item {
    /// Amount of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<Ranged<isize>>,
    /// The durability of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durability: Option<Ranged<isize>>,
    /// List of enchantments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enchantments: Vec<Enchantment>,
    /// List of stored enchantments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stored_enchantments: Vec<Enchantment>,
    /// A list of item IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
    /// An NBT string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,
    /// A brewed potion ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub potion: Option<String>,
    /// An item data pack tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Minecraft enchantment.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Enchantment {
    /// An enchantment ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enchantment: Option<String>,
    /// The level of the enchantment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Ranged<isize>>,
}

/// Predicate Flags to be checked for an entity.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct EntityFlags {
    /// Test whether the entity is or is not on fire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_on_fire: Option<bool>,
    /// Test whether the entity is or is not sneaking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_sneaking: Option<bool>,
    /// Test whether the entity is or is not sprinting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_sprinting: Option<bool>,
    /// Test whether the entity is or is not swimming.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_swimming: Option<bool>,
    /// Test whether the entity is or is not a baby variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_baby: Option<bool>,
}

/// Lightning bolt properties.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct LightningBolt {
    /// Number of blocks set on fire by this lightning bolt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks_set_on_fire: Option<isize>,
    /// Entity properties of entities struck by this lightning bolt. If present, this tag must
    /// match one or more entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_struck: Option<Box<Entity>>,
    /// Properties of this lightning bolt as an entity.
    #[serde(flatten)]
    pub lightning_entity: Entity,
}

/// Player properties.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Player {
    /// The entity that the player is looking at, as long as it is visible and within a radius
    /// of 100 blocks. Visibility is defined through the line from the player's eyes to the
    /// entity's eyes, rather than the direction that the player is looking in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub looking_at: Option<Box<Entity>>,
    /// A map of advancements to check.
    ///
    /// Item: An advancement ID.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub advancements: BTreeMap<String, AdvancementStatus>,
    /// The game mode of the player.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gamemode: Option<GameMode>,
    /// The experience level of the player.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<Ranged<isize>>,
    /// A map of recipes to check.
    ///
    /// Item: A recipe ID. `True` if the recipe is known to the player.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub recipes: BTreeMap<String, bool>,
    /// List of [`Statistic`]s to match.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats: Vec<Statistic>,
}

/// Status of an advancement.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdvancementStatus {
    /// Acquired?
    Advancement(bool),
    /// Achieved some criteria of this advancement?
    Criteria(BTreeMap<String, bool>),
}

/// The game mode of a player.
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum GameMode {
    Survival,
    Adventure,
    Creative,
    Spectator,
}

/// A statistic. Note that unlike when adding scoreboard objectives, the base (for example,
/// `minecraft:custom`) and the statistic (for example, `minecraft:sneak_time`) are split and
/// use proper namespaces instead of the dot-notation (`minecraft.custom:minecraft.sneak_time`).
#[derive(Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Statistic {
    /// The statistic base. Possible values are `minecraft:custom`, `minecraft:crafted`,
    /// `minecraft:used`, `minecraft:broken`, `minecraft:mined`, `minecraft:killed`,
    /// `minecraft:picked_up`, `minecraft:dropped` and `minecraft:killed_by`.
    pub r#type: String,
    /// The statistic ID. Mostly mimics the criteria used for defining scoreboard objectives.
    pub stat: String,
    /// The value of the statistic.
    pub value: Ranged<isize>,
}

/// Properties of damage source.
#[derive(Eq, PartialEq, Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct DamageSource {
    /// Checks if the damage bypassed the armor of the player (suffocation damage predominantly).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypasses_armor: Option<bool>,
    /// Checks if the damage bypassed the invulnerability status of the player (void or /kill damage).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypasses_invulnerability: Option<bool>,
    /// Checks if the damage was caused by starvation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypasses_magic: Option<bool>,
    /// The entity that was the direct cause of the damage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_entity: Option<Box<Entity>>,
    /// Checks if the damage originated from an explosion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_explosion: Option<bool>,
    /// Checks if the damage originated from fire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_fire: Option<bool>,
    /// Checks if the damage originated from magic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_magic: Option<bool>,
    /// Checks if the damage originated from a projectile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_projectile: Option<bool>,
    /// Checks if the damage originated from lightning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_lightning: Option<bool>,
    /// Checks the entity that was the source of the damage (for example: The skeleton that shot the arrow).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<Box<Entity>>,
}
