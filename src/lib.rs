/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#[macro_use]
extern crate bitflags;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

use gio_sys::GAsyncResult;
use glib_sys::{GError, GVariant};
use gobject_sys::GObject;

use libc::{c_char, c_uint, c_void, uint32_t};

type GAsyncReadyCallback = unsafe extern fn(source_object: *mut GObject, res: *mut GAsyncResult, user_data: *mut c_void);
type GBusAcquiredCallback = unsafe extern fn(connection: *mut GDBusConnection, name: *const c_char, user_data: *mut c_void);
type GBusNameAcquiredCallback = unsafe extern fn(connection: *mut GDBusConnection, name: *const c_char, user_data: *mut c_void);
type GBusNameAppearedCallback = unsafe extern fn(connection: *mut GDBusConnection, name: *const c_char, name_owner: *const c_char, user_data: *mut c_void);
type GBusNameLostCallback = unsafe extern fn(connection: *mut GDBusConnection, name: *const c_char, user_data: *mut c_void);
type GBusNameVanishedCallback = unsafe extern fn(connection: *mut GDBusConnection, name: *const c_char, user_data: *mut c_void);
type GDBusInterfaceGetPropertyFunc = unsafe extern fn(connection: *mut GDBusConnection, sender: *const c_char, object_path: *const c_char, interface_name: *const c_char, property_name: *const c_char, error: *mut *mut GError, user_data: *mut c_void);
type GDBusInterfaceMethodCallFunc = unsafe extern fn(connection: *mut GDBusConnection, sender: *const c_char, object_path: *const c_char, interface_name: *const c_char, method_name: *const c_char, parameters: *mut GVariant, invocation: *mut GDBusMethodInvocation, user_data: *mut c_void);
type GDBusInterfaceSetPropertyFunc = unsafe extern fn(connection: *mut GDBusConnection, sender: *const c_char, object_path: *const c_char, interface_name: *const c_char, property_name: *const c_char, value: *mut GVariant, error: *mut *mut GError, user_data: *mut c_void);
type GDestroyNotify = unsafe extern fn(data: *mut c_void);

bitflags! {
    #[repr(C)]
    pub flags GDBusSendMessageFlags: c_uint {
        const G_DBUS_SEND_MESSAGE_FLAGS_NONE = 0,
        const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL = 1,
    }
}

bitflags! {
    #[repr(C)]
    pub flags GBusNameOwnerFlags: c_uint {
        const G_BUS_NAME_OWNER_FLAGS_NONE = 0,
        const G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT = 1,
        const G_BUS_NAME_OWNER_FLAGS_REPLACE = 2,
    }
}

