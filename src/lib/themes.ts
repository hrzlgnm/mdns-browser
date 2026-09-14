// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT
// Theme presets ported from zux. The types live here (rather than in
// types.ts) because that file is generated from the Rust boundary types.

export type PresetName =
  | 'dark'
  | 'light'
  | 'solarized-dark'
  | 'solarized-light'
  | 'catppuccin-latte'
  | 'catppuccin-frappe'
  | 'catppuccin-macchiato'
  | 'catppuccin-mocha'
  | 'dracula'
  | 'nord'
  | 'tokyo-night'
  | 'gruvbox-dark'

export type ThemeName = PresetName | 'system'

export interface ThemeColors {
  bgPrimary: string
  bgSecondary: string
  bgTertiary: string
  borderPrimary: string
  borderAccent: string
  textPrimary: string
  textSecondary: string
  textMuted: string
  textTertiary: string
  textPlaceholder: string
  accent: string
  accentHover: string
  serviceTypeBg: string
  serviceTypeBorder: string
  serviceTypeFont: string
  instanceBg: string
  instanceBorder: string
  instanceFont: string
  hostBg: string
  hostBorder: string
  hostFont: string
  addressBg: string
  addressBorder: string
  addressFont: string
  edgeTypeInstance: string
  edgeInstanceHost: string
  edgeHostAddress: string
  offlineBg: string
  offlineBorder: string
  offlineFont: string
}

export interface ThemePreset {
  name: PresetName
  label: string
  colors: ThemeColors
}

const dark: ThemePreset = {
  name: 'dark',
  label: 'Dark',
  colors: {
    bgPrimary: '#292929',
    bgSecondary: '#1f1f1f',
    bgTertiary: '#141414',
    borderPrimary: '#666666',
    borderAccent: '#479ef5',
    textPrimary: '#ffffff',
    textSecondary: '#d6d6d6',
    textMuted: '#adadad',
    textTertiary: '#999999',
    textPlaceholder: '#5c5c5c',
    accent: '#479ef5',
    accentHover: '#62abf5',
    serviceTypeBg: '#5cbeea',
    serviceTypeBorder: '#26a8e3',
    serviceTypeFont: '#f2f6f7',
    instanceBg: '#6cda88',
    instanceBorder: '#3bce60',
    instanceFont: '#f3f7f4',
    hostBg: '#daa36c',
    hostBorder: '#ce843b',
    hostFont: '#f7f5f3',
    addressBg: '#b56cda',
    addressBorder: '#9c3bce',
    addressFont: '#f5f3f7',
    edgeTypeInstance: '#90a4ae',
    edgeInstanceHost: '#78909c',
    edgeHostAddress: '#b39ddb',
    offlineBg: '#616161',
    offlineBorder: '#424242',
    offlineFont: '#9e9e9e',
  },
}

const light: ThemePreset = {
  name: 'light',
  label: 'Light',
  colors: {
    bgPrimary: '#ffffff',
    bgSecondary: '#fafafa',
    bgTertiary: '#f5f5f5',
    borderPrimary: '#d1d1d1',
    borderAccent: '#0f6cbd',
    textPrimary: '#242424',
    textSecondary: '#424242',
    textMuted: '#616161',
    textTertiary: '#707070',
    textPlaceholder: '#bdbdbd',
    accent: '#0f6cbd',
    accentHover: '#115ea3',
    serviceTypeBg: '#3976b2',
    serviceTypeBorder: '#2a5784',
    serviceTypeFont: '#24384c',
    instanceBg: '#43a85c',
    instanceBorder: '#327c44',
    instanceFont: '#274930',
    hostBg: '#a87643',
    hostBorder: '#7c5732',
    hostFont: '#493827',
    addressBg: '#8643a8',
    addressBorder: '#63327c',
    addressFont: '#3e2749',
    edgeTypeInstance: '#9e9e9e',
    edgeInstanceHost: '#bdbdbd',
    edgeHostAddress: '#7b1fa2',
    offlineBg: '#bdbdbd',
    offlineBorder: '#9e9e9e',
    offlineFont: '#757575',
  },
}

