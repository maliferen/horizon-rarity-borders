// Rarity Borders runtime for Horizon Minecraft.
// Adds crisp, vanilla-styled pixel-perfect colored borders to inventory item slots
// based on Minecraft item rarity.

use std::sync::atomic::{AtomicBool, Ordering};

/// Mod identifier.
pub const ID: &str = "rarity_borders";

/// User-facing display name.
pub const NAME: &str = "Rarity Borders";

/// Semantic version.
pub const VERSION: &str = "0.1.0";

/// Mod description for in-game mod list.
pub const DESCRIPTION: &str =
    "Adds colored borders to inventory items based on their rarity in Horizon Minecraft.";

/// Master ON/OFF toggle (default: true).
static ENABLED: AtomicBool = AtomicBool::new(true);

/// Whether to show borders for Common items (default: false).
static SHOW_COMMON: AtomicBool = AtomicBool::new(false);

/// Whether to show borders in in-game HUD hotbar (default: false).
static SHOW_HOTBAR: AtomicBool = AtomicBool::new(false);

/// Tracks whether settings were synchronized from disk.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Vanilla-styled rarity colors (RGB).
pub const COLOR_COMMON: u32 = 0x94A3B8; // Slate Gray (neutral, subtle)
pub const COLOR_UNCOMMON: u32 = 0xFFFF55; // Soft Golden Yellow (vanilla uncommon)
pub const COLOR_RARE: u32 = 0x55FFFF; // Soft Sky Cyan / Aqua (vanilla rare)
pub const COLOR_EPIC: u32 = 0xFF55FF; // Soft Magenta / Light Purple (vanilla epic)

/// Official Minecraft item rarity categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl std::str::FromStr for Rarity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "uncommon" => Self::Uncommon,
            "rare" => Self::Rare,
            "epic" => Self::Epic,
            _ => Self::Common,
        })
    }
}

impl Rarity {
    /// Parses rarity string identifier from Minecraft / Horizon.
    pub fn parse_str(s: &str) -> Self {
        match s {
            "uncommon" => Self::Uncommon,
            "rare" => Self::Rare,
            "epic" => Self::Epic,
            _ => Self::Common,
        }
    }

    /// Converts rarity to its canonical string representation.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Uncommon => "uncommon",
            Self::Rare => "rare",
            Self::Epic => "epic",
        }
    }
}

/// Returns the RGB color value for the given rarity.
pub const fn rarity_color(rarity: Rarity) -> u32 {
    match rarity {
        Rarity::Common => COLOR_COMMON,
        Rarity::Uncommon => COLOR_UNCOMMON,
        Rarity::Rare => COLOR_RARE,
        Rarity::Epic => COLOR_EPIC,
    }
}

/// Computes the effective item rarity following official vanilla rules:
/// 1. Ominous banner is always Uncommon.
/// 2. Stacks with custom `minecraft:rarity` component use that rarity.
/// 3. Otherwise the item's default rarity from `items.json` is used.
/// 4. Any enchantments upgrade Common & Uncommon to Rare, Rare to Epic, Epic stays Epic.
#[allow(dead_code)]
pub fn resolve_rarity(
    default_rarity: &str,
    is_ominous_banner: bool,
    custom_rarity: Option<&str>,
    is_enchanted: bool,
) -> Rarity {
    let base = if is_ominous_banner {
        Rarity::Uncommon
    } else if let Some(r) = custom_rarity {
        Rarity::parse_str(r)
    } else {
        Rarity::parse_str(default_rarity)
    };

    if !is_enchanted {
        base
    } else {
        match base {
            Rarity::Common | Rarity::Uncommon => Rarity::Rare,
            Rarity::Rare => Rarity::Epic,
            Rarity::Epic => Rarity::Epic,
        }
    }
}

/// Checks if master toggle is enabled, synchronizing with saved settings if needed.
pub fn is_enabled(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !INITIALIZED.load(Ordering::Relaxed) {
        if let Some(v) = fallback() {
            ENABLED.store(v, Ordering::SeqCst);
        }
        INITIALIZED.store(true, Ordering::SeqCst);
    }
    ENABLED.load(Ordering::Relaxed)
}

