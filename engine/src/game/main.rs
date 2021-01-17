// This is the main controller for setting up the game.

use std::collections::HashMap;

use super::view::{Belt, Building, BuildingState, Material, UserAction, GAME_STATE, WORLD};
use crate::engine::{
    debug, rng, shortest_path, Cell, CellCoord, Layer, Layout, Point, Shape, Sprite, Texture,
    TextureBorder, UserEvent,
};

use wasm_bindgen::prelude::*;

pub fn run() -> Result<(), JsValue> {
    WORLD.with(|w| -> Result<(), JsValue> {
        debug(format!("generating world"));

        let mut cell_ids: Vec<String> = Vec::new();
        let mut cell_coords: Vec<CellCoord> = Vec::new();
        let mut building_sprite_ids: Vec<String> = Vec::new();

        let mut bg_layer = Layer::new("background");
        let mut building_layer = Layer::new("buildings");
        let mut belt_layer = Layer::new("belts");
        let belt_preview_layer = Layer::new("belt_preview");

        // Background
        {
            GAME_STATE.with(|game_state| {
                let cell_shape = Shape::Cell;

                for coord in &game_state.borrow().world {
                    let cell = Cell::new(coord.x as f32, coord.y as f32, coord.z as f32);
                    let position = w.borrow().layout.cell_to_pixel(&cell);
                    let texture = {
                        let mut tex = Texture::new();
                        let tex_border = TextureBorder::new(1, "black");
                        tex.border = Some(tex_border);
                        tex.background_color = Some("lime".to_owned());
                        tex
                    };
                    let sprite_id = rng::uid().to_string();
                    let sprite = Sprite::new(&sprite_id, &cell_shape, &position, &texture);
                    cell_ids.push(sprite.id().to_owned());
                    cell_coords.push(cell.coord());
                    bg_layer.sprites.insert(cell.coord(), sprite);
                }
            });
        }

        // Buildings
        {
            let building_shape = Shape::Rectangle {
                width: 15,
                height: 15,
            };

            let cell = Cell::new(-1.0, -1.0, 2.0);
            let position = w.borrow().layout.cell_to_pixel(&cell);

            let texture = {
                let mut tex = Texture::new();
                //let tex_border = TextureBorder::new(1, "black");
                //tex.border = Some(tex_border);
                //tex.background_color = Some("brown".to_owned());
                tex.image = Some("game/textures/factory.svg".to_owned());
                tex
            };
            let sprite_id = rng::uid().to_string();
            let building_sprite = Sprite::new(&sprite_id, &building_shape, &position, &texture);

            building_sprite_ids.push(sprite_id.to_owned());

            building_layer.sprites.insert(cell.coord(), building_sprite);

            GAME_STATE.with(|game_state| {
                game_state.borrow_mut().add_building(Building::new(
                    cell.coord(),
                    HashMap::new(),
                    BuildingState::Disconnected,
                ));
            });
        }
        // Belts
        {
            let belt_shape = Shape::Cell;

            let cell = Cell::new(-2.0, -1.0, 3.0);
            let position = w.borrow().layout.cell_to_pixel(&cell);

            let texture = {
                let mut tex = Texture::new();
                let tex_border = TextureBorder::new(1, "black");
                tex.border = Some(tex_border);
                tex.background_color = Some("gray".to_owned());
                tex
            };
            let sprite_id = rng::uid().to_string();
            let belt_sprite = Sprite::new(&sprite_id, &belt_shape, &position, &texture);

            belt_layer.sprites.insert(cell.coord(), belt_sprite);

            GAME_STATE.with(|game_state| {
                let mut contents: HashMap<CellCoord, Option<Material>> = HashMap::new();
                contents.insert(cell.coord().clone(), None);
                game_state.borrow_mut().add_belt(Belt::new(contents));
            });
        }

        w.borrow_mut().viewport.insert_layer(0, bg_layer);
        w.borrow_mut().viewport.insert_layer(1, building_layer);
        w.borrow_mut().viewport.insert_layer(2, belt_layer);
        w.borrow_mut().viewport.insert_layer(3, belt_preview_layer);

        w.borrow_mut().render()?;

        // Attach an event to a building.
        for building_sprite_id in &building_sprite_ids {
            let id = building_sprite_id.to_owned();
            Sprite::on(&building_sprite_id, UserEvent::MouseClick, move |_| {
                //alert_js(format!("You clicked building {:?}", id));
            })?;
        }

        // Attach event to the world to find the cell that was clicked.
        w.borrow()
            .on_mouse_event(UserEvent::MouseClick, |event: web_sys::MouseEvent| {
                WORLD
                    .with(|w| -> Result<(), JsValue> {
                        let cell = w.borrow().event_cell(&event);

                        //alert_js(format!("Hello from cell {:?}", cell));

                        Ok(())
                    })
                    .expect("mouse event failed");
            })?;

        w.borrow()
            .on_mouse_event(UserEvent::MouseDown, |event: web_sys::MouseEvent| {
                WORLD.with(|w| {
                    let cell = w
                        .borrow()
                        .event_cell(&event)
                        .expect("failed to get event cell");

                    // FIXME: Need to check if starting from building node. Otherwise, every mouse down will start placing a belt.
                    GAME_STATE.with(|game_state| {
                        let current_action = &mut game_state.borrow_mut().current_action;
                        match *current_action {
                            Some(UserAction::PlacingBelt {
                                begin: _,
                                end: _,
                                previous_check: _,
                            }) => {}
                            _ => {
                                *current_action = Some(UserAction::PlacingBelt {
                                    begin: cell.coord(),
                                    end: Some(cell.coord()),
                                    previous_check: cell.coord(),
                                });
                                debug(format!("starting new belt path"));
                            }
                        };
                    });
                })
            })?;

        w.borrow()
            .on_mouse_event(UserEvent::MouseMove, |event: web_sys::MouseEvent| {
                WORLD.with(|w| {
                    let cell = w
                        .borrow()
                        .event_cell(&event)
                        .expect("failed to get event cell");

                    // FIXME: Need to check if starting from building node. Otherwise, every mouse down will start placing a belt.
                    GAME_STATE.with(|game_state| {
                        let belt_preview = match game_state.borrow().current_action {
                            Some(UserAction::PlacingBelt {
                                begin,
                                end,
                                previous_check,
                            }) => Some((begin, end, previous_check)),
                            None => None,
                        };

                        if let Some((begin, mut end, mut previous_check)) = belt_preview {
                            if previous_check != cell.coord() {
                                let begin_cell = w.borrow().layout.cell_from_coord(&begin);
                                let current_end = cell.clone();
                                let path = shortest_path(
                                    &begin_cell,
                                    &current_end,
                                    &game_state.borrow().world,
                                    &game_state.borrow().collisions(),
                                );

                                if let Some(p) = path {
                                    debug(format!(
                                        "new belt path found {:?} {:?}",
                                        end,
                                        cell.coord()
                                    ));

                                    // Update end to the current cell.
                                    end = Some(current_end.coord());

                                    show_belt_preview(&p);
                                } else {
                                    // Update end with None to show that the current cell is not a valid path.
                                    end = None;
                                }

                                // Update previous check with the current cell so we do not check again until a new cell is entered.
                                previous_check = current_end.coord();

                                game_state.borrow_mut().current_action =
                                    Some(UserAction::PlacingBelt {
                                        begin: begin,
                                        end: end,
                                        previous_check: previous_check,
                                    });
                            }
                        }
                    });
                })
            })?;

        w.borrow()
            .on_mouse_event(UserEvent::MouseUp, |event: web_sys::MouseEvent| {
                WORLD.with(|w| {
                    //let cell = w.borrow().event_cell(&event);

                    GAME_STATE.with(|game_state| {
                        // TODO: Place belt along the shortest path.
                        let belt_preview = match game_state.borrow().current_action {
                            Some(UserAction::PlacingBelt {
                                begin,
                                end,
                                previous_check,
                            }) => Some((begin, end, previous_check)),
                            None => None,
                        };

                        if let Some((begin, end, _)) = belt_preview {
                            if end != None {
                                let begin_cell = w.borrow().layout.cell_from_coord(&begin);
                                let end_cell = w
                                    .borrow()
                                    .layout
                                    .cell_from_coord(&end.expect("end belt path is None"));
                                let path = shortest_path(
                                    &begin_cell,
                                    &end_cell,
                                    &game_state.borrow().world,
                                    &game_state.borrow().collisions(),
                                );

                                if let Some(p) = path {
                                    // TODO: Draw final belt.
                                    debug(format!("drawing belt {:?}", p));

                                    // Look at the end of the belt.
                                    let end_position = w
                                        .borrow()
                                        .layout
                                        .pixel_from_coord(&end.expect("end belt path is None"));
                                    // Look at the end of the belt.
                                    w.borrow_mut()
                                        .look_at(&end_position)
                                        .expect("failed to look at end position");
                                    debug(format!("looking at {:?}", &end_position));
                                    w.borrow_mut().render().expect("failed to render");
                                } else {
                                    debug(format!("no valid path"));
                                }
                            }
                        }

                        game_state.borrow_mut().current_action = None;
                        w.borrow_mut()
                            .viewport
                            .clear_layer("belt_preview")
                            .expect("failed to clear belt_preview");
                    });
                })
            })?;

        Ok(())
    })?;

    Ok(())
}

fn show_belt_preview<C>(path: &[C])
where
    C: Cell,
{
    WORLD.with(|w| {
        w.borrow_mut().clear_layer("belt_preview");
    });

    let belt_shape = Shape::Cell;

    for c in path {
        // This works because it is able to infer the type of Cell from the function calls.
        let cell = Cell::new(c.coord().x as f32, c.coord().y as f32, c.coord().z as f32);

        let position = WORLD.with(|w| -> Point { w.borrow().layout.cell_to_pixel(&cell) });

        let texture = {
            let mut tex = Texture::new();
            let tex_border = TextureBorder::new(1, "black");
            tex.border = Some(tex_border);
            tex.background_color = Some("gray".to_owned());
            tex
        };
        let sprite_id = rng::uid().to_string();
        let belt_preview = Sprite::new(&sprite_id, &belt_shape, &position, &texture);

        WORLD.with(|w| {
            w.borrow_mut()
                .viewport
                .layer_mut("belt_preview")
                .unwrap()
                .add_sprite(cell.coord(), belt_preview);
        });
    }

    WORLD.with(|w| {
        w.borrow_mut().render_layer("belt_preview");
    });
}