const solarizedDark: ThemePreset = {
  name: 'solarized-dark',
  label: 'Solarized Dark',
  colors: {
    bgPrimary: '#002b36',
    bgSecondary: '#073642',
    bgTertiary: '#002b36',
    borderPrimary: '#073642',
    borderAccent: '#268bd2',
    textPrimary: '#839496',
    textSecondary: '#93a1a1',
    textMuted: '#586e75',
    textTertiary: '#657b83',
    textPlaceholder: '#586e75',
    accent: '#268bd2',
    accentHover: '#2aa198',
    serviceTypeBg: '#3389c5',
    serviceTypeBorder: '#266794',
    serviceTypeFont: '#f3f5f7',
    instanceBg: '#44b460',
    instanceBorder: '#338848',
    instanceFont: '#f3f6f4',
    hostBg: '#b47c44',
    hostBorder: '#885d33',
    hostFont: '#f6f5f3',
    addressBg: '#8f44b4',
    addressBorder: '#6c3388',
    addressFont: '#f5f3f6',
    edgeTypeInstance: '#586e75',
    edgeInstanceHost: '#657b83',
    edgeHostAddress: '#6c71c4',
    offlineBg: '#073642',
    offlineBorder: '#586e75',
    offlineFont: '#657b83',
  },
}

const solarizedLight: ThemePreset = {
  name: 'solarized-light',
  label: 'Solarized Light',
  colors: {
    bgPrimary: '#fdf6e3',
    bgSecondary: '#eee8d5',
    bgTertiary: '#fdf6e3',
    borderPrimary: '#eee8d5',
    borderAccent: '#268bd2',
    textPrimary: '#657b83',
    textSecondary: '#586e75',
    textMuted: '#93a1a1',
    textTertiary: '#839496',
    textPlaceholder: '#93a1a1',
    accent: '#268bd2',
    accentHover: '#2aa198',
    serviceTypeBg: '#4486b4',
    serviceTypeBorder: '#336588',
    serviceTypeFont: '#263b4a',
    instanceBg: '#4dab64',
    instanceBorder: '#3a814b',
    instanceFont: '#294731',
    hostBg: '#ab7c4d',
    hostBorder: '#815d3a',
    hostFont: '#473829',
    addressBg: '#8c4dab',
    addressBorder: '#693a81',
    addressFont: '#3d2947',
    edgeTypeInstance: '#93a1a1',
    edgeInstanceHost: '#839496',
    edgeHostAddress: '#6c71c4',
    offlineBg: '#eee8d5',
    offlineBorder: '#93a1a1',
    offlineFont: '#586e75',
  },
}

const catppuccinLatte: ThemePreset = {
  name: 'catppuccin-latte',
  label: 'Catppuccin Latte',
  colors: {
    bgPrimary: '#eff1f5',
    bgSecondary: '#e6e9ef',
    bgTertiary: '#ccd0da',
    borderPrimary: '#9ca0b0',
    borderAccent: '#1e66f5',
    textPrimary: '#4c4f69',
    textSecondary: '#5c5f77',
    textMuted: '#7c7f93',
    textTertiary: '#8c8fa1',
    textPlaceholder: '#9ca0b0',
    accent: '#1e66f5',
    accentHover: '#8839ef',
    serviceTypeBg: '#4472cf',
    serviceTypeBorder: '#2c56aa',
    serviceTypeFont: '#21304f',
    instanceBg: '#4ec56c',
    instanceBorder: '#35a150',
    instanceFont: '#244c2e',
    hostBg: '#c58a4e',
    hostBorder: '#a16b35',
    hostFont: '#4c3824',
    addressBg: '#9d4ec5',
    addressBorder: '#7d35a1',
    addressFont: '#3f244c',
    edgeTypeInstance: '#9ca0b0',
    edgeInstanceHost: '#7c7f93',
    edgeHostAddress: '#8839ef',
    offlineBg: '#acb0be',
    offlineBorder: '#9ca0b0',
    offlineFont: '#6c6f85',
  },
}

