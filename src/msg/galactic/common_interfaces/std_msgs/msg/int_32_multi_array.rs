// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int32MultiArray__init(msg: *mut Int32MultiArray) -> bool;
    fn std_msgs__msg__Int32MultiArray__fini(msg: *mut Int32MultiArray);
    fn std_msgs__msg__Int32MultiArray__Sequence__init(msg: *mut Int32MultiArraySequence, size: usize) -> bool;
    fn std_msgs__msg__Int32MultiArray__Sequence__fini(msg: *mut Int32MultiArraySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32MultiArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int32MultiArray {
    pub layout: MultiArrayLayout,
    pub data: crate::msg::I32Seq<0>,
}

impl Int32MultiArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32MultiArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int32MultiArray {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int32MultiArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int32MultiArraySequence {
    data: *mut Int32MultiArray,
    size: usize,
    capacity: usize,
}

impl Int32MultiArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int32MultiArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int32MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int32MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int32MultiArraySequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int32MultiArray__Sequence__fini(self) };
    }
}

unsafe impl Send for Int32MultiArraySequence {}
unsafe impl Sync for Int32MultiArraySequence {}


impl TopicMsg for Int32MultiArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32MultiArray()
        }
    }
}
