// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn sensor_msgs__srv__SetCameraInfo_Request__init(msg: *mut SetCameraInfoRequest) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Request__fini(msg: *mut SetCameraInfoRequest);
    fn sensor_msgs__srv__SetCameraInfo_Request__Sequence__init(msg: *mut SetCameraInfoRequestSequence, size: usize) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Request__Sequence__fini(msg: *mut SetCameraInfoRequestSequence);
    fn sensor_msgs__srv__SetCameraInfo_Response__init(msg: *mut SetCameraInfoResponse) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Response__fini(msg: *mut SetCameraInfoResponse);
    fn sensor_msgs__srv__SetCameraInfo_Response__Sequence__init(msg: *mut SetCameraInfoResponseSequence, size: usize) -> bool;
    fn sensor_msgs__srv__SetCameraInfo_Response__Sequence__fini(msg: *mut SetCameraInfoResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__sensor_msgs__srv__SetCameraInfo() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoRequest {
    pub camera_info: CameraInfo,
}

#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoResponse {
    pub success: bool,
    pub status_message: crate::msg::RosString<0>,
}

impl SetCameraInfoRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetCameraInfoRequest {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoRequestSequence {
    data: *mut SetCameraInfoRequest,
    size: usize,
    capacity: usize,
}

impl SetCameraInfoRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SetCameraInfoRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SetCameraInfoRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for SetCameraInfoRequestSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Request__Sequence__fini(self) };
    }
}

unsafe impl Send for SetCameraInfoRequestSequence {}
unsafe impl Sync for SetCameraInfoRequestSequence {}


impl SetCameraInfoResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetCameraInfoResponse {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SetCameraInfoResponseSequence {
    data: *mut SetCameraInfoResponse,
    size: usize,
    capacity: usize,
}

impl SetCameraInfoResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__srv__SetCameraInfo_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SetCameraInfoResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SetCameraInfoResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for SetCameraInfoResponseSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__srv__SetCameraInfo_Response__Sequence__fini(self) };
    }
}

unsafe impl Send for SetCameraInfoResponseSequence {}
unsafe impl Sync for SetCameraInfoResponseSequence {}


pub struct SetCameraInfo;

impl ServiceMsg for SetCameraInfo {
    type Request = SetCameraInfoRequest;
    type Response = SetCameraInfoResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__sensor_msgs__srv__SetCameraInfo()
        }
    }
}

