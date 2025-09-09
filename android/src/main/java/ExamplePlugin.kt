package com.plugin.plauth

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class PingArgs {
  var value: String? = null
}

@InvokeArg
class AuthArgs {
  var url: String? = null
  var callbackUrlScheme: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    @Command
    fun authenticate(invoke: Invoke) {
        val args = invoke.parseArgs(AuthArgs::class.java)

        val ret = JSObject()
        ret.put("success", implementation.authenticate(args.url ?: "", args.callbackUrlScheme ?: ""))
        invoke.resolve(ret)
    }
}