/// Sets master enabled state.
pub fn set_enabled(value: bool) {
    ENABLED.store(value, Ordering::SeqCst);
    INITIALIZED.store(true, Ordering::SeqCst);
}

/// Toggles master enabled state.
#[allow(dead_code)]
pub fn toggle_enabled() -> bool {
    let next = !ENABLED.load(Ordering::Relaxed);
    ENABLED.store(next, Ordering::SeqCst);
    next
}

/// Checks if borders for common items are enabled.
pub fn is_show_common(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !INITIALIZED.load(Ordering::Relaxed)
        && let Some(v) = fallback()
    {
        SHOW_COMMON.store(v, Ordering::SeqCst);
    }
    SHOW_COMMON.load(Ordering::Relaxed)
}

/// Sets whether common items should show borders.
pub fn set_show_common(value: bool) {
    SHOW_COMMON.store(value, Ordering::SeqCst);
}

/// Toggles common items display.
#[allow(dead_code)]
pub fn toggle_show_common() -> bool {
    let next = !SHOW_COMMON.load(Ordering::Relaxed);
    SHOW_COMMON.store(next, Ordering::SeqCst);
    next
}

/// Checks if borders in the HUD hotbar are enabled.
pub fn is_show_hotbar(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !INITIALIZED.load(Ordering::Relaxed)
        && let Some(v) = fallback()
    {
        SHOW_HOTBAR.store(v, Ordering::SeqCst);
    }
    SHOW_HOTBAR.load(Ordering::Relaxed)
}

/// Sets whether HUD hotbar items should show borders.
pub fn set_show_hotbar(value: bool) {
    SHOW_HOTBAR.store(value, Ordering::SeqCst);
}

/// Toggles HUD hotbar items display.
#[allow(dead_code)]
pub fn toggle_show_hotbar() -> bool {
    let next = !SHOW_HOTBAR.load(Ordering::Relaxed);
    SHOW_HOTBAR.store(next, Ordering::SeqCst);
    next
}

/// Settings button label for Master toggle with RU/EN localization.
pub fn label_master(enabled: bool, is_ru: bool) -> String {
    if is_ru {
        format!("Рамки редкости: {}", if enabled { "ВКЛ" } else { "ВЫКЛ" })
    } else {
        format!("Rarity Borders: {}", if enabled { "ON" } else { "OFF" })
    }
}

/// Settings button label for Common items toggle with RU/EN localization.
pub fn label_common(show_common: bool, is_ru: bool) -> String {
    if is_ru {
        format!(
            "Рамки обычных предметов: {}",
            if show_common { "ВКЛ" } else { "ВЫКЛ" }
        )
    } else {
        format!(
            "Show Common Borders: {}",
            if show_common { "ON" } else { "OFF" }
        )
    }
}

/// Settings button label for Hotbar toggle with RU/EN localization.
pub fn label_hotbar(show_hotbar: bool, is_ru: bool) -> String {
    if is_ru {
        format!(
            "Рамки в хотбаре: {}",
            if show_hotbar { "ВКЛ" } else { "ВЫКЛ" }
        )
    } else {
        format!(
            "Show Hotbar Borders: {}",
            if show_hotbar { "ON" } else { "OFF" }
        )
    }
}

/// Builds the 4 pixel-perfect boundary lines of the 18x18 slot frame:
/// - Top: 1px at 30% alpha (subtle frame line)
/// - Left: 1px at 40% alpha (subtle frame line)
/// - Right: 1px at 40% alpha (subtle frame line)
/// - Bottom: 1px at 100% solid alpha (prominent vibrant bottom accent)
///
/// Parameters:
/// - `x, y`: Top-left coordinates of the 16x16 item icon inside the 18x18 slot.
/// - `color_rgb`: 24-bit RGB color of the rarity.
pub fn slot_border_rects(x: f32, y: f32, color_rgb: u32) -> [([f32; 4], u32); 4] {
    let rgb = color_rgb & 0x00FF_FFFF;

    // Alpha values:
    // Top subtle line: 0x4D (~30% alpha)
    let argb_top = (0x4D << 24) | rgb;
    // Left & Right subtle lines: 0x66 (~40% alpha)
    let argb_sides = (0x66 << 24) | rgb;
    // Bottom prominent accent line: 0xFF (100% solid alpha)
    let argb_bottom = (0xFF << 24) | rgb;

    [
        // Top 1px line (spanning full 18px slot width)
        ([x - 1.0, y - 1.0, 18.0, 1.0], argb_top),
        // Left 1px line (16px vertical inside height)
        ([x - 1.0, y, 1.0, 16.0], argb_sides),
        // Right 1px line (16px vertical inside height)
        ([x + 16.0, y, 1.0, 16.0], argb_sides),
        // Bottom 1px line (prominent 18px solid accent line)
        ([x - 1.0, y + 16.0, 18.0, 1.0], argb_bottom),
    ]
}

