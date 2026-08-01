// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

package com.github.hrzlgnm.mdns_browser.android_updater

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.provider.Settings
import androidx.core.content.FileProvider
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.io.File

@TauriPlugin
class AndroidUpdaterPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun checkInstallPermission(invoke: Invoke) {
        val allowed = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            activity.packageManager.canRequestPackageInstalls()
        } else {
            true
        }
        invoke.resolveObject(allowed)
    }

    @Command
    fun requestInstallPermission(invoke: Invoke) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            if (!activity.packageManager.canRequestPackageInstalls()) {
                val intent = Intent(Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES).apply {
                    data = Uri.parse("package:${activity.packageName}")
                }
                activity.startActivity(intent)
            }
        }
        invoke.resolve()
    }

    @Command
    fun installApk(invoke: Invoke) {
        val path = invoke.parseArgs(String::class.java)
        try {
            val file = File(path)
            if (!file.exists()) {
                invoke.resolveObject(false)
                return
            }

            val uri = FileProvider.getUriForFile(
                activity,
                "${activity.packageName}.fileprovider",
                file
            )

            val intent = Intent(Intent.ACTION_VIEW).apply {
                setDataAndType(uri, "application/vnd.android.package-archive")
                addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            }

            if (intent.resolveActivity(activity.packageManager) != null) {
                activity.startActivity(intent)
                invoke.resolveObject(true)
            } else {
                invoke.resolveObject(false)
            }
        } catch (e: Exception) {
            e.printStackTrace()
            invoke.reject(e.message, e, null)
        }
    }
}
