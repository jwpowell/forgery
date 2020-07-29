use std::cell::{Ref, RefCell};
use std::cmp;
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use crate::cell::{Cell, CellCoord, Hex, Point, Rectangle};
use crate::game_view::{BeltView, BuildingState, BuildingView, GameStateView};
use crate::layout::{HexLayout, HexOrientation, Layout};
use crate::log;
use crate::world::WorldMap;

use web_sys::{Document, Element, Event, MouseEvent, SvgElement, SvgsvgElement};

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

//we can't have mutable statics by default so we use this to enable it
thread_local! {
    pub static WORLD: RefCell<WorldMap<Hex, HexLayout> > = RefCell::new(
        create_hex_world()
    );
}

fn create_hex_world() -> WorldMap<Hex, HexLayout> {
    let hex_layout = HexLayout::new(
        HexOrientation::flat(),
        Rectangle::new(10.0, 10.0),
        Point::new(0.0, 0.0),
    );

    let mut factory_1_nodes: HashMap<CellCoord, bool> = HashMap::new();
    factory_1_nodes.insert(CellCoord::new(1, 1, 1), false);

    let factory_1 = BuildingView::new(
        CellCoord::new(-4, -2, 6),
        factory_1_nodes,
        BuildingState::Working,
    );

    let mut factory_2_nodes: HashMap<CellCoord, bool> = HashMap::new();
    factory_2_nodes.insert(CellCoord::new(0, 1, 1), false);

    let factory_2 = BuildingView::new(
        CellCoord::new(-1, -4, 5),
        factory_2_nodes,
        BuildingState::Working,
    );

    let game_state_view = GameStateView::new(&[factory_1, factory_2], &[]);

    let hex_world = WorldMap::new(hex_layout, game_state_view);

    hex_world
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn get_document() -> Result<Document, RenderError> {
    // Use `web_sys`'s global `window` function to get a handle on the global window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    Ok(document)
}

pub fn get_target(document: &Document, target_id: &str) -> Result<Element, RenderError> {
    Ok(document
        .get_element_by_id(target_id)
        .expect(format!("target_id does not exist: {}", target_id).as_str()))
}

pub fn add_event<H>(el: &Element, event_type: &str, event_listener: H)
where
    H: 'static + FnMut(Event),
{
    let cl = Closure::wrap(Box::new(event_listener) as Box<dyn FnMut(_)>);
    el.add_event_listener_with_callback(event_type, cl.as_ref().unchecked_ref())
        .unwrap();
    cl.forget();
}

#[derive(Debug)]
pub struct RenderError {
    details: String,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error rendering: {}", self.details)
    }
}

impl Error for RenderError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<wasm_bindgen::JsValue> for RenderError {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        let details = match value.as_string() {
            Some(v) => v,
            None => "could not stringify JsValue".into(),
        };
        RenderError { details }
    }
}

impl From<RenderError> for wasm_bindgen::JsValue {
    fn from(err: RenderError) -> Self {
        wasm_bindgen::JsValue::from_str(err.details.as_str())
    }
}

pub trait Renderer {
    fn render(&self, target_id: &str) -> Result<(), RenderError>;
}

