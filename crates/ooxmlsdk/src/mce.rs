use crate::common::{SdkError, XmlNamespace};
use crate::sdk::{FileFormatVersion, MarkupCompatibilityProcessSettings};

#[derive(Clone, Debug)]
struct McePrefixList<'a> {
  value: std::borrow::Cow<'a, [u8]>,
}

impl<'a> McePrefixList<'a> {
  fn new(value: &'a [u8]) -> Result<Self, SdkError> {
    Ok(Self {
      value: decode_mce_attr_value(value)?,
    })
  }

  fn prefixes(&self) -> impl Iterator<Item = &[u8]> {
    split_xml_whitespace(self.value.as_ref())
  }
}

#[derive(Clone, Debug)]
struct MceQNameList<'a> {
  value: std::borrow::Cow<'a, [u8]>,
  patterns: Vec<MceQNamePattern<'a>>,
}

#[derive(Clone, Debug)]
enum MceQNamePattern<'a> {
  Any,
  Qualified {
    namespace: &'a [u8],
    local_name: MceQNameLocal,
  },
}

#[derive(Clone, Debug)]
enum MceQNameLocal {
  Any,
  Range(std::ops::Range<usize>),
}

impl<'a> MceQNameList<'a> {
  fn new(
    context: &'a MceContext<'a>,
    namespaces: &'a [XmlNamespace],
    value: &'a [u8],
  ) -> Result<Self, SdkError> {
    let value = decode_mce_attr_value(value)?;
    let mut patterns = Vec::new();
    for range in split_xml_whitespace_ranges(value.as_ref()) {
      let token = &value[range.clone()];
      if token == b"*" {
        patterns.push(MceQNamePattern::Any);
        continue;
      }
      let Some(separator) = token.iter().position(|byte| *byte == b':') else {
        continue;
      };
      let prefix = &token[..separator];
      let Some(namespace) = context.namespace_for_prefix_with_current_bytes(namespaces, prefix)
      else {
        continue;
      };
      let local_start = range.start + separator + 1;
      let local_name = if local_start == range.end {
        continue;
      } else if &value[local_start..range.end] == b"*" {
        MceQNameLocal::Any
      } else {
        MceQNameLocal::Range(local_start..range.end)
      };
      patterns.push(MceQNamePattern::Qualified {
        namespace,
        local_name,
      });
    }

    Ok(Self { value, patterns })
  }

