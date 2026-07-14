#!/usr/bin/env python3

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("run_sushi_comparison.py")
SPEC = importlib.util.spec_from_file_location("run_sushi_comparison", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class ComparisonReportTests(unittest.TestCase):
    def test_identity_pairing_retains_duplicate_resource_ids(self):
        missing = ["Coverage/example", "Patient/example"]
        extra = [
            "CRDCoverage/example",
            "CRDPatient/example",
            "Medication/med0320",
        ]

        paired_missing, paired_extra = MODULE.find_resource_identity_pairs(missing, extra)

        self.assertEqual(paired_missing, set(missing))
        self.assertEqual(
            paired_extra,
            {"CRDCoverage/example", "CRDPatient/example"},
        )

    def test_json_report_preserves_counts_when_details_are_truncated(self):
        mismatched = [
            {
                "resource": f"Patient/{index}",
                "path": "$.active",
                "sushi": True,
                "rh_fsh": False,
            }
            for index in range(101)
        ]
        result = MODULE.ProjectResult(
            name="fixture",
            repo="fixture:test",
            path="fixture",
            fsh_files=1,
            sushi=MODULE.ToolResult(True, 0.0),
            rh_fsh=MODULE.ToolResult(True, 0.0),
            mismatched=mismatched,
        )

        report = MODULE.project_to_json(result)

        self.assertEqual(report["mismatch_count"], 101)
        self.assertEqual(len(report["mismatched"]), 100)

    def test_json_shape_diagnostics_include_category_and_family(self):
        reference = {
            "Parameters/example": {
                "resourceType": "Parameters",
                "id": "example",
                "parameter": [{"name": "input", "valueString": "value"}],
            }
        }
        actual = {
            "Parameters/example": {
                "resourceType": "Parameters",
                "id": "example",
                "parameter": {"name": "input", "valueString": "value"},
            }
        }

        _, _, mismatched = MODULE.compare_resources(reference, actual)

        self.assertEqual(mismatched[0]["path"], "$.parameter")
        self.assertEqual(mismatched[0]["category"], "json_shape")
        self.assertEqual(mismatched[0]["shape_family"], "parameters_part")

    def test_pairwise_artifacts_preserve_raw_and_normalized_views(self):
        sushi = {
            "Patient/example": {
                "resourceType": "Patient",
                "id": "example",
                "version": "1.0.0",
                "active": True,
            },
            "Observation/sushi-only": {
                "resourceType": "Observation",
                "id": "sushi-only",
            },
            "Observation/numeric": {
                "resourceType": "Observation",
                "id": "numeric",
                "valueQuantity": {"value": 1},
            },
        }
        rh_fsh = {
            "Patient/example": {
                "resourceType": "Patient",
                "id": "example",
                "version": "2.0.0",
                "active": True,
            },
            "Observation/rh-only": {
                "resourceType": "Observation",
                "id": "rh-only",
            },
            "Observation/numeric": {
                "resourceType": "Observation",
                "id": "numeric",
                "valueQuantity": {"value": 1.0},
            },
        }

        with tempfile.TemporaryDirectory() as tmp:
            artifacts = Path(tmp) / "fixture"
            MODULE.write_comparison_artifacts(artifacts, "fixture", sushi, rh_fsh)

            patient_filename = "Patient%2Fexample.json"
            self.assertTrue((artifacts / "sushi" / patient_filename).is_file())
            self.assertTrue((artifacts / "rh-fsh" / patient_filename).is_file())
            self.assertNotEqual(
                (artifacts / "sushi" / patient_filename).read_text(),
                (artifacts / "rh-fsh" / patient_filename).read_text(),
            )
            self.assertEqual(
                (artifacts / "normalized" / "sushi" / patient_filename).read_text(),
                (artifacts / "normalized" / "rh-fsh" / patient_filename).read_text(),
            )

            manifest = json.loads((artifacts / "manifest.json").read_text())
            entries = {item["resource"]: item for item in manifest["resources"]}
            self.assertEqual(entries["Patient/example"]["status"], "normalized-match")
            self.assertEqual(entries["Observation/sushi-only"]["status"], "sushi-only")
            self.assertEqual(entries["Observation/rh-only"]["status"], "rh-fsh-only")
            self.assertTrue(entries["Observation/numeric"]["raw_equal"])
            self.assertFalse(entries["Observation/numeric"]["serialized_equal"])
            self.assertTrue(
                (artifacts / "diffs" / "Patient%2Fexample.diff").is_file()
            )
            self.assertTrue(
                (artifacts / "diffs" / "Observation%2Fnumeric.diff").is_file()
            )

    def test_pairwise_artifacts_include_normalized_shape_diagnostics(self):
        sushi = {
            "Parameters/example": {
                "resourceType": "Parameters",
                "id": "example",
                "parameter": [{"name": "input"}],
            }
        }
        rh_fsh = {
            "Parameters/example": {
                "resourceType": "Parameters",
                "id": "example",
                "parameter": {"name": "input"},
            }
        }

        with tempfile.TemporaryDirectory() as tmp:
            artifacts = Path(tmp) / "fixture"
            MODULE.write_comparison_artifacts(artifacts, "fixture", sushi, rh_fsh)

            manifest = json.loads((artifacts / "manifest.json").read_text())
            entry = manifest["resources"][0]
            self.assertEqual(entry["path"], "$.parameter")
            self.assertEqual(entry["category"], "json_shape")
            self.assertEqual(entry["shape_family"], "parameters_part")


if __name__ == "__main__":
    unittest.main()
