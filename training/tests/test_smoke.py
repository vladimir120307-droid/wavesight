"""Smoke tests — verify the package imports and the CLI exposes commands."""

from click.testing import CliRunner

import wavesight_train
from wavesight_train.cli import main


def test_version_exposed():
    assert isinstance(wavesight_train.__version__, str)
    assert wavesight_train.__version__.count(".") >= 1


def test_cli_help():
    runner = CliRunner()
    result = runner.invoke(main, ["--help"])
    assert result.exit_code == 0
    assert "train" in result.output
    assert "export" in result.output
    assert "info" in result.output


def test_cli_info_subcommand_runs():
    runner = CliRunner()
    result = runner.invoke(main, ["info"])
    assert result.exit_code == 0
