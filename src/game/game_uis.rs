use crate::{
    constants::ItemType,
    game::*,
    game_ui::{ContainerNeighbors, GameUIElements, NeighborDirection},
    inventory::Inventory,
};

pub enum PlayerInventoryPage {
    Survival,
    Creative,
}

impl Game {
    pub fn player_inventory_loop(&mut self, page: PlayerInventoryPage) -> GameState {
        match page {
            PlayerInventoryPage::Survival => self.player_inventory_survival_loop(),
            PlayerInventoryPage::Creative => self.player_inventory_creative_loop(),
        }

        GameState::InGame
    }

    fn player_inventory_survival_loop(&mut self) {
        // Clear the hud
        self.renderer
            .draw_game(&mut self.world, &self.player, 0, &self.hud, false);

        let inventories = [
            &mut self.player.inventory,
            &mut self.crafting_manager.crafting_inventory_2x2,
        ];

        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(65, 86), 6, 3, 0, 0, 6)
            .with_slot_grid(Vector2::new(65, 184), 6, 1, 0, 18, 0)
            .with_slot_grid(Vector2::new(97, 16), 2, 2, 1, 24, 0)
            .with_element(
                GameUIElements::create_one_way_slot_slot(1, 4),
                Vector2::new(193, 32),
                28,
                ContainerNeighbors::default(),
            )
            .with_element(
                GameUIElements::Arrow { filling: 0. },
                Vector2::new(161, 32),
                29,
                ContainerNeighbors::default(),
            )
            .with_links(&[
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
                (26, 1, NeighborDirection::Bottom),
                (27, 2, NeighborDirection::Bottom),
                (28, 4, NeighborDirection::Bottom),
                (25, 28, NeighborDirection::Right),
                (27, 28, NeighborDirection::Right),
            ])
            .sync(&inventories);

