import axios from 'axios'

export const API_BASE_URL = import.meta.env.VITE_API_BASEURL || window.location.origin

export const api = axios.create({
  baseURL: API_BASE_URL,
})
