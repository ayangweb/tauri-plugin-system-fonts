import { invoke } from "@tauri-apps/api/core";

export const COMMAND = {
  GET_SYSTEM_FONTS: "plugin:system-fonts|get_system_fonts",
};

export interface Font {
  id: string;
  name: string;
  fontName: string;
  path: string;
  weight: number;
  style: string;
  monospaced: boolean;
}

/**
 * Get all fonts installed on the system.
 *
 * @example
 * import { getSystemFonts } from "tauri-plugin-system-fonts-api";
 *
 * const fonts = await getSystemFonts();
 */
export const getSystemFonts = () => {
  return invoke<Font[]>(COMMAND.GET_SYSTEM_FONTS);
};