bitflags! {
    #[repr(C)]
    pub flags GBusNameWatcherFlags: c_uint {
        const G_BUS_NAME_WATCHER_FLAGS_NONE = 0,
        const G_BUS_NAME_WATCHER_FLAGS_AUTO_START = 1,
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub enum GDBusMessageType {
    G_DBUS_MESSAGE_TYPE_INVALID,
    G_DBUS_MESSAGE_TYPE_METHOD_CALL,
    G_DBUS_MESSAGE_TYPE_METHOD_RETURN,
    G_DBUS_MESSAGE_TYPE_ERROR,
    G_DBUS_MESSAGE_TYPE_SIGNAL,
}

#[repr(C)]
pub enum GBusType {
    G_BUS_TYPE_STARTER = -1,
    G_BUS_TYPE_NONE = 0,
    G_BUS_TYPE_SYSTEM = 1,
    G_BUS_TYPE_SESSION = 2,
}

pub enum GDBusConnection {}
pub enum GDBusAnnotationInfo {}
pub enum GDBusMessage {}
pub enum GDBusMethodInvocation {}
pub enum GDBusPropertyInfo {}
pub enum GDBusSignalInfo {}

#[repr(C)]
pub struct GDBusArgInfo {
    pub ref_count: isize,
    pub name: *const c_char,
    pub signature: *const c_char,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}

#[repr(C)]
pub struct GDBusInterfaceInfo {
    pub ref_count: isize,
    pub name: *const c_char,
    pub methods: *mut *mut GDBusMethodInfo,
    pub signals: *mut *mut GDBusSignalInfo,
    pub properties: *mut *mut GDBusPropertyInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}

#[repr(C)]
pub struct GDBusMethodInfo {
    pub ref_count: isize,
    pub name: *const c_char,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}

#[repr(C)]
pub struct GDBusNodeInfo {
    pub ref_count: isize,
    pub path: *const c_char,
    pub interfaces: *mut *mut GDBusInterfaceInfo,
    pub nodes: *mut *mut GDBusNodeInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}

#[repr(C)]
pub struct GDBusInterfaceVTable {
    pub method_call: GDBusInterfaceMethodCallFunc,
    pub get_property: GDBusInterfaceGetPropertyFunc,
    pub set_property: GDBusInterfaceSetPropertyFunc,
}

extern {
    pub fn g_bus_own_name(bus_type: GBusType, name: *const c_char, flags: GBusNameOwnerFlags, bus_acquired_handler: GBusAcquiredCallback, name_acquired_handler: GBusNameAcquiredCallback, name_lost_handler: GBusNameLostCallback, user_data: *mut c_void, user_data_free_func: GDestroyNotify) -> c_uint;
    pub fn g_bus_unown_name(owner_id: c_uint);

    pub fn g_bus_watch_name(bus_type: GBusType, name: *const c_char, flags: GBusNameWatcherFlags, name_appeared_handler: GBusNameAppearedCallback, name_vanished_handler: GBusNameVanishedCallback, user_data: *mut c_void, user_data_free_func: GDestroyNotify) -> c_uint;
    pub fn g_bus_unwatch_name(watcher_id: c_uint);

    pub fn g_dbus_connection_register_object(connection: *mut GDBusConnection, object_path: *const c_char, interface_info: *mut GDBusInterfaceInfo, vtable: *const GDBusInterfaceVTable, user_data: *mut c_void, user_data_free_func: GDestroyNotify, error: *mut *mut GError) -> c_uint;
    pub fn g_dbus_connection_send_message_with_reply(connection: *mut GDBusConnection, message: *mut GDBusMessage, flags: GDBusSendMessageFlags, timeout_msec: isize, out_serial: *mut uint32_t, cancellable: *mut c_void, callback: GAsyncReadyCallback, user_data: *mut c_void) -> *mut GDBusMessage;
    pub fn g_dbus_connection_send_message_with_reply_finish(connection: *mut GDBusConnection, res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusMessage;
    pub fn g_dbus_connection_send_message_with_reply_sync(connection: *mut GDBusConnection, message: *mut GDBusMessage, flags: GDBusSendMessageFlags, timeout_msec: isize, out_serial: *mut uint32_t, cancellable: *mut c_void, error: *mut *mut GError) -> *mut GDBusMessage;

    pub fn g_dbus_node_info_new_for_xml(xml_data: *const c_char, error: *mut *mut GError) -> *mut GDBusNodeInfo;
    pub fn g_dbus_node_info_unref(info: *mut GDBusNodeInfo);

    pub fn g_dbus_message_new_method_call(name: *const c_char, path: *const c_char, interface: *const c_char, method: *const c_char) -> *mut GDBusMessage;
    pub fn g_dbus_message_get_message_type(message: *mut GDBusMessage) -> GDBusMessageType;
    pub fn g_dbus_message_get_body(message: *mut GDBusMessage) -> *mut GVariant;
    pub fn g_dbus_message_set_body(message: *mut GDBusMessage, body: *mut GVariant);
    pub fn g_dbus_message_to_gerror(message: *mut GDBusMessage, error: *mut *mut GError);

    pub fn g_dbus_method_invocation_return_value(invocation: *mut GDBusMethodInvocation, parameters: *mut GVariant);
}
