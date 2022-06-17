// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Point32__init(msg: *mut Point32) -> bool;
    fn geometry_msgs__msg__Point32__fini(msg: *mut Point32);
    fn geometry_msgs__msg__Point32__Sequence__init(msg: *mut Point32Sequence, size: usize) -> bool;
    fn geometry_msgs__msg__Point32__Sequence__fini(msg: *mut Point32Sequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point32() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point32 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point32__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Point32 {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Point32__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Point32Sequence {
    data: *mut Point32,
    size: usize,
    capacity: usize,
}

impl Point32Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point32__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Point32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Point32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Point32Sequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Point32__Sequence__fini(self) };
    }
}

unsafe impl Send for Point32Sequence {}
unsafe impl Sync for Point32Sequence {}


impl TopicMsg for Point32 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point32()
        }
    }
}
