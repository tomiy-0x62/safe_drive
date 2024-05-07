// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn lifecycle_msgs__srv__GetState_Request__init(msg: *mut GetStateRequest) -> bool;
    fn lifecycle_msgs__srv__GetState_Request__fini(msg: *mut GetStateRequest);
    fn lifecycle_msgs__srv__GetState_Request__Sequence__init(
        msg: *mut GetStateRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn lifecycle_msgs__srv__GetState_Request__Sequence__fini(msg: *mut GetStateRequestSeqRaw);
    fn lifecycle_msgs__srv__GetState_Response__init(msg: *mut GetStateResponse) -> bool;
    fn lifecycle_msgs__srv__GetState_Response__fini(msg: *mut GetStateResponse);
    fn lifecycle_msgs__srv__GetState_Response__Sequence__init(
        msg: *mut GetStateResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn lifecycle_msgs__srv__GetState_Response__Sequence__fini(msg: *mut GetStateResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__GetState(
    ) -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Request(
    ) -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Response(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct GetStateRequest {
    _unused: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct GetStateResponse {
    pub current_state: State,
}

impl GetStateRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__GetState_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetStateRequest {
    fn drop(&mut self) {
        unsafe { lifecycle_msgs__srv__GetState_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GetStateRequestSeqRaw {
    data: *mut GetStateRequest,
    size: size_t,
    capacity: size_t,
}

/// Sequence of GetStateRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetStateRequestSeq<const N: usize> {
    data: *mut GetStateRequest,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> GetStateRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetStateRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__GetState_Request__Sequence__init(&mut msg, size) } {
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
        let msg: GetStateRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GetStateRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GetStateRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GetStateRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, GetStateRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for GetStateRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetStateRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { lifecycle_msgs__srv__GetState_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GetStateRequestSeq<N> {}
unsafe impl<const N: usize> Sync for GetStateRequestSeq<N> {}

impl GetStateResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__GetState_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetStateResponse {
    fn drop(&mut self) {
        unsafe { lifecycle_msgs__srv__GetState_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GetStateResponseSeqRaw {
    data: *mut GetStateResponse,
    size: size_t,
    capacity: size_t,
}

/// Sequence of GetStateResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetStateResponseSeq<const N: usize> {
    data: *mut GetStateResponse,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> GetStateResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetStateResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { lifecycle_msgs__srv__GetState_Response__Sequence__init(&mut msg, size) } {
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
        let msg: GetStateResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GetStateResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GetStateResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GetStateResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, GetStateResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for GetStateResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetStateResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { lifecycle_msgs__srv__GetState_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GetStateResponseSeq<N> {}
unsafe impl<const N: usize> Sync for GetStateResponseSeq<N> {}

pub struct GetState;

impl ServiceMsg for GetState {
    type Request = GetStateRequest;
    type Response = GetStateResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__lifecycle_msgs__srv__GetState()
        }
    }
}

impl TypeSupport for GetStateRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Request()
        }
    }
}

impl TypeSupport for GetStateResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__lifecycle_msgs__srv__GetState_Response()
        }
    }
}
