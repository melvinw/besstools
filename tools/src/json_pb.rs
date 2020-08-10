use failure::format_err;
use failure::Error;
use serde_json::json;
use serde_json::Value as jValue;
use serde_protobuf::descriptor::{Descriptors, FieldLabel, FieldType, MessageDescriptor};
use serde_protobuf::value::{Field, Message, Value};

fn jsonval2pb(v: &jValue, f: FieldType, descriptors: &Descriptors) -> Result<Value, Error> {
    match f {
        FieldType::Int32 => match v.as_i64() {
            Some(x) => Ok(Value::I32(x as i32)),
            _ => Err(format_err!("cannot convert non-int type to i32")),
        },
        FieldType::Int64 => match v.as_i64() {
            Some(x) => Ok(Value::I64(x)),
            _ => Err(format_err!("cannot convert non-int type to i64")),
        },
        FieldType::UInt32 => match v.as_u64() {
            Some(x) => Ok(Value::U32(x as u32)),
            _ => Err(format_err!("cannot convert non-uint type to u32")),
        },
        FieldType::UInt64 => match v.as_u64() {
            Some(x) => Ok(Value::U64(x)),
            _ => Err(format_err!("cannot convert non-uint type to u64")),
        },
        FieldType::Float => match v.as_f64() {
            Some(x) => Ok(Value::F32(x as f32)),
            _ => Err(format_err!("cannot convert non-float type to f32")),
        },
        FieldType::Double => match v.as_f64() {
            Some(x) => Ok(Value::F64(x)),
            _ => Err(format_err!("cannot convert non-double type to f64")),
        },
        FieldType::Bool => match v.as_bool() {
            Some(x) => Ok(Value::Bool(x)),
            _ => Err(format_err!("cannot convert non-bool type to bool")),
        },
        FieldType::String => match v.as_str() {
            Some(x) => Ok(Value::String(x.to_string())),
            _ => Err(format_err!("cannot convert non-string type to string")),
        },
        FieldType::Message(child_desc) => match v.as_object() {
            Some(x) => match msg_from_json(child_desc, descriptors, x) {
                Ok(msg) => Ok(Value::Message(msg)),
                Err(e) => Err(e),
            },
            _ => Err(format_err!("cannot convert non-object type to message")),
        },
        _ => Err(format_err!("how did i get here?")),
    }
}

fn msg_from_json(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<Message, Error> {
    let mut ret = Message::new(descriptor);
    for field in descriptor.fields() {
        if !obj.contains_key(field.name()) {
            continue;
        }
        if !obj.contains_key(field.name()) {
            continue;
        }
        let jv = obj.get(field.name()).unwrap();
        if jv.is_null() {
            ret.fields.insert(field.number(), Field::Singular(None));
            continue;
        }

        let v = match field.field_label() {
            FieldLabel::Repeated => match jv.as_array() {
                Some(varr) => {
                    let mut r = Vec::<Value>::new();
                    for x in varr.iter() {
                        r.push(jsonval2pb(x, field.field_type(descriptors), descriptors)?);
                    }
                    Field::Repeated(r)
                }
                _ => {
                    return Err(format_err!(
                        "cannot convert non-array type to repeated field"
                    ));
                }
            },
            _ => Field::Singular(Some(jsonval2pb(
                jv,
                field.field_type(descriptors),
                descriptors,
            )?)),
        };
        ret.fields.insert(field.number(), v);
    }

    if let Some(t) = obj.get("@type") {
        if !t.is_string() {
            return Err(format_err!("type URL must be a string"));
        }
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
                Field::Singular(Some(Value::Bytes(ret.write_to_bytes()?))),
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
) -> Result<Message, Error> {
    let m: serde_json::Value = serde_json::from_str(&jstr)?;
    match m.as_object() {
        Some(obj) => msg_from_json(descriptor, descriptors, obj),
        _ => Err(format_err!("cannot convert non-object type into message")),
    }
}

fn pbval2json(
    v: &Value,
    descriptor: Option<&MessageDescriptor>,
    descriptors: &Descriptors,
) -> Result<jValue, Error> {
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
            _ => Err(format_err!("how did i get here?")),
        },
        _ => Err(format_err!("how did i get here?")),
    }
}

fn msg_to_json(
    descriptor: &MessageDescriptor,
    descriptors: &Descriptors,
    msg: &Message,
) -> Result<serde_json::Map<String, jValue>, Error> {
    let mut ret = serde_json::Map::new();
    for (fnum, field) in &msg.fields {
        let fdesc = descriptor.field_by_number(*fnum);
        if fdesc.is_none() {
            return Err(format_err!("no field with number {} exists", *fnum));
        }
        let child_desc: Option<&MessageDescriptor> = match fdesc.unwrap().field_type(&descriptors) {
            FieldType::Message(md) => Some(md),
            _ => None,
        };
        let v = match field {
            Field::Repeated(varr) => {
                let mut r = Vec::<jValue>::new();
                for x in varr.iter() {
                    r.push(pbval2json(x, child_desc, descriptors)?);
                }
                jValue::Array(r)
            }
            Field::Singular(Some(v)) => pbval2json(v, child_desc, descriptors)?,
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
) -> Result<String, Error> {
    match msg_to_json(descriptor, descriptors, msg) {
        Ok(obj) => Ok(serde_json::to_string(&obj)?),
        Err(e) => Err(e),
    }
}
