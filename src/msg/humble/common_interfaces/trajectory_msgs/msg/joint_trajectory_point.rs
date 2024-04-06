// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn trajectory_msgs__msg__JointTrajectoryPoint__init(msg: *mut JointTrajectoryPoint) -> bool;
    fn trajectory_msgs__msg__JointTrajectoryPoint__fini(msg: *mut JointTrajectoryPoint);
    fn trajectory_msgs__msg__JointTrajectoryPoint__are_equal(
        lhs: *const JointTrajectoryPoint,
        rhs: *const JointTrajectoryPoint,
    ) -> bool;
    fn trajectory_msgs__msg__JointTrajectoryPoint__Sequence__init(
        msg: *mut JointTrajectoryPointSeqRaw,
        size: usize,
    ) -> bool;
    fn trajectory_msgs__msg__JointTrajectoryPoint__Sequence__fini(
        msg: *mut JointTrajectoryPointSeqRaw,
    );
    fn trajectory_msgs__msg__JointTrajectoryPoint__Sequence__are_equal(
        lhs: *const JointTrajectoryPointSeqRaw,
        rhs: *const JointTrajectoryPointSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectoryPoint(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectoryPoint {
    pub positions: crate::msg::F64Seq<0>,
    pub velocities: crate::msg::F64Seq<0>,
    pub accelerations: crate::msg::F64Seq<0>,
    pub effort: crate::msg::F64Seq<0>,
    pub time_from_start: builtin_interfaces::UnsafeDuration,
}

impl JointTrajectoryPoint {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectoryPoint__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JointTrajectoryPoint {
    fn drop(&mut self) {
        unsafe { trajectory_msgs__msg__JointTrajectoryPoint__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct JointTrajectoryPointSeqRaw {
    data: *mut JointTrajectoryPoint,
    size: usize,
    capacity: usize,
}

/// Sequence of JointTrajectoryPoint.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectoryPointSeq<const N: usize> {
    data: *mut JointTrajectoryPoint,
    size: usize,
    capacity: usize,
}

impl<const N: usize> JointTrajectoryPointSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: JointTrajectoryPointSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__init(&mut msg, size) } {
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
        let msg: JointTrajectoryPointSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[JointTrajectoryPoint] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [JointTrajectoryPoint] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, JointTrajectoryPoint> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, JointTrajectoryPoint> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for JointTrajectoryPointSeq<N> {
    fn drop(&mut self) {
        let mut msg = JointTrajectoryPointSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for JointTrajectoryPointSeq<N> {}
unsafe impl<const N: usize> Sync for JointTrajectoryPointSeq<N> {}

impl TypeSupport for JointTrajectoryPoint {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectoryPoint()
        }
    }
}

impl PartialEq for JointTrajectoryPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { trajectory_msgs__msg__JointTrajectoryPoint__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for JointTrajectoryPointSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = JointTrajectoryPointSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = JointTrajectoryPointSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            trajectory_msgs__msg__JointTrajectoryPoint__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
