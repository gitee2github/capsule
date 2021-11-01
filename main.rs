// Copyright (c) 2020 Huawei Technologies Co.,Ltd. All rights reserved.
//
// Capsule is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Capsule Core Kernel Module

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::{
    c_str,
    file::File,
    file_operations::{FileOpener, FileOperations, IoctlCommand},
    miscdev,
    sync::Ref,
};
use alloc::boxed::Box;
use core::marker::PhantomPinned;

module! {
    type: CapsuleCore,
    name: b"capsule_core",
    author: b"Capsule Contributors",
    description: b"Capsule Core Kernel Modules",
    license: b"GPL v2",
}

struct GlobalMiscdev;

impl FileOpener<Pin<Ref<NoThing>>> for GlobalMiscdev {
    fn open(context: &Pin<Ref<NoThing>>) -> Result<Self::Wrapper> {
        Ok(context.clone())
    }
}

impl FileOperations for GlobalMiscdev {
    type Wrapper = Pin<Ref<NoThing>>;

    kernel::declare_file_operations!(ioctl);

    fn ioctl(
        _this: &Ref<NoThing>,
        _file: &File,
        _cmd: &mut IoctlCommand,
    ) -> Result<i32> {
        pr_info!("ioctl invoked");
        Ok(0)
    }
}

struct NoThing {
    _pin: PhantomPinned,
}

struct CapsuleCore {
    _dev: Pin<Box<miscdev::Registration<Pin<Ref<NoThing>>>>>,
}

impl KernelModule for CapsuleCore {
    fn init() -> Result<Self> {
        pr_info!("Capsule Core init\n");

        let context = Ref::pinned(Ref::try_new(NoThing{
            _pin: PhantomPinned,
        })?);

        Ok(CapsuleCore {
            _dev: miscdev::Registration::new_pinned::<GlobalMiscdev>(c_str!("capsule_core"), None, context)?,
        })
    }
}

impl Drop for CapsuleCore {
    fn drop(&mut self) {
        pr_info!("Capsule Core exit\n");
    }
}
