/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::sync::Arc;

use allocative::Allocative;
use buck2_core::base_deferred_key::BaseDeferredKey;
use dupe::Dupe;

use crate::dynamic::DynamicLambdaResultsKey;

/// The base key. We can actually get rid of this and just use 'DeferredKey' if rule analysis is an
/// 'Deferred' itself. This is used to construct the composed 'DeferredKey::Deferred' or
/// 'DeferredKey::Base' type.
#[derive(
    Hash,
    Eq,
    PartialEq,
    Clone,
    Dupe,
    derive_more::Display,
    Debug,
    Allocative
)]

pub enum DeferredHolderKey {
    Base(BaseDeferredKey),
    // While DynamicLambdaResultsKey is Dupe, it has quite a lot of Arc's inside it, so maybe an Arc here makes sense?
    // Maybe not?
    DynamicLambda(Arc<DynamicLambdaResultsKey>),
}

impl DeferredHolderKey {
    pub fn owner(&self) -> &BaseDeferredKey {
        match self {
            DeferredHolderKey::Base(base) => base,
            DeferredHolderKey::DynamicLambda(lambda) => lambda.owner(),
        }
    }

    /// Create action_key information from the ids, uniquely
    /// identifying this action within this target.
    pub fn action_key(&self) -> String {
        // FIXME(ndmitchell): We'd like to have some kind of user supplied name/category here,
        // rather than using the usize ids, so things are a bit more stable and as these strings
        // are likely to come up in error messages users might see (e.g. with paths).
        match self {
            DeferredHolderKey::Base(_) => String::new(),
            DeferredHolderKey::DynamicLambda(lambda) => lambda.action_key(),
        }
    }
}
