use std::hash::Hasher;
use std::hash::Hash;
use std::fmt::Debug;

bitflags! {
    pub struct StateFlag: u32 {
        const NONE = 0;
        const HAS_AXE = 0b00000001;
        const HAS_WOOD = 0b00000010;
        const AXE_AVAILABLE = 0b00000100;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Context {
    pub state: StateFlag,
}

impl Context {
    pub fn new() -> Self { Self { state: StateFlag::NONE } }
    pub fn with_state(state: StateFlag) -> Self { Self { state } }
    pub fn would_be_reached_by(&self, a: &dyn Action) -> bool {
        self.state.contains(a.effects())
    }
    // pub fn with_item(mut self, i:Item, c:u8) -> Self {
    //     self.items.entry(i).and_modify(|e| *e = c).or_insert(c);
    //     self
    // }
    // pub fn with_available_item(mut self, i:Item, c:u8) -> Self {
    //     self.availaible_items.entry(i).and_modify(|e| *e = c).or_insert(c);
    //     self
    // }
}
// #[derive(PartialEq, Eq, Debug, Hash, Clone)]
// enum Item {
//     Axe,
//     Wood,
// }
trait Action: Debug {
    fn preconditions(&self) -> StateFlag;
    fn effects(&self) -> StateFlag;
    fn cost(&self) -> i32;
    fn exec(&self, ch: &mut crate::Char, _state: &mut crate::MainState) {
        println!("EXEC ch {:?} action {:?}", ch, self);
    }
}

#[derive(Default, Debug)]
struct ChopWood;
impl Action for ChopWood {
    fn preconditions(&self) -> StateFlag { StateFlag::HAS_AXE }
    fn effects(&self) -> StateFlag { StateFlag::HAS_WOOD }
    fn cost(&self) -> i32 { 4 }
    fn exec(&self, ch: &mut crate::Char, state: &mut crate::MainState) {}
}

#[derive(Default, Debug)]
struct CollectBranches;
impl Action for CollectBranches {
    fn preconditions(&self) -> StateFlag { StateFlag::NONE }
    fn effects(&self) -> StateFlag { StateFlag::HAS_WOOD }
    fn cost(&self) -> i32 { 8 }
}

#[derive(Default, Debug)]
struct GetAxe;
impl Action for GetAxe {
    fn preconditions(&self) -> StateFlag { StateFlag::AXE_AVAILABLE }
    fn effects(&self) -> StateFlag { StateFlag::HAS_AXE }
    fn cost(&self) -> i32 { 2 }
}


#[derive(Clone, Debug)]
struct Step<'a> {
    state: Context,
    action: Option<&'a dyn Action>,
}

impl<'a> PartialEq for Step<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<'a> Eq for Step<'a> {}

impl<'a> Hash for Step<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

struct Planner;

impl Planner {
    fn predecessors<'a>(n: &Step<'a>, actions: &[&'a Action]) -> Vec<(Step<'a>, i32)> {
        let mut v = Vec::with_capacity(actions.len());
        for a in actions {
            if n.state.would_be_reached_by(*a) {
                println!("state {:?} contains action {:?} state {:?}", n.state, a, a.effects());
                let step_ctx = (n.state.state - a.effects()) | a.preconditions();
                println!("  step state {:?}", step_ctx);
                v.push((Step { state: Context::with_state(step_ctx), action: Some(*a) }, a.cost()));
            }
        }
        v
    }
    pub fn find_path(start: &Context, end: &Context, actions: &[&Action]) {
        use pathfinding::directed::dijkstra;
        let end_step = Step {
            action: None,
            state: end.clone(),
        };
        let path = dijkstra::dijkstra(&end_step, |n| Self::predecessors(n, actions), |n| &n.state == start);
        if let Some(path) = path {
            println!("Path: {} steps, cost {}", path.0.len(), path.1);

            for step in path.0.iter().rev() {
            println!("  {:?}", step);

            }

        }
    }
}

#[test]
fn name() {
    use std::default::Default;

    // let ctx:Context = Context::new();

    let chop: ChopWood = Default::default();
    let collect: CollectBranches = Default::default();
    let get_axe: GetAxe = Default::default();

    let start = Context::with_state(StateFlag::AXE_AVAILABLE);
    let target = Context::with_state(StateFlag::HAS_WOOD);
    Planner::find_path(&start, &target, &[&chop, &collect, &get_axe]);
    // ctx.items.insert(Item::Axe, 1);

    // let hasAxe = Has(Item::Axe);
    // println!("{:?}: {}", hasAxe, hasAxe.is_valid(&ctx));
    // let a: ChopWood = Default::default();
    // println!("ctx {:?} action {:?} isValid {}", ctx, a, a.is_valid(&ctx));
    // a.apply(&mut ctx);
    // println!("ctx {:?}", ctx);
}