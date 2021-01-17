use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::cell::{Cell, CellCoord};
use super::layout::{Layout, Point};
use super::logging::{debug, info};
use super::renderer::{
    add_mouse_event, get_body, Camera, RenderError, Renderable, UserEvent, Viewport,
};

use web_sys::MouseEvent;

use wasm_bindgen::prelude::*;

// TODO: This will eventually be more complex and involve movement costs, etc.
type Collisions = HashSet<CellCoord>;

pub struct World<C, L>
where
    C: Cell,
    L: Layout<C = C>,
{
    pub base_map: HashSet<CellCoord>,
    pub layout: L,
    pub viewport: Viewport,
    cameras: Vec<Camera>,
    active_camera: usize,
}

impl<C, L> World<C, L>
where
    C: Cell,
    L: Layout<C = C>,
{
    pub fn new(
        target_id: &str,
        layout: L,
        width: i32,
        height: i32,
    ) -> Result<World<C, L>, RenderError> {
        Ok(World {
            base_map: HashSet::new(),
            layout: layout,
            viewport: Viewport::new(target_id, width, height)?,
            cameras: vec![Camera::new(width, height)],
            active_camera: 0,
        })
    }

    pub fn camera(&self, id: usize) -> &Camera {
        &self.cameras[id]
    }

    pub fn camera_mut(&mut self, id: usize) -> &mut Camera {
        &mut self.cameras[id]
    }

    pub fn active_camera(&self) -> usize {
        self.active_camera
    }

    pub fn set_active_camera(&mut self, id: usize) -> Result<(), RenderError> {
        if id >= self.cameras.len() {
            return Err(RenderError::new("invalid camera id"));
        }

        self.active_camera = id;

        Ok(())
    }

    pub fn look_at(&mut self, position: &Point) -> Result<(), RenderError> {
        // FIXME: This is awkward.
        self.camera_mut(self.active_camera()).look_at(position);
        let camera = self.camera_mut(self.active_camera()).clone();
        self.viewport.look_at(&camera)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), RenderError> {
        debug("rendering world".to_owned());

        self.viewport.render(&self.layout)?;

        debug("world rendered".to_owned());

        Ok(())
    }

    pub fn clear_layer(&mut self, layer_name: &str) {
        self.viewport
            .layer_mut(layer_name)
            .expect("layer does not exist")
            .clear()
            .expect("failed to clear layer on render layer");
    }

    pub fn render_layer(&self, layer_name: &str) {
        self.viewport
            .layer(layer_name)
            .expect("layer does not exist")
            .render(&self.layout)
            .expect(format!("failed to render layer: {:?}", layer_name).as_str());
    }

    pub fn event_cell(&self, event: &MouseEvent) -> Result<C, RenderError> {
        let pixel = self.viewport.event_point(event)?;
        Ok(self.layout.pixel_to_cell(&pixel))
    }

    pub fn on_mouse_event<H>(&self, event: UserEvent, handler: H) -> Result<(), JsValue>
    where
        H: 'static + FnMut(MouseEvent),
    {
        add_mouse_event(&get_body(), &event, handler);

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct CellPriority<C: Cell> {
    cell: C,
    priority: i32,
}

impl<C: Cell> CellPriority<C> {
    fn new(cell: C, priority: i32) -> CellPriority<C> {
        CellPriority { cell, priority }
    }
}

impl<C: Cell> Eq for CellPriority<C> {}

impl<C: Cell> Ord for CellPriority<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<C: Cell> PartialOrd for CellPriority<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Cell> PartialEq for CellPriority<C> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub fn shortest_path<C>(
    from: &C,
    to: &C,
    world: &HashSet<CellCoord>,
    collisions: &Collisions,
) -> Option<Vec<C>>
where
    C: Cell,
{
    a_star_search(from, to, world, collisions)
}

fn heuristic(a: &CellCoord, b: &CellCoord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn a_star_search<C: Cell>(
    start: &C,
    end: &C,
    base_map: &HashSet<CellCoord>,
    collisions: &HashSet<CellCoord>,
) -> Option<Vec<C>> {
    debug(format!("=============================="));
    let mut frontier: BinaryHeap<Reverse<CellPriority<C>>> = BinaryHeap::new();
    frontier.push(Reverse(CellPriority::new(start.clone(), 0)));

    let mut came_from: HashMap<CellCoord, CellPriority<C>> = HashMap::new();
    came_from.insert(start.coord(), CellPriority::new(start.clone(), 0));

    let mut cost_so_far: HashMap<CellCoord, i32> = HashMap::new();
    cost_so_far.insert(start.coord(), 0);

    while !frontier.is_empty() {
        if cost_so_far.len() > 1000 {
            // Safety break;
            break;
        }
        if let Some(current) = frontier.pop() {
            if current.0.cell.coord() == end.coord() {
                // Stop if we have reached the end.

                info("  found a path".to_owned());
                let mut path: Vec<C> = Vec::new();

                path.push(end.clone());

                info(format!("path: {:?}", &end.coord()));

                let mut previous = came_from
                    .get(&end.coord())
                    .expect("failed to where came from");
                while previous.cell.coord() != start.coord() {
                    info(format!("path: {:?}", &previous.cell.coord()));
                    path.push(previous.cell.clone());
                    previous = came_from
                        .get(&previous.cell.coord())
                        .expect("failed to get previous");
                }

                path.push(start.clone());
                info(format!("path: {:?}", &start.coord()));

                path.reverse();
                info("DONE".to_owned());
                return Some(path);
            }

            for next in &current.0.cell.neighbors() {
                if !base_map.contains(&next.coord()) || collisions.contains(&next.coord()) {
                    continue;
                }
                let current_cost = cost_so_far
                    .get(&current.0.cell.coord())
                    .or(Some(&0))
                    .expect("failed to get cost so far");
                let cost_to_next = 1; // This is the cost of traversing to this next cell.
                let new_cost = current_cost + cost_to_next;
                let next_cost = *cost_so_far
                    .get(&next.coord())
                    .or(Some(&0))
                    .expect("failed to set new cost");
                if !cost_so_far.contains_key(&next.coord()) || new_cost < next_cost {
                    cost_so_far.insert(next.coord(), new_cost);
                    let priority = new_cost + heuristic(&end.coord(), &next.coord());
                    frontier.push(Reverse(CellPriority::new(next.clone(), priority)));
                    came_from.insert(next.coord(), current.0.clone());
                }
            }
        }
    }

    // Could not find any path.
    None
}
