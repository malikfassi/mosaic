import * as core from '@actions/core';
import * as cache from '@actions/cache';
import * as crypto from 'crypto';

const CACHE_KEY_PREFIX = 'gist-cache';
const CACHE_TTL = 1000 * 60 * 60; // 1 hour in milliseconds

// Helper to generate cache key
function getCacheKey(gistId, componentHash) {
  const hash = crypto.createHash('sha256')
    .update(`${gistId}:${componentHash}`)
    .digest('hex');
  return `${CACHE_KEY_PREFIX}-${hash}`;
}

// Helper to check if cache is still valid
function isCacheValid(cacheData) {
  if (!cacheData || !cacheData.timestamp) return false;
  const age = Date.now() - cacheData.timestamp;
  return age < CACHE_TTL;
}

// Save data to cache
export async function saveToCache(gistId, componentHash, data) {
  try {
    const cacheKey = getCacheKey(gistId, componentHash);
    const cacheData = {
      data,
      timestamp: Date.now()
    };
    await cache.saveCache([cacheKey], cacheData);
    core.debug(`Saved data to cache with key ${cacheKey}`);
  } catch (error) {
    core.warning(`Failed to save to cache: ${error.message}`);
  }
}

// Load data from cache
export async function loadFromCache(gistId, componentHash) {
  try {
    const cacheKey = getCacheKey(gistId, componentHash);
    const cacheData = await cache.restoreCache([cacheKey]);
    
    if (cacheData && isCacheValid(cacheData)) {
      core.debug(`Loaded data from cache with key ${cacheKey}`);
      return cacheData.data;
    }
    
    core.debug(`No valid cache found for key ${cacheKey}`);
    return null;
  } catch (error) {
    core.warning(`Failed to load from cache: ${error.message}`);
    return null;
  }
}

// Clear expired cache entries
export async function clearExpiredCache() {
  try {
    const cacheKeys = await cache.restoreKeys(CACHE_KEY_PREFIX);
    for (const key of cacheKeys) {
      const cacheData = await cache.restoreCache([key]);
      if (cacheData && !isCacheValid(cacheData)) {
        await cache.deleteCache([key]);
        core.debug(`Cleared expired cache with key ${key}`);
      }
    }
  } catch (error) {
    core.warning(`Failed to clear expired cache: ${error.message}`);
  }
} 