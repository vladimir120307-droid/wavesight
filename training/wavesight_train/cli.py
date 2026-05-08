"""Command-line entry point for WaveSight training utilities."""

from __future__ import annotations

import click


@click.group()
@click.version_option(package_name="wavesight-train")
def main() -> None:
    """WaveSight training CLI."""


@main.command()
@click.option("--config", type=click.Path(exists=True), required=True)
def train(config: str) -> None:
    """Train a head defined by a YAML config."""
    click.echo(f"train: {config} (Lightning runner lands in M2)")


@main.command()
@click.option("--checkpoint", type=click.Path(exists=True), required=True)
@click.option("--out", type=click.Path(), required=True)
def export(checkpoint: str, out: str) -> None:
    """Export a Lightning checkpoint to safetensors for Candle inference."""
    click.echo(f"export: {checkpoint} -> {out} (safetensors writer lands in M2)")


@main.command()
@click.option("--dataset", default="wavesight/csi-v0")
def info(dataset: str) -> None:
    """Print metadata about a HuggingFace-hosted dataset."""
    click.echo(f"info: {dataset} (HuggingFace fetch lands when first dataset is published)")
