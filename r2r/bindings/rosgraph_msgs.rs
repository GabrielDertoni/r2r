  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Clock {

                              pub clock: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for Clock { 

            type CStruct = rosgraph_msgs__msg__Clock; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Clock() }
            }

            fn create_msg() -> *mut rosgraph_msgs__msg__Clock {

                unsafe { rosgraph_msgs__msg__Clock__create() }

            }

            fn destroy_msg(msg: *mut rosgraph_msgs__msg__Clock) -> () {

                unsafe { rosgraph_msgs__msg__Clock__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Clock {
  Clock {
clock: builtin_interfaces::msg::Time::from_native(&msg.clock),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.clock.copy_to_native(&mut msg.clock);
}



        }


                          
                          impl Default for Clock {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Clock>::new();
                                  Clock::from_native(&msg_native)
                              }
                          }
             


                      }
