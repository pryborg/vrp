use crate::construction::constraints::ConstraintPipeline;
use crate::construction::heuristics::{InsertionContext, RegistryContext, SolutionContext};
use crate::helpers::models::problem::*;
use crate::models::common::IdDimension;
use crate::models::problem::{Fleet, Job, Jobs, ObjectiveCost};
use crate::models::solution::Registry;
use crate::models::{Problem, Solution};
use crate::utils::DefaultRandom;
use std::sync::Arc;

pub fn create_empty_problem_with_constraint(constraint: ConstraintPipeline) -> Arc<Problem> {
    create_empty_problem_with_constraint_and_fleet(constraint, test_fleet())
}

pub fn create_empty_problem() -> Arc<Problem> {
    create_empty_problem_with_constraint(ConstraintPipeline::default())
}

pub fn create_empty_problem_with_constraint_and_fleet(constraint: ConstraintPipeline, fleet: Fleet) -> Arc<Problem> {
    let transport = TestTransportCost::new_shared();
    let fleet = Arc::new(fleet);
    let jobs = Arc::new(Jobs::new(fleet.as_ref(), vec![], &transport));
    Arc::new(Problem {
        fleet,
        jobs,
        locks: vec![],
        constraint: Arc::new(constraint),
        activity: Arc::new(TestActivityCost::default()),
        transport,
        objective: Arc::new(ObjectiveCost::default()),
        extras: Arc::new(Default::default()),
    })
}

pub fn create_empty_solution() -> Solution {
    Solution {
        registry: Registry::new(&test_fleet()),
        routes: vec![],
        unassigned: Default::default(),
        extras: Arc::new(Default::default()),
    }
}

pub fn create_empty_solution_context() -> SolutionContext {
    SolutionContext {
        required: vec![],
        ignored: vec![],
        unassigned: Default::default(),
        locked: Default::default(),
        routes: vec![],
        registry: RegistryContext::new(Registry::new(&test_fleet())),
        state: Default::default(),
    }
}

pub fn create_empty_insertion_context() -> InsertionContext {
    InsertionContext {
        problem: create_empty_problem(),
        solution: create_empty_solution_context(),
        random: Arc::new(DefaultRandom::default()),
    }
}

pub fn get_customer_ids_from_routes_sorted(insertion_ctx: &InsertionContext) -> Vec<Vec<String>> {
    let mut result = get_customer_ids_from_routes(insertion_ctx);
    result.sort();
    result
}

pub fn get_sorted_customer_ids_from_jobs(jobs: &[Job]) -> Vec<String> {
    let mut ids = jobs.iter().map(|job| get_customer_id(&job)).collect::<Vec<String>>();
    ids.sort();
    ids
}

pub fn get_customer_ids_from_routes(insertion_ctx: &InsertionContext) -> Vec<Vec<String>> {
    insertion_ctx
        .solution
        .routes
        .iter()
        .map(|rc| {
            rc.route
                .tour
                .all_activities()
                .filter(|a| a.job.is_some())
                .map(|a| a.retrieve_job().unwrap())
                .map(|job| get_customer_id(&job))
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn get_customer_id(job: &Job) -> String {
    job.dimens().get_id().unwrap().clone()
}
