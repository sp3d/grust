/* This file is part of Grust, GObject introspection bindings for Rust
 *
 * Copyright (C) 2013  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301  USA
 */

use plumbing::GMainLoop;

use na;
use glib;

use core::comm::stream;

pub struct EventLoop {
    priv raw: *GMainLoop
}

fn run_thread(func: ~fn()) {
    let mut (port, _) = stream::<task::TaskResult>();
    task::task()
        .sched_mode(task::ThreadPerTask)
        .future_result(|p| {
            port = p;
        })
        .spawn(func);
    match (port.recv()) {
        task::Success => {}
        task::Failure => { fail!(); }
    }
}

impl EventLoop {
    pub fn new() -> EventLoop {
        unsafe {
            EventLoop { raw: na::grustna_main_loop_new_thread_local() }
        }
    }

    pub fn run(&self) {
        run_thread(|| unsafe {
                na::grustna_main_loop_run_thread_local(self.raw);
            });
    }

    pub fn quit(&self) {
        unsafe {
            glib::g_main_loop_quit(self.raw);
        }
    }
}

#[unsafe_destructor]
impl Drop for EventLoop {
    fn finalize(&self) {
        unsafe {
            glib::g_main_loop_unref(self.raw);
        }
    }
}

impl Clone for EventLoop {
    fn clone(&self) -> EventLoop {
        unsafe {
            glib::g_main_loop_ref(self.raw);
            EventLoop { raw: self.raw }
        }
    }
}
