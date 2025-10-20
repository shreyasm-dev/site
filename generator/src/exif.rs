use crate::util::{Exif, Metadata, Output};
use little_exif::{exif_tag::ExifTag, filetype::FileExtension};

pub fn exif(input: &[u8], filename: &str) -> Output {
  let metadata =
    little_exif::metadata::Metadata::new_from_vec(&input.to_vec(), FileExtension::JPEG).unwrap();

  Output {
    metadata: Metadata {
      title: None,
      tags: Vec::new(),
      exif: Some(Exif {
        make: metadata
          .get_tag(&ExifTag::Make("".to_string()))
          .collect::<Vec<_>>()
          .get(0)
          .map(|x| match x {
            ExifTag::Make(s) => s.clone(),
            _ => unreachable!(),
          }),
        model: metadata
          .get_tag(&ExifTag::Model("".to_string()))
          .collect::<Vec<_>>()
          .get(0)
          .map(|x| match x {
            ExifTag::Model(s) => s.clone(),
            _ => unreachable!(),
          }),
        lens: metadata
          .get_tag(&ExifTag::LensModel("".to_string()))
          .collect::<Vec<_>>()
          .get(0)
          .map(|x| match x {
            ExifTag::LensModel(s) => s.clone(),
            _ => unreachable!(),
          }),
        aperture: metadata
          .get_tag(&ExifTag::ApertureValue(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::ApertureValue(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
            _ => unreachable!(),
          }),
        f: metadata
          .get_tag(&ExifTag::FNumber(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::FNumber(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
            _ => unreachable!(),
          }),
        iso: metadata
          .get_tag(&ExifTag::ISO(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::ISO(s) => s.get(0).copied(),
            _ => unreachable!(),
          }),
        iso_speed: metadata
          .get_tag(&ExifTag::ISOSpeed(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::ISOSpeed(s) => s.get(0).copied(),
            _ => unreachable!(),
          }),
        exposure_time: metadata
          .get_tag(&ExifTag::ExposureTime(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::ExposureTime(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
            _ => unreachable!(),
          }),
        focal_length: metadata
          .get_tag(&ExifTag::FocalLength(Vec::new()))
          .collect::<Vec<_>>()
          .get(0)
          .and_then(|x| match x {
            ExifTag::FocalLength(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
            _ => unreachable!(),
          }),
      }),
    },
    content: input.to_vec(),
    filename: filename.to_string(),
  }
}
