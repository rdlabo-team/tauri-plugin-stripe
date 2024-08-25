package com.plugin.stripe.identity

import android.app.Activity
import android.content.ContentResolver
import android.content.res.Resources
import android.net.Uri
import androidx.appcompat.app.AppCompatActivity
import app.tauri.annotation.Command
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.stripe.android.identity.IdentityVerificationSheet

@CapacitorPlugin(name = "StripeIdentity")
class StripeIdentityPlugin(private val activity: AppCompatActivity): Plugin(activity) {
    private var identityVerificationCallbackId: String? = null

    private val implementation: StripeIdentity = StripeIdentity(
        this::activity,
        this::trigger,
        getLogTag()
    )

    fun load() {
        val resources: Resources = activity.resources
        val resourceId =
            resources.getIdentifier("ic_launcher", "mipmap", activity.packageName)
        val icon = Uri.Builder()
            .scheme(ContentResolver.SCHEME_ANDROID_RESOURCE)
            .authority(resources.getResourcePackageName(resourceId))
            .appendPath(resources.getResourceTypeName(resourceId))
            .appendPath(resources.getResourceEntryName(resourceId))
            .build()

        implementation.verificationSheet =
            IdentityVerificationSheet.create(
                activity,
                IdentityVerificationSheet.Configuration(icon)
            ) { verificationFlowResult ->
                // handle verificationResult
                if (verificationFlowResult is IdentityVerificationSheet.VerificationFlowResult.Completed) {
                    // The user has completed uploading their documents.
                    // Let them know that the verification is processing.
                    implementation.onVerificationCompleted(bridge, identityVerificationCallbackId)
                } else if (verificationFlowResult is IdentityVerificationSheet.VerificationFlowResult.Canceled) {
                    // The user did not complete uploading their documents.
                    // You should allow them to try again.
                    implementation.onVerificationCancelled(bridge, identityVerificationCallbackId)
                } else if (verificationFlowResult is IdentityVerificationSheet.VerificationFlowResult.Failed) {
                    // If the flow fails, you should display the localized error
                    // message to your user using throwable.getLocalizedMessage()
                    implementation.onVerificationFailed(bridge, identityVerificationCallbackId)
                }
            }
    }

    @Command
    fun initialize(call: Invoke) {
        implementation.initialize(call)
    }

    @Command
    fun create(call: Invoke) {
        implementation.create(call)
    }

    @Command
    fun present(call: Invoke) {
        identityVerificationCallbackId = call.getCallbackId()
        bridge.saveCall(call)

        implementation.present(call)
    }
}