  fn contains(&self, namespace: &[u8], local_name: &[u8]) -> bool {
    self.patterns.iter().any(|pattern| match pattern {
      MceQNamePattern::Any => true,
      MceQNamePattern::Qualified {
        namespace: candidate_namespace,
        local_name: candidate_local_name,
      } => {
        *candidate_namespace == namespace
          && match candidate_local_name {
            MceQNameLocal::Any => true,
            MceQNameLocal::Range(range) => self.value[range.clone()] == *local_name,
          }
      }
    })
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct MceContext<'a> {
  parent: Option<&'a MceContext<'a>>,
  namespaces: &'a [XmlNamespace],
  ignorable_namespaces: Vec<&'a [u8]>,
  preserve_attributes: Option<MceQNameList<'a>>,
  preserve_elements: Option<MceQNameList<'a>>,
  process_content: Option<MceQNameList<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ElementAction {
  Normal,
  Ignore,
  ProcessContent,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MceContextAttributes<'a> {
  pub(crate) ignorable: Option<&'a [u8]>,
  pub(crate) preserve_attributes: Option<&'a [u8]>,
  pub(crate) preserve_elements: Option<&'a [u8]>,
  pub(crate) process_content: Option<&'a [u8]>,
  pub(crate) must_understand: Option<&'a [u8]>,
}

impl<'a> MceContext<'a> {
  pub(crate) fn child_context(
    &'a self,
    namespaces: &'a [XmlNamespace],
    mce_attrs: MceContextAttributes<'a>,
    settings: &MarkupCompatibilityProcessSettings,
  ) -> Result<Self, SdkError> {
    let context = Self {
      parent: Some(self),
      namespaces,
      ignorable_namespaces: self.current_ignorable_namespaces(namespaces, mce_attrs.ignorable)?,
      preserve_attributes: mce_attrs
        .preserve_attributes
        .map(|value| MceQNameList::new(self, namespaces, value))
        .transpose()?,
      preserve_elements: mce_attrs
        .preserve_elements
        .map(|value| MceQNameList::new(self, namespaces, value))
        .transpose()?,
      process_content: mce_attrs
        .process_content
        .map(|value| MceQNameList::new(self, namespaces, value))
        .transpose()?,
    };

    context.validate_must_understand(mce_attrs.must_understand, settings)?;
    Ok(context)
  }

  fn validate_must_understand(
    &self,
    mc_must_understand: Option<&[u8]>,
    settings: &MarkupCompatibilityProcessSettings,
  ) -> Result<(), SdkError> {
    if let Some(value) = mc_must_understand {
      let prefixes = McePrefixList::new(value)?;
      for prefix in prefixes.prefixes() {
        let Some(ns) = self.namespace_for_prefix_bytes(prefix) else {
          let prefix = String::from_utf8_lossy(prefix);
          return Err(SdkError::CommonError(format!(
            "MCE MustUnderstand prefix `{prefix}` is not declared"
          )));
        };
        if !namespace_supported(ns, settings.target_file_format_version) {
          let namespace = String::from_utf8_lossy(ns);
          return Err(SdkError::CommonError(format!(
            "MCE MustUnderstand namespace `{namespace}` is not supported by target file format {:?}",
            settings.target_file_format_version
          )));
        }
      }
    }

    Ok(())
  }

  pub(crate) fn should_remove_attribute_qname_bytes(&self, qname: &[u8]) -> bool {
    let Some((namespace, local_name)) = self.qname_parts_with_current_bytes(&[], qname) else {
      return false;
    };
    self.is_ignorable_namespace_bytes(namespace)
      && !self.is_preserved_attribute_qname_bytes(namespace, local_name)
  }

  #[inline]
  pub(crate) fn element_action(
    &self,
    namespace: &[u8],
    local_name: &[u8],
    minimum_version: FileFormatVersion,
    target_version: FileFormatVersion,
  ) -> ElementAction {
    if file_format_rank(minimum_version) <= file_format_rank(target_version)
      || !self.is_ignorable_namespace_bytes(namespace)
      || self.is_preserved_element_qname_bytes(namespace, local_name)
    {
      ElementAction::Normal
    } else if self.is_process_content_qname_bytes(namespace, local_name) {
      ElementAction::ProcessContent
    } else {
      ElementAction::Ignore
    }
  }

  fn is_preserved_attribute_qname_bytes(&self, namespace: &[u8], local_name: &[u8]) -> bool {
    self
      .preserve_attributes
      .as_ref()
      .is_some_and(|list| list.contains(namespace, local_name))
      || self
        .parent
        .is_some_and(|parent| parent.is_preserved_attribute_qname_bytes(namespace, local_name))
  }

  fn is_preserved_element_qname_bytes(&self, namespace: &[u8], local_name: &[u8]) -> bool {
    self
      .preserve_elements
      .as_ref()
      .is_some_and(|list| list.contains(namespace, local_name))
      || self
        .parent
        .is_some_and(|parent| parent.is_preserved_element_qname_bytes(namespace, local_name))
  }

  fn is_process_content_qname_bytes(&self, namespace: &[u8], local_name: &[u8]) -> bool {
    self
      .process_content
      .as_ref()
      .is_some_and(|list| list.contains(namespace, local_name))
      || self
        .parent
        .is_some_and(|parent| parent.is_process_content_qname_bytes(namespace, local_name))
  }

  pub(crate) fn namespace_for_prefix_bytes(&self, prefix: &[u8]) -> Option<&[u8]> {
    self
      .namespace_for_prefix_in_frame_bytes(self.namespaces, prefix)
      .or_else(|| {
        self
          .parent
          .and_then(|parent| parent.namespace_for_prefix_bytes(prefix))
      })
  }

  fn namespace_for_prefix_with_current_bytes<'b>(
    &'b self,
    namespaces: &'b [XmlNamespace],
    prefix: &[u8],
  ) -> Option<&'b [u8]> {
    self
      .namespace_for_prefix_in_frame_bytes(namespaces, prefix)
      .or_else(|| self.namespace_for_prefix_bytes(prefix))
  }

