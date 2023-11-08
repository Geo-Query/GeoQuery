use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::path::PathBuf;
use crate::err::TIFFErrorState;
use crate::err::TIFFErrorState::UnexpectedFormat;
use crate::tag::{EntryValue, IFDEntry};

mod ifd;
mod util;
mod err;
mod tag;
mod header;
mod geokeydirectory;

pub trait FileDescriptor {
    fn get_path(&self) -> &PathBuf;
}

pub struct Region {
    bottom_left: (f64, f64),
    top_right: (f64, f64)
}

pub fn parse_tiff(descriptor: Box<dyn FileDescriptor>) -> Result<Region, TIFFErrorState> {
    match File::open(&descriptor.get_path()) {
        Ok(file_handle) => {
            let mut reader = BufReader::new(file_handle);
            let (byte_order, offset) = match header::parse_header(&mut reader) {
                Ok((byte_order, offset)) => (byte_order, offset),
                Err(e) => {
                    eprintln!("Encountered Error! {:?} Invalid Header!", e);
                    panic!();
                }
            };
            println!("Byte Order: {:?}, Offset: {}", byte_order, offset);
            let (mut entries, next_ifd) = match ifd::parse_ifd(&mut reader, SeekFrom::Start(offset as u64), &byte_order) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Encountered Error! {:?} Invalid IFD!", e);
                    panic!();
                }
            };
            println!("Entries: {:?}", entries);
            let mut tag_lookup: HashMap<u16, IFDEntry> = entries.into_iter().map(|entry| (entry.tag, entry)).collect();












            println!("NEXTIFD: {:?}", next_ifd);
            println!("ImageWidth: {:?}", tag_lookup.get_mut(&256u16).expect("No ImageWidth Entry!").resolve(&byte_order, &mut reader));
            println!("ImageLength: {:?}", tag_lookup.get_mut(&257u16).expect("No ImageLength Entry!").resolve(&byte_order, &mut reader));
            println!("ModelTiePoint: {:?}", tag_lookup.get_mut(&33922u16).expect("No ModelTiePoint Entry!").resolve(&byte_order, &mut reader));
            println!("ModelPixelScale: {:?}", tag_lookup.get_mut(&33550u16).expect("No ModelPixelScale!").resolve(&byte_order, &mut reader));
            println!("GEOTAGS\n---------------");
            println!("GeoKeyDirectory: {:?}", tag_lookup.get_mut(&34735u16).expect("No GeoKeyDirectory!").resolve(&byte_order, &mut reader));
            // println!("GeoDoubleParams: {:?}", tag_lookup.get_mut(&34736u16).expect("No GeoDoubleParams!").resolve(&byte_order, &mut reader));
            println!("GeoAsciiParams: {:?}", tag_lookup.get_mut(&34737u16).expect("No GeoAsciiParams!").resolve(&byte_order, &mut reader));

            let geo_ascii = match tag_lookup.get_mut(&34737u16) {
                Some(ascii) => match ascii.resolve(&byte_order, &mut reader) {
                    Ok(ascii) => match ascii {
                        EntryValue::ASCII(values) => match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
                            }
                        },
                        _ => {
                            return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
                        }
                    },
                    Err(_) => {
                        return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
                    }
                },
                None => {
                    println!("FOUND NO GEODATA");
                    return Err(UnexpectedFormat(String::from("No GeoAscii Params")));
                }
            };
            let top_left = match tag_lookup.get_mut(&33922u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::DOUBLE(values) => (match values.get(3) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
                            }
                        }, match values.get(4) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
                            }
                        }),
                        _ => {
                            return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
                        }
                    }
                    Err(_) => {
                        return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
                    }
                },
                None => {
                    return Err(UnexpectedFormat(String::from("No ModelTiePoint")));
                }
            };
            let scale = match tag_lookup.get_mut(&33550u16) {
                Some(t) => match t.resolve(&byte_order, &mut reader) {
                    Ok(v) => match v {
                        EntryValue::DOUBLE(values) => (match values.get(0) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(UnexpectedFormat(String::from("No Scale")));
                            }
                        }, match values.get(1) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(UnexpectedFormat(String::from("No Scale")));
                            }
                        }),
                        _ => {
                            return Err(UnexpectedFormat(String::from("No Scale")));
                        }
                    }
                    Err(_) => {
                        return Err(UnexpectedFormat(String::from("No Scale")));
                    }
                },
                None => {
                    return Err(UnexpectedFormat(String::from("No Scale")));
                }
            };
            // let image_dimensions = (match tag_lookup.get_mut(&256u16) {
            //     Some(t) => match t.resolve(&byte_order, &mut reader) {
            //         Ok(v) => match v {
            //             EntryValue::SHORT(values) => match values.get(0) {
            //                 Some(v) => v.clone(),
            //                 None => {
            //                     return Err(InvalidOrUnhandledFormat(descriptor));
            //                 }
            //             },
            //             _ => {
            //                 return Err(InvalidOrUnhandledFormat(descriptor));
            //             }
            //         }
            //         Err(_) => {
            //             return Err(InvalidOrUnhandledFormat(descriptor));
            //         }
            //     },
            //     None => {
            //         return Err(NoGeoData(descriptor));
            //     }
            // }, match tag_lookup.get_mut(&257u16) {
            //     Some(t) => match t.resolve(&byte_order, &mut reader) {
            //         Ok(v) => match v {
            //             EntryValue::SHORT(values) => match values.get(0) {
            //                 Some(v) => v.clone(),
            //                 None => {
            //                     return Err(InvalidOrUnhandledFormat(descriptor));
            //                 }
            //             },
            //             _ => {
            //                 return Err(InvalidOrUnhandledFormat(descriptor));
            //             }
            //         }
            //         Err(_) => {
            //             return Err(InvalidOrUnhandledFormat(descriptor));
            //         }
            //     },
            //     None => {
            //         return Err(NoGeoData(descriptor));
            //     }
            // });
            // let proj_build = match Proj::new_known_crs(&geo_ascii, "EPSG:4326", None) {
            //     Ok(v) => v,
            //     Err(e) => {
            //         eprintln!("FOO: {:?}", e);
            //         return Err(NoGeoData(descriptor));
            //     }
            // };
            //
            // eprintln!("FOO: {:?}", proj_build);
            //
            //
            //
            //
            //
            // println!("{}", geo_ascii);
            // if geo_ascii == "British National Grid (ORD SURV GB)|OSGB 1936|British National Grid (ORD SURV GB)|\0" {
            //     println!("FOUND BRITISH NATIONAL GRID FILE!");
            // } else if (geo_ascii == "WGS 84|\0") {
            //     let bottom_right = (
            //         (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
            //         (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
            //     );
            //
            //     let top_right = (
            //         (top_left.0 + (bottom_right.0 - top_left.0)),
            //         top_left.1
            //     );
            //
            //     let bottom_left = (
            //         (bottom_right.0 - (bottom_right.0 - top_left.0)),
            //         bottom_right.1
            //     );
            // } else if (geo_ascii == "OSGB 1936 / British National Grid|OSGB 1936|\0") {
            //     println!("FOUND OSGB 1936");
            // } else {
            //     eprintln!("NO COORD SYSTEM GIVEN");
            //     panic!();
            // }
            //
            //
            //
            //
            //
            //
            //
            //
            // let bottom_right = (
            //     (top_left.0 + (image_dimensions.0.clone() as f64) * scale.0),
            //     (top_left.1 - (image_dimensions.1.clone() as f64) * scale.1)
            // );
            //
            // let top_right = (
            //     (top_left.0 + (bottom_right.0 - top_left.0)),
            //     top_left.1
            // );
            //
            // let bottom_left = (
            //     (bottom_right.0 - (bottom_right.0 - top_left.0)),
            //     bottom_right.1
            // );
            //
            // println!("Got bounding box: {:?} :: {:?}", bottom_left, top_right);
            //
            //
            // println!("Next IFD: {:?}", next_ifd);

            return Ok(Region{
                bottom_left: (0.0,0.0),
                top_right: (0.0,0.0)
            })
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(TIFFErrorState::FailedToParseTag)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
