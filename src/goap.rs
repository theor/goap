use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
struct Context {
    pub items: HashMap<Item, u8>,
    pub availaible_items: HashMap<Item, u8>,
}

impl Context {
    pub fn new() -> Self { Default::default() }
    pub fn with_item(mut self, i:Item, c:u8) -> Self {
        self.items.entry(i).and_modify(|e| *e = c).or_insert(c);
        self
    }
    pub fn with_available_item(mut self, i:Item, c:u8) -> Self {
        self.availaible_items.entry(i).and_modify(|e| *e = c).or_insert(c);
        self
    }
}

trait PreCondition: Debug + Clone {
    fn is_valid(&self, ctx: &Context) -> bool;
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Item {
    Axe,
    Wood,
}

#[derive(Debug, Clone)]
struct Not<T: PreCondition>(T);
impl<T: PreCondition> PreCondition for Not<T> {
    fn is_valid(&self, ctx: &Context) -> bool {
        !self.0.is_valid(ctx)
    }
}

#[derive(Debug, Clone)]
struct Has(Item);
impl PreCondition for Has {
    fn is_valid(&self, ctx: &Context) -> bool {
        *ctx.items.get(&self.0).unwrap_or(&0u8) > 0u8
    }
}

#[derive(Debug, Clone)]
struct Available(Item);
impl PreCondition for Available {
    fn is_valid(&self, ctx: &Context) -> bool {
        *ctx.availaible_items.get(&self.0).unwrap_or(&0u8) > 0u8
    }
}

trait Effect: Debug + Clone {
    fn apply(&self, ctx: &mut Context);
}

#[derive(Debug, Clone)]
struct Take(Item);

impl Effect for Take {
    fn apply(&self, ctx: &mut Context) {
        ctx.items.entry(self.0.clone()).and_modify(|e| *e += 1).or_insert(1);
        ctx.availaible_items.entry(self.0.clone()).and_modify(|e| *e -= 1).or_insert(0);
    }
}

#[derive(Debug, Clone)]
struct Put(Item);

impl Effect for Put {
    fn apply(&self, ctx: &mut Context) {
        ctx.availaible_items.entry(self.0.clone()).and_modify(|e| *e += 1).or_insert(1);
        ctx.items.entry(self.0.clone()).and_modify(|e| *e -= 1).or_insert(0);
    }
}

trait Action: Debug/*PreCondition+Effect*/ {
    fn apply(&self, ctx: &mut Context);
    fn revert(&self, ctx: &mut Context);
    fn is_valid(&self, ctx: &Context) -> bool;
    fn cost(&self) -> i32;

    // preconditions: Vec<Box<PreCondition>>,
    // effects: Vec<Effect>,
}

#[derive(Default, Debug)]
struct ChopWood;

impl Action for ChopWood {
    fn cost(&self) -> i32 { 4 }
    fn is_valid(&self, ctx: &Context) -> bool {
        Has(Item::Axe).is_valid(ctx)
    }
    fn apply(&self, ctx: &mut Context) {
        Take(Item::Wood).apply(ctx)
    }
    fn revert(&self, ctx: &mut Context) {
        Put(Item::Wood).apply(ctx)
    }
}

#[derive(Default, Debug)]
struct CollectBranches;
impl Action for CollectBranches {
    fn cost(&self) -> i32 { 8 }
    fn is_valid(&self, _ctx: &Context) -> bool { true }
    fn apply(&self, ctx: &mut Context) {
        Take(Item::Wood).apply(ctx)
    }
    fn revert(&self, ctx: &mut Context) {
        Put(Item::Wood).apply(ctx)
    }
}

#[derive(Default, Debug)]
struct GetAxe;
impl Action for GetAxe {
    fn cost(&self) -> i32 { 2 }
    fn is_valid(&self, ctx: &Context) -> bool {
        Not(Has(Item::Axe)).is_valid(ctx) &&
        Available(Item::Axe).is_valid(ctx)
    }
    fn apply(&self, ctx: &mut Context) {
        Take(Item::Axe).apply(ctx)
    }
}

#[derive(Debug)]
struct Planner {

}

impl Planner {
    pub fn find_actions_to(ctx: &Context, actions: &[&Action]) {


    }
    pub fn find_path(start: &Context, end: &Context, actions: &[&Action]) {
        use std::collections::{
            vec_deque::VecDeque,
            HashMap,
        };
        // let queue = VecDeque::new();

        // let map = HashMap::new();

        // queue.push_back(end);
        // while let Some(u) = queue.pop_front() {

        // }

    }
}

#[test]
fn name() {
    use std::default::Default;

    let mut ctx:Context = Default::default();

    let chop: ChopWood = Default::default();
    let collect: CollectBranches = Default::default();
    let getAxe: GetAxe = Default::default();

    let target = Context::new().with_item(Item::Wood, 1);
     Planner::find_actions_to(&target, &[&chop, &collect, &getAxe]);
    // ctx.items.insert(Item::Axe, 1);

    // let hasAxe = Has(Item::Axe);
    // println!("{:?}: {}", hasAxe, hasAxe.is_valid(&ctx));
    // let a: ChopWood = Default::default();
    // println!("ctx {:?} action {:?} isValid {}", ctx, a, a.is_valid(&ctx));
    // a.apply(&mut ctx);
    // println!("ctx {:?}", ctx);
}