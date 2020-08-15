// This is the main controller for setting up the game.


use std::cell::{Ref, RefCell};
use std::cmp;
use std::collections::HashMap;

use super::view::{BeltView, BuildingState, BuildingView, GameStateView, WORLD};
use super::rng::{Rand};
use crate::engine::{
    alert_js, debug, Cell, CellCoord, Hex, HexLayout, HexOrientation, Layer, Layout, Point,
    Rectangle, Shape, Sprite, Texture, TextureBorder, UserEvent,
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

use web_sys::{Document, Element, Event, HtmlElement, MouseEvent};

pub fn run() -> Result<(), JsValue> {
    let target_id = "workspace";

    debug(format!("target_id: {:?}", target_id));

    //alert(format!("target_id: {}", target_id).as_str());
    /*
        let mut world = create_hex_world();

        WORLD.with(|w| {
            *w = RefCell::new(world);
        });

        world.generate_hexgon(10);

        world.render(target_id)?;
    */

    WORLD.with(|w| -> Result<(), JsValue> {
        debug(format!("generating world"));

        let mut cell_ids: Vec<String> = Vec::new();
        let mut cell_coords: Vec<CellCoord> = Vec::new();

        let mut rng = Rand::new(0);
        
        let mut building_sprite_ids: Vec<String> = Vec::new();

        // Background
        {
            let mut bg_layer = Layer::new("background");

            let radius = 10;
            let cell_shape = Shape::Cell;

            for q in -radius..=radius {
                let r1 = cmp::max(-radius, -q - radius);
                let r2 = cmp::min(radius, -q + radius);
                for r in r1..=r2 {
                    let cell = Cell::new(q as f32, r as f32, (-q - r) as f32);
                    let position = w.borrow().layout.cell_to_pixel(&cell);
                    let texture = {
                        let mut tex = Texture::new();
                        let tex_border = TextureBorder::new(1, "black");
                        tex.border = Some(tex_border);
                        tex.background_color = Some("lime".to_owned());
                        tex
                    };
                    let sprite_id = (rng.rand_range(1,1000000000) as f32 + position.x + position.y).to_string();
                    let sprite = Sprite::new(&sprite_id, &cell_shape, &position, &texture);
                    
                    cell_ids.push(sprite.id().to_owned());
                    cell_coords.push(cell.coord());

                    bg_layer.sprites.insert(cell.coord(), sprite);
                }
            }

            w.borrow_mut().layers.push(bg_layer);
        }

        // Buildings
        {
            let mut building_layer = Layer::new("buildings");

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
            let sprite_id = (rng.rand_range(1,1000000000) as f32 + position.x + position.y).to_string();
            let building_sprite = Sprite::new(&sprite_id, &building_shape, &position, &texture);

            building_sprite_ids.push(sprite_id.to_owned());

            building_layer.sprites.insert(cell.coord(), building_sprite);

            

            w.borrow_mut().layers.push(building_layer);
        }
        // Belts
        {
            let mut belt_layer = Layer::new("belts");

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
            let sprite_id = (rng.rand_range(1,1000000000) as f32 + position.x + position.y).to_string();
            let belt_sprite = Sprite::new(&sprite_id, &belt_shape, &position, &texture);

            belt_layer.sprites.insert(cell.coord(), belt_sprite);

            w.borrow_mut().layers.push(belt_layer);
        }


        w.borrow_mut().render(target_id)?;


        // Attach an event to a building.
        for building_sprite_id in &building_sprite_ids {

            let id = building_sprite_id.to_owned();
            Sprite::on(&building_sprite_id, UserEvent::MouseClick, move |_| {
                alert_js(format!("You clicked building {:?}", id));
            })?;

        }


        // Attach event to the world to find the cell that was clicked.
        w.borrow().on_mouse_event(UserEvent::MouseClick, |event: web_sys::MouseEvent| {
            WORLD.with(|w| -> Result<(), JsValue> {
                let cell = w.borrow().event_cell(&event);

                alert_js(format!("Hello from cell {:?}", cell));

                Ok(())
            }).expect("mouse event failed");
        })?;

        
/*
        // Events
        {
            let mut events_layer = Layer::new("events");

            let event_shape = Shape::Cell;

            let mut event_sprites: Vec<Sprite> = Vec::new();

            for cell_coord in &cell_coords {

                let cell = Cell::new(cell_coord.x as f32, cell_coord.y as f32, cell_coord.z as f32);
                let position = w.borrow().layout.cell_to_pixel(&cell);

                let sprite_id = (rng.rand_range(1,1000000000) as f32 + position.x + position.y).to_string();
                let event_sprite = Sprite::new(sprite_id, &event_shape, &position, &Texture::new());

                events_layer.sprites.insert(cell.coord(), event_sprite.clone());

                event_sprites.push(event_sprite);
            }

            w.borrow_mut().layers.push(events_layer);

            w.borrow().render(target_id)?;


            for event_sprite in &event_sprites {

                let id = event_sprite.position.clone();
                Sprite::on(&event_sprite.id(), UserEvent::MouseClick, move |_| {
                    alert_js(format!("Hello from {:?}", id));
                })?;
                
            }
            
        }
*/
        

        Ok(())
    })?;

    Ok(())
}
