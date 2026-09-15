// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT

import { isTauri } from '@tauri-apps/api/core'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { attachLogger, debug, error, info, LogLevel, warn } from '@tauri-apps/plugin-log'

// Bridges console.* and the backend logger in both directions:
//
// - console.debug/info/warn/error keep printing locally and are additionally
//   forwarded to the backend, so frontend messages reach stdout and the
//   --log-to-file output with their level intact.
// - Backend records (TargetKind::Webview) are printed to the devtools console;
//   without this listener the Webview target has no effect.
//
// The listener prints through the captured originals, so backend echoes of
// forwarded frontend messages terminate instead of looping. Those echoes do
// show each forwarded message a second time in devtools.
export async function initLogger(): Promise<UnlistenFn> {
  const noop: UnlistenFn = () => {}
  if (!isTauri()) return noop
  try {
    const original = {
      debug: console.debug.bind(console),
      info: console.info.bind(console),
      warn: console.warn.bind(console),
      error: console.error.bind(console),
    } as const
    type ConsoleMethod = keyof typeof original
    const forwarders = { debug, info, warn, error } as const
    const methods: Array<ConsoleMethod> = ['debug', 'info', 'warn', 'error']
    for (const method of methods) {
      console[method] = (...args: Array<unknown>) => {
        original[method](...args)
        void forwarders[method](formatArgs(args))
      }
    }
    return await attachLogger(({ level, message }) => {
      switch (level) {
        case LogLevel.Trace:
        case LogLevel.Debug:
          original.debug(message)
          break
        case LogLevel.Info:
          original.info(message)
          break
        case LogLevel.Warn:
          original.warn(message)
          break
        case LogLevel.Error:
          original.error(message)
          break
      }
    })
  } catch {
    return noop
  }
}

function formatArgs(args: Array<unknown>): string {
  return args.map(formatArg).join(' ')
}

function formatArg(arg: unknown): string {
  if (typeof arg === 'string') return arg
  if (arg instanceof Error) return arg.stack ?? `${arg.name}: ${arg.message}`
  try {
    return JSON.stringify(arg) ?? String(arg)
  } catch {
    return String(arg)
  }
}
