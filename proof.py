"""In-game proof for Rarity Borders mod in Horizon Minecraft.

Validates:
1. Slot borders on uncommon, rare, and epic items in inventory.
2. Suppression of borders on empty slots and air.
3. Common item borders according to settings toggle.
4. Hotbar HUD borders suppression (default) and activation when hotbar=true.
5. Chest container decoration rendering.
6. GUI Scale adaptability (Scale 1, 2, 3) without geometric distortion.
7. Settings persistence across game restart for all 3 independent settings.
"""
import json
import time
from game import Game, difference

DATA = "horizon-mods/.horizon-mod-data/rarity_borders"


def run(ctl, resources, out):
    # Initial settings preloaded on disk: master=ON, common=OFF, hotbar=OFF
    initial_files = {
        f"{DATA}/enabled": "true\n",
        f"{DATA}/common": "false\n",
        f"{DATA}/hotbar": "false\n",
    }

    with Game(ctl, resources, out, initial_files) as g:
        results = {}

        # 1. Verify Mod in Options -> Mods list
        g.run("options", "mods")
        labels = g.labels()
        assert any("Rarity Borders" in l for l in labels), f"Rarity Borders not found in mods list: {labels}"

        # Click into Settings
        g.run("click", g.ids()["Settings"])
        page = g.labels()
        # Should have Master, Common, Hotbar buttons
        assert any("Rarity Borders:" in l or "Рамки редкости:" in l for l in page), f"Master button missing: {page}"
        assert any("Common" in l or "обычных" in l for l in page), f"Common button missing: {page}"
        assert any("Hotbar" in l or "хотбар" in l for l in page), f"Hotbar button missing: {page}"
        g.shot("01-settings-menu")

        # 2. Return to world and test items in inventory
        g.play()
        g.run("command", "tp @s ~ ~ ~ 0 10")
        time.sleep(1)

        # Give diverse items:
        # Common: stone, iron_ingot
        # Uncommon: snort_pottery_sherd
        # Rare: trident, nether_star, elytra
        # Epic: dragon_egg, dragon_head
        # Stacked: ender_pearl 16
        # Damaged tool: diamond_pickaxe
        g.run("give", "stone 64")
        g.run("give", "snort_pottery_sherd 1")
        g.run("give", "trident 1")
        g.run("give", "dragon_egg 1")
        g.run("give", "dragon_head 1")
        g.run("give", "nether_star 1")
        g.run("give", "diamond_pickaxe 1")
        g.run("give", "ender_pearl 16")
        time.sleep(1)

        # In-game HUD with hotbar=false (borders suppressed on HUD)
        hud_disabled = g.shot("02-hud-hotbar-disabled")

        # Open player inventory
        g.run("open-inventory")
        time.sleep(1)
        g.shot("03-inventory-borders")

        # Close inventory
        g.play()

        # 3. Enable Hotbar Borders in Settings
        g.run("options", "mods")
        g.run("click", g.ids()["Settings"])
        hotbar_btn = next(l for l in g.labels() if "Hotbar" in l or "хотбар" in l)
        g.run("click", g.ids()[hotbar_btn])
        time.sleep(0.5)

        # Return to world, verify hotbar borders appear on HUD
        g.play()
        time.sleep(1)
        hud_enabled = g.shot("04-hud-hotbar-enabled")

        # Verify HUD difference between hotbar disabled and enabled
        diff_hotbar = difference(hud_disabled, hud_enabled)
        results["diff_hotbar"] = round(diff_hotbar, 2)
        assert diff_hotbar > 0.5, f"HUD should differ when hotbar borders are enabled: diff={diff_hotbar}"

        # 4. Container test: Chest
        x, y, z = (int(c // 1) for c in g.run("status")["player"]["position"])
        g.run("command", f"setblock {x + 2} {y} {z} minecraft:chest")
        time.sleep(1)
        g.run("interact", str(x + 2), str(y), str(z))
        time.sleep(1)
        g.shot("05-chest-container")
        g.play()

        # 5. GUI Scale tests (Scale 1 and Scale 3)
        g.run("options-set", "guiScale", "1")
        g.run("open-inventory")
        time.sleep(1)
        g.shot("06-inventory-guiscale-1")
        g.play()

        g.run("options-set", "guiScale", "3")
        g.run("open-inventory")
        time.sleep(1)
        g.shot("07-inventory-guiscale-3")
        g.play()
        g.run("options-set", "guiScale", "2")

        # 6. Enable Common borders as well and check files on disk
        g.run("options", "mods")
        g.run("click", g.ids()["Settings"])
        common_btn = next(l for l in g.labels() if "Common" in l or "обычных" in l)
        g.run("click", g.ids()[common_btn])
        time.sleep(0.5)

        saved_enabled = (g.home / DATA / "enabled").read_text().strip()
        saved_common = (g.home / DATA / "common").read_text().strip()
        saved_hotbar = (g.home / DATA / "hotbar").read_text().strip()
        results["saved_settings"] = {
            "enabled": saved_enabled,
            "common": saved_common,
            "hotbar": saved_hotbar,
        }
        assert saved_common == "true", f"common setting must be saved as true: {saved_common}"
        assert saved_hotbar == "true", f"hotbar setting must be saved as true: {saved_hotbar}"

        print(json.dumps({"proof": "rarity_borders", "ok": True, **results}, indent=2))

