// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__Joy__init(msg: *mut Joy) -> bool;
    fn sensor_msgs__msg__Joy__fini(msg: *mut Joy);
    fn sensor_msgs__msg__Joy__are_equal(lhs: *const Joy, rhs: *const Joy) -> bool;
    fn sensor_msgs__msg__Joy__Sequence__init(msg: *mut JoySeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__Joy__Sequence__fini(msg: *mut JoySeqRaw);
    fn sensor_msgs__msg__Joy__Sequence__are_equal(
        lhs: *const JoySeqRaw,
        rhs: *const JoySeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Joy(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Joy {
    pub header: std_msgs::msg::Header,
    pub axes: crate::msg::F32Seq<0>,
    pub buttons: crate::msg::I32Seq<0>,
}

impl Joy {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Joy__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Joy {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Joy__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JoySeqRaw {
    data: *mut Joy,
    size: usize,
    capacity: usize,
}

/// Sequence of Joy.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JoySeq<const N: usize> {
    data: *mut Joy,
    size: usize,
    capacity: usize,
}

impl<const N: usize> JoySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JoySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Joy__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: JoySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Joy] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Joy] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Joy> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Joy> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for JoySeq<N> {
    fn drop(&mut self) {
        let mut msg = JoySeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__msg__Joy__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JoySeq<N> {}
unsafe impl<const N: usize> Sync for JoySeq<N> {}

impl TypeSupport for Joy {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Joy() }
    }
}

impl PartialEq for Joy {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sensor_msgs__msg__Joy__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for JoySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = JoySeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = JoySeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            sensor_msgs__msg__Joy__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
