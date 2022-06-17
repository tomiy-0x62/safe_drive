// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn diagnostic_msgs__srv__AddDiagnostics_Request__init(msg: *mut AddDiagnosticsRequest) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Request__fini(msg: *mut AddDiagnosticsRequest);
    fn diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__init(msg: *mut AddDiagnosticsRequestSequence, size: usize) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__fini(msg: *mut AddDiagnosticsRequestSequence);
    fn diagnostic_msgs__srv__AddDiagnostics_Response__init(msg: *mut AddDiagnosticsResponse) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Response__fini(msg: *mut AddDiagnosticsResponse);
    fn diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__init(msg: *mut AddDiagnosticsResponseSequence, size: usize) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__fini(msg: *mut AddDiagnosticsResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__AddDiagnostics() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsRequest {
    pub load_namespace: crate::msg::RosString<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsResponse {
    pub success: bool,
    pub message: crate::msg::RosString<0>,
}

impl AddDiagnosticsRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AddDiagnosticsRequest {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsRequestSequence {
    data: *mut AddDiagnosticsRequest,
    size: usize,
    capacity: usize,
}

impl AddDiagnosticsRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[AddDiagnosticsRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [AddDiagnosticsRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for AddDiagnosticsRequestSequence {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__fini(self) };
    }
}

unsafe impl Send for AddDiagnosticsRequestSequence {}
unsafe impl Sync for AddDiagnosticsRequestSequence {}


impl AddDiagnosticsResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AddDiagnosticsResponse {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsResponseSequence {
    data: *mut AddDiagnosticsResponse,
    size: usize,
    capacity: usize,
}

impl AddDiagnosticsResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[AddDiagnosticsResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [AddDiagnosticsResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for AddDiagnosticsResponseSequence {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__fini(self) };
    }
}

unsafe impl Send for AddDiagnosticsResponseSequence {}
unsafe impl Sync for AddDiagnosticsResponseSequence {}


pub struct AddDiagnostics;

impl ServiceMsg for AddDiagnostics {
    type Request = AddDiagnosticsRequest;
    type Response = AddDiagnosticsResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__AddDiagnostics()
        }
    }
}

