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

use core::time::Duration;

extern crate alloc;
use alloc::boxed::Box;

use examples_common::ComplexType;
use iceoryx2::prelude::*;
use iceoryx2_bb_log::info;

const CYCLE_TIME: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn core::error::Error>> {
    set_log_level_from_env_or(LogLevel::Info);
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let service = node
        .service_builder(&"CrossLanguageComplexTypes".try_into()?)
        .publish_subscribe::<ComplexType>()
        .open_or_create()?;

    let subscriber = service.subscriber_builder().create()?;

    info!("Subscriber ready to receive data!");

    while node.wait(CYCLE_TIME).is_ok() {
        while let Some(sample) = subscriber.receive()? {
            info!("received: {}", sample.some_matrix[2][5]);
        }
    }

    info!("exit");

    Ok(())
}
