import { Footer } from '../../components/Footer'
import { Header } from '../../components/Header'
import { LayoutContainer } from './styles'
import { Outlet } from 'react-router-dom'

export function DefaultLayout() {
  return (
    <LayoutContainer>
      <Header />
      <Outlet />
      <Footer />
    </LayoutContainer>
  )
}
