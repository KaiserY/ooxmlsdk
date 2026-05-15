#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent
DATA_DIR = REPO_ROOT / "data" / "schemas"
NAMESPACES_JSON = REPO_ROOT / "sdk_data" / "namespaces.json"
RUST_SCHEMAS_DIR = REPO_ROOT / "crates" / "ooxmlsdk" / "src" / "schemas"
UPSTREAM_GENERATED_DIR = (
    REPO_ROOT
    / ".."
    / "Open-XML-SDK"
    / "generated"
    / "DocumentFormat.OpenXml"
    / "DocumentFormat.OpenXml.Generator"
    / "DocumentFormat.OpenXml.Generator.OpenXmlGenerator"
)
UPSTREAM_PARTICLES_JSON = (
    REPO_ROOT
    / ".."
    / "Open-XML-SDK"
    / "test"
    / "DocumentFormat.OpenXml.Packaging.Tests"
    / "data"
    / "Particles.json"
)
DEFAULT_JSON_REPORT = REPO_ROOT / "particle_alignment_report.json"
DEFAULT_MD_REPORT = REPO_ROOT / "particle_alignment_report.md"


@dataclass
class LocalType:
    module_name: str
    target_namespace: str
    raw: dict[str, Any]

    @property
    def class_name(self) -> str:
        return self.raw["ClassName"]

    @property
    def qname(self) -> str:
        return self.raw.get("Name", "")

    @property
    def composite_type(self) -> str:
        return self.raw.get("CompositeType", "")

    @property
    def version(self) -> str:
        return self.raw.get("Version", "")

    @property
    def particle(self) -> dict[str, Any]:
        return self.raw.get("Particle") or {}

    @property
    def particle_kind(self) -> str:
        return self.particle.get("Kind", "")


@dataclass
class UpstreamType:
    clr_key: str
    namespace: str
    class_name: str
    element_uri: str | None
    element_local: str | None
    type_uri: str | None
    type_local: str | None
    source_file: Path


@dataclass
class RustSchemaIndex:
    path: Path
    text: str
    type_names: list[str]


@dataclass
class GeneratedMember:
    kind: str
    qname: str | None
    cardinality: str | None
    payload_type: str | None


@dataclass
class GeneratedTypeDef:
    name: str
    kind: str
    members: list[GeneratedMember]
    alias_target: str | None = None


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def write_json(path: Path, value: Any) -> None:
    with path.open("w", encoding="utf-8") as fh:
        json.dump(value, fh, indent=2, ensure_ascii=False)
        fh.write("\n")


def split_prefixed(name: str) -> tuple[str, str]:
    if ":" not in name:
        raise ValueError(f"expected prefixed name, got {name!r}")
    return tuple(name.split(":", 1))  # type: ignore[return-value]


def load_prefix_map() -> dict[str, str]:
    items = load_json(NAMESPACES_JSON)
    return {item["Prefix"]: item["Uri"] for item in items}


def expand_prefixed(name: str, prefix_map: dict[str, str]) -> tuple[str, str]:
    prefix, local = split_prefixed(name)
    uri = prefix_map.get(prefix)
    if uri is None:
        raise KeyError(f"namespace prefix {prefix!r} not found for {name!r}")
    return uri, local


def expand_schema_particle_name(name: str, prefix_map: dict[str, str]) -> dict[str, str]:
    type_name, element_name = name.split("/", 1)
    type_uri, type_local = expand_prefixed(type_name, prefix_map)
    element_uri, element_local = expand_prefixed(element_name, prefix_map)
    return {
        "type_qname": f"{type_uri}:{type_local}",
        "element_qname": f"{element_uri}:{element_local}",
    }


def normalize_local_particle(
    particle: dict[str, Any], prefix_map: dict[str, str]
) -> dict[str, Any]:
    kind = particle.get("Kind") or ("Element" if particle.get("Name") else "")
    occurs = particle.get("Occurs") or []
    if occurs:
        first = occurs[0]
        min_occurs = first.get("Min")
        max_occurs = first.get("Max")
    else:
        min_occurs = None
        max_occurs = None

    normalized: dict[str, Any] = {
        "kind": kind,
        "occurs_present": bool(occurs),
        "min_occurs": min_occurs,
        "max_occurs": max_occurs,
        "version": particle.get("InitialVersion", ""),
        "require_filter": particle.get("RequireFilter", False),
    }

    name = particle.get("Name")
    if name:
        normalized["name"] = name
        normalized.update(expand_schema_particle_name(name, prefix_map))

    items = particle.get("Items") or []
    if items:
        normalized["children"] = [
            normalize_local_particle(item, prefix_map) for item in items
        ]

    return normalized


def normalize_upstream_particle(particle: dict[str, Any]) -> dict[str, Any]:
    normalized: dict[str, Any] = {
        "kind": particle.get("ParticleType", ""),
        "min_occurs": particle.get("MinOccurs"),
        "max_occurs": particle.get("MaxOccurs"),
    }

    type_info = particle.get("Type")
    if type_info:
        normalized["element_qname"] = type_info.get("Name")
        normalized["type_qname"] = type_info.get("Type")

    children = particle.get("ChildrenParticles") or []
    if children:
        normalized["children"] = [normalize_upstream_particle(child) for child in children]

    return normalized


def compact_expanded_qname(value: str | None, prefix_map: dict[str, str]) -> str | None:
    if not value or ":" not in value:
        return value
    uri, local = value.rsplit(":", 1)
    for prefix, candidate_uri in prefix_map.items():
        if candidate_uri == uri:
            return f"{prefix}:{local}"
    return value