  fn namespace_for_prefix_in_frame_bytes<'b>(
    &'b self,
    namespaces: &'b [XmlNamespace],
    prefix: &[u8],
  ) -> Option<&'b [u8]> {
    namespaces.iter().rev().find_map(|candidate| {
      let (candidate_prefix, candidate_uri) = candidate.parts();
      (candidate_prefix == prefix).then_some(candidate_uri)
    })
  }

  fn qname_parts_with_current_bytes<'b>(
    &'b self,
    namespaces: &'b [XmlNamespace],
    qname: &'b [u8],
  ) -> Option<(&'b [u8], &'b [u8])> {
    let prefix_len = qname.iter().position(|byte| *byte == b':')?;
    let namespace =
      self.namespace_for_prefix_with_current_bytes(namespaces, &qname[..prefix_len])?;
    Some((namespace, &qname[prefix_len + 1..]))
  }

  fn current_ignorable_namespaces<'b>(
    &'b self,
    namespaces: &'b [XmlNamespace],
    mc_ignorable: Option<&'b [u8]>,
  ) -> Result<Vec<&'b [u8]>, SdkError> {
    let mut ignorable_namespaces = Vec::new();
    if let Some(value) = mc_ignorable {
      let prefixes = McePrefixList::new(value)?;
      for prefix in prefixes.prefixes() {
        if let Some(ns) = self.namespace_for_prefix_with_current_bytes(namespaces, prefix) {
          ignorable_namespaces.push(ns);
        }
      }
    }
    Ok(ignorable_namespaces)
  }

  pub(crate) fn is_ignorable_namespace_bytes(&self, namespace: &[u8]) -> bool {
    self.ignorable_namespaces.contains(&namespace)
      || self
        .parent
        .is_some_and(|parent| parent.is_ignorable_namespace_bytes(namespace))
  }
}

pub(crate) fn namespace_supported(ns: &[u8], target: FileFormatVersion) -> bool {
  namespace_minimum_version(ns)
    .is_some_and(|version| file_format_rank(version) <= file_format_rank(target))
}

fn namespace_minimum_version(ns: &[u8]) -> Option<FileFormatVersion> {
  if bytes_contains(ns, b"openxmlformats.org")
    || bytes_contains(ns, b"/2006/")
    || bytes_contains(ns, b":office:")
  {
    Some(FileFormatVersion::Office2007)
  } else if bytes_contains(ns, b"/2010/") || bytes_contains(ns, b"14") {
    Some(FileFormatVersion::Office2010)
  } else if bytes_contains(ns, b"/2012/") || bytes_contains(ns, b"15") {
    Some(FileFormatVersion::Office2013)
  } else if bytes_contains(ns, b"/2016/") || bytes_contains(ns, b"16") {
    Some(FileFormatVersion::Office2016)
  } else if bytes_contains(ns, b"/2019/") {
    Some(FileFormatVersion::Office2019)
  } else if bytes_contains(ns, b"/2021/") {
    Some(FileFormatVersion::Office2021)
  } else if bytes_contains(ns, b"/2022/")
    || bytes_contains(ns, b"/2023/")
    || bytes_contains(ns, b"/2024/")
    || bytes_contains(ns, b"/2025/")
  {
    Some(FileFormatVersion::Microsoft365)
  } else {
    None
  }
}

fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
  haystack
    .windows(needle.len())
    .any(|window| window == needle)
}

fn file_format_rank(version: FileFormatVersion) -> u8 {
  match version {
    FileFormatVersion::Office2007 => 0,
    FileFormatVersion::Office2010 => 1,
    FileFormatVersion::Office2013 => 2,
    FileFormatVersion::Office2016 => 3,
    FileFormatVersion::Office2019 => 4,
    FileFormatVersion::Office2021 => 5,
    FileFormatVersion::Microsoft365 => 6,
  }
}

const fn is_xml_whitespace(value: u8) -> bool {
  matches!(value, b' ' | b'\r' | b'\n' | b'\t')
}

fn split_xml_whitespace(value: &[u8]) -> impl Iterator<Item = &[u8]> {
  value
    .split(|byte| is_xml_whitespace(*byte))
    .filter(|part| !part.is_empty())
}

fn split_xml_whitespace_ranges(value: &[u8]) -> impl Iterator<Item = std::ops::Range<usize>> + '_ {
  let mut offset = 0;
  std::iter::from_fn(move || {
    while offset < value.len() && is_xml_whitespace(value[offset]) {
      offset += 1;
    }
    if offset == value.len() {
      return None;
    }
    let start = offset;
    while offset < value.len() && !is_xml_whitespace(value[offset]) {
      offset += 1;
    }
    Some(start..offset)
  })
}

fn decode_mce_attr_value(value: &[u8]) -> Result<std::borrow::Cow<'_, [u8]>, SdkError> {
  if !value.contains(&b'&') {
    return Ok(std::borrow::Cow::Borrowed(value));
  }

  let value = std::str::from_utf8(value)
    .map_err(|err| SdkError::CommonError(format!("invalid MCE attribute value: {err}")))?;
  let decoded = quick_xml::escape::unescape(value)
    .map_err(|err| SdkError::CommonError(format!("invalid MCE attribute value: {err}")))?;
  Ok(match decoded {
    std::borrow::Cow::Borrowed(value) => std::borrow::Cow::Borrowed(value.as_bytes()),
    std::borrow::Cow::Owned(value) => std::borrow::Cow::Owned(value.into_bytes()),
  })
}

