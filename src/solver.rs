use crate::answer::Answer;
use crate::data;
use crate::problem::Id;
use crate::SolverFunc;

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub id: Id,
    solver: SolverFunc,
    str: &'static str,
}

impl Input {
    pub fn new(id: Id) -> anyhow::Result<Self> {
        let str = data::get_input(id)?;
        let solver = data::get_solver(id)?;
        Ok(Self { id, solver, str })
    }

    pub fn solve(&self) -> anyhow::Result<Output> {
        let answer = (self.solver)(self.str)?;
        Ok(Output {
            id: self.id,
            answer,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub id: Id,
    pub answer: Answer,
}

impl Output {
    pub fn analyze(self) -> Analysis {
        let correct_answer = data::get_correct_answer(self.id);
        Analysis {
            answer: self.answer,
            correct_answer,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Analysis {
    pub answer: Answer,
    pub correct_answer: Option<Answer>,
}

impl Analysis {
    pub fn is_correct(self) -> bool {
        self.correct_answer.is_some() && self.answer == self.correct_answer.unwrap()
    }

    pub fn is_incorrect(self) -> bool {
        self.correct_answer.is_some() && self.answer != self.correct_answer.unwrap()
    }
}
