#!/usr/bin/env bash
# Copyright (c) 2024 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache Software License 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
# which is available at https://opensource.org/licenses/MIT.
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

set -e

cd $(git rev-parse --show-toplevel)

sudo ./internal/scripts/install_dependencies_ubuntu.sh

sudo useradd testuser1
sudo useradd testuser2
sudo groupadd testgroup1
sudo groupadd testgroup2
