extern crate serde;
extern crate serde_json;
extern crate serde_protobuf;

use serde_protobuf::descriptor::{Descriptors, FieldLabel, FieldType};
use serde_protobuf::value::{Field, Value};
use std::fs;
use std::io::prelude::*;

use protobuf::Message;
//use protobuf::reflect::value::ProtobufValue;
use protobuf::reflect::MessageDescriptor;

fn msg_from_json(
    descriptor: Option<&serde_protobuf::descriptor::MessageDescriptor>,
    descriptors: &Descriptors,
    obj: &serde_json::Map<String, serde_json::Value>,
) -> Result<Value, ()> {
    let opt = obj.get("@type");
    if opt.is_none() {
        return Err(());
    }
    let m_type: Option<String> = match opt {
        Some(t) => Some(t.as_str().unwrap().to_string()),
        _ => None,
    };

    assert!(m_type.is_some() || descriptor.is_some());
    let desc = match m_type {
        Some(ref t) => descriptors.message_by_name(&t).unwrap(),
        None => descriptor.unwrap(),
    };
    let mut ret = serde_protobuf::value::Message::new(desc);
    for field in desc.fields() {
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
                                    msg_from_json(
                                        Some(child_desc),
                                        descriptors,
                                        x.as_object().unwrap(),
                                    )
                                    .unwrap()
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
                    FieldType::String => {
                        Field::Singular(Some(Value::String(jv.as_str().unwrap().to_string())))
                    }
                    FieldType::Message(child_desc) => Field::Singular(Some(msg_from_json(
                        Some(child_desc),
                        descriptors,
                        jv.as_object().unwrap(),
                    )?)),
                    _ => panic!(),
                },
            };
            ret.fields.insert(field.number(), v);
        }
    }

    if let Some(t) = m_type {
        if let Some(true_desc) = descriptor {
            if true_desc.name() == ".google.protobuf.Any" {
                let mut ret_any = serde_protobuf::value::Message::new(true_desc);
                ret_any.fields.insert(
                    1,
                    Field::Singular(Some(Value::String(format!(
                        "type.googleapis.com/{}",
                        t.strip_prefix(".").unwrap_or(&t)
                    )))),
                );
                ret_any.fields.insert(
                    2,
                    Field::Singular(Some(Value::Bytes(ret.write_to_bytes().unwrap()))),
                );
                return Ok(Value::Message(ret_any));
            }
        }
    }

    Ok(Value::Message(ret))
}

fn main() {
    let mut descriptors = Descriptors::new();
    let mut f = fs::File::open("tools/port_msg.pb").unwrap();
    let proto = protobuf::parse_from_reader(&mut f).unwrap();
    descriptors.add_file_set_proto(&proto);
    let mut f = fs::File::open("tools/bess_msg.pb").unwrap();
    let proto = protobuf::parse_from_reader(&mut f).unwrap();
    descriptors.add_file_set_proto(&proto);
    descriptors.resolve_refs();

    let mut f = fs::File::open("tools/blah2.json").unwrap();
    let mut jstr = String::new();
    f.read_to_string(&mut jstr).unwrap();
    let mut m: serde_json::Value = serde_json::from_str(&jstr).unwrap();
    let msg = match msg_from_json(None, &descriptors, m.as_object().unwrap()) {
        Ok(Value::Message(m)) => m,
        _ => panic!(),
    };

    let mut p = libbess::port_msg::VPortArg::new();
    p.set_ifname("eth_bob".to_string());
    p.set_rxq_cpus(vec![0, 1]);
    p.set_ip_addrs(protobuf::RepeatedField::from_vec(vec![
        "10.255.99.1/24".to_string(),
        "fdd4:955b:82c1:0cb7::2/64".to_string(),
    ]));
    p.set_tx_tci(0);
    p.set_tx_outer_tci(0);
    p.set_loopback(false);

    let v = msg.write_to_bytes().unwrap();
    let pv = p.write_to_bytes().unwrap();
    println!("{:?}", v);
    println!("{:?}", pv);
    assert!(pv == v);

    println!("---");
    let mut f2 = fs::File::open("tools/blah3.json").unwrap();
    let mut jstr2 = String::new();
    f2.read_to_string(&mut jstr2).unwrap();
    let mut m2: serde_json::Value = serde_json::from_str(&jstr2).unwrap();
    let req = match msg_from_json(None, &descriptors, m2.as_object().unwrap()) {
        Ok(Value::Message(x)) => x,
        _ => panic!(),
    };
    let rv = req.write_to_bytes().unwrap();

    let mut cargs = libbess::bess_msg::CreatePortRequest::new();
    cargs.set_arg(protobuf::well_known_types::Any::pack(&p).unwrap());
    let cv = cargs.write_to_bytes().unwrap();
    println!("{:?}", rv);
    println!("{:?}", cv);
    assert!(rv == cv);
}
