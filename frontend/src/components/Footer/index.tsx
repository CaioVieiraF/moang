import { NavLink } from 'react-router-dom'
import { FooterContainer } from './styles'

export function Footer() {
  return (
    <FooterContainer>
      <h2>
        <NavLink to="about">
          Sobre
        </NavLink>
      </h2>
    </FooterContainer>
  )
}
