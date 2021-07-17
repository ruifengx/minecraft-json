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

//! [Raw JSON text format](https://minecraft.fandom.com/wiki/Raw_JSON_text_format)

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::Number;
use std::fmt::{Display, Formatter};

mod defaults {
    use crate::minecraft::text::{TextComponent, TextComponentTags, StringLike, Formatting, Colour, ColourName};

    #[inline(always)]
    pub fn is_false(&x: &bool) -> bool { !x }

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

/// Text colours, either pre-defined or custom hexadecimal colours.
///
/// ```
/// # use minecraft_json::{assert_equiv, minecraft::text::{Colour, ColourName, HexColour}};
/// assert_equiv!(r#""dark_purple""#, Colour::Named(ColourName::DarkPurple));
/// assert_equiv!(r##""#66CCFF""##, Colour::Hex(HexColour { red: 0x66, green: 0xCC, blue: 0xFF }));
/// ```
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Colour {
    Named(ColourName),
    Hex(HexColour),
}

/// Pre-defined colour names.
///
/// ```
/// # use minecraft_json::{assert_equiv, minecraft::text::ColourName};
/// assert_equiv!(r#""dark_purple""#, ColourName::DarkPurple);
/// assert_equiv!(r#""reset""#, ColourName::Reset);
/// ```
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum ColourName {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    /// Cancels out the effects of colors used by parent objects.
    Reset,
}

/// Hexadecimal colours.
///
/// Set to "`#<hex>`" to insert any color in the hexadecimal color format.
/// Example: Using "`#FF0000`" makes the component red.
/// Must be a full 6-digit value, not 3-digit.
///
/// ```
/// # use minecraft_json::{assert_equiv, assert_cannot_deserialize, minecraft::text::HexColour};
/// assert_equiv!(r##""#66CCFF""##, HexColour { red: 0x66, green: 0xCC, blue: 0xFF });
/// assert_cannot_deserialize!(r##""#6CF"## => HexColour);
/// ```
#[derive(Eq, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct HexColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Display for HexColour {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl Serialize for HexColour {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        format!("{}", self).serialize(s)
    }
}

impl<'de> Deserialize<'de> for HexColour {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<HexColour, D::Error> {
        use serde::{de, de::Unexpected};

        fn check_format(text: &str) -> bool {
            text.len() == "#FFFFFF".len() && text.is_ascii()
                && text.as_bytes()[0] == b'#'
                && text.as_bytes()[1..].iter().all(|c| (*c as char).is_ascii_hexdigit())
        }

        fn read_hex2(s: &[u8], k: usize) -> u8 {
            (s[k] as char).to_digit(16).unwrap() as u8 * 16
                + (s[k + 1] as char).to_digit(16).unwrap() as u8
        }

        let text = <&str>::deserialize(d)?;
        if !check_format(text) {
            return Err(de::Error::invalid_value(
                Unexpected::Str(text),
                &"hexadecimal color (e.g. #FFFFFF)"));
        }
        let n = &text.as_bytes()[1..];
        Ok(HexColour {
            red: read_hex2(n, 0),
            green: read_hex2(n, 2),
            blue: read_hex2(n, 4),
        })
    }
}

/// Common properties for [`TextComponent`].
///
/// Due to the `extra` tag, the above format may be recursively nested to produce complex and
/// functional text strings. However, a raw JSON text doesn't have to be complicated at all:
/// virtually all properties are optional and may be left out.
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct TextComponentTags {
    /// A list of additional raw JSON text components to be displayed after this one.
    ///
    /// Each element is a child text component. Child text components inherit all formatting and
    /// interactivity from the parent component, unless they explicitly override them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<TextComponent>,
    /// Formatting properties.
    #[serde(flatten)]
    pub formatting: Formatting,
    /// Interactivity properties.
    #[serde(flatten)]
    pub interactivity: Interactivity,
}

/// Formatting properties for a [`TextComponent`], can be inherited.
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Formatting {
    /// The color to render the content in. See also [`Colour`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Colour>,
    /// The resource location of the font for this component in the resource pack within
    /// `assets/<namespace>/font`. Defaults to `"minecraft:default"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Whether to render the content in bold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Whether to render the content in italics. Note that text that is italicized by default,
    /// such as custom item names, can be unitalicized by setting this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Whether to underline the content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    /// Whether to strikethrough the content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// Whether to render the content obfuscated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
}

