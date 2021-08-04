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

//! `minecraft-json` is for processing Minecraft JSON data.

#![warn(missing_docs)]

pub mod minecraft;

mod test;

mod defaults {
    use crate::minecraft::text::{TextComponent, TextComponentTags, StringLike, Formatting, Colour, ColourName};

    #[inline(always)]
    pub fn is_default<T: Default + Eq>(x: &T) -> bool { *x == Default::default() }

    #[inline(always)]
    pub fn is_false(&x: &bool) -> bool { !x }

    #[inline(always)]
    pub fn is_true(&x: &bool) -> bool { x }

    #[inline(always)]
    pub fn r#true() -> bool { true }

    pub fn entity_names_separator() -> Box<TextComponent> {
        Box::new(TextComponent::Text {
            text: StringLike::String(", ".into()),
            properties: TextComponentTags {
                formatting: Formatting {
                    color: Some(Colour::Named(ColourName::Gray)),
                    ..Formatting::default()
                },
                ..TextComponentTags::default()
            },
        })
    }
}
