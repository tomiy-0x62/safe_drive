// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn visualization_msgs__msg__MarkerArray__init(msg: *mut MarkerArray) -> bool;
    fn visualization_msgs__msg__MarkerArray__fini(msg: *mut MarkerArray);
    fn visualization_msgs__msg__MarkerArray__are_equal(
        lhs: *const MarkerArray,
        rhs: *const MarkerArray,
    ) -> bool;
    fn visualization_msgs__msg__MarkerArray__Sequence__init(
        msg: *mut MarkerArraySeqRaw,
        size: usize,
    ) -> bool;
    fn visualization_msgs__msg__MarkerArray__Sequence__fini(msg: *mut MarkerArraySeqRaw);
    fn visualization_msgs__msg__MarkerArray__Sequence__are_equal(
        lhs: *const MarkerArraySeqRaw,
        rhs: *const MarkerArraySeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MarkerArray(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct MarkerArray {
    pub markers: MarkerSeq<0>,
}

impl MarkerArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MarkerArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MarkerArray {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__MarkerArray__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MarkerArraySeqRaw {
    data: *mut MarkerArray,
    size: usize,
    capacity: usize,
}

/// Sequence of MarkerArray.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MarkerArraySeq<const N: usize> {
    data: *mut MarkerArray,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MarkerArraySeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MarkerArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MarkerArray__Sequence__init(&mut msg, size) } {
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
        let msg: MarkerArraySeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MarkerArray] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MarkerArray] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MarkerArray> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MarkerArray> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MarkerArraySeq<N> {
    fn drop(&mut self) {
        let mut msg = MarkerArraySeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { visualization_msgs__msg__MarkerArray__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MarkerArraySeq<N> {}
unsafe impl<const N: usize> Sync for MarkerArraySeq<N> {}

impl TypeSupport for MarkerArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MarkerArray()
        }
    }
}

impl PartialEq for MarkerArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { visualization_msgs__msg__MarkerArray__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MarkerArraySeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MarkerArraySeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MarkerArraySeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            visualization_msgs__msg__MarkerArray__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
