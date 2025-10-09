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

use std::alloc::Layout;
use std::mem::MaybeUninit;
use std::rc::Rc;

use clap::Parser;
use iceoryx2::prelude::{
    ipc, set_log_level_from_env_or, LogLevel, NodeBuilder, WaitSetAttachmentId, WaitSetBuilder,
};
use iceoryx2_bb_log::info;
use iceoryx2_tunnels_end_to_end_tests::cli::*;
use iceoryx2_tunnels_end_to_end_tests::config::*;
use iceoryx2_tunnels_end_to_end_tests::payload::*;
use iceoryx2_tunnels_end_to_end_tests::testing::*;

fn run_pinger<P: PayloadWriter>() -> Result<(), Box<dyn core::error::Error>> {
    let node = NodeBuilder::new().create::<ipc::Service>()?;

    let ping_publisher = node
        .service_builder(&PING_SERVICE_NAME.try_into()?)
        .publish_subscribe::<P::PayloadType>()
        .history_size(HISTORY_SIZE)
        .open_or_create()?
        .publisher_builder()
        .create()?;

    let ping_notifier = node
        .service_builder(&PING_SERVICE_NAME.try_into()?)
        .event()
        .open_or_create()?
        .notifier_builder()
        .create()?;

    let pong_subscriber = node
        .service_builder(&PONG_SERVICE_NAME.try_into()?)
        .publish_subscribe::<P::PayloadType>()
        .history_size(HISTORY_SIZE)
        .open_or_create()?
        .subscriber_builder()
        .create()?;

    let pong_listener = node
        .service_builder(&PONG_SERVICE_NAME.try_into()?)
        .event()
        .open_or_create()?
        .listener_builder()
        .create()?;

    let waitset = WaitSetBuilder::new().create::<ipc::Service>()?;

    let pong_guard = waitset.attach_notification(&pong_listener)?;
    let pong_id = WaitSetAttachmentId::from_guard(&pong_guard);

    let timeout_guard = waitset.attach_interval(TIMEOUT_DURATION)?;
    let timeout_id = WaitSetAttachmentId::from_guard(&timeout_guard);

    // Create the payload on the heap
    let ptr = unsafe { std::alloc::alloc(Layout::new::<MaybeUninit<P::PayloadType>>()) }
        as *mut MaybeUninit<P::PayloadType>;
    unsafe {
        P::write_payload(ptr.cast());
    }

    // Wrap in Rc since on_event required to be FnMut as closure technically can run N times
    let payload = Rc::from(ptr as *const P::PayloadType);

    let on_event = |id: WaitSetAttachmentId<ipc::Service>| {
        if id == pong_id {
            match pong_subscriber.receive() {
                Ok(sample) => match sample {
                    Some(pong_sample) => {
                        if pong_sample.payload() == unsafe { &**payload } {
                            pass_test();
                        } else {
                            fail_test(&format!(
                                "Unexpected sample received at subscriber. Sent: {:?}, Received: {:?}",
                                *payload,
                                *pong_sample.payload()
                            ));
                        }
                    }
                    None => {
                        fail_test("None sample at Pong Subscriber");
                    }
                },
                Err(e) => {
                    fail_test(&format!("Error receiving from Pong Subscriber: {e}"));
                }
            }
        }
        if id == timeout_id {
            fail_test("Timed out");
        }

        fail_test("Unexpected Event");
    };

    let mut ping_sample = ping_publisher.loan_uninit()?;

    // The bytes of the payload are copied directly into shared memory, by-passing stack
    unsafe {
        std::ptr::copy_nonoverlapping(
            *payload as *const u8,
            ping_sample.payload_mut().as_mut_ptr().cast(),
            std::mem::size_of::<P::PayloadType>(),
        );
    }

    let ping_sample = unsafe { ping_sample.assume_init() };
    ping_sample.send()?;
    ping_notifier.notify()?;

    waitset.wait_and_process(on_event)?;

    unsafe { std::alloc::dealloc(ptr as *mut u8, Layout::new::<MaybeUninit<P::PayloadType>>()) };

    Ok(())
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    set_log_level_from_env_or(LogLevel::Warn);

    let args = Args::parse();

    info!("Running with payload type: {:?}", args.payload_type);

    match args.payload_type {
        PayloadType::Primitive => run_pinger::<PrimitivePayload>(),
        PayloadType::Complex => run_pinger::<ComplexPayload>(),
    }
}