def particle_stats(tree: dict[str, Any] | None) -> dict[str, Any]:
    if not tree:
        return {
            "node_count": 0,
            "depth": 0,
            "element_count": 0,
            "choice_count": 0,
            "sequence_count": 0,
            "all_count": 0,
            "any_count": 0,
        }

    stats = {
        "node_count": 0,
        "depth": 0,
        "element_count": 0,
        "choice_count": 0,
        "sequence_count": 0,
        "all_count": 0,
        "any_count": 0,
    }

    def walk(node: dict[str, Any], depth: int) -> None:
        stats["node_count"] += 1
        stats["depth"] = max(stats["depth"], depth)
        kind = node.get("kind", "")
        if kind == "Element":
            stats["element_count"] += 1
        elif kind == "Choice":
            stats["choice_count"] += 1
        elif kind == "Sequence":
            stats["sequence_count"] += 1
        elif kind == "All":
            stats["all_count"] += 1
        elif kind == "Any":
            stats["any_count"] += 1
        for child in node.get("children", []):
            walk(child, depth + 1)

    walk(tree, 1)
    return stats


def local_cardinality(node: dict[str, Any]) -> str:
    if not node.get("occurs_present", False):
        return "one"
    min_occurs = node.get("min_occurs")
    max_occurs = node.get("max_occurs")
    if min_occurs is None and max_occurs is None:
        return "many"
    if min_occurs is None and max_occurs == 1:
        return "optional"
    if max_occurs is None or (isinstance(max_occurs, int) and max_occurs > 1):
        return "many"
    if min_occurs == 0:
        return "optional"
    return "one"


def transparent_local_node(node: dict[str, Any]) -> dict[str, Any]:
    kind = node.get("kind", "")
    if kind == "Group":
        children = node.get("children", [])
        if len(children) == 1:
            child = dict(transparent_local_node(children[0]))
            child["occurs_present"] = node.get("occurs_present", False) or child.get(
                "occurs_present", False
            )
            if node.get("min_occurs") is not None:
                child["min_occurs"] = node.get("min_occurs")
            if node.get("max_occurs") is not None:
                child["max_occurs"] = node.get("max_occurs")
            return child
    return node


def normalize_local_structure_tree(node: dict[str, Any] | None) -> dict[str, Any] | None:
    if not node:
        return None
    node = transparent_local_node(node)
    kind = node.get("kind", "")
    if kind in {"Sequence", "All", "Choice"}:
        return {
            "kind": kind,
            "cardinality": local_cardinality(node),
            "children": [
                child
                for child in (
                    normalize_local_structure_tree(child)
                    for child in node.get("children", [])
                )
                if child is not None
            ],
        }
    if kind == "Any":
        return {"kind": "Any", "cardinality": local_cardinality(node)}
    if kind in {"Element", ""}:
        return {
            "kind": "Element",
            "cardinality": local_cardinality(node),
            "qname": node.get("name") or node.get("element_qname"),
        }
    return {"kind": kind, "cardinality": local_cardinality(node)}


def upstream_cardinality(node: dict[str, Any]) -> str:
    min_occurs = node.get("min_occurs")
    max_occurs = node.get("max_occurs")
    if min_occurs is None and max_occurs is None:
        return "one"
    if max_occurs == 0 or (isinstance(max_occurs, int) and max_occurs > 1):
        return "many"
    if min_occurs == 0 and (max_occurs is None or max_occurs == 1):
        return "optional"
    return "one"


def normalize_upstream_structure_tree(
    node: dict[str, Any] | None, prefix_map: dict[str, str]
) -> dict[str, Any] | None:
    if not node:
        return None
    kind = node.get("kind", "")
    if kind == "Group":
        children = node.get("children", [])
        if len(children) == 1:
            child = normalize_upstream_structure_tree(children[0], prefix_map)
            if child is None:
                return None
            child = dict(child)
            child["cardinality"] = upstream_cardinality(node)
            return child
    if kind in {"Sequence", "All", "Choice"}:
        return {
            "kind": kind,
            "cardinality": upstream_cardinality(node),
            "children": [
                child
                for child in (
                    normalize_upstream_structure_tree(child, prefix_map)
                    for child in node.get("children", [])
                )
                if child is not None
            ],
        }
    if kind == "Any":
        return {"kind": "Any", "cardinality": upstream_cardinality(node)}
    if kind in {"Element", ""}:
        type_qname = compact_expanded_qname(node.get("type_qname"), prefix_map)
        element_qname = compact_expanded_qname(node.get("element_qname"), prefix_map)
        qname = None
        if type_qname and element_qname:
            qname = f"{type_qname}/{element_qname}"
        else:
            qname = element_qname or type_qname
        return {
            "kind": "Element",
            "cardinality": upstream_cardinality(node),
            "qname": qname,
        }
    return {"kind": kind, "cardinality": upstream_cardinality(node)}


def normalize_emitted_structure_tree(tree: dict[str, Any] | None) -> dict[str, Any] | None:
    if tree is None:
        return None

    kind = tree.get("kind", "")
    children = [
        child
        for child in (
            normalize_emitted_structure_tree(child) for child in tree.get("children", [])
        )
        if child is not None
    ]

    normalized = dict(tree)
    if children:
        normalized["children"] = children

    if kind in {"Sequence", "All"}:
        emitted_children: list[dict[str, Any]] = []
        for child in children:
            child_kind = child.get("kind", "")
            if child_kind in {"Sequence", "All"} and child.get("cardinality") == "one":
                emitted_children.extend(child.get("children", []))
            else:
                emitted_children.append(child)
        normalized["children"] = emitted_children
    elif kind == "Choice":
        emitted_children = []
        for child in children:
            emitted_child = dict(child)
            emitted_child["cardinality"] = "one"
            emitted_children.append(emitted_child)
        normalized["children"] = emitted_children

    return normalized


