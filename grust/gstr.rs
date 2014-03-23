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

use ffi;
use types::*;

use std::libc;
use std::str;
use std::fmt;

pub struct utf8 {
    priv data: *gchar,
}

impl utf8 {
    pub unsafe fn wrap(data: *gchar) -> utf8 {
        utf8 { data: data }
    }
}

impl Drop for utf8 {
    fn drop(&mut self) {
        unsafe {
            ffi::g_free(self.data as *());
        }
    }
}

impl Clone for utf8 {
    fn clone(&self) -> utf8 {
        unsafe {
            utf8::wrap(ffi::g_strdup(self.data))
        }
    }
}

impl fmt::Show for utf8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.buf.write_str(unsafe {str::raw::c_str_to_static_slice(self.data)})
    }
}

impl Eq for utf8 {
    fn eq(&self, other: &utf8) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }

    fn ne(&self, other: &utf8) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) != 0
        }
    }
}

impl TotalEq for utf8 {
    fn equals(&self, other: &utf8) -> bool {
        unsafe {
            libc::strcmp(self.data, other.data) == 0
        }
    }
}
