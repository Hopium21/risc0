// Copyright 2025 RISC Zero, Inc.
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

use std::path::PathBuf;

use cargo_metadata::Package;
use serde::{Deserialize, Serialize};

/// Options for configuring a docker build environment.
#[derive(Clone, Serialize, Deserialize)]
pub struct DockerOptions {
    /// Specify the root directory for docker builds.
    ///
    /// The current working directory is used if `None` is specified.
    pub root_dir: Option<PathBuf>,
}

/// Options defining how to embed a guest package in
/// [`crate::embed_methods_with_options`].
#[derive(Default, Clone)]
pub struct GuestOptions {
    /// Features for cargo to build the guest with.
    pub features: Vec<String>,

    /// Use a docker environment for building.
    pub use_docker: Option<DockerOptions>,
}

/// Metadata defining options to build a guest
#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct GuestMetadata {
    /// Configuration flags to build the guest with.
    #[serde(rename = "rustc-flags")]
    pub(crate) rustc_flags: Option<Vec<String>>,

    /// Indicates whether the guest program is a kernel.
    #[serde(default)]
    pub(crate) kernel: bool,
}

impl From<&Package> for GuestMetadata {
    fn from(pkg: &Package) -> Self {
        let Some(obj) = pkg.metadata.get("risc0") else {
            return Default::default();
        };
        serde_json::from_value(obj.clone()).unwrap()
    }
}

/// Extended options defining how to embed a guest package in
/// [`crate::embed_methods_with_options`].
#[derive(Default, Clone)]
pub(crate) struct GuestInfo {
    /// Options specified by build script or library usage.
    pub(crate) options: GuestOptions,

    /// Metadata specified in guest crate `Cargo.toml`.
    pub(crate) metadata: GuestMetadata,
}
