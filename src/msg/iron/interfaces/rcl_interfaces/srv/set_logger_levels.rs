// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__srv__SetLoggerLevels_Request__init(msg: *mut SetLoggerLevelsRequest)
        -> bool;
    fn rcl_interfaces__srv__SetLoggerLevels_Request__fini(msg: *mut SetLoggerLevelsRequest);
    fn rcl_interfaces__srv__SetLoggerLevels_Request__Sequence__init(
        msg: *mut SetLoggerLevelsRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__srv__SetLoggerLevels_Request__Sequence__fini(
        msg: *mut SetLoggerLevelsRequestSeqRaw,
    );
    fn rcl_interfaces__srv__SetLoggerLevels_Response__init(
        msg: *mut SetLoggerLevelsResponse,
    ) -> bool;
    fn rcl_interfaces__srv__SetLoggerLevels_Response__fini(msg: *mut SetLoggerLevelsResponse);
    fn rcl_interfaces__srv__SetLoggerLevels_Response__Sequence__init(
        msg: *mut SetLoggerLevelsResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__srv__SetLoggerLevels_Response__Sequence__fini(
        msg: *mut SetLoggerLevelsResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetLoggerLevels(
    ) -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetLoggerLevels_Request(
    ) -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetLoggerLevels_Response(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsRequest {
    pub levels: LoggerLevelSeq<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsResponse {
    pub results: SetLoggerLevelsResultSeq<0>,
}

impl SetLoggerLevelsRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetLoggerLevels_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetLoggerLevelsRequest {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__SetLoggerLevels_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetLoggerLevelsRequestSeqRaw {
    data: *mut SetLoggerLevelsRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of SetLoggerLevelsRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsRequestSeq<const N: usize> {
    data: *mut SetLoggerLevelsRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetLoggerLevelsRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetLoggerLevelsRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetLoggerLevels_Request__Sequence__init(&mut msg, size) } {
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
        let msg: SetLoggerLevelsRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[SetLoggerLevelsRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetLoggerLevelsRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetLoggerLevelsRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetLoggerLevelsRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetLoggerLevelsRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetLoggerLevelsRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__srv__SetLoggerLevels_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetLoggerLevelsRequestSeq<N> {}
unsafe impl<const N: usize> Sync for SetLoggerLevelsRequestSeq<N> {}

impl SetLoggerLevelsResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetLoggerLevels_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetLoggerLevelsResponse {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__srv__SetLoggerLevels_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetLoggerLevelsResponseSeqRaw {
    data: *mut SetLoggerLevelsResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of SetLoggerLevelsResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsResponseSeq<const N: usize> {
    data: *mut SetLoggerLevelsResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SetLoggerLevelsResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetLoggerLevelsResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__srv__SetLoggerLevels_Response__Sequence__init(&mut msg, size) }
        {
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
        let msg: SetLoggerLevelsResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[SetLoggerLevelsResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetLoggerLevelsResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetLoggerLevelsResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetLoggerLevelsResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetLoggerLevelsResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetLoggerLevelsResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__srv__SetLoggerLevels_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetLoggerLevelsResponseSeq<N> {}
unsafe impl<const N: usize> Sync for SetLoggerLevelsResponseSeq<N> {}

pub struct SetLoggerLevels;

impl ServiceMsg for SetLoggerLevels {
    type Request = SetLoggerLevelsRequest;
    type Response = SetLoggerLevelsResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetLoggerLevels()
        }
    }
}

impl TypeSupport for SetLoggerLevelsRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetLoggerLevels_Request()
        }
    }
}

impl TypeSupport for SetLoggerLevelsResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetLoggerLevels_Response()
        }
    }
}
