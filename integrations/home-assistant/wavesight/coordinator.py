"""DataUpdateCoordinator that polls the WaveSight REST API."""

from __future__ import annotations

import logging
from datetime import timedelta
from typing import Any

import aiohttp
from homeassistant.core import HomeAssistant
from homeassistant.helpers.update_coordinator import (
    DataUpdateCoordinator,
    UpdateFailed,
)

_LOGGER = logging.getLogger(__name__)

SCAN_INTERVAL = timedelta(seconds=2)


class WaveSightCoordinator(DataUpdateCoordinator[dict[str, Any]]):
    """Periodic poller for the /api/v1/scene endpoint."""

    def __init__(self, hass: HomeAssistant, server_url: str) -> None:
        super().__init__(
            hass,
            _LOGGER,
            name="wavesight",
            update_interval=SCAN_INTERVAL,
        )
        self.server_url = server_url.rstrip("/")
        self.session = aiohttp.ClientSession()

    async def _async_update_data(self) -> dict[str, Any]:
        url = f"{self.server_url}/api/v1/scene"
        try:
            async with self.session.get(url, timeout=aiohttp.ClientTimeout(total=3)) as resp:
                resp.raise_for_status()
                return await resp.json()
        except Exception as err:
            raise UpdateFailed(f"wavesight poll failed: {err}") from err

    async def async_shutdown(self) -> None:
        await self.session.close()
