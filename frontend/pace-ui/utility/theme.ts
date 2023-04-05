import { fetchTheme } from '../apis/api';

export type ThemeVars = { [key: string]: string };
let themeVars: ThemeVars;
let themeVarsLoaded = false;
let themeVarsLoading = false;

export function getThemeVar(key: string): string {
  if (!themeVars && !themeVarsLoaded) {
    throw new Error('Theme vars were not loaded yet');
  }

  const value = themeVars[key];
  if (value === undefined) {
    throw new Error(`Theme vars "${key}" is not defined`);
  }
  return value;
}

export async function initTheme(): Promise<boolean> {
  if (themeVarsLoaded || themeVarsLoading) {
    return false;
  }
  themeVarsLoading = true;
  const response = await fetchTheme();
  if (response.status !== 200) {
    throw new Error('Could not fetch theme from backend');
  }
  themeVars = response.data;
  themeVarsLoaded = true;
  themeVarsLoading = false;
  return true;
}