/// Interactivity properties for a [`TextComponent`], can be inherited.
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Interactivity {
    /// When the text is shift-clicked by a player, this string is inserted in their chat input.
    /// It does not overwrite any existing text the player was writing. This only works in chat
    /// messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    /// Allows for events to occur when the player clicks on text. Only work in chat messages
    /// and written books, unless specified otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    /// Allows for a tooltip to be displayed when the player hovers their mouse over text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

/// Event upon mouse click on this text component.
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct ClickEvent {
    /// The action to perform when clicked. See also [`ClickEventAction`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    action: Option<ClickEventAction>,
    /// The URL, file path, chat, command or book page used by the specified action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

/// Action performed upon a [`ClickEvent`].
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventAction {
    /// Opens `value` as a URL in the user's default web browser.
    OpenUrl,
    /// Opens the file at `value` on the user's computer. This is used in messages automatically
    /// generated by the game (e.g., on taking a screenshot) and cannot be used by players for
    /// security reasons.
    OpenFile,
    /// Works in signs, but only on the root text component, not on any children. Activated by
    /// using the sign. In chat and written books, this has `value` entered in chat as though the
    /// player typed it themselves and pressed enter. This can be used to run commands, provided
    /// the player has the required permissions. Since they are being run from chat, commands must
    /// be prefixed with the usual `"/"` slash. In signs, the command is run by the server at the
    /// sign's location, with the player who used the sign as `@s`. Since they are run by the
    /// server, sign commands have the same permission level as a command block instead of using
    /// the player's permission level, are not restricted by chat length limits, and do not need
    /// to be prefixed with a `"/"` slash.
    RunCommand,
    /// Opens chat and fills in `value`. If a chat message was already being composed, it is
    /// overwritten. This does not work in books.
    SuggestCommand,
    /// Can only be used in written books. Changes to page `value` if that page exists.
    ChangePage,
    /// Copies `value` to the clipboard.
    CopyToClipboard,
}

/// Part of [`HoverEventStructure`].
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueOrContents<V, C> {
    /// The deprecated `value` field.
    Value(V),
    /// The recommended `contents` field.
    Contents(C),
}

/// Event upon mouse hover on this text component.
///
/// ```
/// # use minecraft_json::{assert_equiv, minecraft::text};
/// # use text::{HoverEvent, ValueOrContents, TextComponent, TextComponentTags, StringLike};
/// assert_equiv!(
///     r#"{"action":"show_text","contents":{"text":"hello"}}"#,
///     HoverEvent::ShowText(ValueOrContents::Contents(
///         Box::new(TextComponent::Text {
///             text: StringLike::String("hello".into()),
///             properties: TextComponentTags::default(),
///         })
///     ))
/// );
/// ```
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum HoverEvent {
    /// Shows a raw JSON text component. `value`/`contents` can be any valid text component type.
    /// Note that `clickEvent` and `hoverEvent` do not function within the tooltip.
    ShowText(ValueOrContents<Box<TextComponent>, Box<TextComponent>>),
    /// Shows the tooltip of an item as if it was being hovering over it in an inventory.
    ShowItem(ValueOrContents<String, Item>),
    /// Shows an entity's name, type, and UUID.
    /// Used by `selector` (see [`TextComponent::EntityNames`]).
    ShowEntity(ValueOrContents<LegacyEntity, Entity>),
}

/// The item that should be displayed by [`HoverEvent::ShowItem`].
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Item {
    /// The namespaced item ID. Present minecraft:air if invalid.
    pub id: String,
    /// Size of the item stack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<Number>,
    /// A string containing the serialized NBT of the additional information about the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// The entity that should be displayed by [`HoverEvent::ShowEntity`].
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Entity {
    /// Hidden if not present. A raw JSON text that is displayed as the name of the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<TextComponent>>,
    /// A string containing the type of the entity. Should be a namespaced entity ID.
    /// Present `minecraft:pig` if invalid.
    pub r#type: String,
    /// A string containing the UUID of the entity in the hyphenated hexadecimal format.
    /// Should be a valid UUID.
    pub id: String,
}

/// The entity that should be displayed by [`HoverEvent::ShowEntity`].
#[derive(Eq, PartialEq, Default, Debug)]
#[derive(Deserialize, Serialize)]
pub struct LegacyEntity {
    /// Hidden if not present. An NBT string containing some JSON that is parsed as a text
    /// component and displayed as the name of the entity. If the NBT string cannot be parsed
    /// as a text component, the entire tooltip is replaced with the text `"Invalid Entity!"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Hidden if not present. An NBT string containing some plain text that is displayed as
    /// the type of the entity. Can be any text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Shown as empty line if not present. An NBT string containing some plain text that is
    /// displayed as the UUID of the entity. Can be any text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Displays a score from the scoreboard. Requires component resolution.
/// This component is resolved into a text component containing the scoreboard value.
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
pub struct Score {
    /// The name of the score holder whose score should be displayed.
    /// This can be a selector like `@p` or an explicit name. If the text is a selector, the
    /// selector must be guaranteed to never select more than one entity, possibly by adding
    /// `limit=1`. If the text is `"*"`, it shows the reader's own score (for example,
    /// `/tellraw @a {"score":{"name":"*","objective":"obj"}}` shows every online player their
    /// own score in the `"obj"` objective).
    pub name: String,
    /// The internal name of the objective to display the player's score in.
    pub objective: String,
    /// Optional. If present, this value is displayed regardless of what the score would have been.
    pub value: String,
}


/// A string containing plain text to display directly.
/// Can also be a number or boolean that is displayed directly.
///
/// ```
/// use std::str::FromStr;
/// # use minecraft_json::{assert_equiv, minecraft::text::StringLike};
/// assert_equiv!(r#""some text""#, StringLike::String("some text".to_string()));
/// assert_equiv!(r#"42"#, StringLike::Number(serde_json::Number::from(42)));
/// // waiting for [serde-json#785](https://github.com/serde-rs/json/issues/785)
/// assert_equiv!(r#"1.9e10"#, StringLike::Number(serde_json::Number::from_str("1.9e10").unwrap()));
/// assert_equiv!("true", StringLike::Boolean(true));
/// ```
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringLike {
    /// A boolean is converted to a string ("true" or "false") to display directly.
    /// This is the same as an object that only has a `text` tag.
    /// For example, `true`, `"true"`, and `{"text": "true"}` are equivalent.
    Boolean(bool),
    /// A number is converted to a string to display directly.
    /// This is the same as an object that only has a `text` tag.
    /// For example, `1.9E10`, `"1.9E10"`, and `{"text": "1.9E10"}` are equivalent.
    Number(Number),
    /// A string containing plain text to display directly.
    String(String),
}

/// Raw JSON text is made up of text components. There is a single root component, which can
/// have child components, which can have their own children and so on. Components can also
/// have formatting and interactivity added to them, which is inherited by their children.
#[derive(Eq, PartialEq, Debug)]
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum TextComponent {
    /// Displays plain text.
    RawTextLike(StringLike),
    /// A list of raw JSON text components.
    /// Same as having all components after the first one appended to the first's `extra` array.
    /// For example, `["A", "B", "C"]` is equivalent to `{"text": "A", "extra": ["B", "C"]}`.
    RawTextList(Vec<TextComponent>),
    /// Displays plain text.
    Text {
        #[allow(missing_docs)]
        text: StringLike,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
    /// Displays a translated piece of text from the currently selected language. This uses the
    /// client's selected language, so if players with their games set to different languages are
    /// logged into the same server, each will see the component in their own language.
    ///
    /// Translations are defined in language files in resource packs, including the built-in
    /// resource pack.
    ///
    /// Translations can contain slots for text that is not known ahead of time, such as player
    /// names. When displaying the translated text, slots will be filled from a provided list of
    /// text components. The slots are defined in the language file, and generally take the form
    /// `%s` (displays the next component in the list), or `%3$s` (displays the third component in
    /// the list; replace 3 with whichever index is desired). For example, the built-in English
    /// language file contains the translation
    /// `"chat.type.advancement.task": "%s has made the advancement %s",`.
    Translated {
        /// A translation identifier, corresponding to the identifiers found in loaded language
        /// files. Displayed as the corresponding text in the player's selected language. If no
        /// corresponding translation can be found, the identifier itself is used as the
        /// translated text.
        translate: String,
        /// Optional.
        /// A list of raw JSON text components to be inserted into slots in the translation text.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<TextComponent>,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
    /// Displays a score from the scoreboard.
    ScoreBoard {
        /// Displays a score holder's current score in an objective. Displays nothing if the
        /// given score holder or the given objective do not exist, or if the score holder is
        /// not tracked in the objective.
        score: Score,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
    /// Displays the name of one or more entities found by a selector.
    ///
    /// If exactly one entity is found, the entity's name is displayed by itself. If more are
    /// found, their names are displayed in the form `"Name1, Name2, Name3"`, with gray commas.
    /// If none are found, the component is displayed as no text.
    ///
    /// Hovering over a name shows a tooltip with the name, type, and UUID of the target.
    /// Clicking a player's name suggests a command to whisper to that player. Shift-clicking
    /// a player's name inserts that name into chat. Shift-clicking a non-player entity's name
    /// inserts its UUID into chat.
    EntityNames {
        /// A string containing a selector.
        selector: String,
        /// Optional, defaults to `{"color": "gray", "text": ", "}`. A raw JSON text component.
        /// Used as the separator between different names, if the component selects multiple entities.
        #[serde(default = "defaults::entity_names_separator")]
        separator: Box<TextComponent>,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
    /// Displays the name of the button that is currently bound to a certain configurable control.
    /// This uses the client's own control scheme, so if players with different control schemes
    /// are logged into the same server, each will see their own keybind.
    KeyBind {
        /// A keybind identifier, to be displayed as the name of the button that is currently
        /// bound to that action. For example, `{"keybind": "key.inventory"}` displays "e" if
        /// the player is using the default control scheme.
        keybind: String,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
    /// Displays NBT values from entities, block entities, or command storage.
    ///
    /// NBT strings display their contents. Other NBT values are displayed as SNBT, with no
    /// spacing between symbols. If `interpret` is set to true, the game will instead attempt to
    /// parse and display that text as its own raw JSON text component. That usually only works
    /// on strings, since JSON and SNBT are not compatible. If `interpret` is true and parsing
    /// fails, the component is displayed as no text. If more than one NBT value is found, either
    /// by selecting multiple entities or by using a multi-value path, they are displayed in the
    /// form `"Value1, Value2, Value3, Value4"`.
    ///
    /// Requires component resolution.
    ///
    /// - If `interpret` is `false`, the component is resolved into a text component containing
    ///   the display text.
    ///   - If multiple values are selected and `separator` is not present, the entire component
    ///     is still resolved into a single text component, with the text `", "` between the
    ///     display text of each value.
    ///   - If multiple values are selected and `separator` is present, each value is resolved
    ///     into an individual text component, and all values after the first will be added to
    ///     the first's `extra` list, separated by copies of the `separator` component.
    /// - If `interpret` is `true`, the component is resolved into the parsed text component.
    ///   For any non-content tags that are present on both the parsed text component and the
    ///   component being resolved, the tag on the component being resolved will be used.
    ///   - If multiple values are selected, all values after the first will be added to the
    ///     first's `extra` list, separated by copies of the `separator` component (or its
    ///     default, if not present). This means that all values after the first will inherit
    ///     the first value's formatting tags, if any.
    NbtValue {
        /// The NBT path used for looking up NBT values from an entity, block entity, or storage.
        /// Requires one of `block`, `entity`, or `storage`. Having more than one is allowed, but
        /// only one is used.
        nbt: String,
        /// Optional, defaults to `false`. If `true`, the game attempts to parse the text of each
        /// NBT value as a raw JSON text component. Ignored if `nbt` is not present.
        ///
        /// See also the documentation for [`TextComponent::NbtValue`].
        #[serde(default, skip_serializing_if = "defaults::is_false")]
        interpret: bool,
        /// Optional, defaults to `{"text": ", "}`. A raw JSON text component. Used as the
        /// separator between different tags, if the component selects multiple tags.
        separator: Box<TextComponent>,
        /// A string specifying the coordinates of the block entity from which the NBT value
        /// is obtained. The coordinates can be absolute or relative. Ignored if `nbt` is not
        /// present.
        block: Option<String>,
        /// A string specifying the target selector for the entity or entities from which the
        /// NBT value is obtained. Ignored if `nbt` is not present.
        entity: Option<String>,
        /// A string specifying the namespaced ID of the command storage from which the NBT
        /// value is obtained. Ignored if `nbt` is not present.
        storage: Option<String>,
        /// Common additional properties.
        #[serde(flatten)]
        properties: TextComponentTags,
    },
}
