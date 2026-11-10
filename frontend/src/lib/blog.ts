import { API_BASE_URL } from './axios'

/** Username whose outbox and objects this frontend renders. */
export const BLOG_USERNAME = import.meta.env.VITE_BLOG_USERNAME || 'caio'

/** Builds `users/{username}/...` paths from a single place. */
export function userEndpoint(suffix: string = '') {
  return `users/${BLOG_USERNAME}${suffix}`
}

/** The backend serves the feed on the API origin. */
export const FEED_URL = `${API_BASE_URL}/feed.xml`

/** A `Create` activity returned by the outbox, as consumed by the frontend. */
export interface PostObject {
  id: string,
  name: string,
  object: {
    id: string
    type?: string
    content: string
    published?: string
    updated?: string
    to?: string[]
    cc?: string[]
    source?: {
      content: string
      mediaType: string
    }
    likes?: {
      totalItems: number
    }
    shares?: {
      totalItems: number
    }
    replies?: {
      totalItems: number
    }
  },
  published: Date
}

export interface OrderedCollectionPage {
  totalItems: number,
  next?: string,
  prev?: string,
  orderedItems: PostObject[]
}