def canonical_module_key(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", value.lower())


def split_camel_words(value: str) -> list[str]:
    return re.findall(r"[A-Z]+(?=[A-Z][a-z]|$)|[A-Z]?[a-z]+|[0-9]+", value) or [value]


def rust_name_candidates(class_name: str) -> list[str]:
    words = split_camel_words(class_name)
    candidate = "".join(word[:1].upper() + word[1:].lower() for word in words if word)
    out = [class_name]
    if candidate and candidate not in out:
        out.append(candidate)
    return out


def expected_root_choice_field_names(tree: dict[str, Any] | None) -> list[str]:
    if not tree:
        return []

    kind = tree.get("kind", "")
    if kind == "Choice":
        return ["choice"]
    if kind not in {"Sequence", "All"}:
        return []

    count = sum(
        1 for child in root_emitted_nodes(tree) if child.get("kind", "") == "Choice"
    )
    if count <= 0:
        return []
    if count == 1:
        return ["choice"]
    return [f"choice{i}" for i in range(1, count + 1)]


def root_emitted_nodes(node: dict[str, Any]) -> list[dict[str, Any]]:
    kind = node.get("kind", "")
    if kind in {"Sequence", "All", "Group"}:
        emitted: list[dict[str, Any]] = []
        for child in node.get("children", []):
            emitted.extend(root_emitted_nodes(child))
        return emitted
    return [node]


def load_local_types() -> list[LocalType]:
    types: list[LocalType] = []
    for path in sorted(DATA_DIR.glob("*.json")):
        schema = load_json(path)
        if (
            not isinstance(schema, dict)
            or "Types" not in schema
            or "TargetNamespace" not in schema
        ):
            continue
        module_name = path.stem
        target_namespace = schema["TargetNamespace"]
        for raw_type in schema["Types"]:
            types.append(
                LocalType(
                    module_name=module_name,
                    target_namespace=target_namespace,
                    raw=raw_type,
                )
            )
    return types


def find_local_type(
    types: list[LocalType], class_name: str | None, qname: str | None
) -> LocalType:
    matches = [
        item
        for item in types
        if (class_name and item.class_name == class_name)
        or (qname and item.qname == qname)
    ]
    if not matches:
        raise SystemExit("no local type matched --class/--qname")
    if len(matches) > 1:
        rendered = "\n".join(
            f"  - {item.class_name} [{item.module_name}] {item.qname}" for item in matches
        )
        raise SystemExit(f"multiple local types matched:\n{rendered}")
    return matches[0]


def build_upstream_index() -> list[UpstreamType]:
    entries: list[UpstreamType] = []
    namespace_re = re.compile(r"^\s*namespace\s+([A-Za-z0-9_.]+)")
    class_re = re.compile(r"^\s*public partial class\s+([A-Za-z0-9_]+)\s*:")
    qname_re = re.compile(
        r'ElementQName = new\("(?P<uri>[^"]+)",\s*"(?P<local>[^"]+)"\);'
    )
    type_re = re.compile(
        r'ElementTypeName = new\("(?P<uri>[^"]+)",\s*"(?P<local>[^"]+)"\);'
    )

    for path in sorted(UPSTREAM_GENERATED_DIR.glob("*.g.cs")):
        namespace = None
        lines = path.read_text(encoding="utf-8").splitlines()
        i = 0
        while i < len(lines):
            line = lines[i]
            namespace_match = namespace_re.match(line)
            if namespace_match:
                namespace = namespace_match.group(1)
                i += 1
                continue

            class_match = class_re.match(line)
            if not class_match or namespace is None:
                i += 1
                continue

            class_name = class_match.group(1)
            element_uri = element_local = type_uri = type_local = None
            j = i
            while j < min(i + 40, len(lines)):
                qname_match = qname_re.search(lines[j])
                if qname_match and element_uri is None:
                    element_uri = qname_match.group("uri")
                    element_local = qname_match.group("local")
                type_match = type_re.search(lines[j])
                if type_match and type_uri is None:
                    type_uri = type_match.group("uri")
                    type_local = type_match.group("local")
                if element_uri and type_uri:
                    break
                j += 1

            entries.append(
                UpstreamType(
                    clr_key=f"{namespace}.{class_name}",
                    namespace=namespace,
                    class_name=class_name,
                    element_uri=element_uri,
                    element_local=element_local,
                    type_uri=type_uri,
                    type_local=type_local,
                    source_file=path,
                )
            )
            i = j
            continue

        i += 1

    return entries


def build_upstream_lookup(
    local_types: list[LocalType], prefix_map: dict[str, str]
) -> dict[str, UpstreamType]:
    generated = build_upstream_index()
    lookup: dict[tuple[str, str, str, str, str], UpstreamType] = {}
    for entry in generated:
        if (
            entry.element_uri is None
            or entry.element_local is None
            or entry.type_uri is None
            or entry.type_local is None
        ):
            continue
        key = (
            entry.class_name,
            entry.element_uri,
            entry.element_local,
            entry.type_uri,
            entry.type_local,
        )
        lookup[key] = entry

    resolved: dict[str, UpstreamType] = {}
    for local_type in local_types:
        if not local_type.qname:
            continue
        try:
            name_info = expand_schema_particle_name(local_type.qname, prefix_map)
        except Exception:
            continue
        element_uri, element_local = name_info["element_qname"].rsplit(":", 1)
        type_uri, type_local = name_info["type_qname"].rsplit(":", 1)
        key = (
            local_type.class_name,
            element_uri,
            element_local,
            type_uri,
            type_local,
        )
        if key in lookup:
            resolved[f"{local_type.module_name}:{local_type.class_name}"] = lookup[key]
    return resolved


def load_upstream_particles() -> dict[str, list[dict[str, Any]]]:
    raw = load_json(UPSTREAM_PARTICLES_JSON)
    return {item["Key"]: item["Value"] for item in raw}


def build_rust_schema_indexes() -> dict[str, RustSchemaIndex]:
    indexes: dict[str, RustSchemaIndex] = {}
    type_re = re.compile(r"^\s*pub (?:struct|enum|type)\s+([A-Za-z0-9_]+)\b", re.M)
    for path in sorted(RUST_SCHEMAS_DIR.glob("*.rs")):
        text = path.read_text(encoding="utf-8")
        type_names = type_re.findall(text)
        index = RustSchemaIndex(path=path, text=text, type_names=type_names)
        indexes[path.stem] = index
        indexes[path.stem.replace("-", "_")] = index
        indexes[path.stem.replace("_", "-")] = index
        indexes[canonical_module_key(path.stem)] = index
    return indexes


def split_top_level_items(body: str) -> list[str]:
    items: list[str] = []
    current: list[str] = []
    depth_paren = depth_angle = depth_brace = 0
    for ch in body:
        current.append(ch)
        if ch == "(":
            depth_paren += 1
        elif ch == ")":
            depth_paren -= 1
        elif ch == "<":
            depth_angle += 1
        elif ch == ">":
            depth_angle = max(0, depth_angle - 1)
        elif ch == "{":
            depth_brace += 1
        elif ch == "}":
            depth_brace = max(0, depth_brace - 1)
        elif ch == "," and depth_paren == depth_angle == depth_brace == 0:
            item = "".join(current[:-1]).strip()
            if item:
                items.append(item)
            current = []
    tail = "".join(current).strip()
    if tail:
        items.append(tail)
    return items


def extract_type_blocks(text: str) -> dict[str, tuple[str, str]]:
    matches = list(
        re.finditer(r"(?m)^.*\bpub (struct|enum)\s+([A-Za-z0-9_]+)\b.*\{.*$", text)
    )
    blocks: dict[str, tuple[str, str]] = {}
    for index, match in enumerate(matches):
        kind = match.group(1)
        name = match.group(2)
        line = match.group(0)
        if "{}" in line.replace(" ", ""):
            blocks[name] = (kind, "")
            continue
        start = match.end()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        blocks[name] = (kind, text[start:end])
    return blocks


def extract_type_aliases(text: str) -> dict[str, str]:
    aliases: dict[str, str] = {}
    for match in re.finditer(r"(?m)^\s*pub type\s+([A-Za-z0-9_]+)\s*=\s*(.+?);\s*$", text):
        aliases[match.group(1)] = match.group(2).strip()
    return aliases


def infer_cardinality_from_type(type_expr: str) -> str:
    type_expr = type_expr.strip()
    if type_expr.startswith("Vec<"):
        return "many"
    if type_expr.startswith("Option<"):
        return "optional"
    return "one"


def unwrap_type_expr(type_expr: str) -> str:
    type_expr = type_expr.strip()
    for prefix in ("Option<", "Vec<", "std::boxed::Box<", "Box<"):
        if type_expr.startswith(prefix) and type_expr.endswith(">"):
            inner = type_expr[len(prefix) : -1]
            return unwrap_type_expr(inner)
    return type_expr.strip()


def parse_qnames_from_attr(attr_text: str) -> list[str]:
    return re.findall(r'qname\s*=\s*"([^"]+)"', attr_text)


STRUCTURE_QNAME_OVERRIDES = {
    "w:CT_NonNegativeShort/w:defaultTabStop": "w:CT_TwipsMeasure/w:defaultTabStop",
    "w:CT_TblWidthDxaNil/w:left": "w:CT_TblWidth/w:left",
    "w:CT_TblWidthDxaNil/w:right": "w:CT_TblWidth/w:right",
    "w:CT_TblWidthShort/w:tblInd": "w:CT_TblWidth/w:tblInd",
}


def canonicalize_structure_qname(qname: str | None) -> str | None:
    if qname is None:
        return None
    if qname in STRUCTURE_QNAME_OVERRIDES:
        return STRUCTURE_QNAME_OVERRIDES[qname]
    parts = qname.split("/")
    if len(parts) == 2:
        type_name, element_name = parts
        if type_name.endswith(":CT_OnOffOnly"):
            prefix, _ = type_name.split(":", 1)
            _, local_name = element_name.split(":", 1)
            return f"{prefix}:CT_OnOff/{prefix}:{local_name}"
    return qname


def collect_attr_blocks(lines: list[str], start: int) -> tuple[list[str], int]:
    attrs: list[str] = []
    i = start
    while i < len(lines):
        line = lines[i].strip()
        if not line.startswith("#["):
            break
        block = [line]
        while not block[-1].strip().endswith(")]") and not block[-1].strip().endswith("]"):
            i += 1
            if i >= len(lines):
                break
            block.append(lines[i].strip())
        attrs.append("\n".join(block))
        i += 1
    return attrs, i


def collect_statement(lines: list[str], start: int) -> tuple[str, int]:
    parts = [lines[start].strip()]
    i = start + 1
    depth_angle = parts[0].count("<") - parts[0].count(">")
    depth_paren = parts[0].count("(") - parts[0].count(")")
    while i < len(lines):
        text = " ".join(parts).strip()
        if depth_angle <= 0 and depth_paren <= 0 and text.endswith(","):
            break
        part = lines[i].strip()
        parts.append(part)
        depth_angle += part.count("<") - part.count(">")
        depth_paren += part.count("(") - part.count(")")
        i += 1
    return " ".join(parts).strip(), i


def parse_generated_type_defs(rust_index: RustSchemaIndex) -> dict[str, GeneratedTypeDef]:
    blocks = extract_type_blocks(rust_index.text)
    aliases = extract_type_aliases(rust_index.text)
    parsed: dict[str, GeneratedTypeDef] = {}
    for name, (kind, body) in blocks.items():
        members: list[GeneratedMember] = []
        lines = body.splitlines()
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            if not line:
                i += 1
                continue
            if line == "}" or line.startswith("///"):
                i += 1
                continue
            if line.startswith("#[doc"):
                i += 1
                continue
            current_attrs, next_i = collect_attr_blocks(lines, i)
            if next_i != i:
                i = next_i
                if i >= len(lines):
                    break
                line = lines[i].strip()
            attrs = "\n".join(current_attrs)
            if kind == "struct":
                statement, next_i = collect_statement(lines, i)
                field_match = re.search(
                    r"pub\s+((?:r#)?[A-Za-z0-9_]+)\s*:\s*(.+),\s*$", statement, re.S
                )
                if not field_match:
                    i += 1
                    continue
                field_type = field_match.group(2).strip()
                if "#[sdk(attr" in attrs or "#[sdk(text)]" in attrs or "#[sdk(text(" in attrs:
                    i = next_i
                    continue
                if "#[sdk(child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=qnames[0] if qnames else None,
                            cardinality=infer_cardinality_from_type(field_type),
                            payload_type=unwrap_type_expr(field_type),
                        )
                    )
                elif "#[sdk(child)]" in attrs:
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=None,
                            cardinality=infer_cardinality_from_type(field_type),
                            payload_type=unwrap_type_expr(field_type),
                        )
                    )
                elif "#[sdk(empty_child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=qnames[0] if qnames else None,
                            cardinality=infer_cardinality_from_type(field_type),
                            payload_type=None,
                        )
                    )
                elif "#[sdk(text_child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    cardinality = (
                        "one" if "#[sdk(text_child(list" in attrs else infer_cardinality_from_type(field_type)
                    )
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=qnames[0] if qnames else None,
                            cardinality=cardinality,
                            payload_type=None,
                        )
                    )
                elif "#[sdk(any" in attrs or "#[sdk(any_child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    members.append(
                        GeneratedMember(
                            kind="Any",
                            qname=qnames[0] if qnames else None,
                            cardinality=infer_cardinality_from_type(field_type),
                            payload_type=unwrap_type_expr(field_type),
                        )
                    )
                elif "#[sdk(choice" in attrs:
                    members.append(
                        GeneratedMember(
                            kind="Choice",
                            qname=None,
                            cardinality=infer_cardinality_from_type(field_type),
                            payload_type=unwrap_type_expr(field_type),
                        )
                    )
                i = next_i
            else:
                statement, next_i = collect_statement(lines, i)
                variant_match = re.search(
                    r"((?:r#)?[A-Za-z0-9_]+)\s*(?:\((.+)\))?,\s*$", statement, re.S
                )
                if not variant_match:
                    i += 1
                    continue
                payload_type = (
                    unwrap_type_expr(variant_match.group(2))
                    if variant_match.group(2)
                    else None
                )
                if "#[sdk(child(" in attrs or "#[sdk(text_child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=qnames[0] if qnames else None,
                            cardinality="one",
                            payload_type=None,
                        )
                    )
                elif "#[sdk(empty_child(" in attrs:
                    qnames = parse_qnames_from_attr(attrs)
                    members.append(
                        GeneratedMember(
                            kind="Element",
                            qname=qnames[0] if qnames else None,
                            cardinality="one",
                            payload_type=None,
                        )
                    )
                elif "#[sdk(any" in attrs or "#[sdk(any_child(" in attrs:
                    members.append(
                        GeneratedMember(
                            kind="Any",
                            qname=None,
                            cardinality="one",
                            payload_type=None,
                        )
                    )
                elif "#[sdk(sequence)]" in attrs:
                    members.append(
                        GeneratedMember(
                            kind="Sequence",
                            qname=None,
                            cardinality="one",
                            payload_type=payload_type,
                        )
                    )
                elif payload_type:
                    inferred_kind = (
                        "Choice" if payload_type.endswith("Choice") else "Sequence"
                    )
                    members.append(
                        GeneratedMember(
                            kind=inferred_kind,
                            qname=None,
                            cardinality="one",
                            payload_type=payload_type,
                        )
                    )
                i = next_i
        parsed[name] = GeneratedTypeDef(name=name, kind=kind, members=members)
    for name, target in aliases.items():
        parsed[name] = GeneratedTypeDef(
            name=name,
            kind="alias",
            members=[],
            alias_target=target,
        )
    return parsed


