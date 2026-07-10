#!/usr/bin/env python3

import importlib.util
import sys
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


if __name__ == "__main__":
    unittest.main()
