use failure::format_err;
use failure::Error;
use protobuf::stream::CodedInputStream;
use protobuf::well_known_types::Any;
use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};
use serde_protobuf::value::Message;

pub fn make_any(type_url: &str, pkg: &str, msg: &Message) -> Result<Any, Error> {
    let mut ret = Any::new();
    let buf = msg.write_to_bytes();
    ret.set_type_url(
        format!(
            "type.googleapis.com/{}.{}",
            pkg,
            type_url.to_string().strip_prefix(".").unwrap_or(&type_url)
        )
        .to_string(),
    );
    ret.set_value(buf?);
    Ok(ret)
}

pub fn unpack_any<'a>(
    msg: &'a Any,
    descriptors: &'a Descriptors,
) -> Result<(&'a MessageDescriptor, Message), Error> {
    let parts: Vec<&str> = msg.get_type_url().split(".").collect();
    // TODO(melvin): figure out what the deal is with these descriptor paths. All this path munging is annoying.
    let mtype = format!(".{}", parts[parts.len() - 1]);
    if let Some(desc) = descriptors.message_by_name(&mtype) {
        let mut stream = CodedInputStream::from_bytes(msg.get_value());
        let mut ret = Message::new(desc);
        ret.merge_from(descriptors, desc, &mut stream)?;
        return Ok((desc, ret));
    } else {
        return Err(format_err!("could not find descriptor for {}", mtype));
    }
}