def build_generated_structure_tree(
    type_name: str,
    generated_types: dict[str, GeneratedTypeDef],
    seen: set[str] | None = None,
) -> dict[str, Any] | None:
    if seen is None:
        seen = set()
    if type_name in seen:
        return {"kind": "RecursiveRef", "type": type_name}
    type_def = generated_types.get(type_name)
    if type_def is None:
        return None
    seen = set(seen)
    seen.add(type_name)
    if type_def.kind == "alias":
        target = (type_def.alias_target or "").strip()
        if target == "Vec<String>" or target == "Vec<str>" or target == "Vec<std::string::String>":
            return {
                "kind": "Sequence",
                "cardinality": "one",
                "children": [{"kind": "Any", "cardinality": "many"}],
            }
        return {
            "kind": "Alias",
            "cardinality": "one",
            "target": target,
        }
    if type_def.kind == "struct":
        return {
            "kind": "Sequence",
            "cardinality": "one",
            "children": [
                build_generated_member_tree(member, generated_types, seen)
                for member in type_def.members
            ],
        }
    return {
        "kind": "Choice",
        "cardinality": "one",
        "children": [
            build_generated_member_tree(member, generated_types, seen)
            for member in type_def.members
        ],
    }


def build_generated_member_tree(
    member: GeneratedMember,
    generated_types: dict[str, GeneratedTypeDef],
    seen: set[str],
) -> dict[str, Any]:
    if member.kind == "Element":
        if member.qname is None and member.payload_type:
            payload_tree = build_generated_structure_tree(
                member.payload_type, generated_types, seen
            )
            if payload_tree is not None:
                payload_tree = dict(payload_tree)
                payload_tree["cardinality"] = member.cardinality
                return payload_tree
        return {
            "kind": "Element",
            "cardinality": member.cardinality,
            "qname": member.qname,
        }
    if member.kind == "Any":
        if member.qname and member.payload_type:
            payload_tree = build_generated_structure_tree(
                member.payload_type, generated_types, seen
            )
            if payload_tree and payload_tree.get("kind") == "Sequence":
                children = payload_tree.get("children", [])
                if len(children) == 1 and children[0].get("kind") == "Any":
                    return {
                        "kind": "Element",
                        "cardinality": member.cardinality,
                        "qname": member.qname,
                    }
        return {"kind": "Any", "cardinality": member.cardinality}
    if member.kind in {"Choice", "Sequence"} and member.payload_type:
        payload_tree = build_generated_structure_tree(
            member.payload_type, generated_types, seen
        )
        if payload_tree is None:
            return {
                "kind": member.kind,
                "cardinality": member.cardinality,
                "unresolved_type": member.payload_type,
            }
        payload_tree = dict(payload_tree)
        payload_tree["cardinality"] = member.cardinality
        return payload_tree
    return {"kind": member.kind, "cardinality": member.cardinality}


