package com.plugin.stripe.identity

import android.app.Activity
import android.content.Context
import androidx.core.util.Supplier
import app.tauri.plugin.JSObject
import com.getcapacitor.community.stripe.identity.IdentityVerificationSheetEvent
import com.google.android.gms.common.util.BiConsumer
import com.plugin.stripe.identity.models.PluginCall
import com.stripe.android.identity.IdentityVerificationSheet



class StripeIdentity(
    activitySupplier: Supplier<Activity>,
    notifyListenersFunction: BiConsumer<String, JSObject>,
    pluginLogTag: String
) : com.plugin.stripe.identity.models.Executor(
    activitySupplier,
    notifyListenersFunction,
    pluginLogTag,
    "StripeIdentityExecutor"
) {
    var verificationSheet: IdentityVerificationSheet? = null
    private val emptyObject: JSObject = JSObject()

    private var verificationId: String? = null
    private var ephemeralKeySecret: String? = null

    fun initialize(call: PluginCall) {
        call.resolve()
    }

    fun create(call: PluginCall) {
        verificationId = call.getString("verificationId", null)
        ephemeralKeySecret = call.getString("ephemeralKeySecret", null)

        if (verificationId == null || ephemeralKeySecret == null) {
            val errorText =
                "Invalid Params. This method require verificationId or ephemeralKeySecret."
            notifyListeners(
                IdentityVerificationSheetEvent.FailedToLoad.webEventName,
                JSObject().put("error", errorText)
            )
            call.reject(errorText)
            return
        }

        this.notifyListeners(IdentityVerificationSheetEvent.Loaded.webEventName, emptyObject)
        call.resolve()
    }

    fun present(call: PluginCall) {
        try {
            verificationSheet!!.present(
                verificationId!!,
                ephemeralKeySecret!!
            )
            Logger.info("Presented Identity Verification Sheet")
        } catch (ex: Exception) {
            call.reject(ex.localizedMessage, ex)
        }
    }

    fun onVerificationCompleted(bridge: Bridge, callbackId: String?) {
        val call: PluginCall = bridge.getSavedCall(callbackId)
        notifyListeners(IdentityVerificationSheetEvent.Completed.webEventName, emptyObject)
        call.resolve(
            JSObject().put(
                "identityVerificationResult",
                IdentityVerificationSheetEvent.Completed.webEventName
            )
        )
    }

    fun onVerificationCancelled(bridge: Bridge, callbackId: String?) {
        val call: PluginCall = bridge.getSavedCall(callbackId)
        notifyListeners(IdentityVerificationSheetEvent.Canceled.webEventName, emptyObject)
        call.resolve(
            JSObject().put(
                "identityVerificationResult",
                IdentityVerificationSheetEvent.Canceled.webEventName
            )
        )
    }

    fun onVerificationFailed(bridge: Bridge, callbackId: String?) {
        val call: PluginCall = bridge.getSavedCall(callbackId)
        notifyListeners(IdentityVerificationSheetEvent.Failed.webEventName, emptyObject)
        call.resolve(
            JSObject().put(
                "identityVerificationResult",
                IdentityVerificationSheetEvent.Failed.webEventName
            )
        )
    }
}