impl<C: Cell, L: Layout<C = C>> Renderer for WorldMap<C, L> {
    fn render(&self, target_id: &str) -> Result<(), RenderError> {
        log::debug(format!("rendering WorldMap to {:?}", target_id));

        let document = get_document()?;
        let target = get_target(&document, target_id)?;

        // Clear the target of any existing elements.
        target.set_inner_html("");

        // Add the root svg container.
        let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
        svg.set_attribute("id", "world_view")?;
        let view_box_min_x = -200;
        let view_box_min_y = -200;
        svg.set_attribute(
            "viewBox",
            format!("{:?} {:?} 500 500", view_box_min_x, view_box_min_y).as_str(),
        )?;

        let layer_background =
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
        let layer_sprites = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
        let layer_overlay = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;

        for (_, cell) in &self.map {
            self.render_background(&document, &layer_background, &cell)?;
            self.render_sprites(&document, &layer_sprites, &cell)?;
            self.render_overlay(&document, &layer_overlay, &cell)?;
        }

        svg.append_child(&layer_background)?;
        svg.append_child(&layer_sprites)?;
        svg.append_child(&layer_overlay)?;

        let cell_size: f32 = self
            .layout
            .cell_corner_offset(0)
            .x
            .max(self.layout.cell_corner_offset(0).y)
            .abs();

        if let Some(path) = self.shortest_path(&C::new(-3.0, -2.0, 5.0), &C::new(0.0, -5.0, 5.0)) {
            for entry_cell in path {
                let path_preview =
                    document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
                path_preview.set_attribute("cx", "0")?;
                path_preview.set_attribute("cy", "0")?;
                path_preview.set_attribute("r", (cell_size * 0.5).to_string().as_str())?;
                path_preview.set_attribute("style", "fill:green;stroke:black;stroke-width:1")?;

                let cell_center = self.layout.cell_to_pixel(&entry_cell);
                path_preview.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;

                svg.append_child(&path_preview)?;
            }
        }

        target.append_child(&svg)?;

        // TODO: cannot borrow world because the closure can outlive it. world needs to be a &'static so JS callback closures and operate on it.
        add_event(&svg, "click", move |e: Event| {
            e.prevent_default();
            let btn = e.clone().dyn_into::<MouseEvent>().unwrap().button();
            if btn == 0 {
                //alert(format!("left click").as_str());
            } else if btn == 2 {
                //alert(format!("right click").as_str());
            }

            let event = e.clone().dyn_into::<MouseEvent>().unwrap();

            //alert(format!("mousedown: {:?},{:?}", event.screen_x(), event.screen_y()).as_str());

            // // Create an SVGPoint for future math
            // var pt = svg.createSVGPoint();

            // // Get point in global SVG space
            // function cursorPoint(evt){
            //   pt.x = evt.clientX; pt.y = evt.clientY;
            //   return pt.matrixTransform(svg.getScreenCTM().inverse());
            // }

            let svg_target = event
                .current_target()
                .unwrap()
                .dyn_into::<SvgsvgElement>()
                .unwrap();
            let svg_point = svg_target.create_svg_point();
            svg_point.set_x(event.client_x() as f32); // + view_box_min_x as f32);
            svg_point.set_y(event.client_y() as f32); //+ view_box_min_y as f32);
            let svg_matrix = svg_target.get_screen_ctm().unwrap().inverse().unwrap();
            let svg_point = svg_point.matrix_transform(&svg_matrix);

            let point = Point::new(svg_point.x(), svg_point.y());
            WORLD.with(|w| {
                let clicked_cell = w.borrow().layout.pixel_to_cell(&point);
                alert(
                    format!(
                        "mousedown: client {:?},{:?} svg {:?},{:?} cell {:?},{:?},{:?}",
                        event.client_x(),
                        event.client_y(),
                        svg_point.x(),
                        svg_point.y(),
                        clicked_cell.coord().x,
                        clicked_cell.coord().y,
                        clicked_cell.coord().z,
                    )
                    .as_str(),
                );
            });
        });

        Ok(())
    }
}

impl<C: Cell, L: Layout<C = C>> WorldMap<C, L> {
    fn render_background(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        let polygon = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon")?;
        let corners = self.layout.polygon_corners(&cell);

        let mut corners_string: String = "".to_owned();
        for corner in corners {
            corners_string.push_str(String::from(corner).as_str());
            corners_string.push_str(" ");
        }

        // Create cell view.

        polygon.set_attribute("points", corners_string.as_str())?;
        polygon.set_attribute("style", "fill:lime;stroke:purple;stroke-width:1")?;

        // Set mouseover text to coordinate.
        let title = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "title")?;
        title.set_inner_html(
            format!(
                "Cell:{:?} Point:{:?}",
                String::from(&cell.coord()),
                self.layout.cell_to_pixel(&cell)
            )
            .as_str(),
        );
        polygon.append_child(&title)?;
        /*
                let cell_center = self.layout.cell_to_pixel(cell);
                polygon.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;
        */
        target.append_child(&polygon)?;

