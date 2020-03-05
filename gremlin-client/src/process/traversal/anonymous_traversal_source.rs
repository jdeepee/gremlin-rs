use crate::process::traversal::step::has::IntoHasStep;
use crate::process::traversal::step::loops::LoopsStep;
use crate::process::traversal::step::not::IntoNotStep;
use crate::process::traversal::step::or::IntoOrStep;
use crate::process::traversal::step::select::IntoSelectStep;
use crate::process::traversal::TraversalBuilder;
use crate::process::traversal::step::where_step::IntoWhereStep;
use crate::structure::{GIDs, Labels, IntoPredicate};
use crate::GValue;

pub struct AnonymousTraversalSource {
    traversal: TraversalBuilder,
}

impl AnonymousTraversalSource {
    pub fn new() -> AnonymousTraversalSource {
        AnonymousTraversalSource {
            traversal: TraversalBuilder::default(),
        }
    }

    pub fn in_v(&self) -> TraversalBuilder
    {
        self.traversal.clone().in_v()
    }

    pub fn aggregate<A>(&self, alias: A) -> TraversalBuilder
    where
        A: Into<String>,
    {
        self.traversal.clone().aggregate(alias)
    }

    pub fn add_v<A>(&self, label: A) -> TraversalBuilder
    where
        A: Into<Labels>,
    {
        self.traversal.clone().add_v(label)
    }

    pub fn property<A>(&self, key: &str, value: A) -> TraversalBuilder
    where
        A: Into<GValue>,
    {
        self.traversal.clone().property(key, value)
    }

    pub fn v<T>(&self, ids: T) -> TraversalBuilder
    where
        T: Into<GIDs>,
    {
        self.traversal.clone().v(ids)
    }

    pub fn count(&self) -> TraversalBuilder {
        self.traversal.clone().count()
    }

    pub fn out<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().out(labels)
    }

    pub fn out_e<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().out_e(labels)
    }

    pub fn out_v(&self) -> TraversalBuilder
    {
        self.traversal.clone().out_v()
    }

    pub fn values<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().values(labels)
    }
    pub fn has_label<L>(&self, labels: L) -> TraversalBuilder
    where
        L: Into<Labels>,
    {
        self.traversal.clone().has_label(labels)
    }

    pub fn as_<A>(&self, alias: A) -> TraversalBuilder
    where
        A: Into<String>,
    {
        self.traversal.clone().as_(alias)
    }

    pub fn has<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has(step)
    }

    pub fn has_many<A>(&self, steps: Vec<A>) -> TraversalBuilder
    where
        A: IntoHasStep,
    {
        self.traversal.clone().has_many(steps)
    }

    pub fn not<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoNotStep,
    {
        self.traversal.clone().not(step)
    }

    pub fn loops<A>(&self, step: A) -> TraversalBuilder
    where
        A: Into<LoopsStep>,
    {
        self.traversal.clone().loops(step)
    }

    pub fn select<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoSelectStep,
    {
        self.traversal.clone().select(step)
    }

    pub fn is<A>(&self, val: A) -> TraversalBuilder
    where
        A: IntoPredicate,
    {
        self.traversal.clone().is(val)
    }

    pub fn or<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoOrStep,
    {
        self.traversal.clone().or(step)
    }

    pub fn where_<A>(&self, step: A) -> TraversalBuilder
    where
        A: IntoWhereStep,
    {
        self.traversal.clone().where_(step)
    }

    pub fn cap(&self, step: &'static str) -> TraversalBuilder {
        self.traversal.clone().cap(step)
    }
}

impl Default for AnonymousTraversalSource {
    fn default() -> Self {
        Self::new()
    }
}
