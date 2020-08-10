use serde_json::json;
use serde_json::Value as jValue;
use serde_protobuf::descriptor::{Descriptors, FieldLabel, FieldType, MessageDescriptor};
use serde_protobuf::value::{Field, Message, Value};

fn msg_from_json(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<Message, ()> {
    let mut ret = Message::new(descriptor);
    for field in descriptor.fields() {
        if !obj.contains_key(field.name()) {
            continue;
        }
        if let Some(jv) = obj.get(field.name()) {
            if jv.is_null() {
                ret.fields.insert(field.number(), Field::Singular(None));
                continue;
            }
            let v = match field.field_label() {
                FieldLabel::Repeated => {
                    let varr = jv.as_array().unwrap();
                    match field.field_type(&descriptors) {
                        FieldType::Int32 => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::I32(x.as_i64().unwrap() as i32))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::Int64 => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::I64(x.as_i64().unwrap()))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::UInt32 => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::U32(x.as_u64().unwrap() as u32))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::UInt64 => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::U64(x.as_u64().unwrap()))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::Float => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::F32(x.as_f64().unwrap() as f32))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::Double => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::F64(x.as_f64().unwrap()))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::Bool => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::Bool(x.as_bool().unwrap()))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::String => Field::Repeated(
                            varr.iter()
                                .map(|x| Value::String(x.as_str().unwrap().to_string()))
                                .collect::<Vec<Value>>(),
                        ),
                        FieldType::Message(child_desc) => Field::Repeated(
                            varr.iter()
                                .map(|x| {
                                    Value::Message(
                                        msg_from_json(
                                            child_desc,
                                            descriptors,
                                            x.as_object().unwrap(),
                                        )
                                        .unwrap(),
                                    )
                                })
                                .collect::<Vec<Value>>(),
                        ),
                        _ => panic!(),
                    }
                }
                _ => match field.field_type(&descriptors) {
                    FieldType::String => {
                        Field::Singular(Some(Value::String(jv.as_str().unwrap().to_string())))
                    }
                    FieldType::Int32 => {
                        Field::Singular(Some(Value::I32(jv.as_i64().unwrap() as i32)))
                    }
                    FieldType::Int64 => Field::Singular(Some(Value::I64(jv.as_i64().unwrap()))),
                    FieldType::UInt32 => {
                        Field::Singular(Some(Value::U32(jv.as_u64().unwrap() as u32)))
                    }
                    FieldType::UInt64 => Field::Singular(Some(Value::U64(jv.as_u64().unwrap()))),
                    FieldType::Float => {
                        Field::Singular(Some(Value::F32(jv.as_f64().unwrap() as f32)))
                    }
                    FieldType::Double => Field::Singular(Some(Value::F64(jv.as_f64().unwrap()))),
                    FieldType::Bool => Field::Singular(Some(Value::Bool(jv.as_bool().unwrap()))),
                    FieldType::Message(child_desc) => Field::Singular(Some(Value::Message(
                        msg_from_json(child_desc, descriptors, jv.as_object().unwrap())?,
                    ))),
                    _ => panic!(),
                },
            };
            ret.fields.insert(field.number(), v);
        }
    }

    if let Some(t) = obj.get("@type") {
        let t_str = t.as_str().unwrap();
        if descriptor.name() == ".google.protobuf.Any" {
            let mut ret_any = Message::new(descriptor);
            ret_any.fields.insert(
                1,
                Field::Singular(Some(Value::String(format!(
                    "type.googleapis.com/{}",
                    t_str.strip_prefix(".").unwrap_or(&t_str)
                )))),
            );
            ret_any.fields.insert(
                2,
                Field::Singular(Some(Value::Bytes(ret.write_to_bytes().unwrap()))),
            );
            return Ok(ret_any);
        }
    }

    Ok(ret)
}

pub fn from_str(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    jstr: &String,
) -> Result<Message, ()> {
    let m: serde_json::Value = serde_json::from_str(&jstr).unwrap();
    match m.as_object() {
        Some(obj) => msg_from_json(descriptor, descriptors, obj),
        _ => Err(()),
    }
}

fn pbval2json(
    v: &Value,
    descriptor: Option<&MessageDescriptor>,
    descriptors: &Descriptors,
) -> Result<jValue, ()> {
    match v {
        Value::I32(x) => Ok(json!(*x)),
        Value::I64(x) => Ok(json!(*x)),
        Value::U32(x) => Ok(json!(*x)),
        Value::U64(x) => Ok(json!(*x)),
        Value::F32(x) => Ok(json!(*x)),
        Value::F64(x) => Ok(json!(*x)),
        Value::Bool(b) => Ok(jValue::Bool(*b)),
        Value::String(s) => Ok(jValue::String(s.clone())),
        Value::Message(msg) => match msg_to_json(descriptor.expect("XXX"), descriptors, msg) {
            Ok(map) => Ok(jValue::Object(map)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn msg_to_json(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    msg: &Message,
) -> Result<serde_json::Map<String, jValue>, ()> {
    let mut ret = serde_json::Map::new();
    for (fnum, field) in &msg.fields {
        let fdesc = descriptor.field_by_number(*fnum).unwrap();
        let child_desc: Option<&MessageDescriptor> = match fdesc.field_type(&descriptors) {
            FieldType::Message(md) => Some(md),
            _ => None,
        };
        let v = match field {
            Field::Repeated(varr) => jValue::Array(
                varr.iter()
                    .map(|x| pbval2json(x, child_desc, descriptors).unwrap())
                    .collect::<Vec<jValue>>(),
            ),
            Field::Singular(Some(v)) => pbval2json(v, child_desc, descriptors).unwrap(),
            Field::Singular(None) => jValue::Null,
        };
        ret.insert(
            descriptor
                .field_by_number(*fnum)
                .unwrap()
                .name()
                .to_string(),
            v,
        );
    }
    Ok(ret)
}

pub fn to_str(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    msg: &Message,
) -> Result<String, ()> {
    match msg_to_json(descriptor, descriptors, msg) {
        Ok(obj) => Ok(serde_json::to_string(&obj).unwrap()),
        _ => Err(()),
    }
}