        Ok(())
    }

    fn render_sprites(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        // Render BuildingViews.
        if let Some(building) = self.game_state.building_at(&cell.coord()) {
            let building_view =
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
            let cell_size: f32 = self
                .layout
                .cell_corner_offset(0)
                .x
                .max(self.layout.cell_corner_offset(0).y)
                .abs();
            building_view.set_attribute("width", cell_size.to_string().as_str())?;
            building_view.set_attribute("height", cell_size.to_string().as_str())?;
            building_view.set_attribute("x", (-cell_size / 2.0).to_string().as_str())?;
            building_view.set_attribute("y", (-cell_size / 2.0).to_string().as_str())?;
            building_view.set_attribute("style", "fill:brown;stroke:purple;stroke-width:1")?;
            building_view.set_attribute("title", String::from(&building.state).as_str())?;

            let cell_center = self.layout.cell_to_pixel(cell);
            building_view.set_attribute(
                "transform",
                format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
            )?;

            target.append_child(&building_view)?;
        }

        Ok(())
    }

    fn render_overlay(
        &self,
        document: &Document,
        target: &Element,
        cell: &C,
    ) -> Result<(), RenderError> {
        let cell_size: f32 = self
            .layout
            .cell_corner_offset(0)
            .x
            .max(self.layout.cell_corner_offset(0).y)
            .abs();

        let cell_center = self.layout.cell_to_pixel(cell);

        // Render BuildingView overlays.
        if let Some(building) = self.game_state.building_at(&cell.coord()) {
            // Add nodes to each edge.
            for direction in cell.directions() {
                let node_view =
                    document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
                let edge_center = self.layout.polygon_edge_center(cell, *direction);
                node_view.set_attribute("cx", (edge_center.x).to_string().as_str())?;
                node_view.set_attribute("cy", (edge_center.y).to_string().as_str())?;
                node_view.set_attribute("r", (cell_size * 0.2).to_string().as_str())?;
                node_view.set_attribute("style", "fill:blue;stroke:black;stroke-width:1")?;
                node_view.set_attribute("class", "building-node")?;

                node_view.set_attribute(
                    "transform",
                    format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
                )?;

                target.append_child(&node_view)?;
            }
        }

        let path_preview =
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        path_preview.set_attribute("cx", "0")?;
        path_preview.set_attribute("cy", "0")?;
        path_preview.set_attribute("r", (cell_size * 0.5).to_string().as_str())?;
        path_preview.set_attribute("style", "fill:green;stroke:black;stroke-width:1")?;
        path_preview.set_attribute("class", "path-preview")?;

        let cell_center = self.layout.cell_to_pixel(&cell);
        path_preview.set_attribute(
            "transform",
            format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
        )?;

        target.append_child(&path_preview)?;

        // Render hidden belt routing for displaying where belt will be placed if accepted.
        // Add nodes to each edge.
        /*
        for direction in cell.directions() {
            let belt_preview_view =
                document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
            let edge_center = self.layout.polygon_edge_center(cell, *direction);
            belt_preview_view.set_attribute("x1", (edge_center.x).to_string().as_str())?;
            belt_preview_view.set_attribute("y1", (edge_center.y).to_string().as_str())?;
            belt_preview_view.set_attribute("x2", (self.layout.origin().x).to_string().as_str())?;
            belt_preview_view.set_attribute("y2", (self.layout.origin().y).to_string().as_str())?;
            belt_preview_view.set_attribute("style", "fill:gray;stroke:black;stroke-width:1")?;
            belt_preview_view.set_attribute("class", "belt-preview")?;

            belt_preview_view.set_attribute(
                "transform",
                format!("translate({},{})", cell_center.x, cell_center.y).as_str(),
            )?;

            target.append_child(&belt_preview_view)?;
        }
        */

        Ok(())
    }
}
