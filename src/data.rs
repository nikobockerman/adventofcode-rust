use std::collections::HashMap;

use crate::problem::Id;
use crate::SolverFunc;

lazy_static::lazy_static! {
    static ref INPUTS: HashMap<(u16, u8), &'static str> = {
        let mut map = HashMap::new();

        map.insert((2024, 5), include_str!("y2024/input-d5.txt"));

        map
    };
}

lazy_static::lazy_static! {
    static ref CORRECT_ANSWERS: HashMap<(u16,u8, u8), crate::answer::Answer> = {
        let mut answers = HashMap::new();

        answers.insert((2024, 5, 1), 4_872u16.into());
        answers.insert((2024, 5, 2), 5_564u16.into());

        answers
    };
}

lazy_static::lazy_static! {
    static ref SOLVERS: HashMap<(u16, u8, u8), SolverFunc> = {
        let mut solvers = HashMap::new();

        solvers.insert((2024, 5, 1), crate::y2024::d5::p1 as SolverFunc);
        solvers.insert((2024, 5, 2), crate::y2024::d5::p2 as SolverFunc);

        solvers
    };
}

pub(crate) fn get_input(id: Id) -> anyhow::Result<&'static str> {
    INPUTS
        .get(&(id.year, id.day))
        .copied()
        .map(str::trim_end)
        .ok_or_else(|| anyhow::anyhow!("Input not found: {}", id))
}

pub(crate) fn get_solver(id: Id) -> anyhow::Result<SolverFunc> {
    SOLVERS
        .get(&(id.year, id.day, id.part as u8))
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Solver not found: {}", id))
}

pub(crate) fn get_correct_answer(id: Id) -> Option<crate::answer::Answer> {
    CORRECT_ANSWERS
        .get(&(id.year, id.day, id.part as u8))
        .copied()
}
