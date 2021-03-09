use crate::construction::heuristics::InsertionContext;
use crate::construction::heuristics::*;
use crate::solver::mutation::recreate::Recreate;
use crate::solver::RefinementContext;
use crate::utils::{Either, Random};
use std::sync::Arc;

/// A recreate method which perturbs the cost by a factor to introduce randomization.
pub struct RecreateWithPerturbation {
    job_selector: Box<dyn JobSelector + Send + Sync>,
    route_selector: Box<dyn RouteSelector + Send + Sync>,
    result_selector: Box<dyn ResultSelector + Send + Sync>,
    insertion_heuristic: InsertionHeuristic,
}

impl RecreateWithPerturbation {
    /// Creates a new instance of `RecreateWithPerturbation`.
    pub fn new(probability: f64, min: f64, max: f64, random: Arc<dyn Random + Send + Sync>) -> Self {
        Self {
            job_selector: Box::new(AllJobSelector::default()),
            route_selector: Box::new(AllRouteSelector::default()),
            result_selector: Box::new(CostPerturbationResultSelector::new(probability, min, max, random)),
            insertion_heuristic: Default::default(),
        }
    }

    /// Creates a new instance of `RecreateWithPerturbation` with default values.
    pub fn new_with_defaults(random: Arc<dyn Random + Send + Sync>) -> Self {
        Self::new(0.05, 0.75, 1.25, random)
    }
}

impl Recreate for RecreateWithPerturbation {
    fn run(&self, refinement_ctx: &RefinementContext, insertion_ctx: InsertionContext) -> InsertionContext {
        self.insertion_heuristic.process(
            insertion_ctx,
            self.job_selector.as_ref(),
            self.route_selector.as_ref(),
            self.result_selector.as_ref(),
            &refinement_ctx.quota,
        )
    }
}

/// Selects best result.
struct CostPerturbationResultSelector {
    probability: f64,
    min: f64,
    max: f64,
    random: Arc<dyn Random + Send + Sync>,
}

impl CostPerturbationResultSelector {
    pub fn new(probability: f64, min: f64, max: f64, random: Arc<dyn Random + Send + Sync>) -> Self {
        Self { probability, min, max, random }
    }
}

impl ResultSelector for CostPerturbationResultSelector {
    fn select_insertion(
        &self,
        _ctx: &InsertionContext,
        left: InsertionResult,
        right: InsertionResult,
    ) -> InsertionResult {
        InsertionResult::choose_best_result(self.try_perturbation(left), self.try_perturbation(right))
    }

    fn select_cost(&self, _route_ctx: &RouteContext, left: f64, right: f64) -> Either {
        let random = self.random.as_ref();

        let left = left * random.uniform_real(self.min, self.max);
        let right = right * random.uniform_real(self.min, self.max);

        if left < right {
            Either::Left
        } else {
            Either::Right
        }
    }
}

impl CostPerturbationResultSelector {
    fn try_perturbation(&self, result: InsertionResult) -> InsertionResult {
        if self.random.is_hit(self.probability) {
            match result {
                InsertionResult::Success(success) => InsertionResult::Success(InsertionSuccess {
                    cost: success.cost * self.random.uniform_real(self.min, self.max),
                    job: success.job,
                    activities: success.activities,
                    context: success.context,
                }),
                _ => result,
            }
        } else {
            result
        }
    }
}
