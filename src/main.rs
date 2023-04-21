
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

use lopdf::{Document};
use std::env;



fn prcextract<P: AsRef<Path> + Debug>(path: P) {
    let mut doc = Document::load(path).map_err(|e| Error::new(ErrorKind::Other, e.to_string())).unwrap();


    doc.decompress();

    let n_extracted = std::cell::RefCell::new(0);
    let last_3d = std::cell::RefCell::new(0);
    doc.traverse_objects(|obj| {
        // println!("object!, {}", obj.type_name().unwrap_or("default"));
        let name: &str = obj.type_name().unwrap_or("default");
        if name == "3D" {
            let stream = obj.as_stream().unwrap();
            let data = stream.content.clone();
            let mut file = File::create(format!("extracted3d-{}.prc", n_extracted.borrow())).unwrap();
            file.write_all(data.as_slice()).unwrap();
            file.write("\n".as_bytes()).unwrap();
            let next = n_extracted.take()+1;
            last_3d.replace(next-1);
            n_extracted.replace(next);
        }
        if name == "3DView" {
            let view = obj.as_dict().unwrap();
            let mut file = File::create(format!("extracted3d-{}.vws", last_3d.borrow())).unwrap();
            let mut content = "VIEW\n".to_owned();
            for (v, obj) in view.iter() {
                // println!("aaa");
                let key = format!("{}", String::from_utf8(v.to_owned()).unwrap());
                if key == "CO" { // Center of orbit
                    content.push_str(format!("\tROO={}\n", obj.as_f32().unwrap()).as_str());
                }
                // std::io::stdout().write_all(v.as_slice());
                // std::io::stdout().write("\n".as_bytes());
                // let name: &str = o.type_n   ame().unwrap_or("default");
                // println!("object!, {}", name);
                if key == "C2W" {

                    let obj = obj.as_array().unwrap();
                    content.push_str("\tC2W=");

                    for (idx, val) in obj.iter().enumerate() {
                        let val = val.as_f32().unwrap();
                        content.push_str(format!("{}", val).as_str());
                        if idx != obj.len() -1 {
                            content.push_str(" ");
                        }
                    }
                    content.push_str("\n");
                }
            }
            content.push_str("END\n");
            file.write_all(content.as_bytes()).unwrap();

            // let stream = obj.as_stream().unwrap();
            // let data = stream.content.clone();
            // let mut file = File::create(format!("extracted3d-{}.prc", last_3d.borrow())).unwrap();
            // file.write_all(data.as_slice()).unwrap();
            // file.write("\n".as_bytes()).unwrap();
        }

    });
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let pdf_path = PathBuf::from(shellexpand::full(file_path).unwrap().to_string());
    prcextract(&pdf_path);
    Ok(())
}
