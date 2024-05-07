// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__JointState__init(msg: *mut JointState) -> bool;
    fn sensor_msgs__msg__JointState__fini(msg: *mut JointState);
    fn sensor_msgs__msg__JointState__are_equal(
        lhs: *const JointState,
        rhs: *const JointState,
    ) -> bool;
    fn sensor_msgs__msg__JointState__Sequence__init(
        msg: *mut JointStateSeqRaw,
        size: usize,
    ) -> bool;
    fn sensor_msgs__msg__JointState__Sequence__fini(msg: *mut JointStateSeqRaw);
    fn sensor_msgs__msg__JointState__Sequence__are_equal(
        lhs: *const JointStateSeqRaw,
        rhs: *const JointStateSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JointState(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct JointState {
    pub header: std_msgs::msg::Header,
    pub name: crate::msg::RosStringSeq<0, 0>,
    pub position: crate::msg::F64Seq<0>,
    pub velocity: crate::msg::F64Seq<0>,
    pub effort: crate::msg::F64Seq<0>,
}

impl JointState {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JointState__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JointState {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__JointState__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JointStateSeqRaw {
    data: *mut JointState,
    size: size_t,
    capacity: size_t,
}

/// Sequence of JointState.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JointStateSeq<const N: usize> {
    data: *mut JointState,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> JointStateSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JointStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__JointState__Sequence__init(&mut msg, size) } {
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
        let msg: JointStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[JointState] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [JointState] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, JointState> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, JointState> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for JointStateSeq<N> {
    fn drop(&mut self) {
        let mut msg = JointStateSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { sensor_msgs__msg__JointState__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JointStateSeq<N> {}
unsafe impl<const N: usize> Sync for JointStateSeq<N> {}

impl TypeSupport for JointState {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__JointState()
        }
    }
}

impl PartialEq for JointState {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sensor_msgs__msg__JointState__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for JointStateSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = JointStateSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = JointStateSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            sensor_msgs__msg__JointState__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
