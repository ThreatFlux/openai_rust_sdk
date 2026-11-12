"""Exercise publication ordering on GitHub without registry credentials."""

import unittest
import pathlib
import tempfile
import sys
import json
import secrets
from unittest import mock
import container_publish_policy as POLICY
from container_publish_policy import aliases, validate_release, require_main_ancestry

OLD = "a" * 40
CURRENT = "b" * 40
NEW = CURRENT
FAKE_TOKEN = secrets.token_hex(16)


def tag(name, sha):
    return {"name": name, "commit": {"sha": sha}}


TAGS = [
    {"name": "v0.9.9", "commit": {"sha": OLD}},
    {"name": "v0.10.0", "commit": {"sha": CURRENT}},
    {"name": "v1.0.0-rc.1", "commit": {"sha": CURRENT}},
]


class PublicationPolicyTests(unittest.TestCase):
    def test_historical_retry_never_rewinds_latest_or_major(self):
        policy = aliases("refs/tags/v0.9.9", OLD, CURRENT, TAGS)
        self.assertEqual(policy, {"runtime": ["0.9"], "base": False})

    def test_current_release_advances_only_its_stable_lines(self):
        policy = aliases("refs/tags/v0.10.0", CURRENT, CURRENT, TAGS)
        self.assertEqual(
            policy, {"runtime": ["main", "latest", "0.10", "0"], "base": True}
        )

    def test_obsolete_main_build_cannot_rewind_any_alias(self):
        self.assertEqual(
            aliases("refs/heads/main", OLD, CURRENT, TAGS),
            {"runtime": [], "base": False},
        )

    def test_current_main_retains_existing_tracking_aliases(self):
        self.assertEqual(
            aliases("refs/heads/main", CURRENT, CURRENT, TAGS),
            {"runtime": ["main", "latest"], "base": True},
        )

    def test_prerelease_never_promotes_stable_aliases(self):
        self.assertEqual(
            aliases("refs/tags/v1.0.0-rc.1", CURRENT, CURRENT, TAGS),
            {"runtime": [], "base": False},
        )

    def test_tag_with_a_different_source_cannot_promote(self):
        self.assertEqual(
            aliases("refs/tags/v0.10.0", OLD, CURRENT, TAGS),
            {"runtime": [], "base": False},
        )

    def test_numeric_version_order_and_past_major_lines(self):
        tags = TAGS + [{"name": "v1.0.0", "commit": {"sha": CURRENT}}]
        self.assertEqual(
            aliases("refs/tags/v0.10.0", CURRENT, CURRENT, tags)["runtime"],
            ["main", "latest", "0.10", "0"],
        )

    def test_tag_must_match_compiled_version(self):
        validate_release("refs/tags/v0.10.0", "0.10.0")
        with self.assertRaises(ValueError):
            validate_release("refs/tags/v0.9.9", "0.10.0")

    def test_detached_source_is_rejected(self):
        for status in ["behind", "diverged"]:
            with self.assertRaises(ValueError):
                require_main_ancestry(OLD, CURRENT, status)
        require_main_ancestry(OLD, CURRENT, "ahead")
        require_main_ancestry(CURRENT, CURRENT, "identical")

    def test_current_stable_retry_cannot_rewind_newer_main_base(self):
        policy = aliases("refs/tags/v0.10.0", CURRENT, "c" * 40, TAGS)
        self.assertFalse(policy["base"])
        self.assertNotIn("latest", policy["runtime"])

    def test_incomplete_sha_is_rejected(self):
        with self.assertRaises(ValueError):
            aliases("refs/heads/main", "abc1234", CURRENT, TAGS)

    @mock.patch.dict(POLICY.os.environ, {"GH_TOKEN": FAKE_TOKEN})
    @mock.patch.object(POLICY.http.client, "HTTPSConnection")
    def test_github_api_uses_fixed_https_authority_and_all_tag_pages(self, connect):
        first_page = [tag(f"v0.1.{number}", OLD) for number in range(100)]
        last_page = [tag("v0.1.100", NEW)]
        responses = []
        for page in (first_page, last_page):
            response = mock.Mock(status=200)
            response.read.return_value = json.dumps(page).encode()
            responses.append(response)
        connection = connect.return_value
        connection.getresponse.side_effect = responses
        result = POLICY.api("repos/ThreatFlux/example/tags?per_page=100", pages=True)
        self.assertEqual(result, first_page + last_page)
        self.assertEqual(
            connect.call_args_list, [mock.call("api.github.com", timeout=60)] * 2
        )
        paths = [call.args[1] for call in connection.request.call_args_list]
        self.assertEqual(
            paths,
            [
                "/repos/ThreatFlux/example/tags?per_page=100&page=1",
                "/repos/ThreatFlux/example/tags?per_page=100&page=2",
            ],
        )
        for call in connection.request.call_args_list:
            self.assertEqual(call.args[0], "GET")
            self.assertEqual(
                call.kwargs["headers"]["Authorization"], f"Bearer {FAKE_TOKEN}"
            )
        self.assertEqual(connection.close.call_count, 2)

    @mock.patch.dict(POLICY.os.environ, {"GH_TOKEN": FAKE_TOKEN})
    @mock.patch.object(POLICY.http.client, "HTTPSConnection")
    def test_github_api_failure_refuses_alias_decision(self, connect):
        response = mock.Mock(status=403)
        connect.return_value.getresponse.return_value = response
        with self.assertRaisesRegex(RuntimeError, "HTTP 403"):
            POLICY.api("repos/ThreatFlux/example/commits/main")
        response.read.assert_not_called()
        connect.return_value.close.assert_called_once()

    def test_virtual_workspace_uses_selected_binary_member_version(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            manifest = root / "Cargo.toml"
            manifest.write_text('[workspace]\nmembers = ["app", "library"]\n')
            for name, version in (("app", "1.2.3"), ("library", "9.0.0")):
                member = root / name
                member.mkdir()
                (member / "Cargo.toml").write_text(
                    f'[package]\nname = "{name}"\nversion = "{version}"\n'
                )
            version = POLICY.selected_version(
                manifest, "app", root / "library/Cargo.toml"
            )
            self.assertEqual(version, "1.2.3")
            POLICY.validate_release("refs/tags/v1.2.3", version)
            with self.assertRaises(ValueError):
                POLICY.validate_release("refs/tags/v9.0.0", version)
            with self.assertRaises(ValueError):
                POLICY.selected_version(manifest, "missing", manifest)

    def test_selected_member_can_inherit_workspace_version(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            manifest = root / "Cargo.toml"
            manifest.write_text(
                '[workspace]\nmembers = ["app"]\n[workspace.package]\nversion = "1.2.3"\n'
            )
            member = root / "app"
            member.mkdir()
            (member / "Cargo.toml").write_text(
                '[package]\nname = "app"\nversion.workspace = true\n'
            )
            self.assertEqual(
                POLICY.selected_version(manifest, "app", manifest), "1.2.3"
            )

    @mock.patch.dict(POLICY.os.environ, {"GITHUB_REF": "refs/heads/main"})
    @mock.patch.object(POLICY, "selected_version")
    @mock.patch.object(sys, "argv", ["container_publish_policy.py", "--validate"])
    def test_non_tag_build_does_not_require_a_root_package_version(self, version):
        POLICY.main()
        version.assert_not_called()


if __name__ == "__main__":
    unittest.main()