const catppuccinFrappe: ThemePreset = {
  name: 'catppuccin-frappe',
  label: 'Catppuccin Frappe',
  colors: {
    bgPrimary: '#303446',
    bgSecondary: '#292c3c',
    bgTertiary: '#414559',
    borderPrimary: '#51576d',
    borderAccent: '#8caaee',
    textPrimary: '#c6d0f5',
    textSecondary: '#b5bfe2',
    textMuted: '#838ba7',
    textTertiary: '#949cbb',
    textPlaceholder: '#737994',
    accent: '#8caaee',
    accentHover: '#ca9ee6',
    serviceTypeBg: '#93ade7',
    serviceTypeBorder: '#6187dc',
    serviceTypeFont: '#f3f4f7',
    instanceBg: '#9dddad',
    instanceBorder: '#70cd87',
    instanceFont: '#f3f6f4',
    hostBg: '#ddbd9d',
    hostBorder: '#cd9e70',
    hostFont: '#f6f5f3',
    addressBg: '#c89ddd',
    addressBorder: '#af70cd',
    addressFont: '#f5f3f6',
    edgeTypeInstance: '#838ba7',
    edgeInstanceHost: '#949cbb',
    edgeHostAddress: '#ca9ee6',
    offlineBg: '#51576d',
    offlineBorder: '#626880',
    offlineFont: '#a5adce',
  },
}

const catppuccinMacchiato: ThemePreset = {
  name: 'catppuccin-macchiato',
  label: 'Catppuccin Macchiato',
  colors: {
    bgPrimary: '#24273a',
    bgSecondary: '#1e2030',
    bgTertiary: '#363a4f',
    borderPrimary: '#494d64',
    borderAccent: '#8aadf4',
    textPrimary: '#cad3f5',
    textSecondary: '#b8c0e0',
    textMuted: '#8087a2',
    textTertiary: '#939ab7',
    textPlaceholder: '#6e738d',
    accent: '#8aadf4',
    accentHover: '#c6a0f6',
    serviceTypeBg: '#92b0ec',
    serviceTypeBorder: '#5e8ae3',
    serviceTypeFont: '#f3f4f7',
    instanceBg: '#9de1ae',
    instanceBorder: '#6ed387',
    instanceFont: '#f3f6f4',
    hostBg: '#e1bf9d',
    hostBorder: '#d3a06e',
    hostFont: '#f6f5f3',
    addressBg: '#ca9de1',
    addressBorder: '#b16ed3',
    addressFont: '#f5f3f6',
    edgeTypeInstance: '#8087a2',
    edgeInstanceHost: '#939ab7',
    edgeHostAddress: '#c6a0f6',
    offlineBg: '#494d64',
    offlineBorder: '#5b6078',
    offlineFont: '#a5adcb',
  },
}

const catppuccinMocha: ThemePreset = {
  name: 'catppuccin-mocha',
  label: 'Catppuccin Mocha',
  colors: {
    bgPrimary: '#1e1e2e',
    bgSecondary: '#181825',
    bgTertiary: '#313244',
    borderPrimary: '#45475a',
    borderAccent: '#89b4fa',
    textPrimary: '#cdd6f4',
    textSecondary: '#bac2de',
    textMuted: '#7f849c',
    textTertiary: '#9399b2',
    textPlaceholder: '#6c7086',
    accent: '#89b4fa',
    accentHover: '#cba6f7',
    serviceTypeBg: '#91b6f2',
    serviceTypeBorder: '#5a92ec',
    serviceTypeFont: '#f2f4f7',
    instanceBg: '#9de6af',
    instanceBorder: '#6cda87',
    instanceFont: '#f3f7f4',
    hostBg: '#e6c29d',
    hostBorder: '#daa46c',
    hostFont: '#f7f5f3',
    addressBg: '#ce9de6',
    addressBorder: '#b66cda',
    addressFont: '#f5f3f7',
    edgeTypeInstance: '#7f849c',
    edgeInstanceHost: '#9399b2',
    edgeHostAddress: '#cba6f7',
    offlineBg: '#45475a',
    offlineBorder: '#585b70',
    offlineFont: '#a6adc8',
  },
}

