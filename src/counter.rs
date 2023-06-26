use crate::CanonicalizationError;
use std::{collections::HashMap, fmt};

const DEFAULT_HNDQ_CALL_LIMIT: usize = 4000;

pub trait HndqCallCounter {
    fn new(max_calls: Option<usize>) -> Self;
    fn add(&mut self, identifier: &str) -> Result<(), CanonicalizationError>;
    fn sum(&self) -> usize;
}

pub struct SimpleHndqCallCounter {
    counter: usize,
    limit: usize,
}

impl Default for SimpleHndqCallCounter {
    fn default() -> Self {
        Self {
            counter: Default::default(),
            limit: DEFAULT_HNDQ_CALL_LIMIT,
        }
    }
}

impl HndqCallCounter for SimpleHndqCallCounter {
    fn new(max_calls: Option<usize>) -> Self {
        let limit = match max_calls {
            Some(limit) => limit,
            None => DEFAULT_HNDQ_CALL_LIMIT,
        };
        Self { counter: 0, limit }
    }

    fn add(&mut self, _identifier: &str) -> Result<(), CanonicalizationError> {
        self.counter += 1;
        if self.counter > self.limit {
            Err(CanonicalizationError::HndqCallLimitExceeded(self.limit))
        } else {
            Ok(())
        }
    }

    fn sum(&self) -> usize {
        self.counter
    }
}

impl fmt::Debug for SimpleHndqCallCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("counter", &self.counter)
            .field("limit", &self.limit)
            .finish()
    }
}

pub struct PerNodeHndqCallCounter {
    counter: HashMap<String, usize>,
    limit: usize,
}

impl Default for PerNodeHndqCallCounter {
    fn default() -> Self {
        Self {
            counter: Default::default(),
            limit: DEFAULT_HNDQ_CALL_LIMIT,
        }
    }
}

impl HndqCallCounter for PerNodeHndqCallCounter {
    fn new(max_calls: Option<usize>) -> Self {
        let limit = match max_calls {
            Some(limit) => limit,
            None => DEFAULT_HNDQ_CALL_LIMIT,
        };
        Self {
            counter: Default::default(),
            limit,
        }
    }

    fn add(&mut self, identifier: &str) -> Result<(), CanonicalizationError> {
        let current = self
            .counter
            .entry(identifier.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
        if current > &mut self.limit {
            Err(CanonicalizationError::HndqCallLimitExceeded(self.limit))
        } else {
            Ok(())
        }
    }

    fn sum(&self) -> usize {
        self.counter
            .values()
            .copied()
            .reduce(|acc, v| acc + v)
            .unwrap_or(0)
    }
}

impl fmt::Debug for PerNodeHndqCallCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("counter", &self.counter)
            .field("limit", &self.limit)
            .field("sum", &self.sum())
            .finish()
    }
}