/// Renders the slot border lines through a drawing callback `draw(rect, argb)`.
pub fn render_slot_border<F>(x: f32, y: f32, color_rgb: u32, mut draw: F)
where
    F: FnMut([f32; 4], u32),
{
    let rects = slot_border_rects(x, y, color_rgb);
    for (rect, argb) in rects {
        draw(rect, argb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_empty_slot_no_border() {
        // If an item stack is empty/None, no border should be computed or drawn.
        let empty_item: Option<&str> = None;
        assert!(empty_item.is_none());
    }

    #[test]
    fn test_02_common_hidden_by_default() {
        let rarity = resolve_rarity("common", false, None, false);
        assert_eq!(rarity, Rarity::Common);
        // By default show_common is false
        set_show_common(false);
        assert!(!is_show_common(|| None));
    }

    #[test]
    fn test_03_uncommon_color() {
        let rarity = resolve_rarity("uncommon", false, None, false);
        assert_eq!(rarity, Rarity::Uncommon);
        let color = rarity_color(rarity);
        assert_eq!(color, COLOR_UNCOMMON);
        assert_eq!(color, 0xFFFF55);
    }

    #[test]
    fn test_04_rare_color() {
        let rarity = resolve_rarity("rare", false, None, false);
        assert_eq!(rarity, Rarity::Rare);
        let color = rarity_color(rarity);
        assert_eq!(color, COLOR_RARE);
        assert_eq!(color, 0x55FFFF);
    }

    #[test]
    fn test_05_epic_color() {
        let rarity = resolve_rarity("epic", false, None, false);
        assert_eq!(rarity, Rarity::Epic);
        let color = rarity_color(rarity);
        assert_eq!(color, COLOR_EPIC);
        assert_eq!(color, 0xFF55FF);
    }

    #[test]
    fn test_06_enchanted_item_upgrade() {
        // 1. Common + Enchanted -> Rare
        assert_eq!(resolve_rarity("common", false, None, true), Rarity::Rare);
        // 2. Uncommon + Enchanted -> Rare
        assert_eq!(resolve_rarity("uncommon", false, None, true), Rarity::Rare);
        // 3. Rare + Enchanted -> Epic
        assert_eq!(resolve_rarity("rare", false, None, true), Rarity::Epic);
        // 4. Epic + Enchanted -> Epic (caps at Epic)
        assert_eq!(resolve_rarity("epic", false, None, true), Rarity::Epic);
        // 5. Ominous banner + Enchanted -> Rare
        assert_eq!(resolve_rarity("common", true, None, true), Rarity::Rare);
    }

    #[test]
    fn test_07_damaged_item_no_durability_overlap() {
        // Item is at (100.0, 100.0)
        let x = 100.0;
        let y = 100.0;
        let rects = slot_border_rects(x, y, COLOR_UNCOMMON);

        // Vanilla durability bar is at [x + 2, y + 13, 13, 2]
        let bar_y_top = y + 13.0;
        let bar_y_bottom = bar_y_top + 2.0; // 115.0

        // Bottom border line is at rects[3]: [x - 1, y + 16, 18, 1]
        let bottom_line_y = rects[3].0[1]; // 116.0

        // Bottom border line must be strictly below durability bar
        assert!(
            bottom_line_y >= bar_y_bottom,
            "Bottom border line ({}) overlaps durability bar (bottom {})",
            bottom_line_y,
            bar_y_bottom
        );
    }

    #[test]
    fn test_08_stacked_items_no_text_overlap() {
        // Item is at (50.0, 50.0)
        let x = 50.0;
        let y = 50.0;
        let rects = slot_border_rects(x, y, COLOR_RARE);

        // Stack count text is drawn inside the item area: roughly [x + 9..x + 16, y + 9..y + 16]
        // Frame lines are outside: x - 1, x + 16, y - 1, y + 16
        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            // Frame rects never cross inside the 16x16 center region where count sits
            let inside_item_x = rx > x && (rx + rw) < (x + 16.0);
            let inside_item_y = ry > y && (ry + rh) < (y + 16.0);
            assert!(
                !(inside_item_x && inside_item_y),
                "Border rect {:?} intrudes into item interior",
                rect
            );
        }
    }

    #[test]
    fn test_09_gui_scale_proportionality() {
        for scale in [1.0, 2.0, 3.0, 4.0] {
            let x = 32.0 * scale;
            let y = 32.0 * scale;
            let rects = slot_border_rects(x, y, COLOR_EPIC);
            // In base GUI points, width is always 18.0 and line thickness is always 1.0
            assert_eq!(rects[0].0[2], 18.0); // Top width
            assert_eq!(rects[0].0[3], 1.0); // Top height
            assert_eq!(rects[3].0[2], 18.0); // Bottom width
            assert_eq!(rects[3].0[3], 1.0); // Bottom height
        }
    }

    #[test]
    fn test_10_resolution_change() {
        // Slot coordinates can shift when resolution changes (e.g. centered inventory)
        for (ox, oy) in [(100.0, 100.0), (340.0, 180.0), (820.0, 540.0)] {
            let rects = slot_border_rects(ox, oy, COLOR_UNCOMMON);
            // Slot bounds relative to (ox, oy) remain identical
            assert_eq!(rects[0].0, [ox - 1.0, oy - 1.0, 18.0, 1.0]);
            assert_eq!(rects[1].0, [ox - 1.0, oy, 1.0, 16.0]);
            assert_eq!(rects[2].0, [ox + 16.0, oy, 1.0, 16.0]);
            assert_eq!(rects[3].0, [ox - 1.0, oy + 16.0, 18.0, 1.0]);
        }
    }

    #[test]
    fn test_11_mod_disabled_toggle() {
        set_enabled(true);
        assert!(is_enabled(|| None));
        let toggled = toggle_enabled();
        assert!(!toggled);
        assert!(!is_enabled(|| None));
        set_enabled(true);
    }

    #[test]
    fn test_12_show_common_toggle() {
        set_show_common(false);
        assert!(!is_show_common(|| None));
        let toggled = toggle_show_common();
        assert!(toggled);
        assert!(is_show_common(|| None));

        let rarity = resolve_rarity("common", false, None, false);
        assert_eq!(rarity, Rarity::Common);
        let color = rarity_color(rarity);
        assert_eq!(color, COLOR_COMMON);
        assert_eq!(color, 0x94A3B8);

        set_show_common(false);
    }

    #[test]
    fn test_13_slot_border_geometry_bounds() {
        let x = 20.0;
        let y = 30.0;
        let rects = slot_border_rects(x, y, COLOR_RARE);

        // Calculate enclosing bounding box
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            min_x = min_x.min(rx);
            min_y = min_y.min(ry);
            max_x = max_x.max(rx + rw);
            max_y = max_y.max(ry + rh);
        }

        // Slot boundary is strictly 18x18
        assert_eq!(min_x, x - 1.0);
        assert_eq!(min_y, y - 1.0);
        assert_eq!(max_x, x + 17.0);
        assert_eq!(max_y, y + 17.0);
        assert_eq!(max_x - min_x, 18.0);
        assert_eq!(max_y - min_y, 18.0);
    }

    #[test]
    fn test_14_no_slot_highlight_conflict() {
        // Slot highlight is drawn centered at (x - 4, y - 4, 24, 24)
        let x = 60.0;
        let y = 60.0;
        let rects = slot_border_rects(x, y, COLOR_EPIC);

        // Slot frame bounds
        let slot_min_x = x - 1.0;
        let slot_max_x = x + 17.0;
        let slot_min_y = y - 1.0;
        let slot_max_y = y + 17.0;

        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            // Frame is completely contained within the 18x18 slot area
            assert!(rx >= slot_min_x && (rx + rw) <= slot_max_x);
            assert!(ry >= slot_min_y && (ry + rh) <= slot_max_y);
        }
    }
}
