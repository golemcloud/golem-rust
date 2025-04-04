// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(unused)]
#[allow(warnings)]
#[rustfmt::skip]
pub mod bindings;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use json::*;

mod transaction;

use bindings::golem::api::host::*;

pub use golem_wasm_rpc as wasm_rpc;

pub use bindings::golem::api::host::oplog_commit;
pub use bindings::golem::api::host::PersistenceLevel;

pub use transaction::*;

#[cfg(feature = "macro")]
pub use golem_rust_macro::*;

impl Display for PromiseId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.worker_id, self.oplog_idx)
    }
}

impl FromStr for PromiseId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let worker_id = WorkerId::from_str(parts[0]).map_err(|_| {
                format!("invalid worker id: {s} - expected format: <component_id>/<worker_name>")
            })?;
            let oplog_idx = parts[1]
                .parse()
                .map_err(|_| format!("invalid oplog index: {s} - expected integer"))?;
            Ok(Self {
                worker_id,
                oplog_idx,
            })
        } else {
            Err(format!(
                "invalid promise id: {s} - expected format: <worker_id>/<oplog_idx>"
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub min_delay: std::time::Duration,
    pub max_delay: std::time::Duration,
    pub multiplier: f64,
    pub max_jitter_factor: Option<f64>,
}

impl From<bindings::golem::api::host::RetryPolicy> for RetryPolicy {
    fn from(value: bindings::golem::api::host::RetryPolicy) -> Self {
        Self {
            max_attempts: value.max_attempts,
            min_delay: std::time::Duration::from_nanos(value.min_delay),
            max_delay: std::time::Duration::from_nanos(value.max_delay),
            multiplier: value.multiplier,
            max_jitter_factor: value.max_jitter_factor,
        }
    }
}

impl From<RetryPolicy> for bindings::golem::api::host::RetryPolicy {
    fn from(val: RetryPolicy) -> Self {
        bindings::golem::api::host::RetryPolicy {
            max_attempts: val.max_attempts,
            min_delay: val.min_delay.as_nanos() as u64,
            max_delay: val.max_delay.as_nanos() as u64,
            multiplier: val.multiplier,
            max_jitter_factor: val.max_jitter_factor,
        }
    }
}

pub struct PersistenceLevelGuard {
    original_level: PersistenceLevel,
}

impl Drop for PersistenceLevelGuard {
    fn drop(&mut self) {
        set_oplog_persistence_level(self.original_level);
    }
}

/// Temporarily sets the oplog persistence level to the given value.
///
/// When the returned guard is dropped, the original persistence level is restored.
#[must_use]
pub fn use_persistence_level(level: PersistenceLevel) -> PersistenceLevelGuard {
    let original_level = get_oplog_persistence_level();
    set_oplog_persistence_level(level);
    PersistenceLevelGuard { original_level }
}

/// Executes the given function with the oplog persistence level set to the given value.
pub fn with_persistence_level<R>(level: PersistenceLevel, f: impl FnOnce() -> R) -> R {
    let _guard = use_persistence_level(level);
    f()
}

pub struct IdempotenceModeGuard {
    original: bool,
}

impl Drop for IdempotenceModeGuard {
    fn drop(&mut self) {
        set_idempotence_mode(self.original);
    }
}

/// Temporarily sets the idempotence mode to the given value.
///
/// When the returned guard is dropped, the original idempotence mode is restored.
#[must_use]
pub fn use_idempotence_mode(mode: bool) -> IdempotenceModeGuard {
    let original = get_idempotence_mode();
    set_idempotence_mode(mode);
    IdempotenceModeGuard { original }
}

/// Executes the given function with the idempotence mode set to the given value.
pub fn with_idempotence_mode<R>(mode: bool, f: impl FnOnce() -> R) -> R {
    let _guard = use_idempotence_mode(mode);
    f()
}

/// Generates an idempotency key. This operation will never be replayed —
/// i.e. not only is this key generated, but it is persisted and committed, such that the key can be used in third-party systems (e.g. payment processing)
/// to introduce idempotence.
pub fn generate_idempotency_key() -> uuid::Uuid {
    Into::into(bindings::golem::api::host::generate_idempotency_key())
}

pub struct RetryPolicyGuard {
    original: RetryPolicy,
}

impl Drop for RetryPolicyGuard {
    fn drop(&mut self) {
        set_retry_policy(Into::into(self.original.clone()));
    }
}

/// Temporarily sets the retry policy to the given value.
///
/// When the returned guard is dropped, the original retry policy is restored.
#[must_use]
pub fn use_retry_policy(policy: RetryPolicy) -> RetryPolicyGuard {
    let original = Into::into(get_retry_policy());
    set_retry_policy(Into::into(policy));
    RetryPolicyGuard { original }
}

/// Executes the given function with the retry policy set to the given value.
pub fn with_retry_policy<R>(policy: RetryPolicy, f: impl FnOnce() -> R) -> R {
    let _guard = use_retry_policy(policy);
    f()
}

pub struct AtomicOperationGuard {
    begin: OplogIndex,
}

impl Drop for AtomicOperationGuard {
    fn drop(&mut self) {
        mark_end_operation(self.begin);
    }
}

/// Marks a block as an atomic operation
///
/// When the returned guard is dropped, the operation gets committed.
/// In case of a failure, the whole operation will be re-executed during retry.
#[must_use]
pub fn mark_atomic_operation() -> AtomicOperationGuard {
    let begin = mark_begin_operation();
    AtomicOperationGuard { begin }
}

/// Executes the given function as an atomic operation.
///
/// In case of a failure, the whole operation will be re-executed during retry.
pub fn atomically<T>(f: impl FnOnce() -> T) -> T {
    let _guard = mark_atomic_operation();
    f()
}
