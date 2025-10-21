// Copyright (c) 2025 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use iceoryx2_bb_testing::instantiate_conformance_tests_with_module;
use iceoryx2_cal_conformance_tests::dynamic_storage_trait::TestData;

use iceoryx2_cal::dynamic_storage::posix_shared_memory::Storage as PosixStorage;
use iceoryx2_cal::dynamic_storage::process_local::Storage as LocalStorage;

instantiate_conformance_tests_with_module!(
    posix_shared_memory,
    iceoryx2_cal_conformance_tests::dynamic_storage_trait,
    super::PosixStorage<super::TestData>,
    super::PosixStorage<u64>
);

instantiate_conformance_tests_with_module!(
    process_local,
    iceoryx2_cal_conformance_tests::dynamic_storage_trait,
    super::LocalStorage<super::TestData>,
    super::LocalStorage<u64>
);
