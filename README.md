# Rarity Borders

**Summary:** Adds colored borders to inventory items based on their rarity in Horizon Minecraft.

## Description

**Rarity Borders** makes rare and enchanted items instantly recognizable at a glance by displaying crisp, pixel-perfect borders around inventory slots.

The mod is designed to feel like an authentic, native part of Minecraft: sharp 90-degree right angles, a prominent vibrant bottom accent line, and subtle 1px frames that never obscure the item icon, durability bar, or stack count.

## Features

- **Color-Coded Rarity Tiers:**
  - **Common:** Hidden by default (optional subtle slate border `#94A3B8`).
  - **Uncommon:** Soft golden yellow (`#FFFF55`).
  - **Rare:** Crisp sky aqua / cyan (`#55FFFF`).
  - **Epic:** Soft magenta / purple (`#FF55FF`).
- **Authentic Vanilla Rarity:** Automatically calculates rarity from custom `minecraft:rarity` components, default item types, ominous banners, and enchantment upgrades (enchanted Common/Uncommon items become Rare; enchanted Rare items become Epic).
- **Clean Vanilla Aesthetic:** 1px pixel-perfect geometry, prominent solid bottom line accent, zero bloom, zero glowing gradients, and zero circular highlights.
- **Shared Decoration Pipeline:** Hooks directly into Horizon's native item decoration renderer (`native_item_decorations`), naturally covering player inventory slots, container screens (chests, barrels, furnaces, crafting grids), and optionally the in-game HUD hotbar.
- **In-Game Settings:** Full live toggles in `Options → Mods → Rarity Borders` without requiring game restarts.
- **Lightweight & Fast:** Pure client-side Rust implementation with zero per-frame heap allocations.

## Configuration & How to Disable

You can customize or disable the mod directly from the in-game menus at any time:

1. Press **Esc** and click **Options** (Настройки).
2. Click **Mods** (Моды) and select **Rarity Borders**.
3. Toggle any of the available options:
   - **Rarity Borders: ON / OFF** — Master switch to completely enable or disable all borders.
   - **Show Common Borders: ON / OFF** — Display a neutral slate-gray border for common items (default: OFF).
   - **Show Hotbar Borders: ON / OFF** — Display rarity borders on the in-game HUD hotbar while exploring the world (default: OFF).

Changes take effect immediately upon clicking without needing to restart the game.

## Installation

1. Download the latest `rarity_borders-0.1.1.hnmod` from [Releases](https://github.com/maliferen/horizon-rarity-borders/releases).
2. Place the `.hnmod` file into your instance's `horizon-native-mods/` directory:
   - **Windows:** `%APPDATA%\.horizon\instances\<instance>\horizon-native-mods\`
   - **Linux:** `~/.local/share/horizon/instances/<instance>/horizon-native-mods/`
3. Launch Horizon Minecraft and enjoy!

## Compatibility

- **Minecraft Version:** 26.3
- **Horizon API:** Native Hooks API 139+
- **Ecosystem Integration:** Runs alongside other Horizon native mods (Durability Tooltip, Rider HUD, Screenshot Clipboard) without interference, as slot decoration rendering operates independently from tooltip and HUD overlay hooks.
- **Client-Side Only:** No server-side components required. Completely safe for vanilla servers.

## License

This project is licensed under the [MIT License](LICENSE).
