// Copyright (c) 2023 - 2025 Restate Software, Inc., Restate GmbH.
// All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

pub mod list_nodes;

use cling::prelude::*;

#[derive(Run, Subcommand, Clone)]
pub enum Nodes {
    /// Print a summary of the active nodes registered in a cluster
    List(list_nodes::ListNodesOpts),
}
