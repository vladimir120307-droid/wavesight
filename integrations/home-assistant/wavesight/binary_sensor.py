"""WaveSight binary_sensor entities — one per node."""

from __future__ import annotations

from typing import Any

from homeassistant.components.binary_sensor import (
    BinarySensorDeviceClass,
    BinarySensorEntity,
)
from homeassistant.config_entries import ConfigEntry
from homeassistant.core import HomeAssistant
from homeassistant.helpers.entity_platform import AddEntitiesCallback

from . import DOMAIN


async def async_setup_entry(
    hass: HomeAssistant,
    entry: ConfigEntry,
    async_add_entities: AddEntitiesCallback,
) -> None:
    coordinator = hass.data[DOMAIN][entry.entry_id]
    nodes = {item["node"] for item in coordinator.data.get("presence", [])}
    async_add_entities(WaveSightPresence(coordinator, node) for node in nodes)


class WaveSightPresence(BinarySensorEntity):
    """Binary presence sensor for one WaveSight node."""

    _attr_device_class = BinarySensorDeviceClass.OCCUPANCY
    _attr_has_entity_name = True
    _attr_name = "Presence"

    def __init__(self, coordinator: Any, node: str) -> None:
        self.coordinator = coordinator
        self._node = node
        self._attr_unique_id = f"wavesight_{node}_presence"

    @property
    def is_on(self) -> bool:
        for item in self.coordinator.data.get("presence", []):
            if item["node"] == self._node:
                return bool(item["present"])
        return False

    @property
    def extra_state_attributes(self) -> dict[str, Any]:
        for item in self.coordinator.data.get("presence", []):
            if item["node"] == self._node:
                return {
                    "energy": item.get("energy"),
                    "uncertainty": item.get("uncertainty"),
                }
        return {}
