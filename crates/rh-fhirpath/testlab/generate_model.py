"""Generate the compact navigation tables from the published core package snapshots."""
import json
from pathlib import Path
import sys

DEST = Path(__file__).resolve().parents[1] / "src/evaluator/metadata"


def generate(directory, release):
    types = {}
    for path in sorted(directory.glob("StructureDefinition-*.json")):
        definition = json.loads(path.read_text())
        if definition.get("derivation") == "constraint" or definition["kind"] == "logical":
            continue
        name = definition["type"]
        if definition["url"] != "http://hl7.org/fhir/StructureDefinition/" + name:
            continue
        base = definition.get("baseDefinition", "").rsplit("/", 1)[-1]
        types[name] = (base, {})
        for element in definition["snapshot"]["element"][1:]:
            owner, field = element["path"].rsplit(".", 1)
            choices = []
            for field_type in element.get("type", []):
                code = field_type["code"]
                if code.startswith("http://hl7.org/fhirpath/System."):
                    code = next((ext.get("valueUrl") or ext.get("valueUri") for ext in field_type.get("extension", [])
                                 if ext["url"].endswith("structuredefinition-fhir-type")), code.rsplit(".", 1)[-1].lower())
                choices.append(code)
            content = element.get("contentReference", "").removeprefix("#")
            if content:
                choices = [content]
            if choices == ["BackboneElement"] or (choices == ["Element"] and any(
                    child["path"].startswith(element["path"] + ".") for child in definition["snapshot"]["element"])):
                types[element["path"]] = (choices[0], {})
                choices = [element["path"]]
            if owner in types and choices:
                types[owner][1][field] = ("*" if element.get("max") not in ["0", "1"] else "") + ",".join(choices)
    lines = []
    for name, (parent, fields) in sorted(types.items()):
        inherited = types.get(parent, ("", {}))[1]
        props = [f"{field}={choices}" for field, choices in sorted(fields.items())
                 if inherited.get(field) != choices]
        lines.append("\t".join([name, parent, *props]))
    (DEST / f"{release}.tsv").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    cache = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.home() / ".fhir/packages"
    for release, version in [("r4", "4.0.1"), ("r5", "5.0.0")]:
        package = cache / f"hl7.fhir.{release}.core#{version}/package"
        if not (package / "StructureDefinition-Patient.json").is_file():
            sys.exit(f"Missing core package at {package}")
        generate(package, release)

    # The R5 distribution also contains the common restricted XHTML schema.
    import xml.etree.ElementTree as ET
    schema = ET.parse(cache / "hl7.fhir.r5.core#5.0.0/package/xml/fhir-xhtml.xsd").getroot()
    named = {(e.tag.split("}")[-1], e.get("name")): e for e in schema}

    def rules(element):
        attributes, children, required = set(), set(), set()
        def visit(node):
            tag = node.tag.split("}")[-1]
            ref = node.get("ref")
            if tag == "attribute":
                attr = ref or node.get("name")
                attributes.add(attr)
                if node.get("use") == "required":
                    required.add(attr)
                return
            if tag == "element" and ref:
                children.add(ref)
                return
            if ref and tag in ["group", "attributeGroup"]:
                visit(named[tag, ref])
            if tag == "extension" and node.get("base"):
                visit(named["complexType", node.get("base")])
            for child in node:
                visit(child)
        visit(element)
        return [" ".join(sorted(items)) for items in (attributes, children, required)]
    rows = ["\t".join([key, *rules(el)]) for (tag, key), el in named.items() if tag == "element"]
    (DEST / "xhtml.tsv").write_text("\n".join(sorted(rows)) + "\n")