pub(crate) fn for_each_mce_prefix(
  value: &[u8],
  mut f: impl FnMut(&[u8]) -> Result<(), SdkError>,
) -> Result<(), SdkError> {
  let prefixes = McePrefixList::new(value)?;
  for prefix in prefixes.prefixes() {
    f(prefix)?;
  }
  Ok(())
}

pub(crate) fn process_alternate_content_children<F>(
  alternate_content: &mut crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent,
  settings: &crate::sdk::MarkupCompatibilityProcessSettings,
  context: &MceContext<'_>,
  mut process_child: F,
) -> Result<(), SdkError>
where
  F: FnMut(Box<[u8]>, &MceContext<'_>) -> Result<(), SdkError>,
{
  use crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::{
    AlternateContent, AlternateContentChoice,
  };

  let AlternateContent {
    xmlns,
    mc_ignorable,
    mc_process_content,
    mc_must_understand,
    alternate_content_choice,
    ..
  } = alternate_content;
  let alternate_context = context.child_context(
    xmlns,
    MceContextAttributes {
      ignorable: mc_ignorable.as_deref(),
      process_content: mc_process_content.as_deref(),
      must_understand: mc_must_understand.as_deref(),
      ..Default::default()
    },
    settings,
  )?;
  let mut fallback_index = None;

  for (index, branch) in alternate_content_choice.iter_mut().enumerate() {
    match branch {
      AlternateContentChoice::Choice(choice) => {
        let supported = choice_requires_supported(
          Some(choice.requires.as_bytes()),
          &alternate_context,
          &[choice.xmlns.as_slice(), xmlns.as_slice()],
          settings.target_file_format_version,
        )?;
        if !supported {
          continue;
        }
        let children = std::mem::take(&mut choice.xml_children);
        let branch_context = alternate_context.child_context(
          &choice.xmlns,
          MceContextAttributes {
            ignorable: choice.mc_ignorable.as_deref(),
            preserve_attributes: choice.mc_preserve_attributes.as_deref(),
            preserve_elements: choice.mc_preserve_elements.as_deref(),
            process_content: choice.mc_process_content.as_deref(),
            must_understand: choice.mc_must_understand.as_deref(),
          },
          settings,
        )?;
        for child in children {
          process_child(child, &branch_context)?;
        }
        return Ok(());
      }
      AlternateContentChoice::Fallback(_) if fallback_index.is_none() => {
        fallback_index = Some(index);
      }
      AlternateContentChoice::Fallback(_) => {}
    }
  }

  let Some(fallback_index) = fallback_index else {
    return Ok(());
  };
  let AlternateContentChoice::Fallback(fallback) = &mut alternate_content_choice[fallback_index]
  else {
    unreachable!();
  };
  let children = std::mem::take(&mut fallback.xml_children);
  let fallback_context = alternate_context.child_context(
    &fallback.xmlns,
    MceContextAttributes {
      ignorable: fallback.mc_ignorable.as_deref(),
      process_content: fallback.mc_process_content.as_deref(),
      must_understand: fallback.mc_must_understand.as_deref(),
      ..Default::default()
    },
    settings,
  )?;
  for child in children {
    process_child(child, &fallback_context)?;
  }
  Ok(())
}

fn choice_requires_supported(
  requires: Option<&[u8]>,
  context: &MceContext<'_>,
  namespace_frames: &[&[XmlNamespace]],
  target: FileFormatVersion,
) -> Result<bool, SdkError> {
  let Some(requires) = requires else {
    return Ok(false);
  };
  let mut supported = true;
  for_each_mce_prefix(requires, |prefix| {
    let Some(ns) = namespace_for_prefix_with_frames(context, namespace_frames, prefix) else {
      supported = false;
      return Ok(());
    };
    if !namespace_supported(ns, target) {
      supported = false;
    }
    Ok(())
  })?;
  Ok(supported)
}

fn namespace_for_prefix_with_frames<'a>(
  context: &'a MceContext<'_>,
  namespace_frames: &'a [&'a [XmlNamespace]],
  prefix: &[u8],
) -> Option<&'a [u8]> {
  namespace_frames
    .iter()
    .find_map(|namespaces| namespace_for_prefix_in_frame(namespaces, prefix))
    .or_else(|| context.namespace_for_prefix_bytes(prefix))
}

fn namespace_for_prefix_in_frame<'a>(
  namespaces: &'a [XmlNamespace],
  prefix: &[u8],
) -> Option<&'a [u8]> {
  namespaces.iter().rev().find_map(|namespace| {
    let (namespace_prefix, namespace_uri) = namespace.parts();
    (namespace_prefix == prefix).then_some(namespace_uri)
  })
}
