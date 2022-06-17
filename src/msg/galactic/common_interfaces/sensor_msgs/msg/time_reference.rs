// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__TimeReference__init(msg: *mut TimeReference) -> bool;
    fn sensor_msgs__msg__TimeReference__fini(msg: *mut TimeReference);
    fn sensor_msgs__msg__TimeReference__Sequence__init(msg: *mut TimeReferenceSequence, size: usize) -> bool;
    fn sensor_msgs__msg__TimeReference__Sequence__fini(msg: *mut TimeReferenceSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__TimeReference() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct TimeReference {
    pub header: std_msgs::msg::Header,
    pub time_ref: builtin_interfaces__msg__Time,
    pub source: crate::msg::RosString<0>,
}

impl TimeReference {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__TimeReference__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TimeReference {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__TimeReference__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeReferenceSequence {
    data: *mut TimeReference,
    size: usize,
    capacity: usize,
}

impl TimeReferenceSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__TimeReference__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TimeReference]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TimeReference]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for TimeReferenceSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__TimeReference__Sequence__fini(self) };
    }
}

unsafe impl Send for TimeReferenceSequence {}
unsafe impl Sync for TimeReferenceSequence {}


impl TopicMsg for TimeReference {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__TimeReference()
        }
    }
}
