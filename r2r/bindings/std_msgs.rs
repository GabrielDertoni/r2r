  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Bool {

                              pub data: bool,

                          }

                          impl WrappedTypesupport for Bool { 

            type CStruct = std_msgs__msg__Bool; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Bool() }
            }

            fn create_msg() -> *mut std_msgs__msg__Bool {

                unsafe { std_msgs__msg__Bool__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Bool) -> () {

                unsafe { std_msgs__msg__Bool__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Bool {
  Bool {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Bool {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Bool>::new();
                                  Bool::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Byte {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for Byte { 

            type CStruct = std_msgs__msg__Byte; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Byte() }
            }

            fn create_msg() -> *mut std_msgs__msg__Byte {

                unsafe { std_msgs__msg__Byte__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Byte) -> () {

                unsafe { std_msgs__msg__Byte__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Byte {
  Byte {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Byte {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Byte>::new();
                                  Byte::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ByteMultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for ByteMultiArray { 

            type CStruct = std_msgs__msg__ByteMultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__ByteMultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__ByteMultiArray {

                unsafe { std_msgs__msg__ByteMultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__ByteMultiArray) -> () {

                unsafe { std_msgs__msg__ByteMultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ByteMultiArray {
  ByteMultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for ByteMultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ByteMultiArray>::new();
                                  ByteMultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Char {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for Char { 

            type CStruct = std_msgs__msg__Char; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Char() }
            }

            fn create_msg() -> *mut std_msgs__msg__Char {

                unsafe { std_msgs__msg__Char__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Char) -> () {

                unsafe { std_msgs__msg__Char__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Char {
  Char {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Char {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Char>::new();
                                  Char::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ColorRGBA {

                              pub r: f32,
pub g: f32,
pub b: f32,
pub a: f32,

                          }

                          impl WrappedTypesupport for ColorRGBA { 

            type CStruct = std_msgs__msg__ColorRGBA; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__ColorRGBA() }
            }

            fn create_msg() -> *mut std_msgs__msg__ColorRGBA {

                unsafe { std_msgs__msg__ColorRGBA__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__ColorRGBA) -> () {

                unsafe { std_msgs__msg__ColorRGBA__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ColorRGBA {
  ColorRGBA {
r: msg.r,
g: msg.g,
b: msg.b,
a: msg.a,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.r = self.r;
msg.g = self.g;
msg.b = self.b;
msg.a = self.a;
}



        }


                          
                          impl Default for ColorRGBA {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ColorRGBA>::new();
                                  ColorRGBA::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Empty {

                              
                          }

                          impl WrappedTypesupport for Empty { 

            type CStruct = std_msgs__msg__Empty; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Empty() }
            }

            fn create_msg() -> *mut std_msgs__msg__Empty {

                unsafe { std_msgs__msg__Empty__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Empty) -> () {

                unsafe { std_msgs__msg__Empty__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Empty {
  Empty {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Empty {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Empty>::new();
                                  Empty::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float32 {

                              pub data: f32,

                          }

                          impl WrappedTypesupport for Float32 { 

            type CStruct = std_msgs__msg__Float32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float32() }
            }

            fn create_msg() -> *mut std_msgs__msg__Float32 {

                unsafe { std_msgs__msg__Float32__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Float32) -> () {

                unsafe { std_msgs__msg__Float32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float32 {
  Float32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Float32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float32>::new();
                                  Float32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float32MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<f32>,

                          }

                          impl WrappedTypesupport for Float32MultiArray { 

            type CStruct = std_msgs__msg__Float32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float32MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Float32MultiArray {

                unsafe { std_msgs__msg__Float32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Float32MultiArray) -> () {

                unsafe { std_msgs__msg__Float32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float32MultiArray {
  Float32MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Float32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float32MultiArray>::new();
                                  Float32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float64 {

                              pub data: f64,

                          }

                          impl WrappedTypesupport for Float64 { 

            type CStruct = std_msgs__msg__Float64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float64() }
            }

            fn create_msg() -> *mut std_msgs__msg__Float64 {

                unsafe { std_msgs__msg__Float64__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Float64) -> () {

                unsafe { std_msgs__msg__Float64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float64 {
  Float64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Float64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float64>::new();
                                  Float64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float64MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<f64>,

                          }

                          impl WrappedTypesupport for Float64MultiArray { 

            type CStruct = std_msgs__msg__Float64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float64MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Float64MultiArray {

                unsafe { std_msgs__msg__Float64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Float64MultiArray) -> () {

                unsafe { std_msgs__msg__Float64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float64MultiArray {
  Float64MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Float64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float64MultiArray>::new();
                                  Float64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Header {

                              pub stamp: builtin_interfaces::msg::Time,
pub frame_id: std::string::String,

                          }

                          impl WrappedTypesupport for Header { 

            type CStruct = std_msgs__msg__Header; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Header() }
            }

            fn create_msg() -> *mut std_msgs__msg__Header {

                unsafe { std_msgs__msg__Header__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Header) -> () {

                unsafe { std_msgs__msg__Header__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Header {
  Header {
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
frame_id: msg.frame_id.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.stamp.copy_to_native(&mut msg.stamp);
msg.frame_id.assign(&self.frame_id);
}



        }


                          
                          impl Default for Header {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Header>::new();
                                  Header::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int16 {

                              pub data: i16,

                          }

                          impl WrappedTypesupport for Int16 { 

            type CStruct = std_msgs__msg__Int16; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int16() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int16 {

                unsafe { std_msgs__msg__Int16__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int16) -> () {

                unsafe { std_msgs__msg__Int16__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int16 {
  Int16 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int16 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int16>::new();
                                  Int16::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int16MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<i16>,

                          }

                          impl WrappedTypesupport for Int16MultiArray { 

            type CStruct = std_msgs__msg__Int16MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int16MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int16MultiArray {

                unsafe { std_msgs__msg__Int16MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int16MultiArray) -> () {

                unsafe { std_msgs__msg__Int16MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int16MultiArray {
  Int16MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int16MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int16MultiArray>::new();
                                  Int16MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int32 {

                              pub data: i32,

                          }

                          impl WrappedTypesupport for Int32 { 

            type CStruct = std_msgs__msg__Int32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int32 {

                unsafe { std_msgs__msg__Int32__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int32) -> () {

                unsafe { std_msgs__msg__Int32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int32 {
  Int32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int32>::new();
                                  Int32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int32MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<i32>,

                          }

                          impl WrappedTypesupport for Int32MultiArray { 

            type CStruct = std_msgs__msg__Int32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int32MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int32MultiArray {

                unsafe { std_msgs__msg__Int32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int32MultiArray) -> () {

                unsafe { std_msgs__msg__Int32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int32MultiArray {
  Int32MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int32MultiArray>::new();
                                  Int32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int64 {

                              pub data: i64,

                          }

                          impl WrappedTypesupport for Int64 { 

            type CStruct = std_msgs__msg__Int64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int64() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int64 {

                unsafe { std_msgs__msg__Int64__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int64) -> () {

                unsafe { std_msgs__msg__Int64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int64 {
  Int64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int64>::new();
                                  Int64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int64MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<i64>,

                          }

                          impl WrappedTypesupport for Int64MultiArray { 

            type CStruct = std_msgs__msg__Int64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int64MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int64MultiArray {

                unsafe { std_msgs__msg__Int64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int64MultiArray) -> () {

                unsafe { std_msgs__msg__Int64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int64MultiArray {
  Int64MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int64MultiArray>::new();
                                  Int64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int8 {

                              pub data: i8,

                          }

                          impl WrappedTypesupport for Int8 { 

            type CStruct = std_msgs__msg__Int8; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int8() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int8 {

                unsafe { std_msgs__msg__Int8__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int8) -> () {

                unsafe { std_msgs__msg__Int8__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int8 {
  Int8 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int8 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int8>::new();
                                  Int8::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int8MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<i8>,

                          }

                          impl WrappedTypesupport for Int8MultiArray { 

            type CStruct = std_msgs__msg__Int8MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int8MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__Int8MultiArray {

                unsafe { std_msgs__msg__Int8MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__Int8MultiArray) -> () {

                unsafe { std_msgs__msg__Int8MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int8MultiArray {
  Int8MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int8MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int8MultiArray>::new();
                                  Int8MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiArrayDimension {

                              pub label: std::string::String,
pub size: u32,
pub stride: u32,

                          }

                          impl WrappedTypesupport for MultiArrayDimension { 

            type CStruct = std_msgs__msg__MultiArrayDimension; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__MultiArrayDimension() }
            }

            fn create_msg() -> *mut std_msgs__msg__MultiArrayDimension {

                unsafe { std_msgs__msg__MultiArrayDimension__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__MultiArrayDimension) -> () {

                unsafe { std_msgs__msg__MultiArrayDimension__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiArrayDimension {
  MultiArrayDimension {
label: msg.label.to_str().to_owned(),
size: msg.size,
stride: msg.stride,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.label.assign(&self.label);
msg.size = self.size;
msg.stride = self.stride;
}



        }


                          
                          impl Default for MultiArrayDimension {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiArrayDimension>::new();
                                  MultiArrayDimension::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiArrayLayout {

                              pub dim: Vec<std_msgs::msg::MultiArrayDimension>,
pub data_offset: u32,

                          }

                          impl WrappedTypesupport for MultiArrayLayout { 

            type CStruct = std_msgs__msg__MultiArrayLayout; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__MultiArrayLayout() }
            }

            fn create_msg() -> *mut std_msgs__msg__MultiArrayLayout {

                unsafe { std_msgs__msg__MultiArrayLayout__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__MultiArrayLayout) -> () {

                unsafe { std_msgs__msg__MultiArrayLayout__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiArrayLayout {
  MultiArrayLayout {
// is_upper_bound_: false
// member.array_size_ : 0
dim : {
let mut temp = Vec::with_capacity(msg.dim.size);
let slice = unsafe { std::slice::from_raw_parts(msg.dim.data, msg.dim.size)};
for s in slice { temp.push(std_msgs::msg::MultiArrayDimension::from_native(s)); }
temp },
data_offset: msg.data_offset,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { std_msgs__msg__MultiArrayDimension__Sequence__fini(&mut msg.dim) };
unsafe { std_msgs__msg__MultiArrayDimension__Sequence__init(&mut msg.dim, self.dim.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.dim.data, msg.dim.size)};
for (t,s) in slice.iter_mut().zip(&self.dim) { s.copy_to_native(t);}
msg.data_offset = self.data_offset;
}



        }


                          
                          impl Default for MultiArrayLayout {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiArrayLayout>::new();
                                  MultiArrayLayout::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct String {

                              pub data: std::string::String,

                          }

                          impl WrappedTypesupport for String { 

            type CStruct = std_msgs__msg__String; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__String() }
            }

            fn create_msg() -> *mut std_msgs__msg__String {

                unsafe { std_msgs__msg__String__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__String) -> () {

                unsafe { std_msgs__msg__String__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> String {
  String {
data: msg.data.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data.assign(&self.data);
}



        }


                          
                          impl Default for String {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<String>::new();
                                  String::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt16 {

                              pub data: u16,

                          }

                          impl WrappedTypesupport for UInt16 { 

            type CStruct = std_msgs__msg__UInt16; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt16() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt16 {

                unsafe { std_msgs__msg__UInt16__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt16) -> () {

                unsafe { std_msgs__msg__UInt16__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt16 {
  UInt16 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt16 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt16>::new();
                                  UInt16::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt16MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<u16>,

                          }

                          impl WrappedTypesupport for UInt16MultiArray { 

            type CStruct = std_msgs__msg__UInt16MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt16MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt16MultiArray {

                unsafe { std_msgs__msg__UInt16MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt16MultiArray) -> () {

                unsafe { std_msgs__msg__UInt16MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt16MultiArray {
  UInt16MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt16MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt16MultiArray>::new();
                                  UInt16MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt32 {

                              pub data: u32,

                          }

                          impl WrappedTypesupport for UInt32 { 

            type CStruct = std_msgs__msg__UInt32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt32() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt32 {

                unsafe { std_msgs__msg__UInt32__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt32) -> () {

                unsafe { std_msgs__msg__UInt32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt32 {
  UInt32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt32>::new();
                                  UInt32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt32MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<u32>,

                          }

                          impl WrappedTypesupport for UInt32MultiArray { 

            type CStruct = std_msgs__msg__UInt32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt32MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt32MultiArray {

                unsafe { std_msgs__msg__UInt32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt32MultiArray) -> () {

                unsafe { std_msgs__msg__UInt32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt32MultiArray {
  UInt32MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt32MultiArray>::new();
                                  UInt32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt64 {

                              pub data: u64,

                          }

                          impl WrappedTypesupport for UInt64 { 

            type CStruct = std_msgs__msg__UInt64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt64() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt64 {

                unsafe { std_msgs__msg__UInt64__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt64) -> () {

                unsafe { std_msgs__msg__UInt64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt64 {
  UInt64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt64>::new();
                                  UInt64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt64MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<u64>,

                          }

                          impl WrappedTypesupport for UInt64MultiArray { 

            type CStruct = std_msgs__msg__UInt64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt64MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt64MultiArray {

                unsafe { std_msgs__msg__UInt64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt64MultiArray) -> () {

                unsafe { std_msgs__msg__UInt64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt64MultiArray {
  UInt64MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt64MultiArray>::new();
                                  UInt64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt8 {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for UInt8 { 

            type CStruct = std_msgs__msg__UInt8; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt8() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt8 {

                unsafe { std_msgs__msg__UInt8__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt8) -> () {

                unsafe { std_msgs__msg__UInt8__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt8 {
  UInt8 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt8 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt8>::new();
                                  UInt8::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt8MultiArray {

                              pub layout: std_msgs::msg::MultiArrayLayout,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for UInt8MultiArray { 

            type CStruct = std_msgs__msg__UInt8MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt8MultiArray() }
            }

            fn create_msg() -> *mut std_msgs__msg__UInt8MultiArray {

                unsafe { std_msgs__msg__UInt8MultiArray__create() }

            }

            fn destroy_msg(msg: *mut std_msgs__msg__UInt8MultiArray) -> () {

                unsafe { std_msgs__msg__UInt8MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt8MultiArray {
  UInt8MultiArray {
layout: std_msgs::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt8MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt8MultiArray>::new();
                                  UInt8MultiArray::from_native(&msg_native)
                              }
                          }
             


                      }
