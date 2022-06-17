// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int64__init(msg: *mut Int64) -> bool;
    fn std_msgs__msg__Int64__fini(msg: *mut Int64);
    fn std_msgs__msg__Int64__Sequence__init(msg: *mut Int64Sequence, size: usize) -> bool;
    fn std_msgs__msg__Int64__Sequence__fini(msg: *mut Int64Sequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int64() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int64 {
    pub data: i64,
}

impl Int64 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int64__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int64 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int64__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int64Sequence {
    data: *mut Int64,
    size: usize,
    capacity: usize,
}

impl Int64Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int64__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int64]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int64]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int64Sequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int64__Sequence__fini(self) };
    }
}

unsafe impl Send for Int64Sequence {}
unsafe impl Sync for Int64Sequence {}


impl TopicMsg for Int64 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int64()
        }
    }
}