def compare_structure_trees(
    reference: dict[str, Any] | None,
    generated: dict[str, Any] | None,
    path: str = "root",
) -> list[dict[str, Any]]:
    mismatches: list[dict[str, Any]] = []
    if reference is None and generated is None:
        return mismatches
    if reference is None or generated is None:
        mismatches.append(
            {
                "kind": "structure_presence",
                "path": path,
                "reference": reference,
                "generated": generated,
            }
        )
        return mismatches

    ref_kind = reference.get("kind")
    gen_kind = generated.get("kind")
    if ref_kind != gen_kind:
        mismatches.append(
            {
                "kind": "structure_kind",
                "path": path,
                "reference": ref_kind,
                "generated": gen_kind,
            }
        )
        return mismatches

    ref_card = reference.get("cardinality")
    gen_card = generated.get("cardinality")
    if path != "root" and ref_card != gen_card:
        mismatches.append(
            {
                "kind": "structure_cardinality",
                "path": path,
                "reference": ref_card,
                "generated": gen_card,
            }
        )

    if ref_kind == "Element":
        ref_qname = canonicalize_structure_qname(reference.get("qname"))
        gen_qname = canonicalize_structure_qname(generated.get("qname"))
        if ref_qname != gen_qname:
            mismatches.append(
                {
                    "kind": "structure_qname",
                    "path": path,
                    "reference": ref_qname,
                    "generated": gen_qname,
                }
            )
        return mismatches

    ref_children = reference.get("children", [])
    gen_children = generated.get("children", [])
    if len(ref_children) != len(gen_children):
        mismatches.append(
            {
                "kind": "structure_child_count",
                "path": path,
                "reference": len(ref_children),
                "generated": len(gen_children),
            }
        )

    for index, (ref_child, gen_child) in enumerate(zip(ref_children, gen_children), start=1):
        mismatches.extend(
            compare_structure_trees(ref_child, gen_child, f"{path}/{ref_kind}[{index}]")
        )
    return mismatches


