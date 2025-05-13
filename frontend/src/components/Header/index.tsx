import { HeaderContainer, Logo } from './styles'
import { NavLink } from 'react-router-dom'

export function Header() {
  return (
    <HeaderContainer>
      <Logo to="/">Moang</Logo>
      <nav>
        <NavLink to="/about" title="about">Sobre</NavLink>
      </nav>
    </HeaderContainer>
  )
}