        ui.selected_amount = None;

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);
            self.crafting_manager.update_2x2();

            let mut inventories = [
                &mut self.player.inventory,
                &mut self.crafting_manager.crafting_inventory_2x2,
            ];

            if !ui.update(&self.input_manager, &mut inventories) {
                // Bring the items back in the inventory
                for slot in 0..4 {
                    let item_stack = inventories[1].get_all_slots()[slot].clone();
                    if item_stack.get_item_type() != ItemType::Air {
                        let remaining = inventories[0].add_item_stack(item_stack);
                        if remaining != 0 {
                            // Hum... wait?!
                            // I have no choice... Spawn the item.
                            // I should be carreful about duplication here...
                            let pos = self.world.get_player_entity().pos;
                            self.world.spawn_item_entity(
                                pos,
                                ItemStack::new(item_stack.get_item_type(), remaining, false),
                            );
                        }
                    }
                }

                // Then clear the crafting inventory.
                inventories[1].fill(ItemStack::void());
                break;
            }

            self.renderer.draw_game_ui(&mut ui);

            nadk::display::wait_for_vblank();
            nadk::time::wait_milliseconds(50);
        }
    }

    fn player_inventory_creative_loop(&mut self) {
        // Clear the hud
        self.renderer
            .draw_game(&mut self.world, &self.player, 0, &self.hud, false);

        // ── Creative inventory contents ───────────────────────────────────────
        // Add new items here in the future: just push more ItemStacks.
        // The scroll system handles any number of items automatically.
        let creative_items: &[crate::constants::ItemType] = &[
            crate::constants::ItemType::StoneBlock,
            crate::constants::ItemType::DirtBlock,
            crate::constants::ItemType::GrassBlock,
            crate::constants::ItemType::SandBlock,
            crate::constants::ItemType::CobblestoneBlock,
            crate::constants::ItemType::BorderBlock,
            crate::constants::ItemType::LogBlock,
            crate::constants::ItemType::LeavesBlock,
            crate::constants::ItemType::PlanksBlock,
            crate::constants::ItemType::RedBlock,
            crate::constants::ItemType::OrangeBlock,
            crate::constants::ItemType::YellowBlock,
            crate::constants::ItemType::LimeBlock,
            crate::constants::ItemType::CyanBlock,
            crate::constants::ItemType::BlueBlock,
            crate::constants::ItemType::PurpleBlock,
            crate::constants::ItemType::MagentaBlock,
            crate::constants::ItemType::PinkBlock,
            crate::constants::ItemType::WhiteBlock,
            crate::constants::ItemType::GrayBlock,
            crate::constants::ItemType::BlackBlock,
            crate::constants::ItemType::TntBlock,
            crate::constants::ItemType::FlintAndSteel,
            crate::constants::ItemType::PigSpawnEgg,
            crate::constants::ItemType::StairsBlock,
            crate::constants::ItemType::SlabBlock,
        ];

        // ── Scroll constants ──────────────────────────────────────────────────
        // The visible creative panel is 3 columns × VISIBLE_ROWS rows.
        // Shift+Down scrolls forward one row; Shift+Up scrolls back one row.
        const COLS: usize = 3;
        const VISIBLE_ROWS: usize = 6;
        const VISIBLE_SLOTS: usize = COLS * VISIBLE_ROWS; // 18 visible at once

        // Grid pixel origin (top-left of the creative panel)
        const GRID_X: u16 = 218;
        const GRID_Y: u16 = 9;

        // Total rows (rounded up so the last partial row is also reachable)
        let total_items = creative_items.len();
        let total_rows = (total_items + COLS - 1) / COLS;
        let max_scroll_row = if total_rows > VISIBLE_ROWS {
            total_rows - VISIBLE_ROWS
        } else {
            0
        };

        // Current scroll state: first visible row index (0 = top)
        let mut scroll_row: usize = 0;

        // ── Build a fixed-size creative inventory (one slot per visible cell) ─
        // We only ever expose VISIBLE_SLOTS slots to GameUI; we repopulate them
        // whenever the user scrolls.
        let mut creative_inventory = Inventory::new(VISIBLE_SLOTS);
        creative_inventory.fill(ItemStack::new(crate::constants::ItemType::Air, 0, true));

        // Helper closure to (re)fill the visible window from creative_items
        let fill_window =
            |inv: &mut Inventory, scroll: usize| {
                for vis_idx in 0..VISIBLE_SLOTS {
                    let item_idx = scroll * COLS + vis_idx;
                    let item_type = if item_idx < creative_items.len() {
                        creative_items[item_idx]
                    } else {
                        crate::constants::ItemType::Air
                    };
                    inv.replace_slot_item_stack(
                        vis_idx,
                        ItemStack::new(item_type, 1, true),
                    );
                }
            };

        fill_window(&mut creative_inventory, scroll_row);

        let mut inventories = [&mut self.player.inventory, &mut creative_inventory];

        // ── Build GameUI ──────────────────────────────────────────────────────
        // Layout:
        //   Left :  player inventory (6×3) slots UI 0-17
        //           hotbar           (6×1) slots UI 18-23
        //   Right:  creative panel   (3×6) slots UI 24-41
        //
        // Navigation links (right edge of player grid → left col of creative):
        //   UI  5 → UI 24  (player row 0 right edge → creative row 0 left)
        //   UI 11 → UI 27  (player row 1 right edge → creative row 1 left)
        //   UI 17 → UI 30  (player row 2 right edge → creative row 2 left)
        //   UI 23 → UI 33  (hotbar right edge       → creative row 3 left)
        let mut ui = GameUI::new(true)
            .with_slot_grid(Vector2::new(10, 41),   6, 3, 0,  0, 6)  // player inv
            .with_slot_grid(Vector2::new(10, 139),  6, 1, 0, 18, 0)  // hotbar
            .with_slot_grid(
                Vector2::new(GRID_X, GRID_Y),
                COLS as u16,
                VISIBLE_ROWS as u16,
                1,
                24,
                0,
            )
            .with_links(&[
                // hotbar ↔ player inventory
                (12, 18, NeighborDirection::Bottom),
                (13, 19, NeighborDirection::Bottom),
                (14, 20, NeighborDirection::Bottom),
                (15, 21, NeighborDirection::Bottom),
                (16, 22, NeighborDirection::Bottom),
                (17, 23, NeighborDirection::Bottom),
                // player rows → creative grid
                ( 5, 24, NeighborDirection::Right),
                (11, 27, NeighborDirection::Right),
                (17, 30, NeighborDirection::Right),
                (23, 33, NeighborDirection::Right),
            ])
            .sync(&inventories);

        ui.selected_amount = None;

        self.timing_manager.reset();

        loop {
            self.input_manager.update();
            self.timing_manager.update();
            self.input_manager.update_timing(&self.timing_manager);

            // ── Scroll handling (Shift + Up/Down) ────────────────────────────
            let shift_held = self.input_manager.is_keydown(nadk::keyboard::Key::Shift);
            let scroll_changed = if shift_held
                && self.input_manager.is_just_pressed(nadk::keyboard::Key::Down)
                && scroll_row < max_scroll_row
            {
                scroll_row += 1;
                true
            } else if shift_held
                && self.input_manager.is_just_pressed(nadk::keyboard::Key::Up)
                && scroll_row > 0
            {
                scroll_row -= 1;
                true
            } else {
                false
            };

            if scroll_changed {
                // Repopulate the visible window with the new row offset
                fill_window(&mut inventories[1], scroll_row);

                // Move the UI cursor back into the visible area so it doesn't
                // get stranded on a now-invisible slot
                let cursor_ui = ui.cursor_id;
                if cursor_ui >= 24 && cursor_ui < 24 + VISIBLE_SLOTS {
                    // Keep the cursor on the same visual column/row position
                    // (clamp to last row if we scrolled past it)
                    let vis_idx = cursor_ui - 24;
                    let row_in_panel = vis_idx / COLS;
                    let col_in_panel = vis_idx % COLS;
                    let clamped_row = row_in_panel.min(VISIBLE_ROWS - 1);
                    ui.cursor_id = 24 + clamped_row * COLS + col_in_panel;
                }

                ui.need_complete_redraw = true;
                // Update slot display from the freshly-populated inventory
                ui.sync_mut(&mut inventories);
            }

            if !ui.update(&self.input_manager, &mut inventories) {
                break;
            }

            // Draw a small scroll indicator: "▲ Shift+↑/↓ ▼" on the right side
            if max_scroll_row > 0 {
                use crate::nadk::display::{ScreenPoint, draw_string};
                use crate::nadk::display::Color565;
                let indicator = if scroll_row == 0 {
                    "Shift+v"
                } else if scroll_row >= max_scroll_row {
                    "Shift+^"
                } else {
                    "Sh ^/v"
                };
                draw_string(
                    indicator,
                    ScreenPoint { x: GRID_X, y: GRID_Y + (VISIBLE_ROWS as u16) * 32 + 2 },
                    false,
                    Color565::from_rgb888(220, 220, 220),
                    Color565::from_rgb888(0, 0, 0),
                );
            }

            self.renderer.draw_game_ui(&mut ui);

            nadk::display::wait_for_vblank();
            nadk::time::wait_milliseconds(50);
        }
    }
}
