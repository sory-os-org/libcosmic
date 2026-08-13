#!/usr/bin/env python3
"""Merge `dep = { ... }` + `dep.features = ...` into valid inline tables."""

from __future__ import annotations

import re
import sys
from pathlib import Path

PROP_RE = re.compile(r"^(\s*)([\w-]+)\.(features|default-features)\s*=\s*(.*)$")
ASSIGN_RE = re.compile(r"^(\s*)([\w-]+)\s*=\s*(\{.*\})\s*$")


def read_prop_value(lines: list[str], idx: int, indent: str, name: str) -> tuple[str, int] | None:
    line = lines[idx]
    match = PROP_RE.match(line)
    if not match or match.group(1) != indent or match.group(2) != name:
        return None

    prop = match.group(3)
    value = match.group(4).strip()
    end = idx + 1

    if value == "[" or (value.startswith("[") and not value.endswith("]")):
        chunks = [value]
        while end < len(lines) and not chunks[-1].endswith("]"):
            chunks.append(lines[end].strip())
            end += 1
        value = " ".join(chunks)

    return f"{prop} = {value}", end


def merge_table(table: str, extras: list[str]) -> str:
    inner = table.strip()
    if inner.startswith("{") and inner.endswith("}"):
        inner = inner[1:-1].strip()
    parts = [inner] if inner else []
    parts.extend(extras)
    return "{ " + ", ".join(parts) + " }"


def collect_following_props(lines: list[str], start: int, indent: str, name: str) -> tuple[list[str], int]:
    extras: list[str] = []
    idx = start
    while idx < len(lines):
        parsed = read_prop_value(lines, idx, indent, name)
        if parsed is None:
            break
        extra, idx = parsed
        extras.append(extra)
    return extras, idx


def fix_lines(lines: list[str]) -> list[str]:
    skip: set[int] = set()
    output: list[str] = []

    idx = 0
    while idx < len(lines):
        if idx in skip:
            idx += 1
            continue

        line = lines[idx]
        assign_match = ASSIGN_RE.match(line)
        if assign_match:
            indent, name, table = assign_match.groups()
            extras, end = collect_following_props(lines, idx + 1, indent, name)
            if extras:
                output.append(f"{indent}{name} = {merge_table(table, extras)}")
                for j in range(idx + 1, end):
                    skip.add(j)
                idx += 1
                continue

        prop_match = PROP_RE.match(line)
        if prop_match:
            indent, name, _, _ = prop_match.groups()
            parsed = read_prop_value(lines, idx, indent, name)
            if parsed is not None:
                extra, prop_end = parsed
                assign_idx = None
                assign_table = None
                for j in range(prop_end, min(prop_end + 6, len(lines))):
                    assign_match = ASSIGN_RE.match(lines[j])
                    if assign_match and assign_match.group(1) == indent and assign_match.group(2) == name:
                        assign_idx = j
                        assign_table = assign_match.group(3)
                        break
                if assign_idx is not None:
                    output.append(f"{indent}{name} = {merge_table(assign_table, [extra])}")
                    for j in range(idx, prop_end):
                        skip.add(j)
                    skip.add(assign_idx)
                    idx += 1
                    continue

        output.append(line)
        idx += 1

    return output


def fix_file(path: Path) -> bool:
    original = path.read_text(encoding="utf-8")
    updated = "\n".join(fix_lines(original.splitlines()))
    if original.endswith("\n"):
        updated += "\n"
    if updated != original:
        path.write_text(updated, encoding="utf-8")
        return True
    return False


def main(argv: list[str]) -> int:
    root = Path(argv[1] if len(argv) > 1 else ".")
    changed = 0
    for path in sorted(root.rglob("Cargo.toml")):
        if fix_file(path):
            print(path)
            changed += 1
    print(f"fixed {changed} file(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
