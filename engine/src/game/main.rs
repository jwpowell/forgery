// This is the main controller for setting up the game.

use std::cell::{Ref, RefCell};
use std::cmp;
use std::collections::HashMap;

use super::view::{BeltView, BuildingState, BuildingView, GameStateView, WORLD};
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
                    let sprite = Sprite::new(&cell_shape, &position, &texture);

                    cell_ids.push(sprite.id().to_owned());

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
            let building_sprite = Sprite::new(&building_shape, &position, &texture);

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
            let belt_sprite = Sprite::new(&belt_shape, &position, &texture);

            belt_layer.sprites.insert(cell.coord(), belt_sprite);

            w.borrow_mut().layers.push(belt_layer);
        }

        w.borrow().render(target_id)?;

        // Events

        for cell_id in cell_ids {
            let id = cell_id.to_owned();
            Sprite::on(&cell_id, UserEvent::MouseClick, move |_| {
                alert_js(format!("Hello from {}", id));
            })?;
        }

        Ok(())
    })?;

    Ok(())
}
