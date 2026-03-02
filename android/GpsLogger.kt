package world.zerox1.node

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.location.Location
import android.os.Looper
import android.util.Log
import androidx.core.content.ContextCompat
import com.google.android.gms.location.*
import java.security.MessageDigest

/**
 * GpsLogger — passive background GPS collector for ZK-PoX.
 *
 * Records a signed GPS point every [intervalMs] milliseconds into an
 * encrypted local database. Each point is signed with the agent's
 * Ed25519 identity key so proofs can reference authenticated data.
 *
 * Integration:
 *   Copy to mobile/android/app/src/main/java/world/zerox1/node/GpsLogger.kt
 *   Add to build.gradle: implementation("com.google.android.gms:play-services-location:21.3.0")
 *   Start from NodeService.onCreate():
 *     gpsLogger = GpsLogger(applicationContext, identityKeyBytes, GpsDatabase(applicationContext))
 *     gpsLogger.start()
 */
class GpsLogger(
    private val context: Context,
    private val identityKey: ByteArray,
    private val db: GpsDatabase,
    private val intervalMs: Long = 5 * 60 * 1000L, // 5 minutes
) {
    companion object {
        private const val TAG = "GpsLogger"
        private const val RETENTION_DAYS = 365L
    }

    private var fusedClient: FusedLocationProviderClient? = null
    private var callback: LocationCallback? = null

    fun start() {
        if (!hasLocationPermission()) {
            Log.w(TAG, "Missing location permission — GPS logger not started.")
            return
        }

        fusedClient = LocationServices.getFusedLocationProviderClient(context)

        val request = LocationRequest.Builder(Priority.PRIORITY_BALANCED_POWER_ACCURACY, intervalMs)
            .setMinUpdateIntervalMillis(intervalMs / 2)
            .setWaitForAccurateLocation(false)
            .build()

        callback = object : LocationCallback() {
            override fun onLocationResult(result: LocationResult) {
                val loc = result.lastLocation ?: return
                recordPoint(loc)
            }
        }

        try {
            fusedClient?.requestLocationUpdates(request, callback!!, Looper.getMainLooper())
            Log.i(TAG, "GPS logger started (interval=${intervalMs / 1000}s)")
        } catch (e: SecurityException) {
            Log.e(TAG, "SecurityException starting GPS logger: $e")
        }

        pruneOldPoints()
    }

    fun stop() {
        callback?.let { fusedClient?.removeLocationUpdates(it) }
        callback = null
        fusedClient = null
        Log.i(TAG, "GPS logger stopped.")
    }

    private fun recordPoint(loc: Location) {
        val lat = loc.latitude
        val lng = loc.longitude
        val accuracy = loc.accuracy
        val timestamp = loc.time / 1000 // seconds

        val signature = signPoint(lat, lng, timestamp)

        db.insertPoint(lat, lng, accuracy, timestamp, signature)

        Log.d(TAG, "GPS point recorded: ($lat, $lng) accuracy=${accuracy}m")
    }

    /**
     * Sign SHA-256(lat_le || lng_le || timestamp_le) with the identity key.
     *
     * NOTE: This is a placeholder using HMAC-SHA256 with the identity key as secret.
     * In production, replace with proper Ed25519 signing using the agent's keypair
     * (e.g. via the zerox1-node Rust binary or a JNI call to ed25519-dalek).
     */
    private fun signPoint(lat: Double, lng: Double, timestamp: Long): ByteArray {
        val message = buildMessage(lat, lng, timestamp)
        val mac = javax.crypto.Mac.getInstance("HmacSHA256")
        val keySpec = javax.crypto.spec.SecretKeySpec(identityKey, "HmacSHA256")
        mac.init(keySpec)
        return mac.doFinal(message)
    }

    private fun buildMessage(lat: Double, lng: Double, timestamp: Long): ByteArray {
        val md = MessageDigest.getInstance("SHA-256")
        md.update(java.nio.ByteBuffer.allocate(8).order(java.nio.ByteOrder.LITTLE_ENDIAN).putDouble(lat).array())
        md.update(java.nio.ByteBuffer.allocate(8).order(java.nio.ByteOrder.LITTLE_ENDIAN).putDouble(lng).array())
        md.update(java.nio.ByteBuffer.allocate(8).order(java.nio.ByteOrder.LITTLE_ENDIAN).putLong(timestamp).array())
        return md.digest()
    }

    private fun pruneOldPoints() {
        val cutoff = System.currentTimeMillis() / 1000 - RETENTION_DAYS * 86_400
        val deleted = db.pruneOlderThan(cutoff)
        if (deleted > 0) {
            Log.i(TAG, "Pruned $deleted GPS points older than $RETENTION_DAYS days")
        }
    }

    private fun hasLocationPermission(): Boolean =
        ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_FINE_LOCATION) ==
            PackageManager.PERMISSION_GRANTED ||
        ContextCompat.checkSelfPermission(context, Manifest.permission.ACCESS_COARSE_LOCATION) ==
            PackageManager.PERMISSION_GRANTED
}
