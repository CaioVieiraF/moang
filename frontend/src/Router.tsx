import { Navigate, Route, Routes } from 'react-router-dom'
import { DefaultLayout } from './layouts/DefaultLayout'
import { About } from './pages/About'
import { Home } from './pages/Home'
import { Post } from './pages/Post'

export function Router() {
  return (
    <Routes>
      <Route path="/" element={<DefaultLayout />}>
        <Route path="/about" element={<About />} />
        <Route path="/" element={<Home />} />
        <Route path="/posts" element={<Navigate to="/" replace />} />
        <Route path="/post/:id" element={<Post />} />
      </Route>
    </Routes>
  )
}
