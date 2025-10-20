#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
  pub title: Option<String>,
  pub tags: Vec<String>,
}

impl Default for Metadata {
  fn default() -> Self {
    Self {
      title: None,
      tags: Vec::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exif {
  pub make: Option<String>,
  pub model: Option<String>,
  pub lens: Option<String>,
  pub aperture: Option<(u32, u32)>,
  pub f: Option<(u32, u32)>,
  pub iso: Option<u16>,
  pub iso_speed: Option<u32>,
  pub exposure_time: Option<(u32, u32)>,
  pub focal_length: Option<(u32, u32)>,
}

impl ToString for Exif {
  fn to_string(&self) -> String {
    vec![
      vec![
        (&"Camera make", self.make.clone()),
        (&"Camera model", self.model.clone()),
        (&"Lens", self.lens.clone()),
      ],
      vec![
        ("Aperture", self.aperture.clone()),
        ("F-stop", self.f.clone()),
        ("Exposure time", self.exposure_time.clone()),
        ("Focal length", self.focal_length.clone()),
      ]
      .iter()
      .map(|(label, data)| (label, data.map(|(n, d)| format!("{}/{}", n, d))))
      .collect(),
      vec![
        ("ISO", self.iso.clone().map(|n| n as u32)),
        ("ISO speed", self.iso_speed.clone()),
      ]
      .iter()
      .map(|(label, data)| (label, data.map(|n| format!("{}", n))))
      .collect(),
    ]
    .concat()
    .iter()
    .map(|(label, data)| {
      format!(
        "**{}:** {}",
        label,
        data.clone().unwrap_or("None".to_string())
      )
    })
    .collect::<Vec<_>>()
    .join("\n\n")
  }
}
