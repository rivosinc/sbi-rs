<!--
SPDX-FileCopyrightText: 2023 Rivos Inc.

SPDX-License-Identifier: Apache-2.0
-->

# Overview

[![REUSE status](https://api.reuse.software/badge/github.com/rivosinc/sbi-rs)](https://api.reuse.software/info/github.com/rivosinc/sbi-rs)

This crate provides interfaces for both implementing(firmware or hypervisor)
and using(S and VS mode supervisors)
[SBI](https://github.com/riscv-non-isa/riscv-sbi-doc/releases) functionality.

# Code Layout

## SBI definitions

SBI-defined types are in `src/<extension_name>.rs`. Each file enumerates the
functions for the given extension and declares all data types it uses.

## API

Interfaces for invoking SBI calls from S-mode are provided in the `src/api`
directory. There is one file per extension.