const dracula: ThemePreset = {
  name: 'dracula',
  label: 'Dracula',
  colors: {
    bgPrimary: '#282a36',
    bgSecondary: '#343746',
    bgTertiary: '#44475a',
    borderPrimary: '#44475a',
    borderAccent: '#bd93f9',
    textPrimary: '#f8f8f2',
    textSecondary: '#e2e4f0',
    textMuted: '#6272a4',
    textTertiary: '#8b9cc4',
    textPlaceholder: '#6272a4',
    accent: '#bd93f9',
    accentHover: '#ff79c6',
    serviceTypeBg: '#be9bf1',
    serviceTypeBorder: '#9b65e9',
    serviceTypeFont: '#f4f2f7',
    instanceBg: '#a5e7b5',
    instanceBorder: '#75da8d',
    instanceFont: '#f3f7f4',
    hostBg: '#e7c6a5',
    hostBorder: '#daa775',
    hostFont: '#f7f5f3',
    addressBg: '#d1a5e7',
    addressBorder: '#b875da',
    addressFont: '#f5f3f7',
    edgeTypeInstance: '#6272a4',
    edgeInstanceHost: '#8b9cc4',
    edgeHostAddress: '#bd93f9',
    offlineBg: '#44475a',
    offlineBorder: '#6272a4',
    offlineFont: '#8b9cc4',
  },
}

const nord: ThemePreset = {
  name: 'nord',
  label: 'Nord',
  colors: {
    bgPrimary: '#2e3440',
    bgSecondary: '#3b4252',
    bgTertiary: '#434c5e',
    borderPrimary: '#4c566a',
    borderAccent: '#88c0d0',
    textPrimary: '#eceff4',
    textSecondary: '#e5e9f0',
    textMuted: '#8f9bb0',
    textTertiary: '#81a1c1',
    textPlaceholder: '#616e88',
    accent: '#88c0d0',
    accentHover: '#8fbcbb',
    serviceTypeBg: '#8dbdcb',
    serviceTypeBorder: '#63a5b8',
    serviceTypeFont: '#f4f5f6',
    instanceBg: '#95c3a0',
    instanceBorder: '#6ead7d',
    instanceFont: '#f4f6f4',
    hostBg: '#c3ac95',
    hostBorder: '#ad8d6e',
    hostFont: '#f6f5f4',
    addressBg: '#b495c3',
    addressBorder: '#986ead',
    addressFont: '#f5f4f6',
    edgeTypeInstance: '#4c566a',
    edgeInstanceHost: '#8f9bb0',
    edgeHostAddress: '#b48ead',
    offlineBg: '#4c566a',
    offlineBorder: '#616e88',
    offlineFont: '#8f9bb0',
  },
}

const tokyoNight: ThemePreset = {
  name: 'tokyo-night',
  label: 'Tokyo Night',
  colors: {
    bgPrimary: '#1a1b26',
    bgSecondary: '#24283b',
    bgTertiary: '#414868',
    borderPrimary: '#414868',
    borderAccent: '#7aa2f7',
    textPrimary: '#c0caf5',
    textSecondary: '#a9b1d6',
    textMuted: '#787c99',
    textTertiary: '#565f89',
    textPlaceholder: '#565f89',
    accent: '#7aa2f7',
    accentHover: '#bb9af7',
    serviceTypeBg: '#83a5ee',
    serviceTypeBorder: '#4d7ee7',
    serviceTypeFont: '#f2f4f7',
    instanceBg: '#90e1a4',
    instanceBorder: '#60d47d',
    instanceFont: '#f3f7f4',
    hostBg: '#e1b890',
    hostBorder: '#d49960',
    hostFont: '#f7f5f3',
    addressBg: '#c690e1',
    addressBorder: '#ad60d4',
    addressFont: '#f5f3f7',
    edgeTypeInstance: '#565f89',
    edgeInstanceHost: '#787c99',
    edgeHostAddress: '#bb9af7',
    offlineBg: '#414868',
    offlineBorder: '#565f89',
    offlineFont: '#787c99',
  },
}

