import { API_BASE_URL } from './axios'

/** Username whose outbox and objects this frontend renders. */
export const BLOG_USERNAME = import.meta.env.VITE_BLOG_USERNAME || 'caio'

/** Builds `users/{username}/...` paths from a single place. */
export function userEndpoint(suffix: string = '') {
  return `users/${BLOG_USERNAME}${suffix}`
}

/** The backend serves the feed on the API origin. */
export const FEED_URL = `${API_BASE_URL}/feed.xml`
