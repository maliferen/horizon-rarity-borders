fn rarity_borders_decorations(
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
    x: f32,
    y: f32,
    batch: &mut crate::hud::HudBatch,
) {
    use crate::game::rarity_borders as m;

    if !m::is_enabled(|| game.native_setting(m::ID, "enabled").map(|v| v != "false")) {
        return;
    }

    // Skip in-game HUD hotbar unless hotbar borders are enabled
    if !m::is_show_hotbar(|| game.native_setting(m::ID, "hotbar").map(|v| v == "true"))
        && game.screen == crate::game::Screen::Playing
    {
        return;
    }

    // Skip floating carried item moving with cursor
    if game.carried.is_some()
        && (x - (game.cursor.0 - 8.0)).abs() < 0.5
        && (y - (game.cursor.1 - 8.0)).abs() < 0.5
    {
        return;
    }

    let default_rarity = game.item_rarity(&stack.item);
    let rarity_str = crate::item_tooltip::rarity(stack, &default_rarity);
    let rarity = m::Rarity::parse_str(rarity_str);

    let show_common =
        m::is_show_common(|| game.native_setting(m::ID, "common").map(|v| v == "true"));
    if !show_common && rarity == m::Rarity::Common {
        return;
    }

    let color = m::rarity_color(rarity);
    m::render_slot_border(x, y, color, |rect, argb| {
        game.native_fill(batch, rect, argb);
    });
}
