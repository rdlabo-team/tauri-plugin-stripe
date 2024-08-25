package com.plugin.stripe.identity.models

import android.app.Activity
import androidx.core.util.Supplier
import androidx.lifecycle.ViewModelProvider.NewInstanceFactory.Companion.instance
import app.tauri.Logger
import app.tauri.annotation.InvokeArg
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.PluginResult
import com.google.android.gms.common.util.BiConsumer

abstract class Executor(
    protected val activitySupplier: Supplier<Activity>,
    protected var trigger: BiConsumer<String, JSObject>,
    pluginLogTag: String,
    executorTag: String
) {
    protected val logTag: String = "$pluginLogTag|$executorTag"

    // Eventually we can change the notification directly here!
    protected fun notifyListeners(eventName: String, data: JSObject) {
        trigger.accept(eventName, data)
    }
}

@InvokeArg
internal class Args {
    lateinit var requiredArg: String
    var allowEdit: Boolean = false
    var quality: Int = 100
}

class PluginCall(val invoke: Invoke) {
    private fun getValueFromParsedArgs(args: Args, key: String) {
        return when (args) {
            key -> args[key]
        }
    }

    fun getBool(key: String) {
        val args = invoke.parseArgs(Args::class.java);
        return getValueFromParsedArgs(args, key)
    }

    fun getBool(key: String, defaultValue: Boolean) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! Bool? {
                    return value
                } else {
                    return defaultValue
                }
                } catch {
                    return defaultValue
                }
            }

    fun getString(key: String) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                return try getValueFromParsedArgs(parsedArgs: args, key: key) as! String?
                } catch {
                    return nil
                }
            }

    fun getString(key: String, defaultValue: String){
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! String? {
                    return value
                } else {
                    return defaultValue
                }
                } catch {
                    return defaultValue
                }
            }


    fun getInt(key: String) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                return try getValueFromParsedArgs(parsedArgs: args, key: key) as! Int?
                } catch {
                    return nil
                }
            }

    fun getInt(key: String, defaultValue: Int) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! Int? {
                    return value
                } else {
                    return defaultValue
                }
                } catch {
                    return defaultValue
                }
            }

    fun getObject(key: String) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                return try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSObject?
                } catch {
                    return nil
                }
            }

    fun getObject(key: String, defaultValue: JSObject) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSObject? {
                    return value
                } else {
                    return defaultValue
                }
                } catch {
                    return defaultValue
                }
            }

    fun getArray(key: String) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                return try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSArray?
                } catch {
                    return nil
                }
            }

    fun getArray(key: String, defaultValue: JSArray) {
        do {
            val args = try self.invoke.parseArgs(Args::class.java);
                if let value = try getValueFromParsedArgs(parsedArgs: args, key: key) as! JSArray? {
                    return value
                } else {
                    return defaultValue
                }
                } catch {
                    return defaultValue
                }
            }

    fun getArray<T>(key: String, ofType: T.Type) {
        return getArray(key) as? [T]
    }

    fun resolve(data: JSObject?) {
        invoke.resolve(data)
    }

    fun resolve() {
        invoke.resolve()
    }

    fun reject(msg: String?, code: String?, ex: Exception?, data: JSObject?) {
        invoke.reject(msg, code, ex, data);
    }

    fun reject(msg: String?, ex: Exception?, data: JSObject?) {
        invoke.reject(msg, null, ex, data)
    }

    fun reject(msg: String?, code: String?, data: JSObject?) {
        invoke.reject(msg, code, null, data)
    }

    fun reject(msg: String?, code: String?, ex: Exception?) {
        invoke.reject(msg, code, ex, null)
    }

    fun reject(msg: String?, data: JSObject?) {
        invoke.reject(msg, null, null, data)
    }

    fun reject(msg: String?, ex: Exception?) {
        invoke.reject(msg, null, ex, null)
    }

    fun reject(msg: String?, code: String?) {
        invoke.reject(msg, code, null, null)
    }

    fun reject(msg: String?) {
        invoke.reject(msg, null, null, null)
    }
}