def rust_excerpt(
    module_name: str, class_name: str, rust_index: RustSchemaIndex, extra_limit: int = 12
) -> dict[str, str]:
    text = rust_index.text
    blocks: dict[str, str] = {}

    def extract_block(type_name: str) -> str | None:
        start_pattern = re.compile(
            rf"(?m)^.*\bpub (?:struct|enum) {re.escape(type_name)}\b.*$"
        )
        start_match = start_pattern.search(text)
        if not start_match:
            return None

        next_pattern = re.compile(r"(?m)^.*\bpub (?:struct|enum) [A-Za-z0-9_]+\b.*$")
        next_match = next_pattern.search(text, start_match.end())
        end = next_match.start() if next_match else len(text)
        return text[start_match.start() : end].rstrip()

    root_name = None
    root = None
    for candidate in rust_name_candidates(class_name):
        root = extract_block(candidate)
        if root:
            root_name = candidate
            break
    if root and root_name:
        blocks[root_name] = root
        related = sorted(
            set(
                re.findall(
                    rf"\b({re.escape(root_name)}Choice[A-Za-z0-9_]*|{re.escape(root_name)}Sequence[A-Za-z0-9_]*)\b",
                    root,
                )
            )
        )
        for type_name in related[:extra_limit]:
            block = extract_block(type_name)
            if block:
                blocks[type_name] = block

    return blocks


