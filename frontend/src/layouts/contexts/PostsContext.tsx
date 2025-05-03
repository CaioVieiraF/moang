import { createContext, ReactNode, useState } from 'react'

interface IPostsContext {
  title: string,
  body: string,
  setPostBody: (value: string) => void
  setPostTitle: (value: string) => void
}

export const PostsContext = createContext({} as IPostsContext)

interface PostsContextProviderProps {
  children: ReactNode
}

export function PostsContextProvider({ children }: PostsContextProviderProps) {
  const [title, setTitle] = useState('')
  const [body, setBody] = useState('')

  const setPostBody = (value: string) => {
    setBody(value)
  }

  const setPostTitle = (value: string) => {
    setTitle(value)
  }

  return (
    <PostsContext.Provider value={{ title, body, setPostBody, setPostTitle }}>
      {children}
    </PostsContext.Provider>
  )
}
