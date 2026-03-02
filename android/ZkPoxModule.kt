package world.zerox1.node

import android.util.Log
import com.facebook.react.bridge.*
import kotlinx.coroutines.*
import org.json.JSONArray
import org.json.JSONObject

/**
 * ZkPoxModule — React Native bridge for ZK-PoX.
 *
 * Exposes GPS stats, proof generation, spoof analysis, and
 * credential listing to the React Native layer.
 *
 * JNI functions match the Rust bridge in zkpox-mobile/src/lib.rs:
 *   - generateProofNative(pointsJson: String, requestJson: String): String
 *   - verifyProofNative(resultJson: String): Int
 *   - analyzeSpoofRiskNative(pointsJson: String): String
 *
 * Integration:
 *   Copy to mobile/android/app/src/main/java/world/zerox1/node/ZkPoxModule.kt
 *   Register in MainApplication's ReactPackage list.
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
            pointsJson: String,
            requestJson: String,
        ): String

        @JvmStatic
        private external fun verifyProofNative(resultJson: String): Int

        @JvmStatic
        private external fun analyzeSpoofRiskNative(pointsJson: String): String
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
     * The Rust bridge runs anti-spoofing analysis before proof generation.
     * If GPS data appears spoofed, proof generation is refused.
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

                val resultStr = generateProofNative(
                    pointsJsonArray.toString(),
                    requestJson,
                )

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
     * Verify a ZK-PoX proof. Returns true if valid, false otherwise.
     */
    @ReactMethod
    fun verifyProof(resultJson: String, promise: Promise) {
        scope.launch {
            try {
                val code = verifyProofNative(resultJson)
                promise.resolve(code == 1)
            } catch (e: Exception) {
                Log.e(TAG, "verifyProof failed", e)
                promise.reject("VERIFY_ERROR", e.message, e)
            }
        }
    }

    /**
     * Analyze GPS data for spoofing indicators.
     * Returns JSON: { total_points, teleport_count, zero_noise_count,
     *   impossible_velocity_count, suspicion_score, verdict }
     */
    @ReactMethod
    fun analyzeSpoofRisk(days: Int, promise: Promise) {
        scope.launch {
            try {
                val now = System.currentTimeMillis() / 1000
                val timeStart = now - days.toLong() * 86_400
                val points = db.getPointsInTimeRange(timeStart, now)

                if (points.isEmpty()) {
                    promise.resolve("{\"verdict\":\"Clean\",\"total_points\":0}")
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

                val resultStr = analyzeSpoofRiskNative(pointsJsonArray.toString())
                promise.resolve(resultStr)
            } catch (e: Exception) {
                Log.e(TAG, "analyzeSpoofRisk failed", e)
                promise.reject("SPOOF_ERROR", e.message, e)
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
