  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DiagnosticArray {

                              pub header: std_msgs::msg::Header,
pub status: Vec<diagnostic_msgs::msg::DiagnosticStatus>,

                          }

                          impl WrappedTypesupport for DiagnosticArray { 

            type CStruct = diagnostic_msgs__msg__DiagnosticArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticArray() }
            }

            fn create_msg() -> *mut diagnostic_msgs__msg__DiagnosticArray {

                unsafe { diagnostic_msgs__msg__DiagnosticArray__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__msg__DiagnosticArray) -> () {

                unsafe { diagnostic_msgs__msg__DiagnosticArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DiagnosticArray {
  DiagnosticArray {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
status : {
let mut temp = Vec::with_capacity(msg.status.size);
let slice = unsafe { std::slice::from_raw_parts(msg.status.data, msg.status.size)};
for s in slice { temp.push(diagnostic_msgs::msg::DiagnosticStatus::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(&mut msg.status) };
unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(&mut msg.status, self.status.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.status.data, msg.status.size)};
for (t,s) in slice.iter_mut().zip(&self.status) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for DiagnosticArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DiagnosticArray>::new();
                                  DiagnosticArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DiagnosticStatus {

                              pub level: u8,
pub name: std::string::String,
pub message: std::string::String,
pub hardware_id: std::string::String,
pub values: Vec<diagnostic_msgs::msg::KeyValue>,

                          }

                          impl WrappedTypesupport for DiagnosticStatus { 

            type CStruct = diagnostic_msgs__msg__DiagnosticStatus; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__DiagnosticStatus() }
            }

            fn create_msg() -> *mut diagnostic_msgs__msg__DiagnosticStatus {

                unsafe { diagnostic_msgs__msg__DiagnosticStatus__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__msg__DiagnosticStatus) -> () {

                unsafe { diagnostic_msgs__msg__DiagnosticStatus__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DiagnosticStatus {
  DiagnosticStatus {
level: msg.level,
name: msg.name.to_str().to_owned(),
message: msg.message.to_str().to_owned(),
hardware_id: msg.hardware_id.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
values : {
let mut temp = Vec::with_capacity(msg.values.size);
let slice = unsafe { std::slice::from_raw_parts(msg.values.data, msg.values.size)};
for s in slice { temp.push(diagnostic_msgs::msg::KeyValue::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.level = self.level;
msg.name.assign(&self.name);
msg.message.assign(&self.message);
msg.hardware_id.assign(&self.hardware_id);
unsafe { diagnostic_msgs__msg__KeyValue__Sequence__fini(&mut msg.values) };
unsafe { diagnostic_msgs__msg__KeyValue__Sequence__init(&mut msg.values, self.values.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.values.data, msg.values.size)};
for (t,s) in slice.iter_mut().zip(&self.values) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for DiagnosticStatus {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DiagnosticStatus>::new();
                                  DiagnosticStatus::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct KeyValue {

                              pub key: std::string::String,
pub value: std::string::String,

                          }

                          impl WrappedTypesupport for KeyValue { 

            type CStruct = diagnostic_msgs__msg__KeyValue; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__msg__KeyValue() }
            }

            fn create_msg() -> *mut diagnostic_msgs__msg__KeyValue {

                unsafe { diagnostic_msgs__msg__KeyValue__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__msg__KeyValue) -> () {

                unsafe { diagnostic_msgs__msg__KeyValue__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> KeyValue {
  KeyValue {
key: msg.key.to_str().to_owned(),
value: msg.value.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.key.assign(&self.key);
msg.value.assign(&self.value);
}



        }


                          
                          impl Default for KeyValue {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<KeyValue>::new();
                                  KeyValue::from_native(&msg_native)
                              }
                          }
             


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod AddDiagnostics {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__AddDiagnostics()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub load_namespace: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = diagnostic_msgs__srv__AddDiagnostics_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__srv__AddDiagnostics_Request() }
            }

            fn create_msg() -> *mut diagnostic_msgs__srv__AddDiagnostics_Request {

                unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__srv__AddDiagnostics_Request) -> () {

                unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
load_namespace: msg.load_namespace.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.load_namespace.assign(&self.load_namespace);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,
pub message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = diagnostic_msgs__srv__AddDiagnostics_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__srv__AddDiagnostics_Response() }
            }

            fn create_msg() -> *mut diagnostic_msgs__srv__AddDiagnostics_Response {

                unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__srv__AddDiagnostics_Response) -> () {

                unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
message: msg.message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.message.assign(&self.message);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod SelfTest {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__SelfTest()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = diagnostic_msgs__srv__SelfTest_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__srv__SelfTest_Request() }
            }

            fn create_msg() -> *mut diagnostic_msgs__srv__SelfTest_Request {

                unsafe { diagnostic_msgs__srv__SelfTest_Request__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__srv__SelfTest_Request) -> () {

                unsafe { diagnostic_msgs__srv__SelfTest_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub id: std::string::String,
pub passed: u8,
pub status: Vec<diagnostic_msgs::msg::DiagnosticStatus>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = diagnostic_msgs__srv__SelfTest_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__diagnostic_msgs__srv__SelfTest_Response() }
            }

            fn create_msg() -> *mut diagnostic_msgs__srv__SelfTest_Response {

                unsafe { diagnostic_msgs__srv__SelfTest_Response__create() }

            }

            fn destroy_msg(msg: *mut diagnostic_msgs__srv__SelfTest_Response) -> () {

                unsafe { diagnostic_msgs__srv__SelfTest_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
id: msg.id.to_str().to_owned(),
passed: msg.passed,
// is_upper_bound_: false
// member.array_size_ : 0
status : {
let mut temp = Vec::with_capacity(msg.status.size);
let slice = unsafe { std::slice::from_raw_parts(msg.status.data, msg.status.size)};
for s in slice { temp.push(diagnostic_msgs::msg::DiagnosticStatus::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.id.assign(&self.id);
msg.passed = self.passed;
unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(&mut msg.status) };
unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(&mut msg.status, self.status.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.status.data, msg.status.size)};
for (t,s) in slice.iter_mut().zip(&self.status) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
