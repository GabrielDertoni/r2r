  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct FloatingPointRange {

                              pub from_value: f64,
pub to_value: f64,
pub step: f64,

                          }

                          impl WrappedTypesupport for FloatingPointRange { 

            type CStruct = rcl_interfaces__msg__FloatingPointRange; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__FloatingPointRange() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__FloatingPointRange {

                unsafe { rcl_interfaces__msg__FloatingPointRange__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__FloatingPointRange) -> () {

                unsafe { rcl_interfaces__msg__FloatingPointRange__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> FloatingPointRange {
  FloatingPointRange {
from_value: msg.from_value,
to_value: msg.to_value,
step: msg.step,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.from_value = self.from_value;
msg.to_value = self.to_value;
msg.step = self.step;
}



        }


                          
                          impl Default for FloatingPointRange {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<FloatingPointRange>::new();
                                  FloatingPointRange::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct IntegerRange {

                              pub from_value: i64,
pub to_value: i64,
pub step: u64,

                          }

                          impl WrappedTypesupport for IntegerRange { 

            type CStruct = rcl_interfaces__msg__IntegerRange; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__IntegerRange() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__IntegerRange {

                unsafe { rcl_interfaces__msg__IntegerRange__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__IntegerRange) -> () {

                unsafe { rcl_interfaces__msg__IntegerRange__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> IntegerRange {
  IntegerRange {
from_value: msg.from_value,
to_value: msg.to_value,
step: msg.step,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.from_value = self.from_value;
msg.to_value = self.to_value;
msg.step = self.step;
}



        }


                          
                          impl Default for IntegerRange {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<IntegerRange>::new();
                                  IntegerRange::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ListParametersResult {

                              pub names: Vec<std::string::String>,
pub prefixes: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for ListParametersResult { 

            type CStruct = rcl_interfaces__msg__ListParametersResult; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ListParametersResult() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ListParametersResult {

                unsafe { rcl_interfaces__msg__ListParametersResult__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ListParametersResult) -> () {

                unsafe { rcl_interfaces__msg__ListParametersResult__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ListParametersResult {
  ListParametersResult {
// is_upper_bound_: false
// member.array_size_ : 0
names: msg.names.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
prefixes: msg.prefixes.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.names.update(&self.names);
msg.prefixes.update(&self.prefixes);
}



        }


                          
                          impl Default for ListParametersResult {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ListParametersResult>::new();
                                  ListParametersResult::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Log {

                              pub stamp: builtin_interfaces::msg::Time,
pub level: u8,
pub name: std::string::String,
pub msg: std::string::String,
pub file: std::string::String,
pub function: std::string::String,
pub line: u32,

                          }

                          impl WrappedTypesupport for Log { 

            type CStruct = rcl_interfaces__msg__Log; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Log() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__Log {

                unsafe { rcl_interfaces__msg__Log__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__Log) -> () {

                unsafe { rcl_interfaces__msg__Log__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Log {
  Log {
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
level: msg.level,
name: msg.name.to_str().to_owned(),
msg: msg.msg.to_str().to_owned(),
file: msg.file.to_str().to_owned(),
function: msg.function.to_str().to_owned(),
line: msg.line,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.stamp.copy_to_native(&mut msg.stamp);
msg.level = self.level;
msg.name.assign(&self.name);
msg.msg.assign(&self.msg);
msg.file.assign(&self.file);
msg.function.assign(&self.function);
msg.line = self.line;
}



        }


                          
                          impl Default for Log {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Log>::new();
                                  Log::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Parameter {

                              pub name: std::string::String,
pub value: rcl_interfaces::msg::ParameterValue,

                          }

                          impl WrappedTypesupport for Parameter { 

            type CStruct = rcl_interfaces__msg__Parameter; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Parameter() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__Parameter {

                unsafe { rcl_interfaces__msg__Parameter__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__Parameter) -> () {

                unsafe { rcl_interfaces__msg__Parameter__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Parameter {
  Parameter {
name: msg.name.to_str().to_owned(),
value: rcl_interfaces::msg::ParameterValue::from_native(&msg.value),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
self.value.copy_to_native(&mut msg.value);
}



        }


                          
                          impl Default for Parameter {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Parameter>::new();
                                  Parameter::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParameterDescriptor {

                              pub name: std::string::String,
pub type_: u8,
pub description: std::string::String,
pub additional_constraints: std::string::String,
pub read_only: bool,
pub dynamic_typing: bool,
pub floating_point_range: Vec<rcl_interfaces::msg::FloatingPointRange>,
pub integer_range: Vec<rcl_interfaces::msg::IntegerRange>,

                          }

                          impl WrappedTypesupport for ParameterDescriptor { 

            type CStruct = rcl_interfaces__msg__ParameterDescriptor; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterDescriptor() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ParameterDescriptor {

                unsafe { rcl_interfaces__msg__ParameterDescriptor__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ParameterDescriptor) -> () {

                unsafe { rcl_interfaces__msg__ParameterDescriptor__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParameterDescriptor {
  ParameterDescriptor {
name: msg.name.to_str().to_owned(),
type_: msg.type_,
description: msg.description.to_str().to_owned(),
additional_constraints: msg.additional_constraints.to_str().to_owned(),
read_only: msg.read_only,
dynamic_typing: msg.dynamic_typing,
// is_upper_bound_: true
// member.array_size_ : 1
floating_point_range : {
let mut temp = Vec::with_capacity(msg.floating_point_range.size);
let slice = unsafe { std::slice::from_raw_parts(msg.floating_point_range.data, msg.floating_point_range.size)};
for s in slice { temp.push(rcl_interfaces::msg::FloatingPointRange::from_native(s)); }
temp },
// is_upper_bound_: true
// member.array_size_ : 1
integer_range : {
let mut temp = Vec::with_capacity(msg.integer_range.size);
let slice = unsafe { std::slice::from_raw_parts(msg.integer_range.data, msg.integer_range.size)};
for s in slice { temp.push(rcl_interfaces::msg::IntegerRange::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
msg.type_ = self.type_;
msg.description.assign(&self.description);
msg.additional_constraints.assign(&self.additional_constraints);
msg.read_only = self.read_only;
msg.dynamic_typing = self.dynamic_typing;
unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__fini(&mut msg.floating_point_range) };
unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__init(&mut msg.floating_point_range, self.floating_point_range.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.floating_point_range.data, msg.floating_point_range.size)};
for (t,s) in slice.iter_mut().zip(&self.floating_point_range) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__IntegerRange__Sequence__fini(&mut msg.integer_range) };
unsafe { rcl_interfaces__msg__IntegerRange__Sequence__init(&mut msg.integer_range, self.integer_range.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.integer_range.data, msg.integer_range.size)};
for (t,s) in slice.iter_mut().zip(&self.integer_range) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for ParameterDescriptor {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParameterDescriptor>::new();
                                  ParameterDescriptor::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParameterEvent {

                              pub stamp: builtin_interfaces::msg::Time,
pub node: std::string::String,
pub new_parameters: Vec<rcl_interfaces::msg::Parameter>,
pub changed_parameters: Vec<rcl_interfaces::msg::Parameter>,
pub deleted_parameters: Vec<rcl_interfaces::msg::Parameter>,

                          }

                          impl WrappedTypesupport for ParameterEvent { 

            type CStruct = rcl_interfaces__msg__ParameterEvent; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEvent() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ParameterEvent {

                unsafe { rcl_interfaces__msg__ParameterEvent__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ParameterEvent) -> () {

                unsafe { rcl_interfaces__msg__ParameterEvent__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParameterEvent {
  ParameterEvent {
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
node: msg.node.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
new_parameters : {
let mut temp = Vec::with_capacity(msg.new_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.new_parameters.data, msg.new_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
changed_parameters : {
let mut temp = Vec::with_capacity(msg.changed_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.changed_parameters.data, msg.changed_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
deleted_parameters : {
let mut temp = Vec::with_capacity(msg.deleted_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.deleted_parameters.data, msg.deleted_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.stamp.copy_to_native(&mut msg.stamp);
msg.node.assign(&self.node);
unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.new_parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.new_parameters, self.new_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.new_parameters.data, msg.new_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.new_parameters) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.changed_parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.changed_parameters, self.changed_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.changed_parameters.data, msg.changed_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.changed_parameters) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.deleted_parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.deleted_parameters, self.deleted_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.deleted_parameters.data, msg.deleted_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.deleted_parameters) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for ParameterEvent {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParameterEvent>::new();
                                  ParameterEvent::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParameterEventDescriptors {

                              pub new_parameters: Vec<rcl_interfaces::msg::ParameterDescriptor>,
pub changed_parameters: Vec<rcl_interfaces::msg::ParameterDescriptor>,
pub deleted_parameters: Vec<rcl_interfaces::msg::ParameterDescriptor>,

                          }

                          impl WrappedTypesupport for ParameterEventDescriptors { 

            type CStruct = rcl_interfaces__msg__ParameterEventDescriptors; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEventDescriptors() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ParameterEventDescriptors {

                unsafe { rcl_interfaces__msg__ParameterEventDescriptors__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ParameterEventDescriptors) -> () {

                unsafe { rcl_interfaces__msg__ParameterEventDescriptors__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParameterEventDescriptors {
  ParameterEventDescriptors {
// is_upper_bound_: false
// member.array_size_ : 0
new_parameters : {
let mut temp = Vec::with_capacity(msg.new_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.new_parameters.data, msg.new_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::ParameterDescriptor::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
changed_parameters : {
let mut temp = Vec::with_capacity(msg.changed_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.changed_parameters.data, msg.changed_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::ParameterDescriptor::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
deleted_parameters : {
let mut temp = Vec::with_capacity(msg.deleted_parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.deleted_parameters.data, msg.deleted_parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::ParameterDescriptor::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(&mut msg.new_parameters) };
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(&mut msg.new_parameters, self.new_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.new_parameters.data, msg.new_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.new_parameters) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(&mut msg.changed_parameters) };
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(&mut msg.changed_parameters, self.changed_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.changed_parameters.data, msg.changed_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.changed_parameters) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(&mut msg.deleted_parameters) };
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(&mut msg.deleted_parameters, self.deleted_parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.deleted_parameters.data, msg.deleted_parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.deleted_parameters) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for ParameterEventDescriptors {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParameterEventDescriptors>::new();
                                  ParameterEventDescriptors::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParameterType {

                              
                          }

                          impl WrappedTypesupport for ParameterType { 

            type CStruct = rcl_interfaces__msg__ParameterType; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterType() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ParameterType {

                unsafe { rcl_interfaces__msg__ParameterType__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ParameterType) -> () {

                unsafe { rcl_interfaces__msg__ParameterType__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> ParameterType {
  ParameterType {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for ParameterType {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParameterType>::new();
                                  ParameterType::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParameterValue {

                              pub type_: u8,
pub bool_value: bool,
pub integer_value: i64,
pub double_value: f64,
pub string_value: std::string::String,
pub byte_array_value: Vec<u8>,
pub bool_array_value: Vec<bool>,
pub integer_array_value: Vec<i64>,
pub double_array_value: Vec<f64>,
pub string_array_value: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for ParameterValue { 

            type CStruct = rcl_interfaces__msg__ParameterValue; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterValue() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__ParameterValue {

                unsafe { rcl_interfaces__msg__ParameterValue__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__ParameterValue) -> () {

                unsafe { rcl_interfaces__msg__ParameterValue__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParameterValue {
  ParameterValue {
type_: msg.type_,
bool_value: msg.bool_value,
integer_value: msg.integer_value,
double_value: msg.double_value,
string_value: msg.string_value.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
byte_array_value: msg.byte_array_value.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
bool_array_value: msg.bool_array_value.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
integer_array_value: msg.integer_array_value.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
double_array_value: msg.double_array_value.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
string_array_value: msg.string_array_value.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_ = self.type_;
msg.bool_value = self.bool_value;
msg.integer_value = self.integer_value;
msg.double_value = self.double_value;
msg.string_value.assign(&self.string_value);
msg.byte_array_value.update(&self.byte_array_value);
msg.bool_array_value.update(&self.bool_array_value);
msg.integer_array_value.update(&self.integer_array_value);
msg.double_array_value.update(&self.double_array_value);
msg.string_array_value.update(&self.string_array_value);
}



        }


                          
                          impl Default for ParameterValue {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParameterValue>::new();
                                  ParameterValue::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SetParametersResult {

                              pub successful: bool,
pub reason: std::string::String,

                          }

                          impl WrappedTypesupport for SetParametersResult { 

            type CStruct = rcl_interfaces__msg__SetParametersResult; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__SetParametersResult() }
            }

            fn create_msg() -> *mut rcl_interfaces__msg__SetParametersResult {

                unsafe { rcl_interfaces__msg__SetParametersResult__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__msg__SetParametersResult) -> () {

                unsafe { rcl_interfaces__msg__SetParametersResult__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SetParametersResult {
  SetParametersResult {
successful: msg.successful,
reason: msg.reason.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.successful = self.successful;
msg.reason.assign(&self.reason);
}



        }


                          
                          impl Default for SetParametersResult {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SetParametersResult>::new();
                                  SetParametersResult::from_native(&msg_native)
                              }
                          }
             


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod DescribeParameters {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__DescribeParameters()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub names: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__DescribeParameters_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__DescribeParameters_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__DescribeParameters_Request {

                unsafe { rcl_interfaces__srv__DescribeParameters_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__DescribeParameters_Request) -> () {

                unsafe { rcl_interfaces__srv__DescribeParameters_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
names: msg.names.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.names.update(&self.names);
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

                              pub descriptors: Vec<rcl_interfaces::msg::ParameterDescriptor>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__DescribeParameters_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__DescribeParameters_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__DescribeParameters_Response {

                unsafe { rcl_interfaces__srv__DescribeParameters_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__DescribeParameters_Response) -> () {

                unsafe { rcl_interfaces__srv__DescribeParameters_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
descriptors : {
let mut temp = Vec::with_capacity(msg.descriptors.size);
let slice = unsafe { std::slice::from_raw_parts(msg.descriptors.data, msg.descriptors.size)};
for s in slice { temp.push(rcl_interfaces::msg::ParameterDescriptor::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(&mut msg.descriptors) };
unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(&mut msg.descriptors, self.descriptors.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.descriptors.data, msg.descriptors.size)};
for (t,s) in slice.iter_mut().zip(&self.descriptors) { s.copy_to_native(t);}
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
    pub mod GetParameterTypes {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameterTypes()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub names: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__GetParameterTypes_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__GetParameterTypes_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__GetParameterTypes_Request {

                unsafe { rcl_interfaces__srv__GetParameterTypes_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__GetParameterTypes_Request) -> () {

                unsafe { rcl_interfaces__srv__GetParameterTypes_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
names: msg.names.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.names.update(&self.names);
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

                              pub types: Vec<u8>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__GetParameterTypes_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__GetParameterTypes_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__GetParameterTypes_Response {

                unsafe { rcl_interfaces__srv__GetParameterTypes_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__GetParameterTypes_Response) -> () {

                unsafe { rcl_interfaces__srv__GetParameterTypes_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
types: msg.types.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.types.update(&self.types);
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
    pub mod GetParameters {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameters()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub names: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__GetParameters_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__GetParameters_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__GetParameters_Request {

                unsafe { rcl_interfaces__srv__GetParameters_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__GetParameters_Request) -> () {

                unsafe { rcl_interfaces__srv__GetParameters_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
names: msg.names.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.names.update(&self.names);
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

                              pub values: Vec<rcl_interfaces::msg::ParameterValue>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__GetParameters_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__GetParameters_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__GetParameters_Response {

                unsafe { rcl_interfaces__srv__GetParameters_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__GetParameters_Response) -> () {

                unsafe { rcl_interfaces__srv__GetParameters_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
values : {
let mut temp = Vec::with_capacity(msg.values.size);
let slice = unsafe { std::slice::from_raw_parts(msg.values.data, msg.values.size)};
for s in slice { temp.push(rcl_interfaces::msg::ParameterValue::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__ParameterValue__Sequence__fini(&mut msg.values) };
unsafe { rcl_interfaces__msg__ParameterValue__Sequence__init(&mut msg.values, self.values.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.values.data, msg.values.size)};
for (t,s) in slice.iter_mut().zip(&self.values) { s.copy_to_native(t);}
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
    pub mod ListParameters {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__ListParameters()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub prefixes: Vec<std::string::String>,
pub depth: u64,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__ListParameters_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__ListParameters_Request {

                unsafe { rcl_interfaces__srv__ListParameters_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__ListParameters_Request) -> () {

                unsafe { rcl_interfaces__srv__ListParameters_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
prefixes: msg.prefixes.to_vec(),
depth: msg.depth,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.prefixes.update(&self.prefixes);
msg.depth = self.depth;
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

                              pub result: rcl_interfaces::msg::ListParametersResult,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__ListParameters_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__ListParameters_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__ListParameters_Response {

                unsafe { rcl_interfaces__srv__ListParameters_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__ListParameters_Response) -> () {

                unsafe { rcl_interfaces__srv__ListParameters_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
result: rcl_interfaces::msg::ListParametersResult::from_native(&msg.result),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.result.copy_to_native(&mut msg.result);
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
    pub mod SetParameters {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParameters()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub parameters: Vec<rcl_interfaces::msg::Parameter>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__SetParameters_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetParameters_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__SetParameters_Request {

                unsafe { rcl_interfaces__srv__SetParameters_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__SetParameters_Request) -> () {

                unsafe { rcl_interfaces__srv__SetParameters_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
parameters : {
let mut temp = Vec::with_capacity(msg.parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.parameters.data, msg.parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.parameters, self.parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.parameters.data, msg.parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.parameters) { s.copy_to_native(t);}
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

                              pub results: Vec<rcl_interfaces::msg::SetParametersResult>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__SetParameters_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetParameters_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__SetParameters_Response {

                unsafe { rcl_interfaces__srv__SetParameters_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__SetParameters_Response) -> () {

                unsafe { rcl_interfaces__srv__SetParameters_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
results : {
let mut temp = Vec::with_capacity(msg.results.size);
let slice = unsafe { std::slice::from_raw_parts(msg.results.data, msg.results.size)};
for s in slice { temp.push(rcl_interfaces::msg::SetParametersResult::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__SetParametersResult__Sequence__fini(&mut msg.results) };
unsafe { rcl_interfaces__msg__SetParametersResult__Sequence__init(&mut msg.results, self.results.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.results.data, msg.results.size)};
for (t,s) in slice.iter_mut().zip(&self.results) { s.copy_to_native(t);}
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
    pub mod SetParametersAtomically {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParametersAtomically()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub parameters: Vec<rcl_interfaces::msg::Parameter>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rcl_interfaces__srv__SetParametersAtomically_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetParametersAtomically_Request() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__SetParametersAtomically_Request {

                unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__SetParametersAtomically_Request) -> () {

                unsafe { rcl_interfaces__srv__SetParametersAtomically_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
// is_upper_bound_: false
// member.array_size_ : 0
parameters : {
let mut temp = Vec::with_capacity(msg.parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.parameters.data, msg.parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.parameters, self.parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.parameters.data, msg.parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.parameters) { s.copy_to_native(t);}
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

                              pub result: rcl_interfaces::msg::SetParametersResult,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rcl_interfaces__srv__SetParametersAtomically_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__srv__SetParametersAtomically_Response() }
            }

            fn create_msg() -> *mut rcl_interfaces__srv__SetParametersAtomically_Response {

                unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__create() }

            }

            fn destroy_msg(msg: *mut rcl_interfaces__srv__SetParametersAtomically_Response) -> () {

                unsafe { rcl_interfaces__srv__SetParametersAtomically_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
result: rcl_interfaces::msg::SetParametersResult::from_native(&msg.result),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.result.copy_to_native(&mut msg.result);
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
