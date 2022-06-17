// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const CIRCLE: i32 = 0;
pub const LINE_STRIP: i32 = 1;
pub const LINE_LIST: i32 = 2;
pub const POLYGON: i32 = 3;
pub const POINTS: i32 = 4;
pub const ADD: i32 = 0;
pub const REMOVE: i32 = 1;

extern "C" {
    fn visualization_msgs__msg__ImageMarker__init(msg: *mut ImageMarker) -> bool;
    fn visualization_msgs__msg__ImageMarker__fini(msg: *mut ImageMarker);
    fn visualization_msgs__msg__ImageMarker__Sequence__init(msg: *mut ImageMarkerSequence, size: usize) -> bool;
    fn visualization_msgs__msg__ImageMarker__Sequence__fini(msg: *mut ImageMarkerSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__ImageMarker() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct ImageMarker {
    pub header: std_msgs::msg::Header,
    pub ns: crate::msg::RosString<0>,
    pub id: i32,
    pub type_: i32,
    pub action: i32,
    pub position: geometry_msgs::msg::Point,
    pub scale: f32,
    pub outline_color: std_msgs::msg::ColorRGBA,
    pub filled: u8,
    pub fill_color: std_msgs::msg::ColorRGBA,
    pub lifetime: builtin_interfaces__msg__Duration,
    pub points: geometry_msgs::msg::PointSequence,
    pub outline_colors: std_msgs::msg::ColorRGBASequence,
}

impl ImageMarker {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__ImageMarker__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ImageMarker {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__ImageMarker__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageMarkerSequence {
    data: *mut ImageMarker,
    size: usize,
    capacity: usize,
}

impl ImageMarkerSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__ImageMarker__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[ImageMarker]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [ImageMarker]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for ImageMarkerSequence {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__ImageMarker__Sequence__fini(self) };
    }
}

unsafe impl Send for ImageMarkerSequence {}
unsafe impl Sync for ImageMarkerSequence {}


impl TopicMsg for ImageMarker {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__ImageMarker()
        }
    }
}
