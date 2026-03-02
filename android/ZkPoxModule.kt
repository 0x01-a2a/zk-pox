package world.zerox1.node

import android.util.Log
import com.facebook.react.bridge.*
import kotlinx.coroutines.*
import org.json.JSONArray
import org.json.JSONObject

/**
 * ZkPoxModule — React Native bridge for ZK-PoX.
 *
 * Exposes GPS stats, proof generation, and credential listing
 * to the React Native layer.
 *
 * Integration:
 *   Copy to mobile/android/app/src/main/java/world/zerox1/node/ZkPoxModule.kt
 *   Register in MainApplication's ReactPackage list.
 *   Load native library: System.loadLibrary("zkpox_mobile") in companion init block.
 */
class ZkPoxModule(reactContext: ReactApplicationContext) :
    ReactContextBaseJavaModule(reactContext) {

    companion object {
        private const val TAG = "ZkPoxModule"

        init {
            try {
                System.loadLibrary("zkpox_mobile")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load zkpox_mobile native library: $e")
            }
        }

        @JvmStatic
        private external fun generateProofNative(
            pointsJson: ByteArray,
            requestJson: ByteArray,
            outBuf: ByteArray,
        ): Int

        @JvmStatic
        private external fun verifyProofNative(resultJson: ByteArray): Int
    }

    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    private val db: GpsDatabase by lazy { GpsDatabase(reactApplicationContext) }

    override fun getName(): String = "ZkPoxModule"

    /**
     * Get GPS collection statistics.
     * Returns: { totalPoints, oldestTimestamp, newestTimestamp, daysTracked }
     */
    @ReactMethod
    fun getGpsStats(promise: Promise) {
        scope.launch {
            try {
                val stats = db.getStats()
                val result = Arguments.createMap().apply {
                    putInt("totalPoints", stats.totalPoints.toInt())
                    stats.oldestTimestamp?.let { putDouble("oldestTimestamp", it.toDouble()) }
                    stats.newestTimestamp?.let { putDouble("newestTimestamp", it.toDouble()) }
                    putInt("daysTracked", stats.daysTracked)
                }
                promise.resolve(result)
            } catch (e: Exception) {
                promise.reject("GPS_STATS_ERROR", e.message, e)
            }
        }
    }

    /**
     * Generate a ZK-PoX proof from local GPS history.
     *
     * @param requestJson JSON string with ProofRequest fields:
     *   { claim_type, center_lat, center_lng, radius_m, time_start, time_end, min_count }
     */
    @ReactMethod
    fun generateProof(requestJson: String, promise: Promise) {
        scope.launch {
            try {
                val req = JSONObject(requestJson)
                val timeStart = req.getLong("time_start")
                val timeEnd = req.getLong("time_end")

                val points = db.getPointsInTimeRange(timeStart, timeEnd)
                if (points.isEmpty()) {
                    promise.reject("NO_POINTS", "No GPS points in the requested time range")
                    return@launch
                }

                val pointsJsonArray = JSONArray()
                for (p in points) {
                    pointsJsonArray.put(JSONObject().apply {
                        put("lat", p.lat)
                        put("lng", p.lng)
                        put("timestamp", p.timestamp)
                        put("accuracy", p.accuracy.toDouble())
                        put("signature", android.util.Base64.encodeToString(
                            p.signature, android.util.Base64.NO_WRAP
                        ))
                    })
                }

                val pointsBytes = pointsJsonArray.toString().toByteArray(Charsets.UTF_8)
                val requestBytes = requestJson.toByteArray(Charsets.UTF_8)
                val outBuf = ByteArray(64 * 1024) // 64 KB buffer

                val written = generateProofNative(pointsBytes, requestBytes, outBuf)

                if (written <= 0) {
                    promise.reject("PROOF_ERROR", "Native proof generation returned empty result")
                    return@launch
                }

                val resultStr = String(outBuf, 0, written, Charsets.UTF_8)
                if (resultStr.startsWith("ERROR:")) {
                    promise.reject("PROOF_ERROR", resultStr.removePrefix("ERROR:"))
                    return@launch
                }

                promise.resolve(resultStr)
            } catch (e: Exception) {
                Log.e(TAG, "generateProof failed", e)
                promise.reject("PROOF_ERROR", e.message, e)
            }
        }
    }

    /**
     * Count nights near a specific location over the past N days.
     */
    @ReactMethod
    fun countNightsNear(
        centerLat: Double,
        centerLng: Double,
        radiusM: Int,
        days: Int,
        promise: Promise,
    ) {
        scope.launch {
            try {
                val count = db.countNightsNear(centerLat, centerLng, radiusM, days)
                promise.resolve(count)
            } catch (e: Exception) {
                promise.reject("COUNT_ERROR", e.message, e)
            }
        }
    }

    override fun onCatalystInstanceDestroy() {
        scope.cancel()
        super.onCatalystInstanceDestroy()
    }
}
