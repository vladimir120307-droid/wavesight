#pragma once

#include <stddef.h>

#include "csi_capture.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Encode an array of CSI samples into a JSON message ready for the
 * WaveSight server. Returns the number of bytes written (excluding the
 * terminating NUL), or 0 on error.
 *
 * The output schema matches `wavesight.csi.v1`:
 *
 *     { "node":   "<name>",
 *       "batch":  [ { "seq": u32, "ts_us": u64, "rssi": i8,
 *                     "ch":  u8,  "bw":   u8,   "iq":  "<base64>" },
 *                   ... ] }
 */
size_t frame_encoder_encode_batch(const ws_csi_sample_t *samples,
                                  size_t                 count,
                                  char                  *out,
                                  size_t                 out_capacity);

#ifdef __cplusplus
}
#endif
