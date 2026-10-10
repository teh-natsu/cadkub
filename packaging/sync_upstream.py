"""Bring CADCraft changes into CadKub's naming after `git merge upstream/main`.

  python3 packaging/sync_upstream.py

1. Renames CADCraft to CadKub in file contents and paths (library crates keep their
   cadcraft-* names), leaving the credits to upstream alone: "based on CADCraft", "CADCraft
   contributors", links to the upstream repository and the like.
   Then runs `cargo fmt --all`, since the shorter name changes line lengths.
2. Lists ArtCraft branding that came in with the merge (Discord, getartcraft.com, logos, the
   upstream company) and exits with status 1 if there is any: remove it by hand, then run again.
3. Notes CADCRAFT_* names it left alone that are not known DXF keys: if one is an environment
   variable, add it to ENV_VARS below and run again.

Safe to run more than once.
"""
import os
import re
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SKIP_DIRS = {".git", "target", "contributors"}
# Upstream's copyright, our own hand-written notices and the sample drawings are never rewritten
# (paths from the root).
SKIP_FILES = {"LICENSE-MIT", "LICENSE-APACHE", "NOTICE", "README.md", "packaging/sync_upstream.py"}
SKIP_EXTENSIONS = {".dxf", ".dwg"}


def library_crates():
    """Crate folders, plus the planned crates named in xtask's layer table (cadcraft-testkit…)."""
    names = set(os.listdir(os.path.join(ROOT, "crates")))
    layers = os.path.join(ROOT, "xtask", "src", "layers.rs")
    if os.path.isfile(layers):
        names |= set(re.findall(r'\("([a-z0-9-]+)", Class::(?!Exempt)', open(layers, encoding="utf-8").read()))
    return sorted(names, key=len, reverse=True)


LIB_CRATES = library_crates()
LOWER = re.compile(r"cadcraft(?![-_](?:" + "|".join(c.replace("-", "[-_]") for c in LIB_CRATES) + r")\b)")

# Environment variables are CadKub's own: CADCRAFT_X becomes CADKUB_X. Every other CADCRAFT name
# is data shared with CADCraft (the DXF xdata application CADCRAFT, the CADCRAFT_CONSTRAINTS
# dictionary entry, CADCRAFT_* header variables) and stays, so drawings open in both apps.
ENV_VARS = ["BUILD_DATE", "BUILD_SHA", "VERSION", "CONTROL_PORT", "DXF_OUT", "MAINTAINER", "NO_NATIVE_MENU", "REQUIRE_WINRES",
            "CONFIG_DIR", "FONTALT", "FONTFALLBACK", "VSYNC", "WAYLAND", "LOCALE"]
UPPER_ENV = re.compile(r"\bCADCRAFT_(?=(?:" + "|".join(ENV_VARS) + r")\b)")
DXF_KEYS = {"CADCRAFT", "CADCRAFT_", "CADCRAFT_CONSTRAINTS", "CADCRAFT_LAYERP", "CADCRAFT_LAYISO"}

# Credits to upstream that must keep the CADCraft name, and the cadcraft prefix shared by the
# library crates.
PROTECTED = [
    "Based on CADCraft",
    "based on CADCraft",
    "CADCraft contributors",
    "CADCraft by the ArtCraft team",
    '"cadcraft-"',
    "`cadcraft-`",
    "cadcraft*",
    "cadcraft_ui*",
]
REPLACEMENTS = [
    (r"ai\.storyteller\.cadcraft", r"io\.github\.teh_natsu\.cadkub"),  # in regular expressions
    ("ai.storyteller.cadcraft", "io.github.teh_natsu.cadkub"),
    ("CADCRAFT SAMPLE", "CADKUB SAMPLE"),
    ("CADCraft", "CadKub"),
    ("CadCraft", "CadKub"),
    ("Cadcraft", "Cadkub"),
]

BRANDING = re.compile(r"discord|crafting apps by|getartcraft|artcraft[-_](mark|logo)|docs/brand|Learning Machines|storyteller\.ai|ai\.storyteller", re.I)
BRANDING_ALLOWED_FILES = {"NOTICE", "README.md", "ROADMAP.md", "LICENSE-MIT", "packaging/sync_upstream.py"}


