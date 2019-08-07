#[macro_export]
macro_rules! try_some {
    ($e:expr) => (
        match $e {
            Ok(x) => x,
            Err(err) => return Some(Err(err)),
        }
    )
}

//advance_to_element
//parse_struct_fields_update
#[macro_export]
macro_rules! parse_struct_update {
    ($rdr:expr,
     $buf:expr,
     $xml_element:expr,
     $data_struct:ident,
     {$($xml_field:expr => $data_struct_field:ident),* $(,)?},
     {$($xml_field_opt:expr => $data_struct_field_opt:ident),* $(,)?}
     ) => (
        match $rdr.read_event($buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"document-id" => {
                        loop {
                            match $rdr.read_event($buf) {
                                Ok(Event::Start(ref e)) => {
                                    match e.name() {
                                        $($xml_field => $data_struct.$data_struct_field = deser_text(e.name(), $rdr,)?,)+
                                        $($xml_field_opt => $data_struct.$data_struct_field_opt = Some(deser_text(e.name(), $rdr,)?),)+
                                        _ => return Err(Error::Deser { src: format!("unrecognized {} element", $xml_element) }),
                                    }
                                },
                                Ok(Event::End(ref e)) => {
                                    if e.name() == $xml_element.as_bytes() { break };
                                },
                                _ => break,
                            }
                        }
                    }
                    _ => return Err(Error::Deser { src: "found element besides doc-id".to_string() }),
                }
            },
            Ok(_) => return Err(Error::Deser { src: format!("found non-start-element besides {}", $xml_element) }),

            Err(err) => return Err(Error::Deser { src: err.to_string() }),
        }
    )
}
