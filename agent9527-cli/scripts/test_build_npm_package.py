import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import build_npm_package


class BuildNpmPackageTest(unittest.TestCase):
    def test_root_package_includes_bundled_language_pack(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            staging_dir = Path(temp_dir)

            build_npm_package.stage_sources(staging_dir, "1.2.3", "agent9527")

            language_dir = staging_dir / "languages" / "zh-CN"
            self.assertEqual(
                sorted(path.name for path in language_dir.iterdir()),
                ["LICENSE", "manifest.json", "messages.ftl"],
            )
            package_json = json.loads((staging_dir / "package.json").read_text())
            self.assertIn("languages", package_json["files"])
            self.assertIn("lib", package_json["files"])
            self.assertTrue((staging_dir / "lib" / "product-policy.js").is_file())

    def test_npm_pack_uses_the_resolved_executable(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            staging_dir = root / "staging"
            staging_dir.mkdir()
            output_path = root / "package.tgz"
            npm_executable = "C:\\node\\npm.cmd"

            def fake_check_output(command: list[str], **_kwargs: object) -> str:
                self.assertEqual(command[0], npm_executable)
                pack_dir = Path(command[command.index("--pack-destination") + 1])
                (pack_dir / "packed.tgz").write_bytes(b"packed")
                return json.dumps([{"filename": "packed.tgz"}])

            with (
                patch.object(
                    build_npm_package.shutil,
                    "which",
                    return_value=npm_executable,
                ),
                patch.object(
                    build_npm_package.subprocess,
                    "check_output",
                    side_effect=fake_check_output,
                ),
            ):
                result = build_npm_package.run_npm_pack(staging_dir, output_path)

            self.assertEqual(result, output_path.resolve())
            self.assertEqual(output_path.read_bytes(), b"packed")


if __name__ == "__main__":
    unittest.main()