const gruvboxDark: ThemePreset = {
  name: 'gruvbox-dark',
  label: 'Gruvbox Dark',
  colors: {
    bgPrimary: '#282828',
    bgSecondary: '#32302f',
    bgTertiary: '#3c3836',
    borderPrimary: '#504945',
    borderAccent: '#fabd2f',
    textPrimary: '#ebdbb2',
    textSecondary: '#d5c4a1',
    textMuted: '#a89984',
    textTertiary: '#928374',
    textPlaceholder: '#928374',
    accent: '#fabd2f',
    accentHover: '#fe8019',
    serviceTypeBg: '#ebb73e',
    serviceTypeBorder: '#d69c16',
    serviceTypeFont: '#f7f6f2',
    instanceBg: '#53d674',
    instanceBorder: '#2dbe52',
    instanceFont: '#f3f7f4',
    hostBg: '#d69453',
    hostBorder: '#be752d',
    hostFont: '#f7f5f3',
    addressBg: '#aa53d6',
    addressBorder: '#8e2dbe',
    addressFont: '#f5f3f7',
    edgeTypeInstance: '#928374',
    edgeInstanceHost: '#a89984',
    edgeHostAddress: '#d3869b',
    offlineBg: '#504945',
    offlineBorder: '#665c54',
    offlineFont: '#a89984',
  },
}

export const themes: ThemePreset[] = [
  dark,
  light,
  solarizedDark,
  solarizedLight,
  catppuccinLatte,
  catppuccinFrappe,
  catppuccinMacchiato,
  catppuccinMocha,
  dracula,
  nord,
  tokyoNight,
  gruvboxDark,
]

export const defaultTheme: ThemeName = 'system'

export function getThemeByName(name: string): ThemePreset {
  return themes.find((t) => t.name === name) ?? themes[0]
}

export const cssVarMap: Record<keyof ThemeColors, string> = {
  bgPrimary: '--bg-primary',
  bgSecondary: '--bg-secondary',
  bgTertiary: '--bg-tertiary',
  borderPrimary: '--border-primary',
  borderAccent: '--border-accent',
  textPrimary: '--text-primary',
  textSecondary: '--text-secondary',
  textMuted: '--text-muted',
  textTertiary: '--text-tertiary',
  textPlaceholder: '--text-placeholder',
  accent: '--accent',
  accentHover: '--accent-hover',
  serviceTypeBg: '--service-type-bg',
  serviceTypeBorder: '--service-type-border',
  serviceTypeFont: '--service-type-font',
  instanceBg: '--instance-bg',
  instanceBorder: '--instance-border',
  instanceFont: '--instance-font',
  hostBg: '--host-bg',
  hostBorder: '--host-border',
  hostFont: '--host-font',
  addressBg: '--address-bg',
  addressBorder: '--address-border',
  addressFont: '--address-font',
  edgeTypeInstance: '--edge-type-instance',
  edgeInstanceHost: '--edge-instance-host',
  edgeHostAddress: '--edge-host-address',
  offlineBg: '--offline-bg',
  offlineBorder: '--offline-border',
  offlineFont: '--offline-font',
}

const darkThemeNames = new Set<string>([
  'dark',
  'solarized-dark',
  'catppuccin-frappe',
  'catppuccin-macchiato',
  'catppuccin-mocha',
  'dracula',
  'nord',
  'tokyo-night',
  'gruvbox-dark',
])

export function isDarkTheme(name: string): boolean {
  return darkThemeNames.has(name)
}
