use crate::{markdown::markdown, util::Output};
use components::types::Exif;
use little_exif::{exif_tag::ExifTag, filetype::FileExtension};
use std::path::PathBuf;

pub fn image(input: &[u8], path: &str) -> Vec<Output> {
  let exif =
    little_exif::metadata::Metadata::new_from_vec(&input.to_vec(), FileExtension::JPEG).unwrap();
  let exif = Exif {
    make: exif
      .get_tag(&ExifTag::Make("".to_string()))
      .collect::<Vec<_>>()
      .get(0)
      .map(|x| match x {
        ExifTag::Make(s) => s.clone(),
        _ => unreachable!(),
      }),
    model: exif
      .get_tag(&ExifTag::Model("".to_string()))
      .collect::<Vec<_>>()
      .get(0)
      .map(|x| match x {
        ExifTag::Model(s) => s.clone(),
        _ => unreachable!(),
      }),
    lens: exif
      .get_tag(&ExifTag::LensModel("".to_string()))
      .collect::<Vec<_>>()
      .get(0)
      .map(|x| match x {
        ExifTag::LensModel(s) => s.clone(),
        _ => unreachable!(),
      }),
    aperture: exif
      .get_tag(&ExifTag::ApertureValue(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::ApertureValue(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
        _ => unreachable!(),
      }),
    f: exif
      .get_tag(&ExifTag::FNumber(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::FNumber(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
        _ => unreachable!(),
      }),
    iso: exif
      .get_tag(&ExifTag::ISO(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::ISO(s) => s.get(0).copied(),
        _ => unreachable!(),
      }),
    iso_speed: exif
      .get_tag(&ExifTag::ISOSpeed(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::ISOSpeed(s) => s.get(0).copied(),
        _ => unreachable!(),
      }),
    exposure_time: exif
      .get_tag(&ExifTag::ExposureTime(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::ExposureTime(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
        _ => unreachable!(),
      }),
    focal_length: exif
      .get_tag(&ExifTag::FocalLength(Vec::new()))
      .collect::<Vec<_>>()
      .get(0)
      .and_then(|x| match x {
        ExifTag::FocalLength(s) => s.get(0).map(|n| (n.nominator, n.denominator)),
        _ => unreachable!(),
      }),
  };

  let title = titlecase::titlecase(
    &PathBuf::from(path)
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap()
      .replace("-", " "),
  );

  vec![
    vec![Output {
      content: input.to_vec(),
      path: path.into(),
    }],
    markdown(
      &format!(
        "---
title=\"{0}\"
---

# {0}

<img alt='no alt description available' src='{1}' style='width: 100%;'>\n\n{2}",
        title,
        PathBuf::from(path).file_name().unwrap().to_str().unwrap(),
        exif.to_string(),
      )
      .into_bytes(),
      PathBuf::from(path).with_extension("md").to_str().unwrap(),
    ),
  ]
  .concat()
}
