use protobuf::well_known_types::Any;
use serde_protobuf::value::Message;

pub fn make_any(type_url: &str, msg: &Message) -> Result<Any, ()> {
    let mut ret = Any::new();
    let buf = msg.write_to_bytes();
    if buf.is_err() {
        return Err(());
    }
    ret.set_type_url(type_url.to_string());
    ret.set_value(buf.unwrap());
    Ok(ret)
}
