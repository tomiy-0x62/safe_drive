// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const STATISTICS_DATA_TYPE_UNINITIALIZED: u8 = 0;
pub const STATISTICS_DATA_TYPE_AVERAGE: u8 = 1;
pub const STATISTICS_DATA_TYPE_MINIMUM: u8 = 2;
pub const STATISTICS_DATA_TYPE_MAXIMUM: u8 = 3;
pub const STATISTICS_DATA_TYPE_STDDEV: u8 = 4;
pub const STATISTICS_DATA_TYPE_SAMPLE_COUNT: u8 = 5;

extern "C" {
    fn statistics_msgs__msg__StatisticDataType__init(msg: *mut StatisticDataType) -> bool;
    fn statistics_msgs__msg__StatisticDataType__fini(msg: *mut StatisticDataType);
    fn statistics_msgs__msg__StatisticDataType__are_equal(
        lhs: *const StatisticDataType,
        rhs: *const StatisticDataType,
    ) -> bool;
    fn statistics_msgs__msg__StatisticDataType__Sequence__init(
        msg: *mut StatisticDataTypeSeqRaw,
        size: usize,
    ) -> bool;
    fn statistics_msgs__msg__StatisticDataType__Sequence__fini(msg: *mut StatisticDataTypeSeqRaw);
    fn statistics_msgs__msg__StatisticDataType__Sequence__are_equal(
        lhs: *const StatisticDataTypeSeqRaw,
        rhs: *const StatisticDataTypeSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__StatisticDataType(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct StatisticDataType {
    _unused: u8,
}

impl StatisticDataType {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { statistics_msgs__msg__StatisticDataType__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for StatisticDataType {
    fn drop(&mut self) {
        unsafe { statistics_msgs__msg__StatisticDataType__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct StatisticDataTypeSeqRaw {
    data: *mut StatisticDataType,
    size: usize,
    capacity: usize,
}

/// Sequence of StatisticDataType.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct StatisticDataTypeSeq<const N: usize> {
    data: *mut StatisticDataType,
    size: usize,
    capacity: usize,
}

impl<const N: usize> StatisticDataTypeSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: StatisticDataTypeSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { statistics_msgs__msg__StatisticDataType__Sequence__init(&mut msg, size) } {
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
        let msg: StatisticDataTypeSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[StatisticDataType] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [StatisticDataType] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, StatisticDataType> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, StatisticDataType> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for StatisticDataTypeSeq<N> {
    fn drop(&mut self) {
        let mut msg = StatisticDataTypeSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { statistics_msgs__msg__StatisticDataType__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for StatisticDataTypeSeq<N> {}
unsafe impl<const N: usize> Sync for StatisticDataTypeSeq<N> {}

impl TypeSupport for StatisticDataType {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__StatisticDataType()
        }
    }
}

impl PartialEq for StatisticDataType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { statistics_msgs__msg__StatisticDataType__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for StatisticDataTypeSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = StatisticDataTypeSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = StatisticDataTypeSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            statistics_msgs__msg__StatisticDataType__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