def rename_text(text, protect=True):
    if protect:
        # A line that names ArtCraft is a credit to upstream ("based on CADCraft by the ArtCraft
        # team", in any language): leave it as it is.
        return "".join(line if "ArtCraft" in line else rename_text(line, protect=None) for line in text.splitlines(keepends=True))
    masks = {}
    if protect is None:
        for i, phrase in enumerate(PROTECTED):
            token = f"\0KEEP{i}\0"
            masks[token] = phrase
            text = text.replace(phrase, token)
    text = text.replace("storytold/cadcraft", "teh-natsu/cadkub")
    for old, new in REPLACEMENTS:
        text = text.replace(old, new)
    text = UPPER_ENV.sub("CADKUB_", text)
    text = LOWER.sub("cadkub", text)
    for token, phrase in masks.items():
        text = text.replace(token, phrase)
    return text


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, encoding="utf-8", check=True).stdout


def rebrand():
    edited = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            path = os.path.join(dirpath, name)
            if os.path.relpath(path, ROOT).replace(os.sep, "/") in SKIP_FILES or os.path.splitext(name)[1].lower() in SKIP_EXTENSIONS:
                continue
            raw = open(path, "rb").read()
            if b"\0" in raw:
                continue
            try:
                text = raw.decode("utf-8")
            except UnicodeDecodeError:
                continue
            new = rename_text(text)
            if new != text:
                open(path, "wb").write(new.encode("utf-8"))
                edited.append(os.path.relpath(path, ROOT).replace(os.sep, "/"))
    moves = {}
    for rel in git("ls-files").split("\n"):
        if not rel or rel.startswith("contributors/") or os.path.splitext(rel)[1].lower() in SKIP_EXTENSIONS:
            continue
        parts = rel.split("/")
        for i in range(len(parts)):
            new = rename_text(parts[i], protect=False)
            if new != parts[i]:
                moves["/".join(parts[: i + 1])] = "/".join(parts[:i] + [new])
    for old in sorted(moves, key=lambda p: p.count("/"), reverse=True):
        target = os.path.join(ROOT, moves[old])
        if os.path.isdir(target) and os.path.isdir(os.path.join(ROOT, old)):
            # The folder already exists under the new name: move the files one by one.
            for f in git("ls-files", old).split("\n"):
                if f:
                    os.makedirs(os.path.dirname(os.path.join(ROOT, moves[old] + f[len(old):])), exist_ok=True)
                    git("mv", f, moves[old] + f[len(old):])
        elif os.path.exists(os.path.join(ROOT, old)):
            git("mv", old, moves[old])
    return edited, moves


def tracked_text():
    for rel in git("ls-files").split("\n"):
        if not rel or rel.startswith("contributors/") or os.path.splitext(rel)[1].lower() in SKIP_EXTENSIONS:
            continue
        try:
            yield rel, open(os.path.join(ROOT, rel), encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue


def branding_left():
    found = []
    for rel, text in tracked_text():
        if rel in BRANDING_ALLOWED_FILES:
            continue
        for n, line in enumerate(text.splitlines(), 1):
            if BRANDING.search(line):
                found.append(f"{rel}:{n}: {line.strip()[:140]}")
    return found


def unknown_upper_names():
    found = set()
    for rel, text in tracked_text():
        if rel == "packaging/sync_upstream.py":
            continue
        for name in re.findall(r"\bCADCRAFT\w*", text):
            if name not in DXF_KEYS:
                found.add(f"{name} ({rel})")
    return sorted(found)


edited, moves = rebrand()
# The new names change line lengths: let rustfmt rewrap (skipped when Rust isn't installed).
try:
    subprocess.run(["cargo", "fmt", "--all"], cwd=ROOT, check=True)
except (OSError, subprocess.CalledProcessError) as e:
    print(f"cargo fmt skipped: {e}")
print(f"renamed text in {len(edited)} files, moved {len(moves)} paths")
for f in edited:
    print(f"  edited  {f}")
for old, new in moves.items():
    print(f"  moved   {old} -> {new}")
unknown = unknown_upper_names()
if unknown:
    print("\nCADCRAFT names left as they are (DXF data stays CADCRAFT; an environment variable goes in ENV_VARS):")
    for name in unknown:
        print(f"  {name}")
left = branding_left()
if left:
    print(f"\nArtCraft branding to remove by hand ({len(left)}):")
    for line in left:
        print(f"  {line}")
    sys.exit(1)
print("no ArtCraft branding left")
