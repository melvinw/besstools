use protobuf::stream::CodedInputStream;
use protobuf::well_known_types::Any;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_protobuf::value::Message;

pub fn make_any(type_url: &str, pkg: &str, msg: &Message) -> Result<Any, ()> {
    let mut ret = Any::new();
    let buf = msg.write_to_bytes();
    if buf.is_err() {
        return Err(());
    }
    ret.set_type_url(
        format!(
            "type.googleapis.com/{}.{}",
            pkg,
            type_url.to_string().strip_prefix(".").unwrap_or(&type_url)
        )
        .to_string(),
    );
    ret.set_value(buf.unwrap());
    Ok(ret)
}

pub fn unpack_any<'a>(
    msg: &'a Any,
    descriptors: &'a Descriptors,
) -> Result<(&'a MessageDescriptor, Message), ()> {
    let parts: Vec<&str> = msg.get_type_url().split(".").collect();
    // TODO(melvin): figure out what the deal is with these descriptor paths. All this path munging is annoying.
    let mtype = format!(".{}", parts[parts.len() - 1]);
    let desc = descriptors.message_by_name(&mtype).unwrap();
    let mut stream = CodedInputStream::from_bytes(msg.get_value());
    let mut ret = Message::new(desc);
    ret.merge_from(descriptors, desc, &mut stream).unwrap();
    Ok((desc, ret))
}
