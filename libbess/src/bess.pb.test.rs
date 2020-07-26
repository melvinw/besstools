#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnnestedDictMsg {
    #[prost(map = "int32, int32", tag = "1")]
    pub dict: ::std::collections::HashMap<i32, i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NestedDictMsg {
    #[prost(message, optional, tag = "1")]
    pub a: ::std::option::Option<UnnestedDictMsg>,
    #[prost(message, optional, tag = "2")]
    pub b: ::std::option::Option<UnnestedDictMsg>,
    #[prost(message, optional, tag = "3")]
    pub c: ::std::option::Option<UnnestedDictMsg>,
}
