use std::cmp;
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::cell::{Cell, CellCoord};
use crate::game_view::GameStateView;
use crate::layout::Layout;
use crate::log;

pub struct WorldMap<C: Cell, L: Layout<C = C>> {
    pub map: HashMap<CellCoord, C>,
    pub layout: L,
    pub game_state: GameStateView,
}

impl<C: Cell, L: Layout<C = C>> WorldMap<C, L> {
    pub fn new(layout: L, game_state: GameStateView) -> WorldMap<C, L> {
        WorldMap {
            map: HashMap::new(),
            layout: layout,
            game_state: game_state,
        }
    }

    pub fn generate_hexgon(&mut self, radius: i32) {
        self.map.clear();

        for q in -radius..=radius {
            let r1 = cmp::max(-radius, -q - radius);
            let r2 = cmp::min(radius, -q + radius);

            for r in r1..=r2 {
                let cell = C::new(q as f32, r as f32, (-q - r) as f32);
                self.map.insert(cell.coord(), cell);
            }
        }
    }

    pub fn shortest_path(&self, from: &C, to: &C) -> Option<Vec<C>> {
        a_star_search(from, to, &self.game_state.collision_set)
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

fn heuristic(a: &CellCoord, b: &CellCoord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn a_star_search<C: Cell>(start: &C, end: &C, collisions: &HashSet<CellCoord>) -> Option<Vec<C>> {
    log::debug(format!("=============================="));
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
        log::debug(format!(
            "START frontier peek {:?} {:?}",
            frontier.peek().unwrap().0.cell.coord(),
            frontier.peek().unwrap().0.priority,
        ));
        log::debug(format!("frontier size: {:?}", frontier.len()));
        if let Some(current) = frontier.pop() {
            log::debug(format!("frontier size: {:?}", frontier.len()));
            log::debug(format!("  current: {:?}", current.0.cell.coord()));
            if current.0.cell.coord() == end.coord() {
                // Stop if we have reached the end.

                log::info("  found a path".to_owned());
                let mut path: Vec<C> = Vec::new();

                path.push(end.clone());

                log::info(format!("path: {:?}", &end.coord()));

                let mut previous = came_from.get(&end.coord()).unwrap();
                while previous.cell.coord() != start.coord() {
                    log::info(format!("path: {:?}", &previous.cell.coord()));
                    path.push(previous.cell.clone());
                    previous = came_from.get(&previous.cell.coord()).unwrap();
                }

                path.push(start.clone());
                log::info(format!("path: {:?}", &start.coord()));

                path.reverse();
                log::info("DONE".to_owned());
                return Some(path);
            }

            for next in &current.0.cell.neighbors() {
                if collisions.contains(&next.coord()) {
                    continue;
                }
                log::debug(format!("next: {:?}", &next.coord()));
                let current_cost = cost_so_far
                    .get(&current.0.cell.coord())
                    .or(Some(&0))
                    .unwrap();
                let cost_to_next = 1; // This is the cost of traversing to this next cell.
                let new_cost = current_cost + cost_to_next;
                let next_cost = *cost_so_far.get(&next.coord()).or(Some(&0)).unwrap();
                if !cost_so_far.contains_key(&next.coord()) || new_cost < next_cost {
                    cost_so_far.insert(next.coord(), new_cost);
                    let priority = new_cost + heuristic(&end.coord(), &next.coord());
                    log::debug(format!("  priority {:?}", priority));
                    frontier.push(Reverse(CellPriority::new(next.clone(), priority)));
                    log::debug(format!(
                        "  frontier peek {:?}",
                        frontier.peek().unwrap().0.cell.coord()
                    ));
                    log::debug(format!("frontier size: {:?}", frontier.len()));
                    came_from.insert(next.coord(), current.0.clone());
                }
            }
            log::debug(format!(
                "  frontier peek {:?}",
                frontier.peek().unwrap().0.cell.coord()
            ));
            log::debug(format!("frontier size: {:?}", frontier.len()));
        }
    }

    // Could not find any path.
    None
}

/*
def heuristic(a, b):
    (x1, y1) = a
    (x2, y2) = b
    return abs(x1 - x2) + abs(y1 - y2)

def a_star_search(graph, start, goal):
    frontier = PriorityQueue()
    frontier.put(start, 0)
    came_from = {}
    cost_so_far = {}
    came_from[start] = None
    cost_so_far[start] = 0
    while not frontier.empty():
        current = frontier.get()
        if current == goal:
            break
        for next in graph.neighbors(current):
            new_cost = cost_so_far[current] + graph.cost(current, next)
            if next not in cost_so_far or new_cost < cost_so_far[next]:
                cost_so_far[next] = new_cost
                priority = new_cost + heuristic(goal, next)
                frontier.put(next, priority)
                came_from[next] = current
    return came_from, cost_so_far
*/
