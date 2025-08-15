import { invoke } from '@tauri-apps/api/core'

export interface PingRequest {
  value?: string;
}

export interface PingResponse {
  value?: string;
}

export interface AuthRequest {
  url: string;
  callbackUrlScheme: string;
}

export interface AuthResponse {
  success: boolean;
  callbackUrl?: string;
  error?: string;
}

export async function ping(value?: string): Promise<string | null> {
  return await invoke<PingResponse>('plugin:plauth|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function authenticate(request: AuthRequest): Promise<AuthResponse> {
  return await invoke<AuthResponse>('plugin:plauth|authenticate', {
    payload: request,
  });
}