def summarize_rust_types(local_type: LocalType, rust_index: RustSchemaIndex) -> dict[str, Any]:
    class_name = local_type.class_name
    root_candidates = rust_name_candidates(class_name)
    root_name = next((name for name in root_candidates if name in rust_index.type_names), None)
    related = sorted(
        name
        for name in rust_index.type_names
        if name == root_name
        or (root_name is not None and name.startswith(f"{root_name}Choice"))
        or (root_name is not None and name.startswith(f"{root_name}Sequence"))
    )
    root_block_map = rust_excerpt(local_type.module_name, class_name, rust_index, extra_limit=0)
    root_block = root_block_map.get(root_name or class_name, "")
    return {
        "root_present": root_name is not None,
        "root_rust_name": root_name,
        "root_choice_field_names": re.findall(
            r"pub\s+([A-Za-z0-9_]+)\s*:\s*(?:Option<)?(?:std::boxed::Box<)?(?:Vec<)?[A-Za-z0-9_]*Choice\d*(?:>)?(?:>)?(?:>)?",
            root_block,
        ),
        "related_type_count": len(related),
        "related_choice_type_count": sum(1 for name in related if "Choice" in name),
        "related_sequence_type_count": sum(1 for name in related if "Sequence" in name),
        "max_related_type_name_length": max((len(name) for name in related), default=0),
        "long_related_type_names": [name for name in related if len(name) >= 80][:30],
        "related_type_names": related[:80],
    }


def build_type_report(
    local_type: LocalType,
    prefix_map: dict[str, str],
    upstream_lookup: dict[str, UpstreamType],
    upstream_particles: dict[str, list[dict[str, Any]]],
    rust_indexes: dict[str, RustSchemaIndex],
    include_blocks: bool,
) -> dict[str, Any]:
    local_particle = local_type.particle
    normalized_local = (
        normalize_local_particle(local_particle, prefix_map) if local_particle else None
    )
    local_stats = particle_stats(normalized_local)

    upstream_type = upstream_lookup.get(f"{local_type.module_name}:{local_type.class_name}")
    upstream_particle_versions: list[dict[str, Any]] = []
    if upstream_type and upstream_type.clr_key in upstream_particles:
        for version_entry in upstream_particles[upstream_type.clr_key]:
            normalized = normalize_upstream_particle(version_entry["Value"])
            upstream_particle_versions.append(
                {
                    "version_key": version_entry["Key"],
                    "particle": normalized,
                    "stats": particle_stats(normalized),
                }
            )

    rust_index = rust_indexes.get(local_type.module_name) or rust_indexes.get(
        canonical_module_key(local_type.module_name)
    )
    rust_summary = (
        summarize_rust_types(local_type, rust_index)
        if rust_index is not None
        else {
            "root_present": False,
            "root_choice_field_names": [],
            "related_type_count": 0,
            "max_related_type_name_length": 0,
            "long_related_type_names": [],
            "related_type_names": [],
        }
    )
    generated_structure = None
    local_reference_structure = normalize_emitted_structure_tree(
        normalize_local_structure_tree(normalized_local)
    )
    reference_structure = local_reference_structure
    if upstream_particle_versions:
        scored_versions: list[tuple[int, dict[str, Any]]] = []
        for version_entry in upstream_particle_versions:
            candidate_structure = normalize_emitted_structure_tree(
                normalize_upstream_structure_tree(version_entry["particle"], prefix_map)
            )
            mismatch_score = len(
                compare_structure_trees(local_reference_structure, candidate_structure)
            )
            scored_versions.append((mismatch_score, version_entry))
        scored_versions.sort(key=lambda item: item[0])
        reference_structure = normalize_emitted_structure_tree(
            normalize_upstream_structure_tree(scored_versions[0][1]["particle"], prefix_map)
        )
    if rust_index is not None and rust_summary["root_present"]:
        generated_types = parse_generated_type_defs(rust_index)
        generated_structure = normalize_emitted_structure_tree(
            build_generated_structure_tree(rust_summary["root_rust_name"], generated_types)
        )

    mismatches: list[dict[str, Any]] = []
    if rust_summary["long_related_type_names"]:
        mismatches.append(
            {
                "kind": "long_generated_names",
                "reference": [],
                "generated": rust_summary["long_related_type_names"],
            }
        )
    if upstream_type is None:
        mismatches.append(
            {
                "kind": "missing_upstream_generated",
                "reference": "present",
                "generated": "missing",
            }
        )
    if not upstream_particle_versions:
        mismatches.append(
            {
                "kind": "missing_upstream_particles",
                "reference": "present",
                "generated": "missing",
            }
        )
    structure_mismatches = compare_structure_trees(
        reference_structure,
        generated_structure,
    )
    mismatches.extend(structure_mismatches[:80])
    report = {
        "module_name": local_type.module_name,
        "class_name": local_type.class_name,
        "qname": local_type.qname,
        "composite_type": local_type.composite_type,
        "version": local_type.version,
        "local_particle": normalized_local,
        "local_particle_stats": local_stats,
        "upstream_generated": None
        if upstream_type is None
        else {
            "clr_key": upstream_type.clr_key,
            "source_file": str(upstream_type.source_file),
            "element_qname": None
            if upstream_type.element_uri is None
            else f"{upstream_type.element_uri}:{upstream_type.element_local}",
            "type_qname": None
            if upstream_type.type_uri is None
            else f"{upstream_type.type_uri}:{upstream_type.type_local}",
        },
        "upstream_particles": upstream_particle_versions,
        "reference_structure": reference_structure,
        "generated_structure": generated_structure,
        "rust_summary": rust_summary,
        "expected_root_choice_field_names": expected_root_choice_field_names(
            reference_structure
        ),
        "mismatches": mismatches,
        "flags": {
            "missing_upstream_generated": upstream_type is None,
            "missing_upstream_particles": not upstream_particle_versions,
            "missing_rust_root": not rust_summary["root_present"],
            "long_generated_names": bool(rust_summary["long_related_type_names"]),
        },
    }

    if include_blocks and rust_index is not None:
        report["rust_blocks"] = rust_excerpt(
            local_type.module_name, local_type.class_name, rust_index
        )

    return report


def rollout_types(local_types: list[LocalType]) -> list[LocalType]:
    return [
        item
        for item in local_types
        if item.composite_type == "OneSequence" and item.particle_kind
    ]


