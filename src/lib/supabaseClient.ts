import { createClient, type SupabaseClient } from '@supabase/supabase-js';

// globalThis에 캐시를 올려 HMR 재로드 시에도 인스턴스를 재사용
const g = globalThis as any;
if (!g.__sbCache) g.__sbCache = new Map<string, SupabaseClient>();
const _cache: Map<string, SupabaseClient> = g.__sbCache;

// Google OAuth 세션 무시 — anon 역할 강제 (RLS: auth.uid ≠ device_id 충돌 방지)
const ANON_OPTIONS = {
  auth: { persistSession: false, autoRefreshToken: false, detectSessionInUrl: false },
} as const;

export function getSupabaseClient(url: string, key: string): SupabaseClient {
  const cacheKey = `${url}::${key}`;
  if (!_cache.has(cacheKey)) {
    _cache.set(cacheKey, createClient(url, key, ANON_OPTIONS));
  }
  return _cache.get(cacheKey)!;
}
