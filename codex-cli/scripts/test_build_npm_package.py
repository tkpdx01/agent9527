import json
import tempfile
import unittest
from pathlib import Path

import build_npm_package


class BuildNpmPackageTest(unittest.TestCase):
    def test_root_package_includes_bundled_language_pack(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            staging_dir = Path(temp_dir)

            build_npm_package.stage_sources(staging_dir, "1.2.3", "codex")

            language_dir = staging_dir / "languages" / "zh-CN"
            self.assertEqual(
                sorted(path.name for path in language_dir.iterdir()),
                ["LICENSE", "manifest.json", "messages.ftl"],
            )
            package_json = json.loads((staging_dir / "package.json").read_text())
            self.assertIn("languages", package_json["files"])
            self.assertIn("lib", package_json["files"])
            self.assertTrue((staging_dir / "lib" / "product-policy.js").is_file())


if __name__ == "__main__":
    unittest.main()