def render_markdown_report(
    report: dict[str, Any], json_path: Path, limit: int = 80
) -> str:
    types = report["types"]
    summary = report["summary"]

    longest_names = sorted(
        (
            item
            for item in types
            if item["rust_summary"]["long_related_type_names"]
        ),
        key=lambda item: item["rust_summary"]["max_related_type_name_length"],
        reverse=True,
    )[:limit]

    missing_upstream = [
        item for item in types if item["flags"]["missing_upstream_generated"]
    ][:limit]
    missing_particles = [
        item for item in types if item["flags"]["missing_upstream_particles"]
    ][:limit]
    mismatched = [item for item in types if item["mismatches"]][:limit]

    lines = [
        "# Particle Alignment Report",
        "",
        f"- Scope: `{report['scope']}`",
        f"- Total types: `{summary['total_types']}`",
        f"- Missing upstream generated match: `{summary['missing_upstream_generated']}`",
        f"- Missing upstream particles: `{summary['missing_upstream_particles']}`",
        f"- Types with long generated names: `{summary['long_generated_names']}`",
        f"- Types with explicit mismatches: `{sum(1 for item in types if item['mismatches'])}`",
        f"- JSON report: `{json_path.name}`",
        "",
        "## Explicit Mismatches",
        "",
        "| Type | Module | Check | Reference | Generated |",
        "| --- | --- | --- | --- | --- |",
    ]

    for item in mismatched:
        for mismatch in item["mismatches"]:
            reference = json.dumps(mismatch["reference"], ensure_ascii=False)
            generated = json.dumps(mismatch["generated"], ensure_ascii=False)
            lines.append(
                f"| `{item['class_name']}` | `{item['module_name']}` | "
                f"`{mismatch['kind']}` | `{reference}` | `{generated}` |"
            )

    lines.extend(
        [
            "",
        "## Long Generated Names",
        "",
        "| Type | Module | Max name len | Example |",
        "| --- | --- | ---: | --- |",
        ]
    )

    for item in longest_names:
        example = item["rust_summary"]["long_related_type_names"][0]
        lines.append(
            f"| `{item['class_name']}` | `{item['module_name']}` | "
            f"{item['rust_summary']['max_related_type_name_length']} | `{example}` |"
        )

    lines.extend(
        [
            "",
            "## Missing Upstream Generated Match",
            "",
            "| Type | Module | QName |",
            "| --- | --- | --- |",
        ]
    )
    for item in missing_upstream:
        lines.append(
            f"| `{item['class_name']}` | `{item['module_name']}` | `{item['qname']}` |"
        )

    lines.extend(
        [
            "",
            "## Missing Upstream Particles",
            "",
            "| Type | Module | QName |",
            "| --- | --- | --- |",
        ]
    )
    for item in missing_particles:
        lines.append(
            f"| `{item['class_name']}` | `{item['module_name']}` | `{item['qname']}` |"
        )

    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Audit generated particle-driven structure against local data/, "
            "upstream generated metadata, and upstream Particles.json."
        )
    )
    parser.add_argument("--class", dest="class_name", help="single local ClassName")
    parser.add_argument("--qname", help="single local schema qname")
    parser.add_argument(
        "--all-types",
        action="store_true",
        help="scan all local schema types; default scope is OneSequence with Particle only",
    )
    parser.add_argument(
        "--include-blocks",
        action="store_true",
        help="include rust block excerpts in JSON report",
    )
    parser.add_argument(
        "--json-out",
        type=Path,
        default=DEFAULT_JSON_REPORT,
        help=f"JSON report path (default: {DEFAULT_JSON_REPORT.name})",
    )
    parser.add_argument(
        "--md-out",
        type=Path,
        default=DEFAULT_MD_REPORT,
        help=f"Markdown report path (default: {DEFAULT_MD_REPORT.name})",
    )
    parser.add_argument(
        "--stdout-json",
        action="store_true",
        help="print JSON report to stdout instead of writing files",
    )
    args = parser.parse_args()

    prefix_map = load_prefix_map()
    local_types = load_local_types()
    upstream_lookup = build_upstream_lookup(local_types, prefix_map)
    upstream_particles = load_upstream_particles()
    rust_indexes = build_rust_schema_indexes()

    if args.class_name or args.qname:
        selected = [find_local_type(local_types, args.class_name, args.qname)]
        scope = "single"
    elif args.all_types:
        selected = local_types
        scope = "all_types"
    else:
        selected = rollout_types(local_types)
        scope = "one_sequence_only"

    type_reports = [
        build_type_report(
            local_type=item,
            prefix_map=prefix_map,
            upstream_lookup=upstream_lookup,
            upstream_particles=upstream_particles,
            rust_indexes=rust_indexes,
            include_blocks=args.include_blocks or scope == "single",
        )
        for item in selected
    ]

    summary = {
        "total_types": len(type_reports),
        "missing_upstream_generated": sum(
            1 for item in type_reports if item["flags"]["missing_upstream_generated"]
        ),
        "missing_upstream_particles": sum(
            1 for item in type_reports if item["flags"]["missing_upstream_particles"]
        ),
        "long_generated_names": sum(
            1 for item in type_reports if item["flags"]["long_generated_names"]
        ),
    }

    report = {
        "scope": scope,
        "summary": summary,
        "types": type_reports,
    }

    if args.stdout_json:
        json.dump(report, sys.stdout, indent=2, ensure_ascii=False)
        sys.stdout.write("\n")
        return 0

    write_json(args.json_out, report)
    args.md_out.write_text(
        render_markdown_report(report, args.json_out), encoding="utf-8"
    )
    print(f"wrote {args.json_out}")
    print(f"wrote {args.md_out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
