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

pub struct EventLoop {
    priv raw: *GMainLoop
}

extern fn init_callback(data: *(), raw_loop: *GMainLoop) {
    let init: ~~fn(&EventLoop) = unsafe { cast::transmute(data) };
    let el = EventLoop { raw: raw_loop };
    (*init)(&el);
    unsafe {
        cast::forget(el);
    }
}

pub fn run_with_init(init: ~fn(&EventLoop)) {
    unsafe {
        na::grustna_run_with_init(init_callback,
                                  cast::transmute(~init));
    }
}

impl EventLoop {
    pub fn new() -> EventLoop {
        unsafe {
            EventLoop { raw: na::grustna_main_loop_new_thread_local() }
        }
    }

    pub fn run(&self) {
        unsafe {
            na::grustna_main_loop_run_thread_local(self.raw);
        }
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
