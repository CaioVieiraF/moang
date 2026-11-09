import { Link } from 'react-router-dom'
import { BannerContainer } from './styles'

export function Banner() {
  return (
    <BannerContainer>
      <h1>Bem-vindo ao meu blog</h1>
      <p>
        Um espaço para registrar ideias sobre desenvolvimento de software,
        redes de computadores, jogos e outras coisas de tecnologia — às vezes
        em artigos mais longos, às vezes em notas rápidas.
      </p>
      <Link to="/about">Conheça melhor minha história →</Link>
    </BannerContainer>
  )
}
